use {ezpz_formatterz::stub_info, pyo3_stub_gen::Result};

fn main() -> Result<()> {
	let stub = stub_info()?;
	stub.generate()?;
	Ok(())
}
