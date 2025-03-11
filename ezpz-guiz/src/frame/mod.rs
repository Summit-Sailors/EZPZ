use bevy::prelude::Window;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowPlugin};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui_plot::{BarChart, Plot, PlotPoints};
use polars::lazy::dsl::*;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use ezpz_stubz::frame::PyDfStubbed;

#[derive(Clone, Resource)]
#[gen_stub_pyclass]
#[pyclass]
pub struct DataFrameViewer {
  df: DataFrame,
  sort_column: Option<String>,
  sort_ascending: bool,
  filter_conditions: Vec<FilterCondition>,
  page: usize,
  rows_per_page: usize,
  column_order: Vec<String>,
  hidden_columns: Vec<String>,
  column_rename_map: HashMap<String, String>,
  current_view: DataFrameView,
  selected_cells: Vec<(usize, usize)>,
  search_text: String,
  theme: Theme,
  custom_layouts: Vec<CustomLayout>,
  selected_x_column: Option<String>,
  selected_y_column: Option<String>,
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

#[derive(Clone)]
enum DataFrameView {
  Table,
  Chart(ChartType),
  Heatmap,
  DataProfile,
  ColumnCorrelation,
}

#[derive(Clone)]
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

#[derive(Clone, Serialize, Deserialize)]
enum Theme {
  Light,
  Dark,
  Custom(String),
}

impl DataFrameViewer {
  fn ui(&mut self, ctx: &mut EguiContext) {
    let ctx = ctx.get_mut();
    self.apply_theme(ctx);
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Polars DataFrame Viewer");
      self.menu_bar(ui);
      self.controls_ui(ui);
      match self.current_view {
        DataFrameView::Table => self.table_ui(ui),
        DataFrameView::Chart(ref chart_type) => self.chart_ui(ui, chart_type),
        DataFrameView::Heatmap => self.heatmap_ui(ui),
        DataFrameView::DataProfile => self.data_profile_ui(ui),
        DataFrameView::ColumnCorrelation => self.column_correlation_ui(ui),
      }
    });
  }

  fn apply_theme(&self, ctx: &egui::Context) {
    match &self.theme {
      Theme::Light => ctx.set_visuals(egui::Visuals::light()),
      Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
      Theme::Custom(color) => {
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::from_hex(color).unwrap_or(egui::Color32::WHITE));
        ctx.set_visuals(visuals);
      }
    }
  }

  fn menu_bar(&mut self, ui: &mut egui::Ui) {
    egui::menu::bar(ui, |ui| {
      ui.menu_button("File", |ui| {
        if ui.button("Export CSV").clicked() {
          if let Err(e) = self.df.write_csv("export.csv") {
            eprintln!("Failed to export CSV: {:?}", e);
          }
        }
        if ui.button("Export Excel").clicked() {
          // Note: This is a placeholder. You'd need to add a crate like `polars_excel` to implement this.
          println!("Export to Excel not implemented yet");
        }
        if ui.button("Export JSON").clicked() {
          if let Err(e) = self.df.write_json("export.json") {
            eprintln!("Failed to export JSON: {:?}", e);
          }
        }
      });
      ui.menu_button("View", |ui| {
        if ui.button("Table").clicked() {
          self.current_view = DataFrameView::Table;
        }
        if ui.button("Bar Chart").clicked() {
          self.current_view = DataFrameView::Chart(ChartType::Bar);
        }
        if ui.button("Line Chart").clicked() {
          self.current_view = DataFrameView::Chart(ChartType::Line);
        }
        if ui.button("Scatter Plot").clicked() {
          self.current_view = DataFrameView::Chart(ChartType::Scatter);
        }
        if ui.button("Heatmap").clicked() {
          self.current_view = DataFrameView::Heatmap;
        }
        if ui.button("Data Profile").clicked() {
          self.current_view = DataFrameView::DataProfile;
        }
        if ui.button("Column Correlation").clicked() {
          self.current_view = DataFrameView::ColumnCorrelation;
        }
      });
      ui.menu_button("Columns", |ui| {
        self.column_management_ui(ui);
      });
      ui.menu_button("Theme", |ui| {
        if ui.button("Light").clicked() {
          self.theme = Theme::Light;
        }
        if ui.button("Dark").clicked() {
          self.theme = Theme::Dark;
        }
        if ui.button("Custom").clicked() {
          let mut color = String::new();
          if ui.text_edit_singleline(&mut color).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.theme = Theme::Custom(color);
          }
        }
      });
      ui.menu_button("Layout", |ui| {
        if ui.button("Save Layout").clicked() {
          self.save_layout();
        }
        if ui.button("Load Layout").clicked() {
          self.load_layout();
        }
      });
    });
  }

  fn controls_ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      self.filter_ui(ui);
      ui.separator();
      self.pagination_ui(ui);
      ui.separator();
      self.search_ui(ui);
    });
  }

  fn filter_ui(&mut self, ui: &mut egui::Ui) {
    ui.group(|ui| {
      ui.label("Filters:");
      for condition in &mut self.filter_conditions {
        ui.horizontal(|ui| {
          egui::ComboBox::from_label("Column").selected_text(&condition.column).show_ui(ui, |ui| {
            for col in self.df.get_column_names() {
              ui.selectable_value(&mut condition.column, col.to_string(), col);
            }
          });
          egui::ComboBox::from_label("Operator").selected_text(format!("{:?}", condition.operator)).show_ui(ui, |ui| {
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
        self.filter_conditions.push(FilterCondition {
          column: self.df.get_column_names()[0].to_string(),
          operator: FilterOperator::Equals,
          value: String::new(),
        });
      }
      if ui.button("Apply Filters").clicked() {
        self.apply_filters();
      }
    });
  }

  fn pagination_ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      if ui.button("◀").clicked() && self.page > 0 {
        self.page -= 1;
      }
      ui.label(format!("Page {} of {}", self.page + 1, (self.df.height() + self.rows_per_page - 1) / self.rows_per_page));
      if ui.button("▶").clicked() && (self.page + 1) * self.rows_per_page < self.df.height() {
        self.page += 1;
      }
      ui.label("Rows per page:");
      egui::ComboBox::from_label("").selected_text(self.rows_per_page.to_string()).show_ui(ui, |ui| {
        for &rows in &[10, 25, 50, 100] {
          ui.selectable_value(&mut self.rows_per_page, rows, rows.to_string());
        }
      });
    });
  }

  fn search_ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      ui.label("Search:");
      if ui.text_edit_singleline(&mut self.search_text).changed() {
        self.apply_search();
      }
      if ui.button("Clear").clicked() {
        self.search_text.clear();
        self.apply_search();
      }
    });
  }

  fn table_ui(&mut self, ui: &mut egui::Ui) {
    let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
    egui::ScrollArea::both().show_rows(ui, text_height, self.df.height(), |ui, row_range| {
      egui::Grid::new("dataframe_grid").striped(true).show(ui, |ui| {
        for col in &self.column_order {
          if !self.hidden_columns.contains(col) {
            let display_name = self.column_rename_map.get(col).unwrap_or(col);
            if ui.button(display_name).clicked() {
              self.sort_by_column(col);
            }
          }
        }
        ui.end_row();
        for row_idx in row_range {
          for (col_idx, col) in self.column_order.iter().enumerate() {
            if !self.hidden_columns.contains(col) {
              let value = self.df.column(col).unwrap().get(row_idx).unwrap();
              let cell_response = ui.label(format!("{:?}", value));
              if cell_response.clicked() {
                self.toggle_cell_selection(row_idx, col_idx);
              }
              if self.selected_cells.contains(&(row_idx, col_idx)) {
                cell_response.paint_at(cell_response.rect, |ui| {
                  ui.painter().rect_stroke(cell_response.rect, 0.0, (2.0, egui::Color32::YELLOW));
                });
              }
            }
          }
          ui.end_row();
        }
      });
    });
  }

  fn chart_ui(&self, ui: &mut egui::Ui, chart_type: &ChartType) {
    ui.horizontal(|ui| {
      egui::ComboBox::from_label("X-axis").selected_text(self.selected_x_column.as_deref().unwrap_or("")).show_ui(ui, |ui| {
        for col in self.df.get_column_names() {
          ui.selectable_value(&mut self.selected_x_column, Some(col.to_string()), col);
        }
      });
      egui::ComboBox::from_label("Y-axis").selected_text(self.selected_y_column.as_deref().unwrap_or("")).show_ui(ui, |ui| {
        for col in self.df.get_column_names() {
          ui.selectable_value(&mut self.selected_y_column, Some(col.to_string()), col);
        }
      });
    });
    if let (Some(x_col), Some(y_col)) = (&self.selected_x_column, &self.selected_y_column) {
      let x_series = self.df.column(x_col).unwrap();
      let y_series = self.df.column(y_col).unwrap();
      let points: Vec<[f64; 2]> = x_series
        .iter()
        .zip(y_series.iter())
        .filter_map(|(x, y)| if let (Some(x), Some(y)) = (x.try_extract::<f64>(), y.try_extract::<f64>()) { Some([x, y]) } else { None })
        .collect();
      Plot::new("chart").view_aspect(2.0).show(ui, |plot_ui| match chart_type {
        ChartType::Bar => {
          let bar_chart = BarChart::new(points.iter().enumerate().map(|(i, &[x, y])| egui_plot::Bar::new(x, y).width(0.9)).collect());
          plot_ui.bar_chart(bar_chart);
        }
        ChartType::Line => {
          plot_ui.line(egui_plot::Line::new(PlotPoints::from_iter(points.iter().cloned())));
        }
        ChartType::Scatter => {
          plot_ui.points(egui_plot::Points::new(PlotPoints::from_iter(points.iter().cloned())));
        }
      });
    }
  }

  fn heatmap_ui(&self, ui: &mut egui::Ui) {
    ui.label("Heatmap view not yet implemented");
  }

  fn data_profile_ui(&self, ui: &mut egui::Ui) {
    ui.heading("Data Profile");
    for col in self.df.get_column_names() {
      ui.collapsing(col, |ui| {
        let series = self.df.column(col).unwrap();
        ui.label(format!("Data type: {:?}", series.dtype()));
        ui.label(format!("Non-null count: {}", series.len() - series.null_count()));
        if let Ok(stats) = series.into_frame().describe(None) {
          ui.label(format!("Min: {:?}", stats.get("min")));
          ui.label(format!("Max: {:?}", stats.get("max")));
          ui.label(format!("Mean: {:?}", stats.get("mean")));
          ui.label(format!("Std: {:?}", stats.get("std")));
        }
      });
    }
  }

  fn column_correlation_ui(&mut self, ui: &mut egui::Ui) {
    ui.label("Column correlation view not yet implemented");
  }

  fn column_management_ui(&mut self, ui: &mut egui::Ui) {
    for col in self.df.clone().get_column_names() {
      ui.horizontal(|ui| {
        let mut hidden = self.hidden_columns.contains(&col.to_string());
        if ui.checkbox(&mut hidden, "").changed() {
          if hidden {
            self.hidden_columns.push(col.to_string());
          } else {
            self.hidden_columns.retain(|c| c != col);
          }
        }
        let mut renamed = self.column_rename_map.get(col).unwrap_or(&col.to_string()).to_string();
        if ui.text_edit_singleline(&mut renamed).changed() {
          self.column_rename_map.insert(col.to_string(), renamed);
        }
        if ui.button("▲").clicked() {
          let idx = self.column_order.iter().position(|c| c == col).unwrap();
          if idx > 0 {
            self.column_order.swap(idx, idx - 1);
          }
        }
        if ui.button("▼").clicked() {
          let idx = self.column_order.iter().position(|c| c == col).unwrap();
          if idx < self.column_order.len() - 1 {
            self.column_order.swap(idx, idx + 1);
          }
        }
        if ui.button("Stats").clicked() {
          self.show_column_statistics(col);
        }
      });
    }
  }

  fn apply_filters(&mut self) {
    let mut lf = self.df.clone().lazy();

    for condition in &self.filter_conditions {
      let filter_expr = match condition.operator {
        FilterOperator::Equals => col(&condition.column).eq(lit(condition.value.clone())),
        FilterOperator::GreaterThan => col(&condition.column).gt(lit(condition.value.parse::<f64>().unwrap())),
        FilterOperator::LessThan => col(&condition.column).lt(lit(condition.value.parse::<f64>().unwrap())),
        FilterOperator::NotEquals => col(&condition.column).neq(lit(condition.value.clone())),
        FilterOperator::Contains => col(&condition.column).str().contains(lit(condition.value.clone()), true),
      };
      lf = lf.filter(filter_expr);
    }

    self.df = lf.collect().unwrap();
    self.page = 0;
  }

  fn apply_search(&mut self) {
    if self.search_text.is_empty() {
      return;
    }
    let search_condition = self
      .df
      .get_column_names()
      .iter()
      .map(|&name| col(name).cast(DataType::String).str().contains(lit(self.search_text.clone()), true))
      .reduce(|acc, expr| acc.or(expr))
      .unwrap_or_else(|| lit(false));
    self.df = self.df.clone().lazy().filter(search_condition).collect().unwrap();
    self.page = 0;
  }

  fn sort_by_column(&mut self, column: &str) {
    if Some(column.to_string()) == self.sort_column {
      self.sort_ascending = !self.sort_ascending;
    } else {
      self.sort_column = Some(column.to_string());
      self.sort_ascending = true;
    }
    let mut lf = self.df.clone().lazy();
    if let Some(sort_col) = &self.sort_column {
      lf = lf.sort(vec![sort_col], SortMultipleOptions::default().with_order_descending_multi(vec![self.sort_ascending]));
    }
    self.df = lf.collect().unwrap();
  }

  fn toggle_cell_selection(&mut self, row: usize, col: usize) {
    let cell = (row, col);
    if let Some(pos) = self.selected_cells.iter().position(|&c| c == cell) {
      self.selected_cells.remove(pos);
    } else {
      self.selected_cells.push(cell);
    }
  }

  fn show_column_statistics(&self, column: &str) {
    if let Ok(stats) = self.df.select([column]).unwrap().describe(None) {
      println!("Statistics for column '{}':", column);
      println!("{:?}", stats);
    }
  }

  fn save_layout(&mut self) {
    let layout = CustomLayout {
      name: "Custom Layout".to_string(),
      column_order: self.column_order.clone(),
      hidden_columns: self.hidden_columns.clone(),
      theme: self.theme.clone(),
    };
    self.custom_layouts.push(layout);
  }

  fn load_layout(&mut self) {
    if let Some(layout) = self.custom_layouts.last() {
      self.column_order = layout.column_order.clone();
      self.hidden_columns = layout.hidden_columns.clone();
      self.theme = layout.theme.clone();
    }
  }
}

