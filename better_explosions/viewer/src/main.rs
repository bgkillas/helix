#![allow(clippy::shadow_reuse)]
use eframe::{Frame, NativeOptions};
use egui::{CentralPanel, ComboBox, DragValue, Ui};
fn main() -> eframe::Result {
    eframe::run_native(
        "explosions",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::<App>::default())),
    )
}
#[derive(Default, Debug)]
struct App {
    wand: Wand,
}
#[derive(Debug)]
enum Wand {
    Explosive,
    Line(isize, isize, isize, isize),
    Arc,
    CellEater,
    SquareEater,
}
impl PartialEq for Wand {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Wand::Explosive, Wand::Explosive)
                | (Wand::Line(_, _, _, _), Wand::Line(_, _, _, _))
                | (Wand::Arc, Wand::Arc)
                | (Wand::CellEater, Wand::CellEater)
                | (Wand::SquareEater, Wand::SquareEater)
        )
    }
}
impl Default for Wand {
    fn default() -> Self {
        Wand::Line(0, 0, 0, 0)
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            if ui.button("Apply").clicked() {
                //TODO
            }
            ComboBox::from_label("Wand")
                .selected_text(match self.wand {
                    Wand::Explosive => "Explosive",
                    Wand::Line(_, _, _, _) => "Line",
                    Wand::Arc => "Arc",
                    Wand::CellEater => "CellEater",
                    Wand::SquareEater => "SquareEater",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.wand, Wand::Explosive, "Explosive");
                    ui.selectable_value(&mut self.wand, Wand::Line(0, 0, 0, 0), "Line");
                    ui.selectable_value(&mut self.wand, Wand::Arc, "Arc");
                    ui.selectable_value(&mut self.wand, Wand::CellEater, "CellEater");
                    ui.selectable_value(&mut self.wand, Wand::SquareEater, "SquareEater");
                });
            match &mut self.wand {
                Wand::Explosive => {
                    //TODO
                }
                Wand::Line(x0, y0, x1, y1) => {
                    ui.add(DragValue::new(x0));
                    ui.add(DragValue::new(y0));
                    ui.add(DragValue::new(x1));
                    ui.add(DragValue::new(y1));
                }
                Wand::Arc => {
                    //TODO
                }
                Wand::CellEater => {
                    //TODO
                }
                Wand::SquareEater => {
                    //TODO
                }
            }
        });
    }
}
