use crate::level::*;
use crate::renderer::*;
use crate::kmath::*;
use crate::rect::*;
use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;
use glutin::event::VirtualKeyCode;

pub const LEVEL_PATH: &'static str = "./levels/";

pub struct Application {
    level_datas: Vec<LevelData>,
    levels: Vec<Level>,
    current_level: usize,

    editing: bool,
    // bool editor etc
    frame_gui: Vec<(GUIElement, Rect)>,
}

impl Application {
    pub fn new() -> Application {
        let level_datas = load_level_data();
        let mut levels =  Vec::new();

        for ld in level_datas.iter() {
            levels.push(Level {
                // data: ld,
                current_solution: ld.fresh_solution(),
                selected_tile: ld.tile_choices[0],
                selected_tile_idx: 0,
            });
        }

        Application {
            level_datas,
            levels,
            current_level: 0,
            editing: false,
            frame_gui: Vec::new(),
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, aspect_ratio: f32, cursor_pos: Vec2) {
        let l = &self.levels[self.current_level];
        let ld = &self.level_datas[self.current_level];
        self.frame_gui = calculate_gui(ld.tile_choices.len(), ld.w, ld.h, aspect_ratio, l.selected_tile_idx);

        let fixed: Vec<bool> = ld.fixed_tiles.iter().map(|x| x.is_some()).collect();
        draw_level(renderer, cursor_pos, l.selected_tile, &ld.tile_choices, &l.current_solution, &fixed, &self.frame_gui);
    }

    pub fn lmb(&mut self, p: Vec2) {
        let l = &mut self.levels[self.current_level];
        let ld = &mut self.level_datas[self.current_level];

        if let Some(idx) = GetClickedGameTile(p, &self.frame_gui) {
            match self.editing {
                false => {
                    if accept(ld.w, ld.h, &l.current_solution, l.selected_tile, idx) {
                        l.current_solution[idx] = Some(l.selected_tile);
                    }
                },
                true => {
                    l.current_solution[idx] = Some(l.selected_tile);
                    ld.fixed_tiles[idx] = Some(l.selected_tile);
                },
            }
        }

        if let Some(idx) = GetClickedMenuTile(p, &self.frame_gui) {
            l.selected_tile_idx = idx as i32;
            l.selected_tile = ld.tile_choices[idx];
        }
    }
    pub fn rmb(&mut self, p: Vec2) {
        let l = &mut self.levels[self.current_level];
        let ld = &mut self.level_datas[self.current_level];

        if let Some(idx) = GetClickedGameTile(p, &self.frame_gui) {
            match self.editing {
                false => {
                    if ld.fixed_tiles[idx].is_none() {
                        l.current_solution[idx] = None;
                    }
                },
                true => {
                    l.current_solution[idx] = None;
                    ld.fixed_tiles[idx] = None;
                },
            }
        }
    }
    pub fn key_press(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::M => {
                if self.current_level < self.levels.len() - 1 {
                    self.current_level += 1;
                }
                let ld = &self.level_datas[self.current_level];
                println!("{} - {}", self.current_level, ld.name)
            },
            VirtualKeyCode::N => {
                if self.current_level > 0 {
                    self.current_level -= 1;
                }
                let ld = &self.level_datas[self.current_level];
                println!("{} - {}", self.current_level, ld.name)
            },
            VirtualKeyCode::Q => {
                self.levels[self.current_level].selected_tile = self.levels[self.current_level].selected_tile.rotate_ccw();
            },
            VirtualKeyCode::E => {
                self.levels[self.current_level].selected_tile = self.levels[self.current_level].selected_tile.rotate_cw();
            },
            VirtualKeyCode::P => {
                if self.editing {
                    println!("editing off");
                    self.editing = false;
                } else {
                    println!("editing on");
                    self.editing = true;
                }
            },
            VirtualKeyCode::S => {
                if self.editing {
                    println!("saving level");
                    self.level_datas[self.current_level].save();
                } else {
                    println!("editing is off");
                }
            },
            _ => {},
        }
    }
}

pub fn load_level_data() -> Vec<LevelData> {
    let mut entries: Vec<String> = fs::read_dir(LEVEL_PATH).unwrap()
        .filter_map(|res| res.ok())
        .map(|res| res.path().into_os_string().into_string().unwrap())
        .collect();

    let mut level_priorities = HashMap::new();
    level_priorities.insert("diamond", 1.0);
    level_priorities.insert("chevron", 2.0);
    level_priorities.insert("stonks", 3.0);
    level_priorities.insert("stripes", 4.0);
    level_priorities.insert("redirect", 5.0);
    level_priorities.insert("bevel", 6.0);
    level_priorities.insert("tricolor", 10.0);
    level_priorities.insert("beaut2", 11.0);
    
    
    let mut level_datas = Vec::new();
    
    for path in entries {
        let mut file = fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        level_datas.push(LevelData::load(contents));
    }    
    level_datas.sort_by_key(|x| (level_priorities.get(&x.name as &str).unwrap_or(&99999.0) * 10000.0) as u64);
    
    level_datas
}