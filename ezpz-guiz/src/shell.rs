use bevy::prelude::*;
use bevy_egui::{
  egui::{menu, CentralPanel, ComboBox, Ui},
  EguiContexts,
};
use egui_extras::{Column, TableBuilder};
use ezpz_stubz::{frame::PyDfStubbed, lazy::PyLfStubbed};
use polars::prelude::*;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::{
  define_stub_info_gatherer,
  derive::{gen_stub_pyclass, gen_stub_pymethods},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Resource)]
#[gen_stub_pyclass]
#[pyclass]
pub struct ViewerSettings {
  filter_conditions: Vec<FilterCondition>,
  column_order: Vec<String>,
  hidden_columns: Vec<String>,
  column_rename_map: HashMap<String, String>,
  current_view: LayoutView,
  search_text: String,
  theme: Theme,
  custom_layouts: Vec<CustomLayout>,
  selected_x_column: Option<String>,
  selected_y_column: Option<String>,
  describe: DataFrame,
}

#[derive(Clone)]
struct FilterCondition {
  column: String,
  operator: FilterOperator,
  value: String,
}

#[derive(Clone, Debug, PartialEq)]
enum FilterOperator {
  Equals,
  GreaterThan,
  LessThan,
  NotEquals,
  Contains,
}

#[derive(Clone, PartialEq)]
enum LayoutView {
  Table,
  Chart(ChartType),
  DataProfile,
}

#[derive(Clone, Debug, PartialEq)]
enum ChartType {
  Bar,
  Line,
  Scatter,
}

#[derive(Clone, Serialize, Deserialize)]
struct CustomLayout {
  name: String,
  column_order: Vec<String>,
  hidden_columns: Vec<String>,
  theme: Theme,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Theme {
  Light,
  Dark,
  Custom(String),
}

#[gen_stub_pymethods]
#[pymethods]
impl ViewerSettings {
  #[pyo3(text_signature = "(py_df)")]
  pub fn new(py_lf: PyDfStubbed) -> Self {
    let lf: LazyFrame = py_lf.clone().into().unwrap();
    let df = lf.clone().collect().unwrap();
    let column_order: Vec<String> = df.get_column_names().into_iter().map(String::from).collect();
    let describe = lf
      .clone()
      .select([Expr::DtypeColumn(vec![DataType::Int32, DataType::Int64, DataType::UInt32, DataType::UInt64, DataType::Float32, DataType::Float64])])
      .select([
        all().count().alias("count"),
        all().mean().alias("mean"),
        all().std(1).alias("std"),
        all().min().alias("min"),
        all().quantile(lit(0.25), QuantileInterpolOptions::Linear).alias("25%"),
        all().median().alias("50%"),
        all().quantile(lit(0.75), QuantileInterpolOptions::Linear).alias("75%"),
        all().max().alias("max"),
        all().null_count().alias("null_count"),
      ])
      .collect()
      .unwrap();
    ViewerSettings {
      filter_conditions: Vec::new(),
      column_order,
      hidden_columns: Vec::new(),
      column_rename_map: HashMap::new(),
      current_view: LayoutView::Table,
      search_text: String::new(),
      theme: Theme::Light,
      custom_layouts: Vec::new(),
      selected_x_column: None,
      selected_y_column: None,
      describe,
    }
  }

  fn search_ui(&mut self, ui: &mut Ui) {
    ui.horizontal(|ui| {
      ui.label("Search:");
      ui.text_edit_singleline(&mut self.search_text);
      if ui.button("Clear").clicked() {
        self.search_text.clear();
      }
    });
  }

  fn table_ui(&self, ui: &mut Ui) {
    ui.label("Table view goes here");
    // Implement table view using self data
  }

  fn chart_ui(&self, ui: &mut Ui, chart_type: &ChartType) {
    ui.label(format!("{:?} chart goes here", chart_type));
    // Implement chart view using self data
  }

