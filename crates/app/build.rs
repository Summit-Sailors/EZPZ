#[dotenvy::load]
fn main() {
	let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
	if profile != "release" {
		println!("cargo:rustc-env=RUST_BACKTRACE=1");
		println!("cargo:rustc-env=CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true");
		println!("cargo:rerun-if-changed=../.env");
	}

	#[cfg(feature = "server")]
	{
		for key in ["DATABASE_URL", "ANTHROPIC_API_KEY", "SERPAPI_API_KEY", "APALIS_DATABASE_URL"] {
			println!("cargo:rustc-env={}={}", key, std::env::var(key).unwrap());
		}
	}
	for key in ["SERVER_URL", "ENV"] {
		println!("cargo:rustc-env={}={}", key, std::env::var(key).unwrap());
	}
}
