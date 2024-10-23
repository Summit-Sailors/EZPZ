use super::Component;
use egui::Ui;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct FilterSettings {
	#[pyo3(get, set)]
	pub max_filters: usize,
	#[pyo3(get, set)]
	pub filter_types: Vec<String>,
}

impl Default for FilterSettings {
	fn default() -> Self {
		FilterSettings { max_filters: 5, filter_types: vec!["Equals".to_string(), "Contains".to_string(), "Greater Than".to_string(), "Less Than".to_string()] }
	}
}

pub struct FilterComponent {
	settings: FilterSettings,
	filters: Vec<String>,
}

impl FilterComponent {
	pub fn new(settings: Option<FilterSettings>) -> Self {
		FilterComponent { settings: settings.unwrap_or_default(), filters: Vec::new() }
	}
}

impl Component for FilterComponent {
	fn ui(&mut self, ui: &mut Ui) {
		ui.label("Filters:");
		for filter in &mut self.filters {
			ui.text_edit_singleline(filter);
		}
		if ui.button("Add Filter").clicked() && self.filters.len() < self.settings.max_filters {
			self.filters.push(String::new());
		}
	}
}
