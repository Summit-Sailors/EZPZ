use components::{create_component, Component, ComponentSettings, ComponentType};
use eframe::egui;
use egui::{CentralPanel, Context, Ui, Vec2};
use ezpz_stubz::lazy::PyLfStubbed;
use polars::prelude::*;
use pyo3::{pyclass, pymethods, PyResult, Python};
use pyo3_stub_gen::{
	define_stub_info_gatherer,
	derive::{gen_stub_pyclass, gen_stub_pymethods},
};

mod components;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct LayoutElement {
	component_type: ComponentType,
	settings: ComponentSettings,
}

#[gen_stub_pymethods]
#[pymethods]
impl LayoutElement {
	#[new]
	fn new(component_type: ComponentType, settings: ComponentSettings) -> Self {
		LayoutElement { component_type, settings }
	}
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct LayoutDescription {
	#[pyo3(get, set)]
	pub elements: Vec<LayoutElement>,
}

#[gen_stub_pymethods]
#[pymethods]
impl LayoutDescription {
	#[new]
	fn new() -> Self {
		LayoutDescription { elements: Vec::new() }
	}
}

#[gen_stub_pyclass]
#[pyclass]
pub struct LayoutBuilder {
	elements: Vec<LayoutElement>,
}

#[gen_stub_pymethods]
#[pymethods]
impl LayoutBuilder {
	#[new]
	fn new() -> Self {
		LayoutBuilder { elements: Vec::new() }
	}

	fn add_component(&mut self, component_type: ComponentType, settings: ComponentSettings) -> PyResult<&mut Self> {
		self.elements.push(LayoutElement { component_type, settings });
		Ok(self)
	}

	fn build(&self) -> PyResult<LayoutDescription> {
		Ok(LayoutDescription { elements: self.elements.clone() })
	}
}

#[gen_stub_pyclass]
#[pyclass]
pub struct LazyFrameViewer {
	layout: LayoutDescription,
	components: Vec<Box<dyn Component>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl LazyFrameViewer {
	#[new]
	pub fn new(py_lf: PyLfStubbed, layout_description: LayoutDescription) -> PyResult<Self> {
		let lf: LazyFrame = py_lf.clone().into().unwrap();
		let df = lf.clone().collect().unwrap();

		let mut components = Vec::new();
		for element in &layout_description.elements {
			let mut settings = element.settings.clone();
			if element.component_type == ComponentType::Data {
				if let Some(data_settings) = &mut settings.data {
					data_settings.df = Some(df.clone());
				}
			}
			components.push(create_component(element.component_type, &settings));
		}

		Ok(ViewerSettings { layout: layout_description, components })
	}

	fn render_layout(&mut self, ui: &mut Ui) {
		for component in &mut self.components {
			component.ui(ui);
		}
	}

	pub fn viewer_ui(&mut self, ctx: &Context) {
		CentralPanel::default().show(ctx, |ui| {
			ui.heading("EZPZ Viewer");
			self.render_layout(ui);
		});
	}

	#[pyo3(text_signature = "($self)")]
	pub fn run(&mut self, py: Python) -> PyResult<()> {
		let options = eframe::NativeOptions { initial_window_size: Some(Vec2::new(1024.0, 768.0)), ..Default::default() };

		eframe::run_native("EZPZ Viewer", options, Box::new(|cc| Box::new(self.clone()))).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

		Ok(())
	}
}

impl eframe::App for ViewerSettings {
	fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
		self.viewer_ui(ctx);
	}
}

define_stub_info_gatherer!(stub_info);
