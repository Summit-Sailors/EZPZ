use super::Component;
use egui::Ui;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
	#[pyo3(get, set)]
	pub columns: usize,
	#[pyo3(get, set)]
	pub row_height: f32,
}

impl Default for LayoutSettings {
	fn default() -> Self {
		LayoutSettings { columns: 2, row_height: 30.0 }
	}
}

pub struct LayoutComponent {
	settings: LayoutSettings,
}

impl LayoutComponent {
	pub fn new(settings: Option<LayoutSettings>) -> Self {
		LayoutComponent { settings: settings.unwrap_or_default() }
	}
}

impl Component for LayoutComponent {
	fn ui(&mut self, ui: &mut Ui) {
		ui.label("Layout Options");
		ui.add(egui::Slider::new(&mut self.settings.columns, 1..=4).text("Columns"));
		ui.add(egui::Slider::new(&mut self.settings.row_height, 20.0..=100.0).text("Row Height"));
	}
}