  fn show_describe(&self, ui: &mut Ui) {
    ui.heading("Describe");
    TableBuilder::new(ui)
      .column(Column::auto())
      .columns(Column::remainder(), self.describe.width())
      .header(20.0, |mut header| {
        header.col(|ui| {
          ui.strong("Statistic");
        });
        self.describe.get_column_names().iter().for_each(|name| {
          header.col(|ui| {
            ui.strong(*name);
          });
        });
      })
      .body(|mut body| {
        self.describe.iter().for_each(|series| {
          body.row(18.0, |mut row| {
            row.col(|ui| {
              ui.label(series.name());
            });
            series.iter().for_each(|value| {
              row.col(|ui| {
                ui.label(value.to_string());
              });
            });
          });
        });
      });
  }

  pub fn viewer_ui_system(mut settings: ResMut<Self>, mut contexts: EguiContexts) {
    CentralPanel::default().show(contexts.ctx_mut(), |ui| {
      ui.heading("EZPZ");
      menu::bar(ui, |ui| {
        ui.menu_button("View", |ui| {
          ui.selectable_value(&mut settings.current_view, LayoutView::Table, "Table");
          ui.menu_button("Chart", |ui| {
            ui.selectable_value(&mut settings.current_view, LayoutView::Chart(ChartType::Bar), "Bar Chart");
            ui.selectable_value(&mut settings.current_view, LayoutView::Chart(ChartType::Line), "Line Chart");
            ui.selectable_value(&mut settings.current_view, LayoutView::Chart(ChartType::Scatter), "Scatter Plot");
          });
          ui.selectable_value(&mut settings.current_view, LayoutView::DataProfile, "Data Profile");
        });
        ui.menu_button("Theme", |ui| {
          ui.selectable_value(&mut settings.theme, Theme::Light, "Light");
          ui.selectable_value(&mut settings.theme, Theme::Dark, "Dark");
        });
      });
      ui.horizontal(|ui| {
        ui.group(|ui| {
          ui.label("Filters:");
          for condition in &mut settings.filter_conditions {
            ui.horizontal(|ui| {
              ComboBox::from_label("Column").selected_text(&condition.column).show_ui(ui, |ui| {
                for col in &settings.column_order {
                  ui.selectable_value(&mut condition.column, col.to_string(), col);
                }
              });
              ComboBox::from_label("Operator").selected_text(format!("{:?}", condition.operator)).show_ui(ui, |ui| {
                ui.selectable_value(&mut condition.operator, FilterOperator::Equals, "Equals");
                ui.selectable_value(&mut condition.operator, FilterOperator::GreaterThan, "Greater Than");
                ui.selectable_value(&mut condition.operator, FilterOperator::LessThan, "Less Than");
                ui.selectable_value(&mut condition.operator, FilterOperator::NotEquals, "Not Equals");
                ui.selectable_value(&mut condition.operator, FilterOperator::Contains, "Contains");
              });
              ui.text_edit_singleline(&mut condition.value);
            });
          }
          if ui.button("Add Filter").clicked() {
            settings.filter_conditions.push(FilterCondition {
              column: settings.column_order[0].clone(),
              operator: FilterOperator::Equals,
              value: String::new(),
            });
          }
        });
        ui.separator();
        ui.horizontal(|ui| {
          ui.label("Search:");
          ui.text_edit_singleline(&mut settings.search_text);
          if ui.button("Clear").clicked() {
            settings.search_text.clear();
          }
        });
      });
      match settings.current_view {
        LayoutView::Table => settings.table_ui(ui),
        LayoutView::Chart(ref chart_type) => settings.chart_ui(ui, chart_type),
        LayoutView::DataProfile => settings.show_describe(ui),
      }
    });
  }
}

impl Plugin for ViewerSettings {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, ViewerSettings::viewer_ui_system);
  }
}

define_stub_info_gatherer!(stub_info);
