use super::Component;
use egui::{Color32, Ui};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
	Light,
	Dark,
	Custom(String, (u8, u8, u8), (u8, u8, u8)),
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
	#[pyo3(get, set)]
	pub available_themes: Vec<Theme>,
	#[pyo3(get, set)]
	pub current_theme: Theme,
}

impl Default for ThemeSettings {
	fn default() -> Self {
		ThemeSettings { available_themes: vec![Theme::Light, Theme::Dark], current_theme: Theme::Light }
	}
}

pub struct ThemeComponent {
	settings: ThemeSettings,
}

impl ThemeComponent {
	pub fn new(settings: Option<ThemeSettings>) -> Self {
		ThemeComponent { settings: settings.unwrap_or_default() }
	}
}

impl Component for ThemeComponent {
	fn ui(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label("Theme:");
			for theme in &self.settings.available_themes {
				let name = match theme {
					Theme::Light => "Light",
					Theme::Dark => "Dark",
					Theme::Custom(name, _, _) => name,
				};
				if ui.button(name).clicked() {
					self.settings.current_theme = theme.clone();
				}
			}
		});

		match &self.settings.current_theme {
			Theme::Light => {
				ui.style_mut().visuals.dark_mode = false;
			},
			Theme::Dark => {
				ui.style_mut().visuals.dark_mode = true;
			},
			Theme::Custom(_, bg, text) => {
				ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(text.0, text.1, text.2));
				ui.style_mut().visuals.window_fill = Color32::from_rgb(bg.0, bg.1, bg.2);
			},
		}
	}
}
