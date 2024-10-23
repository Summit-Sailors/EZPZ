use eframe::egui::Ui;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;
use serde::{Deserialize, Serialize};

mod data;
mod filter;
mod layout;
mod search;
mod settings;
mod theme;

use data::DataComponent;
use filter::FilterComponent;
use layout::LayoutComponent;
use search::SearchComponent;
use theme::ThemeComponent;

#[pyclass(eq, eq_int)]
#[gen_stub_pyclass_enum]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
	Filter,
	Search,
	Layout,
	Theme,
	Data,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct ComponentSettings {
	#[pyo3(get, set)]
	pub filter: Option<filter::FilterSettings>,
	#[pyo3(get, set)]
	pub search: Option<search::SearchSettings>,
	#[pyo3(get, set)]
	pub layout: Option<layout::LayoutSettings>,
	#[pyo3(get, set)]
	pub theme: Option<theme::ThemeSettings>,
	#[pyo3(get, set)]
	pub data: Option<data::DataSettings>,
}

#[pymethods]
impl ComponentSettings {
	#[new]
	fn new() -> Self {
		ComponentSettings { filter: None, search: None, layout: None, theme: None, data: None }
	}
}

pub trait Component: Send + Sync {
	fn ui(&mut self, ui: &mut Ui);
}

pub fn create_component(component_type: ComponentType, settings: &ComponentSettings) -> Box<dyn Component> {
	match component_type {
		ComponentType::Filter => Box::new(FilterComponent::new(settings.filter.clone())),
		ComponentType::Search => Box::new(SearchComponent::new(settings.search.clone())),
		ComponentType::Layout => Box::new(LayoutComponent::new(settings.layout.clone())),
		ComponentType::Theme => Box::new(ThemeComponent::new(settings.theme.clone())),
		ComponentType::Data => Box::new(DataComponent::new(settings.data.clone())),
	}
}
