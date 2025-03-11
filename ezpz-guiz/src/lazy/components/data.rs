use super::Component;
use eframe::egui::{ScrollArea, Ui};
use eframe::egui_extras::{Size, TableBuilder};
use polars::prelude::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct TableSettings {
	#[pyo3(get, set)]
	pub max_rows_display: usize,
	#[pyo3(get, set)]
	pub max_cols_display: usize,
	#[pyo3(get, set)]
	pub show_index: bool,
	#[pyo3(get, set)]
	pub alternating_row_colors: bool,
}

impl Default for TableSettings {
	fn default() -> Self {
		TableSettings { max_rows_display: 100, max_cols_display: 10, show_index: true, alternating_row_colors: true }
	}
}

pub struct TableComponent {
	settings: TableSettings,
	df: Option<DataFrame>,
	column_order: Vec<String>,
}

impl TableComponent {
	pub fn new(settings: TableSettings, df: Option<DataFrame>) -> Self {
		let column_order = if let Some(df) = &df { df.get_column_names().into_iter().map(String::from).collect() } else { Vec::new() };
		TableComponent { settings, df, column_order }
	}

	pub fn settings_ui(&mut self, ui: &mut Ui) {
		ui.add(eframe::egui::Slider::new(&mut self.settings.max_rows_display, 10..=1000).text("Max Rows"));
		ui.add(eframe::egui::Slider::new(&mut self.settings.max_cols_display, 1..=50).text("Max Columns"));
		ui.checkbox(&mut self.settings.show_index, "Show Index");
		ui.checkbox(&mut self.settings.alternating_row_colors, "Alternating Row Colors");
	}
}

impl Component for TableComponent {
	fn ui(&mut self, ui: &mut Ui) {
		ui.label("Table View");
		if let Some(df) = &self.df {
			ScrollArea::both().show(ui, |ui| {
				TableBuilder::new(ui)
					.striped(self.settings.alternating_row_colors)
					.column(Size::remainder().at_least(100.0))
					.header(20.0, |mut header| {
						if self.settings.show_index {
							header.col(|ui| {
								ui.strong("Index");
							});
						}
						for col in self.column_order.iter().take(self.settings.max_cols_display) {
							header.col(|ui| {
								ui.strong(col);
							});
						}
					})
					.body(|mut body| {
						for row in 0..df.height().min(self.settings.max_rows_display) {
							body.row(18.0, |mut row| {
								if self.settings.show_index {
									row.col(|ui| {
										ui.label(row.to_string());
									});
								}
								for col in self.column_order.iter().take(self.settings.max_cols_display) {
									if let Ok(s) = df.column(col) {
										row.col(|ui| {
											ui.label(s.get(row).unwrap().to_string());
										});
									}
								}
							});
						}
					});
			});
		} else {
			ui.label("No data available");
		}
	}
}

#[pyclass]
#[derive(Clone)]
pub struct PyTableSettings {
	#[pyo3(get, set)]
	pub max_rows_display: usize,
	#[pyo3(get, set)]
	pub max_cols_display: usize,
	#[pyo3(get, set)]
	pub show_index: bool,
	#[pyo3(get, set)]
	pub alternating_row_colors: bool,
}

#[pymethods]
impl PyTableSettings {
	#[new]
	fn new() -> Self {
		let settings = TableSettings::default();
		PyTableSettings {
			max_rows_display: settings.max_rows_display,
			max_cols_display: settings.max_cols_display,
			show_index: settings.show_index,
			alternating_row_colors: settings.alternating_row_colors,
		}
	}
}
