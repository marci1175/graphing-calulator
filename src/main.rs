use eframe::NativeOptions;
use graphing_calculator::Application;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Renderer",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )?;

    Ok(())
}
