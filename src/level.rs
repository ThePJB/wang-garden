use crate::kmath::*;
use crate::renderer::*;
use crate::rect::*;
use crate::application::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::fs;

pub type Tile = [Vec3; 4];
pub trait TileRotate {
    fn rotate_cw(&self) -> Tile;
    fn rotate_ccw(&self) -> Tile;
}

impl TileRotate for Tile {
    fn rotate_cw(&self) -> Tile {
        [self[3], self[0], self[1], self[2]]
    }
    fn rotate_ccw(&self) -> Tile {
        [self[1], self[2], self[3], self[0]]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelData {
    pub name: String,
    pub number: i32,
    pub w: usize,
    pub h: usize,
    pub tile_choices: Vec<Tile>,
    pub fixed_tiles: Vec<Option<Tile>>,
}


impl LevelData {
    pub fn fresh_solution(&self) -> Vec<Option<Tile>> {
        self.fixed_tiles.clone()
    }

    pub fn new(number: i32, name: String, w: usize, h: usize, tile_choices: Vec<Tile>) -> LevelData {
        LevelData {
            name,
            number,
            w,
            h,
            tile_choices,
            fixed_tiles: vec![None; w*h],
        }
    }

    pub fn load(json_str: String) -> LevelData {
        serde_json::from_str(&json_str).unwrap()
    }

    pub fn save(&self) {
        let path = format!("{}{:03} - {}.json", LEVEL_PATH, self.number, self.name);
        let json_str = serde_json::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json_str.as_bytes()).unwrap();
    }
}




pub struct Level {
    // pub data: LevelData,
    pub current_solution: Vec<Option<Tile>>,
    pub selected_tile: Tile,
    pub selected_tile_idx: i32,
}

#[derive(Clone, Copy)]
pub enum GUIElement {
    Background,
    Menu,
    Game,
    GameBoard,
    MenuTile(usize),
    GameTile(usize),
    GridLineH,
    GridLineV,
    SelectionIndicator,
}


/*

impl Level {



    pub fn draw(&self, renderer: &mut Renderer) {
        let empty_colour = Vec4::new(0.2, 0.2, 0.2, 1.0);
        let fixed_t = 0.4;

        for (elem_type, rect) in self.gui_elements.iter() {
            match elem_type {
                GUIElement::GameTile(i) => {
                    if let Some(tile) = self.placed_tiles[*i] {
                        if self.fixed[*i] {
                            renderer.draw_tile_reverse_bevel(*rect, tile[0], tile[1], tile[2], tile[3], 10.0, 1.0);
                        } else {
                            renderer.draw_tile(*rect, tile[0], tile[1], tile[2], tile[3], 10.0, 1.0);
                        }
                    } else {
                        renderer.draw_rect(*rect, empty_colour, 10.0);
                    }

                    if let Some(idx) = self.rollover_tile {
                        if idx == *i && self.selected_tile.is_some() && !self.fixed[*i] {
                            let tile = self.selected_tile.unwrap();
                            renderer.draw_tile(*rect, tile[0], tile[1], tile[2], tile[3], 15.0, 0.5);
                        }
                    }
                },
                GUIElement::MenuTile(i) => {
                    let tile = self.tile_choices[*i];
                    renderer.draw_tile(*rect, tile[0], tile[1], tile[2], tile[3], 10.0, 5.0);
                },
                GUIElement::Background => {
                    renderer.draw_rect(*rect, Vec4::new(0.4, 0.4, 0.4, 1.0), 1.0);
                }
                GUIElement::SelectionIndicator => {
                    renderer.draw_rect(*rect, Vec4::new(1.0, 1.0, 0.0, 1.0), 4.0);
                }
                /*
                GUIElement::GridLineH => {
                    renderer.draw_rect(rect.child(0.0, 0.0, 1.0, 0.5), Vec3::new(1.0, 1.0, 1.0), 20.0);
                    renderer.draw_rect(rect.child(0.0, 0.5, 1.0, 0.5), Vec3::new(0.0, 0.0, 0.0), 20.0);
                }
                GUIElement::GridLineV => {
                    renderer.draw_rect(rect.child(0.0, 0.0, 0.5, 1.0), Vec3::new(1.0, 1.0, 1.0), 20.0);
                    renderer.draw_rect(rect.child(0.5, 0.0, 0.5, 1.0), Vec3::new(0.0, 0.0, 0.0), 20.0);
                }
                */
                _ => {},
            }
        }
    }
    
    pub fn handle_click(&mut self, p: Vec2) {
        for (elem_type, rect) in self.gui_elements.iter() {
            if !rect.contains(p) {
                continue;
            }
            match elem_type {
                GUIElement::GameTile(i) => {
                    if let Some(place_tile) = self.selected_tile {
                        if self.accept(place_tile, *i) {
                            self.placed_tiles[*i] = self.selected_tile;
                        }
                    }
                },
                GUIElement::MenuTile(i) => {
                    self.selected_tile = Some(self.tile_choices[*i]);
                },
                _ => {},
            }
        }
    }
    pub fn handle_right_click(&mut self, p: Vec2) {
        for (elem_type, rect) in self.gui_elements.iter() {
            if !rect.contains(p) {
                continue;
            }
            match elem_type {
                GUIElement::GameTile(i) => {
                    if !self.fixed[*i] {
                        self.placed_tiles[*i] = None;
                    }
                },
                _ => {},
            }
        }
    }
}
*/

