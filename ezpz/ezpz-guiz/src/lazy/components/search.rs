use super::Component;
use egui::Ui;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct SearchSettings {
	#[pyo3(get, set)]
	pub case_sensitive: bool,
	#[pyo3(get, set)]
	pub max_results: usize,
}

impl Default for SearchSettings {
	fn default() -> Self {
		SearchSettings { case_sensitive: false, max_results: 100 }
	}
}

pub struct SearchComponent {
	settings: SearchSettings,
	search_text: String,
}

impl SearchComponent {
	pub fn new(settings: Option<SearchSettings>) -> Self {
		SearchComponent { settings: settings.unwrap_or_default(), search_text: String::new() }
	}
}

impl Component for SearchComponent {
	fn ui(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label("Search:");
			ui.text_edit_singleline(&mut self.search_text);
		});
		ui.checkbox(&mut self.settings.case_sensitive, "Case sensitive");
	}
}
