use better_explosions::explosion;
use better_explosions::line::LineIter;
use eframe::Frame;
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::textures::TextureOptions;
use eframe::epaint::{Color32, ColorImage, TextureHandle};
use egui::{
    CentralPanel, CollapsingHeader, ComboBox, DragValue, Key, Panel, PointerButton, ScrollArea, Ui,
};
use noita_api::{Cell, Chunk, ConfigExplosion, GameGlobal, StdBox};
use rand::RngExt as _;
use std::collections::HashMap;
pub fn fill(mat: u16) {
    let game_global = GameGlobal::global();
    let matptr = StdBox::from(&game_global.m_cell_factory.cell_data[usize::from(mat)]);
    let grid_world = game_global.m_grid_world;
    let chunk_map = grid_world.chunk_map.chunk_array;
    for (_, _, mut c) in chunk_map.flat_iter() {
        for (_, _, p) in c.iter_mut() {
            if let Some(ptr) = p {
                ptr.ptr.free();
            }
            *p = Some(Cell::new(matptr));
        }
    }
}
#[allow(unused)]
pub struct App {
    wand: Wand,
    menu: Menu,
    material: u16,
    material_chance: f32,
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
            material: 1,
            material_chance: 0.0,
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
        let color = if rng.random_bool(self.material_chance.into()) {
            self.material
        } else {
            0
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
        let color = pixel.material.graphics.color;
        vec[usize::from(y) * 512 + usize::from(x)] =
            Color32::from_rgba_premultiplied(color.r, color.g, color.b, color.a);
    }
    let image = ColorImage::new([chunk[0].len(), chunk.len()], vec);
    ui.load_texture(format!("{x}x{y}"), image, TextureOptions::NEAREST)
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        Panel::left("left").show_inside(ui, |ui| {
            if ui.button("Apply").clicked() || ui.input(|i| i.key_pressed(Key::Space)) {
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
                            [usize::from(self.material)]
                        .name
                        .clone();
                        config.create_cell_probability =
                            truncate_f32(self.material_chance * 100.0).cast_unsigned();
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
            ui.label("material id");
            ui.add(DragValue::new(&mut self.material));
            ui.label("material chance");
            ui.add(DragValue::new(&mut self.material_chance));
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
                        Wand::Explosive(0, 0, 0.0, isize::MAX, isize::MAX),
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
                let center = ui.max_rect().center().to_vec2();
                let tile_size = self.zoom * 512.0;
                if let Some(pos) = ui.pointer_latest_pos()
                    && ui.max_rect().contains(pos)
                {
                    let get_cursor_pixel = || -> (isize, isize) {
                        let chunk_pos = (pos - center) / tile_size + self.offset.to_vec2();
                        let chunk_rel = chunk_pos - Pos2::new(256.0, 256.0);
                        let pixel_rel = chunk_rel * 512.0;
                        (truncate_f32(pixel_rel.x), truncate_f32(pixel_rel.y))
                    };
                    if ui.input(|i| i.pointer.button_down(PointerButton::Primary)) {
                        let (x, y) = get_cursor_pixel();
                        match &mut self.wand {
                            Wand::Line(x0, y0, _, _) | Wand::Explosive(x0, y0, _, _, _) => {
                                (*x0, *y0) = (x, y);
                            }
                            _ => {}
                        }
                    }
                    if ui.input(|i| i.pointer.button_down(PointerButton::Secondary)) {
                        let (x, y) = get_cursor_pixel();
                        match &mut self.wand {
                            Wand::Line(_, _, x1, y1) => {
                                (*x1, *y1) = (x, y);
                            }
                            Wand::Explosive(x0, y0, r, _, _) => {
                                *r = truncate_isize(x.abs_diff(*x0).cast_signed())
                                    .hypot(truncate_isize(y.abs_diff(*y0).cast_signed()));
                            }
                            _ => {}
                        }
                    }
                    let s = 1.0 / 16.0 / self.zoom;
                    if ui.input(|i| {
                        i.keys_down.contains(&Key::W) || i.keys_down.contains(&Key::ArrowUp)
                    }) {
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
                    if ui.input(|i| i.key_pressed(Key::Q)) {
                        self.zoom *= 2.0 / 3.0;
                    }
                    if ui.input(|i| i.key_pressed(Key::E)) {
                        self.zoom *= 3.0 / 2.0;
                    }
                }
                for (coord, tex) in &self.textures {
                    let pos = (Pos2::new(f32::from(coord.0), f32::from(coord.1)) - self.offset)
                        * tile_size
                        + center;
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
                                    ui.label(format!("color: {}", cell_data.graphics.color));
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
