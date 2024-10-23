use eframe::egui;
use egui::{CentralPanel, Context, Ui, Vec2};
use ezpz_stubz::lazy::PyLfStubbed;
use polars::prelude::*;
use pyo3::{pyclass, pymethods, Py, PyAny, PyResult, Python};
use pyo3_stub_gen::{
	define_stub_info_gatherer,
	derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods},
};
use std::collections::HashMap;

mod components;
use components::*;

#[pyclass(eq, eq_int)]
#[gen_stub_pyclass_enum]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ComponentType {
	Filter,
	Search,
	Layout,
	Theme,
	Data,
}

#[pyclass(eq, eq_int)]
#[gen_stub_pyclass_enum]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LayoutType {
	Horizontal,
	Vertical,
	Grid,
}

#[pyclass(eq)]
#[gen_stub_pyclass_enum]
#[derive(Clone)]
pub enum LayoutNode {
	Container { id: String, layout_type: LayoutType, children: Vec<String>, properties: Option<ContainerProperties> },
	Component { id: String, component_type: ComponentType, properties: Option<ComponentProperties> },
}

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone)]
pub struct ContainerProperties {
	#[pyo3(get, set)]
	pub columns: Option<usize>,
}

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone)]
pub struct ComponentProperties {
	// Add component-specific properties here
}

#[pymethods]
impl LayoutNode {
	#[staticmethod]
	fn container(id: String, layout_type: LayoutType) -> Self {
		LayoutNode::Container { id, layout_type, children: Vec::new(), properties: None }
	}

	#[staticmethod]
	fn component(id: String, component_type: ComponentType) -> Self {
		LayoutNode::Component { id, component_type, properties: None }
	}
}

#[pyclass]
#[gen_stub_pyclass]
pub struct LayoutBuilder {
	nodes: HashMap<String, LayoutNode>,
	root_id: String,
}

#[pymethods]
impl LayoutBuilder {
	#[new]
	fn new() -> Self {
		let root_id = "root".to_string();
		let mut nodes = HashMap::new();
		nodes.insert(root_id.clone(), LayoutNode::container(root_id.clone(), LayoutType::Vertical));
		LayoutBuilder { nodes, root_id }
	}

	fn add_node(&mut self, parent_id: &str, node: LayoutNode) -> PyResult<&mut Self> {
		let node_id = match &node {
			LayoutNode::Container { id, .. } => id,
			LayoutNode::Component { id, .. } => id,
		};

		if let Some(LayoutNode::Container { children, .. }) = self.nodes.get_mut(parent_id) {
			children.push(node_id.clone());
		} else {
			return Err(pyo3::exceptions::PyValueError::new_err("Parent node not found or is not a container"));
		}

		self.nodes.insert(node_id.clone(), node);
		Ok(self)
	}

	fn set_container_properties(&mut self, id: &str, properties: ContainerProperties) -> PyResult<&mut Self> {
		if let Some(LayoutNode::Container { properties: node_properties, .. }) = self.nodes.get_mut(id) {
			*node_properties = Some(properties);
			Ok(self)
		} else {
			Err(pyo3::exceptions::PyValueError::new_err("Node not found or is not a container"))
		}
	}

	fn set_component_properties(&mut self, id: &str, properties: ComponentProperties) -> PyResult<&mut Self> {
		if let Some(LayoutNode::Component { properties: node_properties, .. }) = self.nodes.get_mut(id) {
			*node_properties = Some(properties);
			Ok(self)
		} else {
			Err(pyo3::exceptions::PyValueError::new_err("Node not found or is not a component"))
		}
	}

	fn build(&self) -> PyResult<LayoutDescription> {
		Ok(LayoutDescription { nodes: self.nodes.clone(), root_id: self.root_id.clone() })
	}
}

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone)]
pub struct LayoutDescription {
	nodes: HashMap<String, LayoutNode>,
	root_id: String,
}

#[derive(Clone)]
#[gen_stub_pyclass]
#[pyclass]
pub struct ViewerSettings {
	layout: LayoutDescription,
	components: HashMap<ComponentType, Component>,
}

#[gen_stub_pymethods]
#[pymethods]
impl ViewerSettings {
	#[pyo3(text_signature = "(py_df, layout_description)")]
	#[new]
	pub fn new(py_lf: PyLfStubbed, layout_description: LayoutDescription) -> PyResult<Self> {
		let lf: LazyFrame = py_lf.clone().into().unwrap();
		let df = lf.clone().collect().unwrap();
		let data_component = DataComponent::new(&df, &lf);

		let mut components = HashMap::new();
		components.insert(ComponentType::Filter, Component::Filter(FilterComponent::new(&data_component.column_order)));
		components.insert(ComponentType::Search, Component::Search(SearchComponent::new()));
		components.insert(ComponentType::Layout, Component::Layout(LayoutComponent::new()));
		components.insert(ComponentType::Theme, Component::Theme(ThemeComponent::new()));
		components.insert(ComponentType::Data, Component::Data(data_component));

		Ok(ViewerSettings { layout: layout_description, components })
	}

	pub fn viewer_ui(&mut self, ctx: &Context) {
		CentralPanel::default().show(ctx, |ui| {
			ui.heading("EZPZ");
			self.render_layout(ui, &self.layout.root_id);
		});
	}

	fn render_layout(&mut self, ui: &mut Ui, node_id: &str) {
		if let Some(node) = self.layout.nodes.get(node_id) {
			match node {
				LayoutNode::Container { layout_type, children, properties, .. } => match layout_type {
					LayoutType::Horizontal => {
						ui.horizontal(|ui| {
							for child_id in children {
								self.render_layout(ui, child_id);
							}
						});
					},
					LayoutType::Vertical => {
						ui.vertical(|ui| {
							for child_id in children {
								self.render_layout(ui, child_id);
							}
						});
					},
					LayoutType::Grid => {
						let columns = properties.as_ref().and_then(|p| p.columns).unwrap_or(2);
						egui::Grid::new(node_id).num_columns(columns).show(ui, |ui| {
							for (i, child_id) in children.iter().enumerate() {
								self.render_layout(ui, child_id);
								if (i + 1) % columns == 0 {
									ui.end_row();
								}
							}
						});
					},
				},
				LayoutNode::Component { component_type, .. } => {
					if let Some(component) = self.components.get_mut(component_type) {
						component.ui(ui);
					}
				},
			}
		}
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