pub fn draw_level(renderer: &mut Renderer, cursor_pos: Vec2, selected_tile: Tile, tile_choices: &[Tile], tiles: &[Option<Tile>], fixed: &[bool], gui_elements: &[(GUIElement, Rect)]) {
    let empty_colour = Vec4::new(0.2, 0.2, 0.2, 1.0);
    let fixed_t = 0.4;

    for (elem_type, rect) in gui_elements.iter() {
        match elem_type {
            GUIElement::GameTile(i) => {
                if let Some(tile) = tiles[*i] {
                    if fixed[*i] {
                        renderer.draw_tile_reverse_bevel(*rect, tile[0], tile[1], tile[2], tile[3], 10.0, 1.0);
                    } else {
                        renderer.draw_tile(*rect, tile[0], tile[1], tile[2], tile[3], 10.0, 1.0);
                    }
                } else {
                    renderer.draw_rect(*rect, empty_colour, 10.0);
                }

                if let Some(idx) = GetClickedGameTile(cursor_pos, gui_elements) {
                    if idx == *i && !fixed[*i] {
                        renderer.draw_tile(*rect, selected_tile[0], selected_tile[1], selected_tile[2], selected_tile[3], 15.0, 0.5);
                    }
                }
            },
            GUIElement::MenuTile(i) => {
                let tile = tile_choices[*i];
                renderer.draw_tile(*rect, tile[0], tile[1], tile[2], tile[3], 10.0, 5.0);
            },
            GUIElement::Background => {
                renderer.draw_rect(*rect, Vec4::new(0.4, 0.4, 0.4, 1.0), 1.0);
            }
            GUIElement::SelectionIndicator => {
                renderer.draw_rect(*rect, Vec4::new(1.0, 1.0, 0.0, 1.0), 4.0);
            }
            _ => {},
        }
    }
}

pub fn accept(w: usize, h: usize, tiles: &Vec<Option<Tile>>, place_tile: Tile, place_idx: usize) -> bool {
    let x = place_idx % w;
    let y = place_idx / w;

    if x != 0 {
        if let Some(neigh) = tiles[place_idx - 1] {
            if neigh[1] != place_tile[3] {
                println!("reject left edge neighbour");
                return false;
            }
        }
    }

    if x != w - 1 {
        if let Some(neigh) = tiles[place_idx + 1] {
            if neigh[3] != place_tile[1] {
                println!("reject right edge neighbour");
                return false;
            }
        }
    }

    if y != h - 1 {
        if let Some(neigh) = tiles[place_idx + w] {
            if neigh[0] != place_tile[2] {
                println!("reject bottom edge neighbour");
                return false;
            }
        }
    }
    
    if y != 0 {
        if let Some(neigh) = tiles[place_idx - w] {
            if neigh[2] != place_tile[0] {
                println!("reject top edge neighbour");
                return false;
            }
        }
    }
    true
}

pub fn calculate_gui(n_tiles: usize, w: usize, h: usize, aspect_ratio: f32, selected_tile: i32) -> Vec<(GUIElement, Rect)> {
    let mut vec = Vec::new();

    let screen_rect = Rect::new(0.0, 0.0, 1.0, 1.0);
    vec.push((GUIElement::Background, screen_rect));

    let menu_pane_w = 0.15;
    let menu_rect = screen_rect.child(0.0, 0.0, menu_pane_w, 1.0).dilate(-0.02);
    vec.push((GUIElement::Menu, menu_rect));
    
    let game_rect = screen_rect.child(menu_pane_w, 0.0, 1.0 - menu_pane_w, 1.0);
    let board_rect = game_rect.dilate(-0.11).child_with_aspect_ratio(w as f32 / h as f32 / aspect_ratio);
    vec.push((GUIElement::GameBoard, board_rect));
    
    let menu_tile_size = 0.15;
    for i in 0..n_tiles {
        let sel_rect = menu_rect.child(0.0, i as f32 * menu_tile_size, 1.0, menu_tile_size);
        let choice_rect = sel_rect.dilate(-0.002).child_with_aspect_ratio(1.0 / aspect_ratio);
        vec.push((GUIElement::MenuTile(i), choice_rect));
        if i == selected_tile as usize {
            vec.push((GUIElement::SelectionIndicator, sel_rect));
        }
    }
    
    for i in 0..w {
        for j in 0..h {
            let tile_rect = board_rect.child(i as f32 / w as f32, j as f32 / h as f32, 1.0 / w as f32, 1.0 / h as f32); //.dilate(-0.002);
            vec.push((GUIElement::GameTile(j*w + i), tile_rect));
        }
    }

    vec
}

pub fn GetClickedGameTile(p: Vec2, gui: &[(GUIElement, Rect)]) -> Option<(usize)> {
    for (element, rect) in gui {
        if rect.contains(p) {
            match element {
                GUIElement::GameTile(idx) => {return Some(*idx)},
                _ => {},
            }
        }
    }

    None

}

pub fn GetClickedMenuTile(p: Vec2, gui: &Vec<(GUIElement, Rect)>) -> Option<(usize)> {
    for (element, rect) in gui {
        if rect.contains(p) {
            match element {
                GUIElement::MenuTile(idx) => {return Some(*idx)},
                _ => {},
            }
        }
    }

    None

}