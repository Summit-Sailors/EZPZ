import json
from uuid import uuid4

import polars as pl

from painlezz_spiderz.data_schema import info_schema, vulnerabilities_schema
from pysilo_database.models.db_models import (
  Urls,
  Dists,
  Extras,
  Release,
  Keywords,
  Downloads,
  Classifiers,
  Maintainers,
  PypiPackage,
  ReleaseDigest,
  Vulnerability,
  ReleaseVersion,
  DistsToPypiPackage,
  ExtraaToPypiPackage,
  KeywordsToPypiPackage,
  ClassifierToPypiPackage,
  MaintainersToPypiPackage,
)

undesired_str = ["", "UNKNOWN", "email@example.com", "your name", "you@example.com", "maintainer.email@example.com", "maintainer name"]


def write_database(lf: tuple[pl.LazyFrame, ...]) -> None:
  (
    classifiers_lf,
    downloads_lf,
    package_lf,
    maintainers_lf,
    extras_lf,
    keywords_lf,
    dist_lf,
    urls_lf,
    release_versions_lf,
    releases_lf,
    releases_digests_lf,
    vulnerabilities_lf,
  ) = lf

  classifier_to_pypiPackage_lf = classifiers_lf.select(pl.col("id").alias("classifier_id"), "package_id")
  maintainer_to_pypiPackage_lf = maintainers_lf.select(pl.col("id").alias("maintainer_id"), "package_id")
  extra_to_pypiPackage_lf = extras_lf.select(pl.col("id").alias("extras_id"), "package_id")
  keyword_to_pypiPackage_lf = keywords_lf.select(pl.col("id").alias("keyword_id"), "package_id")
  dist_to_pypiPackage_lf = dist_lf.select(pl.col("id").alias("dist_id"), "package_id")

  PypiPackage.write_df(package_lf)
  Urls.write_df(urls_lf)
  Downloads.write_df(downloads_lf)
  Classifiers.write_df(classifiers_lf)
  Vulnerability.write_df(vulnerabilities_lf)
  Maintainers.write_df(maintainers_lf)
  Keywords.write_df(keywords_lf)
  Extras.write_df(extras_lf)
  Dists.write_df(dist_lf)

  ClassifierToPypiPackage.write_df(classifier_to_pypiPackage_lf)
  MaintainersToPypiPackage.write_df(maintainer_to_pypiPackage_lf)
  ExtraaToPypiPackage.write_df(extra_to_pypiPackage_lf)
  KeywordsToPypiPackage.write_df(keyword_to_pypiPackage_lf)
  DistsToPypiPackage.write_df(dist_to_pypiPackage_lf)
  ReleaseVersion.write_df(release_versions_lf)
  Release.write_df(releases_lf)
  ReleaseDigest.write_df(releases_digests_lf)


def generate_uuids(ls: pl.LazyFrame, column: str, alias: str) -> pl.LazyFrame:
  return ls.with_columns(
    pl.when(pl.col(column).is_null())
    .then(
      pl.format(
        "{}-{}-{}-{}-{}", pl.lit(uuid4().hex[:8]), pl.lit(uuid4().hex[8:12]), pl.lit(uuid4().hex[12:16]), pl.lit(uuid4().hex[16:20]), pl.lit(uuid4().hex[20:])
      ).alias(alias)
    )
    .otherwise(pl.col(column))
    .alias(alias)
  )


def process_package_items(
  model: type[Release]
  | type[PypiPackage]
  | type[Classifiers]
  | type[Maintainers]
  | type[Keywords]
  | type[Extras]
  | type[Dists]
  | type[Urls]
  | type[ReleaseVersion]
  | type[ReleaseDigest],
  table: str,
  column: str,
  lf: pl.LazyFrame,
) -> pl.LazyFrame:
  allowed_tables = {"releasetopyipackage", "extraatopypipackage", "diststopypipackage", "classifiertopypipackage"}
  allowed_columns = {"keyword_id", "extras_id", "dist_id", "classifier_id"}

  if table not in allowed_tables or column not in allowed_columns:
    raise ValueError

  query = f"""
        SELECT package_id, {column}
        FROM {table}
    """
  result = model.read_df(query)  # type: ignore

  if not result.is_empty():
    lf = lf.join(result.lazy(), left_on="package_id", right_on="package_id", how="left")
    lf = generate_uuids(lf, "id", "id")
  else:
    num_rows = lf.select(pl.len()).collect().item()
    lf = lf.with_columns(pl.Series(name="id", values=[str(uuid4()) for _ in range(num_rows)]))

  return lf


