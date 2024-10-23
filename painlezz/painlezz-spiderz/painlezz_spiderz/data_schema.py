import polars as pl

info_schema = pl.Struct(
  {
    "author": pl.String,
    "author_email": pl.String,
    "bugtrack_url": pl.String,
    "classifiers": pl.List(pl.String),
    "description": pl.String,
    "description_content_type": pl.String,
    "docs_url": pl.String,
    "download_url": pl.String,
    "downloads": pl.Struct({"last_day": pl.Int64, "last_month": pl.Int64, "last_week": pl.Int64}),
    "dynamic": pl.String,
    "home_page": pl.String,
    "keywords": pl.String,
    "license": pl.String,
    "maintainer": pl.String,
    "maintainer_email": pl.String,
    "name": pl.String,
    "package_url": pl.String,
    "platform": pl.String,
    "project_url": pl.String,
    "project_urls": pl.Struct({"Download": pl.String, "Homepage": pl.String}),
    "provides_extra": pl.List(pl.String),
    "release_url": pl.String,
    "requires_dist": pl.List(pl.String),
    "requires_python": pl.String,
    "summary": pl.String,
    "version": pl.String,
    "yanked": pl.Boolean,
    "yanked_reason": pl.String,
  }
)

releases_schema = pl.Struct(
  {
    "comment_text": pl.String,
    "digests": pl.Struct({"blake2b_256": pl.String, "md5": pl.String, "sha256": pl.String}),
    "downloads": pl.Int64,
    "filename": pl.String,
    "has_sig": pl.Boolean,
    "md5_digest": pl.String,
    "packagetype": pl.String,
    "python_version": pl.String,
    "requires_python": pl.String,
    "size": pl.Int64,
    "upload_time": pl.String,
    "upload_time_iso_8601": pl.String,
    "url": pl.String,
    "yanked": pl.Boolean,
    "yanked_reason": pl.String,
  }
)

urls_schema = pl.List(
  pl.Struct(
    {
      "comment_text": pl.String,
      "digests": pl.Struct({"blake2b_256": pl.String, "md5": pl.String, "sha256": pl.String}),
      "downloads": pl.Int64,
      "filename": pl.String,
      "has_sig": pl.String,
      "md5_digest": pl.String,
      "packagetype": pl.String,
      "python_version": pl.String,
      "requires_python": pl.String,
      "size": pl.Int64,
      "upload_time": pl.String,
      "upload_time_iso_8601": pl.String,
      "url": pl.String,
      "yanked": pl.Boolean,
      "yanked_reason": pl.String,
    }
  )
)

vulnerabilities_schema = pl.List(
  pl.Struct(
    {
      "aliases": pl.List(pl.String),
      "details": pl.String,
      "fixed_in": pl.List(pl.String),
      "link": pl.String,
      "source": pl.String,
      "summary": pl.String,
      "withdrawn": pl.String,
    }
  )
)
