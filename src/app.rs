#[derive(Default)]
pub struct WsClicker {}

impl WsClicker {
	pub fn new(_cc: &eframe::CreationContext) -> Self {
		Default::default()
	}
}

impl eframe::App for WsClicker {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			
		});
	}
}
