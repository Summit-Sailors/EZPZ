#![allow(non_snake_case)]
use {
	app::router::Route,
	dioxus::prelude::*,
	dioxus_logger::tracing,
	maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame},
};

fn App() -> Element {
	let toast = use_init_toast_ctx();
	rsx! {
		ToastFrame { manager: toast }
		Router::<Route> {}
	}
}

fn main() {
	// #[cfg(not(feature = "server"))]
	// dioxus::fullstack::prelude::server_fn::client::set_server_url(&SERVER_URL);
	dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
	dioxus::LaunchBuilder::new()
		.with_context(server_only!(maestro_diesel::async_client::client::acreate_diesel_pool(env!("DATABASE_URL"))))
		.with_context(server_only!(maestro_anthropic::AnthropicClient::new(env!("ANTHROPIC_API_KEY"))))
		.launch(App);
}