#[gen_stub_pymethods]
#[pymethods]
impl DataFrameViewer {
  #[new]
  #[pyo3(text_signature = "(py_df)")]
  fn new(py_df: PyDfStubbed) -> Self {
    let df: DataFrame = py_df.into();
    let column_order = df.get_column_names().iter().map(|s| s.to_string()).collect();
    DataFrameViewer {
      df,
      sort_column: None,
      sort_ascending: true,
      filter_conditions: Vec::new(),
      page: 0,
      rows_per_page: 100,
      column_order,
      hidden_columns: Vec::new(),
      column_rename_map: HashMap::new(),
      current_view: DataFrameView::Table,
      selected_cells: Vec::new(),
      search_text: String::new(),
      theme: Theme::Light,
      custom_layouts: Vec::new(),
      selected_x_column: None,
      selected_y_column: None,
    }
  }

  fn view(&self, window_title: String, width: u32, height: u32) -> PyResult<()> {
    App::new()
      .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          title: window_title,
          resolution: (width as f32, height as f32).into(),
          present_mode: PresentMode::AutoVsync,
          ..default()
        }),
        ..default()
      }))
      .add_plugins(EguiPlugin)
      .insert_resource(self.clone())
      .add_systems(Update, self.ui)
      .run();
    Ok(())
  }
}

define_stub_info_gatherer!(stub_info);
