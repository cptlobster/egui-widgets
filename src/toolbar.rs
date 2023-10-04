use egui::*;

pub struct Tool<'a> {
    name: &'a str,
    callback: fn() -> ()
}

impl Tool<'_> {
    pub fn on_click(&self) {
        (self.callback)();
    }
}

pub fn toolbar(ui: &mut Ui, tools: Vec<Tool>) -> InnerResponse<()> {
    ui.horizontal(|ui| {
        for i in tools {
            if ui.button(i.name).clicked() {
                (i.callback);
            }
        }
    })
}