def process_batch(lf: pl.LazyFrame) -> tuple[pl.LazyFrame, ...]:
  query = """
    SELECT id, slug
    FROM pypipackage
    """
  packages_ids = PypiPackage.read_df(query)  # type: ignore

  # ######## raw data transformation ########
  pypi_lf = lf.with_columns(
    [pl.col("json_data").str.json_path_match(f"$.{field}").alias(field) for field in ["info", "last_serial", "releases", "urls", "vulnerabilities"]],
    slug=pl.col("package_name").str.split(" ").list.join("-"),
  ).drop("json_data")

  if not packages_ids.is_empty():
    pypi_lf = pypi_lf.join(packages_ids.lazy(), left_on="slug", right_on="slug", how="left")
    pypi_lf = generate_uuids(pypi_lf, "id", "package_id")
  else:
    pypi_lf = pypi_lf.with_columns(pl.Series(name="package_id", values=[str(uuid4()) for _ in range(len(lf.collect()))]))

  # ######## data normalization ########
  pypi_normalized = (
    pypi_lf.select(
      pl.col("package_id"), pl.col("package_name"), pl.col("slug"), pl.col("urls").map_elements(json.loads), pl.col("info").str.json_decode(info_schema)
    )
    .unnest("info")
    .unnest("downloads")
    .with_columns(
      pl.when((pl.col(pl.Utf8).is_null()) | (pl.col(pl.Utf8).str.strip_chars().is_in(undesired_str)))
      .then(pl.lit(None))
      .otherwise(pl.col(pl.Utf8).str.strip_chars())
      .name.keep(),
    )
    .with_columns(
      pl.col("package_name").alias("title"),
      pl.when((pl.col("author").is_null()) & (pl.col("author_email").is_not_null()))
      .then(pl.col("author_email").str.replace(r"\s*<.+>", "").replace(r"[^\s]\w+?@[\w+?\.]+", "").str.strip_chars())
      .otherwise(pl.col("author"))
      .alias("author"),
      pl.when(pl.col("author_email").is_not_null())
      .then(pl.col("author_email").str.split(" ").list.last().str.replace_all(r"<|>", "").str.strip_chars())
      .otherwise(pl.col("author_email"))
      .alias("author_email"),
    )
  )

  # ######## URL extraction and joining ########
  pypi_normalized = (
    pypi_normalized.join(
      pypi_lf.select("urls", "package_name")
      .with_columns(pl.col("urls").map_elements(json.loads))
      .explode("urls")
      .unnest("urls")
      .collect()
      .select(pl.col("upload_time", "upload_time_iso_8601").str.to_datetime(), "package_name")
      .lazy(),
      left_on="title",
      right_on="package_name",
      how="left",
    )
    .drop("urls")
    .unique("slug", keep="first")
  )

  vulnerabilities_lf = (
    pypi_lf.select("package_id", "package_name", "slug", "vulnerabilities")
    .with_columns(pl.col("vulnerabilities").str.json_decode(vulnerabilities_schema), pl.col("package_name").alias("title"))
    .explode("vulnerabilities")
    .filter(pl.col("vulnerabilities").is_not_null())
    .unnest("vulnerabilities")
    .drop("package_name")
  )
  downloads_lf = pypi_normalized.select("package_id", "title", "slug", "last_month", "last_week", "last_day")
  package_lf = package(pypi_normalized)
  classifiers_lf = classifiers(pypi_normalized)
  maintainers_lf = maintainers(pypi_normalized)
  extras_lf = extras(pypi_normalized)
  keywords_lf = keywords(pypi_normalized)
  dist_lf = dists(pypi_normalized)
  urls_lf = urls(pypi_normalized)
  (release_versions_lf, releases_digests_lf, releases_lf) = releases(pypi_lf)

  return (
    classifiers_lf,
    downloads_lf,
    package_lf,
    maintainers_lf,
    extras_lf,
    keywords_lf,
    dist_lf,
    urls_lf,
    release_versions_lf,
    releases_lf,
    releases_digests_lf,
    vulnerabilities_lf,
  )


