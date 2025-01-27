#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct JuztApp {
	label: String,
	#[serde(skip)]
	value: f32,
}

impl Default for JuztApp {
	fn default() -> Self {
		Self { label: "Hello World!".to_owned(), value: 2.7 }
	}
}

impl JuztApp {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}
		Default::default()
	}
}

impl eframe::App for JuztApp {
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				let is_web = cfg!(target_arch = "wasm32");
				if !is_web {
					ui.menu_button("File", |ui| {
						if ui.button("Quit").clicked() {
							ctx.send_viewport_cmd(egui::ViewportCommand::Close);
						}
					});
					ui.add_space(16.0);
				}
				egui::widgets::global_theme_preference_buttons(ui);
			});
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("eframe template");
			ui.horizontal(|ui| {
				ui.label("Write something: ");
				ui.text_edit_singleline(&mut self.label);
			});
			ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
			if ui.button("Increment").clicked() {
				self.value += 1.0;
			}
			ui.separator();
			ui.add(egui::github_link_file!("https://github.com/emilk/eframe_template/blob/main/", "Source code."));
			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				egui::warn_if_debug_build(ui);
			});
		});
	}
}
