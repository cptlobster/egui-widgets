use egui::*;

struct Tool {
    name: &str,
    callback: fn() -> ()
}

fn toolbar(ui: App, tools: Vector<Tool>) {
    ui.horizontal(|ui| {
        
    })
}