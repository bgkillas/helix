#![allow(clippy::shadow_reuse)]
use better_explosions::explosion;
use better_explosions::line::LineIter;
use eframe::{Frame, NativeOptions};
use egui::{
    CentralPanel, CollapsingHeader, Color32, ColorImage, ComboBox, DragValue, Key, Panel, Pos2,
    Rect, ScrollArea, TextureHandle, TextureOptions, Ui, Vec2,
};
use noita_api::{Cell, Chunk, ConfigExplosion, GameGlobal, StdBox};
use rand::RngExt as _;
use std::collections::HashMap;
fn main() -> eframe::Result {
    let mut game_global = GameGlobal::global();
    game_global
        .m_cell_factory
        .generate_cell_data(include_str!("../materials.xml"))
        .unwrap();
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
#[allow(unused)]
struct App {
    wand: Wand,
    menu: Menu,
    paint: u16,
    other: u16,
    other_chance: f32,
    update_textures: bool,
    unloaded: HashMap<(u16, u16), StdBox<Chunk>>,
    textures: HashMap<(u16, u16), TextureHandle>,
    zoom: f32,
    offset: Pos2,
}
#[derive(Debug, PartialEq)]
enum Menu {
    Map,
    Materials,
}
impl Default for App {
    fn default() -> Self {
        Self {
            wand: Wand::Line(0, 0, 0, 0),
            menu: Menu::Map,
            paint: 0,
            other: 1,
            other_chance: 0.5,
            update_textures: true,
            unloaded: HashMap::with_capacity(512),
            textures: HashMap::with_capacity(512),
            zoom: 1.0,
            offset: Pos2::new(256.0, 256.0),
        }
    }
}
#[derive(Debug)]
enum Wand {
    Explosive(isize, isize, f32, isize, isize),
    Line(isize, isize, isize, isize),
    Arc,
    CellEater,
    SquareEater,
}
impl PartialEq for Wand {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (
                Wand::Explosive(_, _, _, _, _),
                Wand::Explosive(_, _, _, _, _)
            ) | (Wand::Line(_, _, _, _), Wand::Line(_, _, _, _))
                | (Wand::Arc, Wand::Arc)
                | (Wand::CellEater, Wand::CellEater)
                | (Wand::SquareEater, Wand::SquareEater)
        )
    }
}
impl App {
    pub fn paint_pixel(&self, pixel: &mut Option<Cell<()>>) {
        let mut rng = rand::rng();
        let color = if rng.random_bool(self.other_chance.into()) {
            self.paint
        } else {
            self.other
        };
        if color == 0 {
            if let Some(p) = pixel {
                p.ptr.free();
                *pixel = None;
            }
        } else if let Some(p) = pixel {
            let game_global = GameGlobal::global();
            p.material = StdBox::from(&game_global.m_cell_factory.cell_data[usize::from(color)]);
            p.hp = p.material.hp;
        } else {
            let game_global = GameGlobal::global();
            let cell = Cell::new(StdBox::from(
                &game_global.m_cell_factory.cell_data[usize::from(color)],
            ));
            *pixel = Some(cell);
        }
    }
    pub fn update_textures(&mut self, ui: &mut Ui) {
        self.textures.clear();
        let game_global = GameGlobal::global();
        let grid_world = game_global.m_grid_world;
        let chunk_map = grid_world.chunk_map.chunk_array;
        for (x, y, c) in chunk_map.flat_iter() {
            let texture = make_texture(ui, x, y, c);
            self.textures.insert((x, y), texture);
        }
    }
}
fn make_texture(ui: &mut Ui, x: u16, y: u16, chunk: StdBox<Chunk>) -> TextureHandle {
    let mut vec =
        vec![Color32::from_rgba_premultiplied(60, 60, 140, 255); chunk[0].len() * chunk.len()];
    for (x, y, pixel) in chunk.flat_iter() {
        let color = pixel.material.wang_color;
        vec[usize::from(y) * 512 + usize::from(x)] =
            Color32::from_rgba_premultiplied(color.r, color.g, color.b, color.a);
    }
    let image = ColorImage::new([chunk[0].len(), chunk.len()], vec);
    ui.load_texture(format!("{x}x{y}"), image, TextureOptions::NEAREST)
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        Panel::left("left").show_inside(ui, |ui| {
            if ui.button("Apply").clicked() {
                self.update_textures = true;
                #[allow(clippy::match_same_arms)]
                match self.wand {
                    Wand::Explosive(x0, y0, r, dur, energy) => {
                        let mut config = ConfigExplosion::default();
                        config.explosion_radius = r;
                        config.max_durability_to_destroy = dur;
                        config.ray_energy = energy;
                        let game_global = GameGlobal::global();
                        config.create_cell_material = game_global.m_cell_factory.cell_data
                            [usize::from(self.other)]
                        .name
                        .clone();
                        config.create_cell_probability =
                            truncate_f32(self.other_chance * 100.0).cast_unsigned();
                        config.hole_enabled = true;
                        explosion(
                            &config,
                            noita_api::Vec2 {
                                x: truncate_isize(x0),
                                y: truncate_isize(y0),
                            },
                        );
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
                                self.paint_pixel(&mut chunk[py][px]);
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
            if self.update_textures {
                self.update_textures = false;
                self.update_textures(ui);
            }
            ui.label("main id");
            ui.add(DragValue::new(&mut self.paint));
            ui.label("other id");
            ui.add(DragValue::new(&mut self.other));
            ui.label("other chance");
            ui.add(DragValue::new(&mut self.other_chance));
            ComboBox::from_label("Wand")
                .selected_text(match self.wand {
                    Wand::Explosive(_, _, _, _, _) => "Explosive",
                    Wand::Line(_, _, _, _) => "Line",
                    Wand::Arc => "Arc",
                    Wand::CellEater => "CellEater",
                    Wand::SquareEater => "SquareEater",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.wand,
                        Wand::Explosive(0, 0, 0.0, 0, 0),
                        "Explosive",
                    );
                    ui.selectable_value(&mut self.wand, Wand::Line(0, 0, 0, 0), "Line");
                    ui.selectable_value(&mut self.wand, Wand::Arc, "Arc");
                    ui.selectable_value(&mut self.wand, Wand::CellEater, "CellEater");
                    ui.selectable_value(&mut self.wand, Wand::SquareEater, "SquareEater");
                });
            #[allow(clippy::match_same_arms)]
            match &mut self.wand {
                Wand::Explosive(x0, y0, r, dur, energy) => {
                    ui.label("start x");
                    ui.add(DragValue::new(x0));
                    ui.label("start y");
                    ui.add(DragValue::new(y0));
                    ui.label("radius");
                    ui.add(DragValue::new(r));
                    ui.label("max durability");
                    ui.add(DragValue::new(dur));
                    ui.label("energy");
                    ui.add(DragValue::new(energy));
                }
                Wand::Line(x0, y0, x1, y1) => {
                    ui.label("start x");
                    ui.add(DragValue::new(x0));
                    ui.label("start y");
                    ui.add(DragValue::new(y0));
                    ui.label("end x");
                    ui.add(DragValue::new(x1));
                    ui.label("end y");
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
        Panel::top("top").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.menu, Menu::Map, "Map");
                ui.selectable_value(&mut self.menu, Menu::Materials, "Materials");
            });
        });
        CentralPanel::default().show_inside(ui, |ui| match self.menu {
            Menu::Map => {
                let s = 1.0 / 16.0 / self.zoom;
                if ui
                    .input(|i| i.keys_down.contains(&Key::W) || i.keys_down.contains(&Key::ArrowUp))
                {
                    self.offset.y -= s;
                }
                if ui.input(|i| {
                    i.keys_down.contains(&Key::S) || i.keys_down.contains(&Key::ArrowDown)
                }) {
                    self.offset.y += s;
                }
                if ui.input(|i| {
                    i.keys_down.contains(&Key::A) || i.keys_down.contains(&Key::ArrowLeft)
                }) {
                    self.offset.x -= s;
                }
                if ui.input(|i| {
                    i.keys_down.contains(&Key::D) || i.keys_down.contains(&Key::ArrowRight)
                }) {
                    self.offset.x += s;
                }
                if ui.input(|i| i.key_released(Key::Q)) {
                    self.zoom *= 2.0 / 3.0;
                }
                if ui.input(|i| i.key_released(Key::E)) {
                    self.zoom *= 3.0 / 2.0;
                }
                let tile_size = self.zoom * 512.0;
                for (coord, tex) in &self.textures {
                    let pos = (Pos2::new(f32::from(coord.0), f32::from(coord.1)) - self.offset)
                        * tile_size
                        + ui.max_rect().center().to_vec2();
                    let rect = Rect::from_min_size(pos.to_pos2(), Vec2::splat(tile_size));
                    ui.painter().image(
                        tex.id(),
                        rect,
                        Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
            Menu::Materials => {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let game_global = GameGlobal::global();
                        for cell_data in game_global.m_cell_factory.cell_data.iter() {
                            CollapsingHeader::new(cell_data.name.as_str())
                                .id_salt(cell_data.material_type)
                                .show(ui, |ui| {
                                    ui.label(format!("id: {}", cell_data.material_type));
                                    ui.label(format!("wang_color: {}", cell_data.wang_color));
                                    ui.label(format!("durability: {}", cell_data.durability));
                                    ui.label(format!("hp: {}", cell_data.hp));
                                });
                        }
                    });
            }
        });
    }
}
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::as_conversions)]
fn truncate_isize(f: isize) -> f32 {
    f as f32
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::as_conversions)]
fn truncate_f32(f: f32) -> isize {
    f as isize
}
