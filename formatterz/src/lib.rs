use {
	enum_map::{Enum, EnumMap, enum_map},
	pyo3::{prelude::*, types::PyList},
	pyo3_stub_gen::{
		define_stub_info_gatherer,
		derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods},
	},
	std::{
		fs,
		io::{self, Write},
		path::{Path, PathBuf},
		process::Command,
		str::FromStr,
		sync::LazyLock,
	},
};

#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::Display, strum::EnumString, Enum)]
enum FileExtension {
	#[strum(to_string = ".py")]
	Py,
	#[strum(to_string = ".pyi")]
	Pyi,
	#[strum(to_string = ".toml")]
	Toml,
	#[strum(to_string = ".js")]
	JS,
	#[strum(to_string = ".jsx")]
	Jsx,
	#[strum(to_string = ".ts")]
	Ts,
	#[strum(to_string = ".tsx")]
	Tsx,
	#[strum(to_string = ".css")]
	Css,
	#[strum(to_string = ".scss")]
	Scss,
	#[strum(to_string = ".json")]
	Json,
	#[strum(to_string = ".md")]
	Md,
	#[strum(to_string = ".yml")]
	Yml,
	#[strum(to_string = ".yaml")]
	Yaml,
	#[strum(to_string = ".rs")]
	Rs,
}

#[derive(Clone)]
struct CmdRepr {
	program: &'static str,
	args: Vec<&'static str>,
}

static E_FILE_EXT_TO_CMD_STRINGS: LazyLock<EnumMap<FileExtension, Vec<CmdRepr>>> = LazyLock::new(|| {
	enum_map! {
			FileExtension::Py|FileExtension::Pyi => vec![ CmdRepr{program:"rye",args:vec!["run","ruff","check","--fix"]},  CmdRepr{program:"rye",args:vec!["run","ruff","format"]}],
			FileExtension::JS|
			FileExtension::Jsx|
			FileExtension::Ts|
			FileExtension::Tsx|
			FileExtension::Css|
			FileExtension::Scss|
			FileExtension::Json|
			FileExtension::Md|
			FileExtension::Yml|
			FileExtension::Yaml=> vec![CmdRepr{program:"pnpm",args:vec!["prettier","-w"]}],
			FileExtension::Rs=>vec![CmdRepr{program:"rustfmt",args:vec!["run","ruff","check","--fix"]}],
			FileExtension::Toml=> vec![CmdRepr{program:"taplo",args:vec!["format"]}]
	}
});

impl FileExtension {
	fn get_cmd_strings(self) -> Vec<CmdRepr> {
		E_FILE_EXT_TO_CMD_STRINGS[self].clone()
	}
}

#[gen_stub_pyclass]
#[pyclass]
struct Formatter;

impl Formatter {
	fn _format_file(file_path: PathBuf) -> PyResult<()> {
		let ext_str = file_path.extension().unwrap().to_str().unwrap().to_owned().to_lowercase();
		for cmd_stem in FileExtension::from_str(&ext_str).unwrap().get_cmd_strings() {
			let output = Command::new(cmd_stem.program).args(cmd_stem.args).output()?;
			io::stdout().write_all(&output.stdout)?;
			io::stderr().write_all(&output.stderr)?;
		}

		Ok(())
	}

	fn _format_paths(paths: Vec<PathBuf>) -> PyResult<()> {
		for path in paths.into_iter() {
			if path.is_file() {
				Self::_format_file(path)?;
			} else if path.is_dir() {
				Self::_format_paths(fs::read_dir(path)?.map(|entry| entry.unwrap().path()).collect())?;
			}
		}

		Ok(())
	}
}

#[gen_stub_pymethods]
#[pymethods]
impl Formatter {
	#[staticmethod]
	fn format_file(file_path: &str) -> PyResult<()> {
		Self::_format_file(Path::new(file_path).into())
	}

	#[staticmethod]
	fn format_paths(paths: &Bound<PyList>) -> PyResult<()> {
		Self::_format_paths(paths.into_iter().map(|path_item| path_item.extract::<PathBuf>().unwrap()).collect())
	}
}

#[pymodule]
fn formatter(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
	m.add_class::<Formatter>()?;
	Ok(())
}

define_stub_info_gatherer!(stub_info);
