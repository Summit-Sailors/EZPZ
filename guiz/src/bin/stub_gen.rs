use {ezpz_guiz::stub_info, pyo3_stub_gen::Result};

fn main() -> Result<()> {
	stub_info()?.generate()?;
	Ok(())
}
