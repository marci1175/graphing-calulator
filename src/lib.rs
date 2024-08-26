use std::ops::RangeInclusive;

use calculator_lib::Calculator;
use eframe::App;
use egui::{Color32, RichText, Slider};
use egui_plot::{PlotItem, PlotPoint, PlotPoints};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Application {
    equation_buffer: String,

    #[serde(skip)]
    calculator_instance: Option<anyhow::Result<Calculator>>,

    #[serde(skip)]
    calculation_err_result: Option<String>,

    equation_range: i64,

    #[serde(skip)]
    calculation_result_points: Vec<[f64; 2]>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            equation_buffer: String::new(),
            calculator_instance: None,
            equation_range: 100,
            calculation_err_result: None,
            calculation_result_points: vec![],
        }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui(ui.available_size(), |ui| {
                egui_plot::Plot::new("graph").show_axes([true, true]).show(ui, |plot| {
                    plot.add(
                        egui_plot::Line::new(PlotPoints::new(self.calculation_result_points.clone()))
                            .color(Color32::RED),
                    );
                });
            });
        });

        egui::TopBottomPanel::bottom("equation").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Equation");
                let text_edit = ui.text_edit_singleline(&mut self.equation_buffer);

                let slider = ui.add(
                    Slider::new(&mut self.equation_range, 0..=3000)
                        .step_by(1.0)
                        .text("Equation range"),
                )
                .on_hover_text("This slider sets the bounds of where the X should be calculated.");


                if text_edit.changed() || slider.changed() {
                    self.calculator_instance = Some(Calculator::new(self.equation_buffer.clone()));

                    if let Some(calculator_state) = &self.calculator_instance {
                        if let Ok(calculator_handle) = calculator_state {
                            //Vec<(x, y)>
                            let mut points: Vec<[f64; 2]> = vec![];

                            //Calculate X in range
                            for n in -self.equation_range..=self.equation_range {
                                let result = calculator_handle.calculate(Some(n as f64));
                                match result {
                                    Ok(result) => {
                                        //If we convert it here, it will be easier to handle it later
                                        points.push([n as f64, result]);
                                    },
                                    Err(err) => {
                                        self.calculation_err_result = Some(err.to_string());

                                        break;
                                    },
                                }
                            }

                            self.calculation_result_points = points;
                        }
                    }
                }

                if let Some(calculator_state) = &self.calculator_instance {
                    if let Err(err) = calculator_state {
                        ui.label(egui::RichText::from(err.to_string()).color(Color32::RED));

                        if let Some(err) = &self.calculation_err_result {
                            ui.label(egui::RichText::from(format!("Calculation error: {err}")).color(Color32::RED));
                        }
                    }
                }

                
            });
        });
    }
}