################## packages details ######################
def package(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  return pypi_normalized.select(
    pl.col("package_id").alias("id"),
    "title",
    "slug",
    "author",
    "author_email",
    "description",
    "description_content_type",
    "license",
    "requires_python",
    "summary",
    "version",
    "platform",
    "upload_time",
    "upload_time_iso_8601",
    "yanked",
    "yanked_reason",
  )


################## maintainers details ######################
def maintainers(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  maintainers_lf = (
    pypi_normalized.select("package_id", "title", "slug", "maintainer", "maintainer_email")
    .with_columns(
      pl.col("maintainer").str.replace_all(r"'|\"|\(|\)", ""),
      pl.col("maintainer_email").str.to_lowercase().str.replace_all(r"'|\"|\(|\)", ""),
    )
    .with_columns(
      pl.when((pl.col("maintainer").is_null()) & (pl.col("maintainer_email").is_not_null()))
      .then(pl.col("maintainer_email").str.replace_all(r"<[^>]+>", "").str.split(",").list.eval(pl.element().str.strip_chars()).list.join(","))
      .otherwise(pl.col("maintainer"))
      .alias("maintainer"),
      pl.when(pl.col("maintainer_email").is_not_null())
      .then(
        pl.when(pl.col("maintainer_email").str.extract_all(r"<[^>]+>") != [])
        .then(pl.col("maintainer_email").str.extract_all(r"<[^>]+>").list.eval(pl.element().str.replace_all(r"<|>", "").str.strip_chars()).list.join(","))
        .otherwise(pl.col("maintainer_email"))
      )
      .otherwise(pl.col("maintainer_email"))
      .alias("maintainer_email"),
    )
    .with_columns(
      pl.when(pl.col("maintainer").is_not_null()).then(pl.col("maintainer").str.split(",").list.eval(pl.element().str.strip_chars())).otherwise(pl.lit([])),
      pl.when(pl.col("maintainer_email").is_not_null())
      .then(pl.col("maintainer_email").str.split(",").list.eval(pl.element().str.strip_chars()))
      .otherwise(pl.lit([])),
    )
    .with_columns(
      pl.col("maintainer").list.eval(pl.when(pl.element() == "").then(pl.lit(None)).otherwise(pl.element())),
      pl.col("maintainer_email").list.eval(pl.when(pl.element() == "").then(pl.lit(None)).otherwise(pl.element())),
    )
    .with_columns(
      pl.col("maintainer", "maintainer_email")
      .list.gather(pl.int_range(pl.col("maintainer", "maintainer_email").list.len().max()), null_on_oob=True)
      .list.eval(pl.element().forward_fill(None))
    )
    .explode("maintainer", "maintainer_email")
    .sort("title")
    .unique()
  ).filter(pl.col("maintainer").is_not_null())

  try:
    return process_package_items(Maintainers, "maintainerstopypipackage", "maintainer_id", maintainers_lf)
  except Exception:
    return pl.LazyFrame({"id": [], "package_id": [], "maintainer": [], "maintainer_email": []})


################## keywords details ######################
def keywords(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  keywords_lf = (
    pypi_normalized.select("package_id", "title", "slug", "keywords")
    .with_columns(
      pl.when((pl.col("keywords").is_null()) | (pl.col("keywords").str.strip_chars().is_in(undesired_str)))
      .then(pl.lit(None))
      .otherwise(pl.col("keywords").str.to_lowercase().str.split(",").list.eval(pl.element().str.strip_chars()).list.unique())
      .alias("keywords")
    )
    .with_columns(
      pl.when(pl.col("keywords").list.len() == 1)
      .then(pl.col("keywords").list.get(0).str.split(" ").list.eval(pl.element().str.strip_chars()).list.unique())
      .otherwise(pl.col("keywords"))
      .alias("name"),
    )
    .explode("name")
    .drop("keywords")
  ).filter(pl.col("name").is_not_null())
  return process_package_items(Keywords, "keywordstopypipackage", "keyword_id", keywords_lf)


################## extras details ######################
def extras(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  extras_lf = (
    pypi_normalized.select("package_id", "title", "slug", "provides_extra")
    .with_columns(
      pl.when((pl.col("provides_extra").is_null()) | (pl.col("provides_extra").str.strip_chars().is_in(undesired_str)))
      .then(pl.lit(None))
      .otherwise(pl.col("provides_extra").str.to_lowercase().str.split(",").list.eval(pl.element().str.strip_chars()).list.unique())
      .alias("provides_extra")
    )
    .with_columns(
      pl.when(pl.col("provides_extra").list.len() == 1)
      .then(pl.col("provides_extra").list.get(0).str.split(" ").list.eval(pl.element().str.strip_chars()).list.unique())
      .otherwise(pl.col("provides_extra"))
      .alias("name"),
    )
    .explode("name")
    .drop("provides_extra")
  )
  extras_lf = process_package_items(Extras, "extraatopypipackage", "extras_id", extras_lf)

  return extras_lf.filter(pl.col("name").is_not_null())


################## dists details ######################
def dists(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  dist_lf = (
    pypi_normalized.select("package_id", "title", "slug", "requires_dist")
    .with_columns(
      pl.when((pl.col("requires_dist").is_null()) | (pl.col("requires_dist").str.strip_chars().is_in(undesired_str)))
      .then(pl.lit(None))
      .otherwise(pl.col("requires_dist").str.to_lowercase().str.split(",").list.eval(pl.element().str.strip_chars()).list.unique())
      .alias("requires_dist")
    )
    .with_columns(
      pl.when(pl.col("requires_dist").list.len() == 1)
      .then(pl.col("requires_dist").list.get(0).str.split(" ").list.eval(pl.element().str.strip_chars()).list.unique())
      .otherwise(pl.col("requires_dist"))
      .alias("name"),
    )
    .explode("name")
    .drop("requires_dist")
  ).filter(pl.col("name").is_not_null())
  return process_package_items(Dists, "diststopypipackage", "dist_id", dist_lf)


################## urls details ######################
def urls(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  package_urls = pypi_normalized.select(
    "package_id",
    "title",
    "slug",
    "bugtrack_url",
    "docs_url",
    "download_url",
    "home_page",
    "package_url",
    "project_url",
    "release_url",
    pl.col("project_urls").explode(),
  ).unnest("project_urls")

  def format_url(url: pl.Expr) -> pl.Expr:
    return pl.when(url.is_null() | url.str.contains(r"^(http|https)://")).then(url).otherwise(pl.format("http://{}", url.str.replace(r"^(http|https)://", "")))

  known_url_columns = ["docs_url", "home_page", "download_url", "release_url", "bugtrack_url", "package_url", "project_url"]

  for col in package_urls.collect_schema().names():
    if col in known_url_columns or col not in [
      "package_id",
      "title",
      "slug",
    ]:
      package_urls = package_urls.with_columns(format_url(pl.col(col)).alias(col))

  capital_columns: dict[str, str] = {
    "Documentation": "docs_url",
    "Homepage": "home_page",
    "Download": "download_url",
    "Release": "release_url",
    "Tracker": "bugtrack_url",
    "Repository": "repository_url",
    "Source": "source_url",
    "Changelog": "changelog_url",
  }

  expressions: list[pl.Expr] = []
  existing_columns = package_urls.collect_schema().names()

  for capital_col, lowercase_col in capital_columns.items():
    if capital_col in existing_columns:
      expressions.append(pl.coalesce(pl.col(lowercase_col), pl.col(capital_col)).alias(lowercase_col))
    elif lowercase_col in existing_columns:
      expressions.append(pl.col(lowercase_col).alias(lowercase_col))
    else:
      expressions.append(pl.lit(None).alias(lowercase_col))

  urls_lf = package_urls.with_columns(expressions)

  columns_to_drop = ["Documentation", "Homepage", "Download", "Repository", "Source", "Tracker", "Changelog"]

  urls_lf = process_package_items(Urls, "urls", "id", urls_lf)

  return urls_lf.drop([col for col in columns_to_drop if col in urls_lf.collect_schema().names()])


################## classifiers details ######################
def classifiers(pypi_normalized: pl.LazyFrame) -> pl.LazyFrame:
  classifiers_lf = (
    pypi_normalized.select("package_id", "title", "slug", "classifiers")
    .with_columns(pl.col("classifiers"))
    .explode("classifiers")
    .with_columns(
      pl.col("classifiers").str.split(" :: ").list.get(0, null_on_oob=True).str.to_lowercase().str.split(" ").list.join("_").alias("classifier_key"),
      pl.col("classifiers").str.split(" :: ").list.get(-1, null_on_oob=True).alias(name="classifier_value"),
    )
    .with_columns(pl.when(pl.col("classifier_key") == "programming_language").then(pl.lit("python_version")).otherwise(pl.col("classifier_key")).name.keep())
  ).filter(pl.col("classifiers").is_not_null())

  return process_package_items(Classifiers, "classifiertopypipackage", "classifier_id", classifiers_lf)


################## releases details ######################
def releases(pypi_lf: pl.LazyFrame) -> tuple[pl.LazyFrame, ...]:
  releases_versions_normalized = (
    pypi_lf.select(pl.col("package_id"), pl.col("package_name").alias("title"), pl.col("slug"), pl.col("releases").map_elements(json.loads))
    .unnest("releases")
    .unpivot(index=["package_id", "title", "slug"], variable_name="version", value_name="releases")
  )

  query = """
    SELECT rv.id AS release_version_id, rv.package_id AS package_id,
           r.id AS release_id, r.release_version_id AS release_version_id,
           rd.id AS release_digest_id, rd.release_id AS release_id
    FROM releaseversion rv
    LEFT JOIN release r ON rv.id = r.release_version_id
    LEFT JOIN releasedigest rd ON r.id = rd.release_id
    """
  joined_data = ReleaseVersion.read_df(query)  # type: ignore

  if not joined_data.is_empty():
    joined_data_lf = joined_data.lazy()

    releases_versions_normalized = releases_versions_normalized.join(joined_data_lf, left_on="package_id", right_on="package_id", how="left")

    releases_versions_normalized = generate_uuids(releases_versions_normalized, "release_version_id", "release_version_id")
    releases_versions_normalized = generate_uuids(releases_versions_normalized, "release_id", "release_id")
    releases_versions_normalized = generate_uuids(releases_versions_normalized, "release_digest_id", "release_digest_id")

  else:
    releases_versions_normalized = releases_versions_normalized.with_columns(
      pl.Series(name="release_version_id", values=[str(uuid4()) for _ in range(len(releases_versions_normalized.collect()))]),
      pl.Series(name="release_id", values=[str(uuid4()) for _ in range(len(releases_versions_normalized.collect()))]),
      pl.Series(name="release_digest_id", values=[str(uuid4()) for _ in range(len(releases_versions_normalized.collect()))]),
    )

  release_versions_lf = releases_versions_normalized.select("release_version_id", "package_id", "title", "slug", "version")
  releases_lf = (
    releases_versions_normalized.rename({"release_version_id": "id"})
    .explode("releases")
    .unnest("releases")
    .filter(pl.col("md5_digest").is_not_null())
    .lazy()
    .with_columns(pl.col("upload_time", "upload_time_iso_8601").str.to_datetime())
  )
  releases_digests_lf = releases_lf.collect().select(pl.col("release_id").alias("release_id"), "md5_digest", "digests").unnest("digests").lazy()

  return release_versions_lf, releases_digests_lf, releases_lf
