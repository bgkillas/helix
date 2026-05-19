#![allow(clippy::shadow_reuse)]
use better_explosions::line::LineIter;
use eframe::{Frame, NativeOptions};
use egui::{CentralPanel, ComboBox, DragValue, Ui};
use noita_api::{Chunk, GameGlobal, StdBox};
fn main() -> eframe::Result {
    let game_global = GameGlobal::global();
    let grid_world = game_global.m_grid_world;
    let mut chunk_map = grid_world.chunk_map.chunk_array;
    chunk_map[256][256] = Some(StdBox::new(Chunk::default()));
    chunk_map[255][256] = Some(StdBox::new(Chunk::default()));
    chunk_map[256][255] = Some(StdBox::new(Chunk::default()));
    chunk_map[255][255] = Some(StdBox::new(Chunk::default()));
    eframe::run_native(
        "explosions",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::<App>::default())),
    )
}
#[derive(Debug)]
struct App {
    wand: Wand,
}
impl Default for App {
    fn default() -> Self {
        Self {
            wand: Wand::Line(0, 0, 0, 0),
        }
    }
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
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            if ui.button("Apply").clicked() {
                #[allow(clippy::match_same_arms)]
                match self.wand {
                    Wand::Explosive => {
                        //TODO
                    }
                    Wand::Line(x0, y0, x1, y1) => {
                        let game_global = GameGlobal::global();
                        let grid_world = game_global.m_grid_world;
                        let chunk_map = grid_world.chunk_map.chunk_array;
                        let mut chunk_x = x0.div_euclid(512);
                        let mut chunk_y = y0.div_euclid(512);
                        if let Some(mut chunk) = chunk_map[(256 + chunk_y).cast_unsigned()]
                            [(256 + chunk_x).cast_unsigned()]
                        {
                            for (x, y) in LineIter::new(x0, y0, x1, y1) {
                                let px = x.rem_euclid(512).cast_unsigned();
                                let py = y.rem_euclid(512).cast_unsigned();
                                if px == 0 || py == 0 || px == 511 || py == 511 {
                                    chunk_x = x.div_euclid(512);
                                    chunk_y = y.div_euclid(512);
                                    if let Some(c) = chunk_map[(256 + chunk_y).cast_unsigned()]
                                        [(256 + chunk_x).cast_unsigned()]
                                    {
                                        chunk = c;
                                    } else {
                                        break;
                                    }
                                }
                                let c = &mut chunk[py][px];
                                if let Some(p) = c {
                                    p.ptr.free();
                                    *c = None;
                                }
                            }
                        }
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
            #[allow(clippy::match_same_arms)]
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
