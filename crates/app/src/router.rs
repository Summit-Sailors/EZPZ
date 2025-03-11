use {
	crate::{layout::Layout, pages::home::Home},
	dioxus::prelude::*,
	strum_macros::EnumIter,
};

#[derive(Clone, PartialEq, EnumIter, Routable)]
pub enum Route {
	#[layout(Layout)]
	#[route("/")]
	Home {},
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::Home {} => "Home",
		}
	}
}
