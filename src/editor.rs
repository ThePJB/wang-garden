/*
make editing pleasant - need to be able to do name, w, h, tile pallette

hardcode level order and then just have it be lexicographic
dont use order but use f32 so u can insert in the middle


hybrid tui editor is the go probably
 * dont worry about order, impose that as a hashtable to names or whatever in the code
 * add a new level
 * select a level
 * change selected level name
 * change selected level tiles - launches a game window -- lazy option: keep it separate and do it now with edit mode
  * chevron a rrbb
  * stonks r rrbb
  * <levelname> <command> 
  * test new
  * test add rrbb
  * test remove rrbb

*/

use wang::application::*;
use wang::level::*;
use wang::kmath::*;

use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn main() {
    println!("Wang Editor\n\nLevels:");

    let mut levels = load_level_data();
    for level in levels.iter() {
        println!("\t{}", level.name);
    }

    let mut colours = HashMap::new();
    colours.insert('r', Vec3::new(1.0, 0.0, 0.0));
    colours.insert('g', Vec3::new(0.0, 1.0, 0.0));
    colours.insert('u', Vec3::new(0.0, 0.0, 1.0));
    colours.insert('b', Vec3::new(0.0, 0.0, 0.0));
    colours.insert('w', Vec3::new(1.0, 1.0, 1.0));
    colours.insert('m', Vec3::new(1.0, 0.0, 1.0));
    colours.insert('y', Vec3::new(1.0, 1.0, 0.0));
    colours.insert('c', Vec3::new(0.0, 1.0, 1.0));


    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {

        let args: Vec<String> = line.unwrap().split(" ").filter(|x| x.len() > 0).map(|x| x.to_owned()).collect();
        if args.len() == 0 {
            continue;
        }

        match &*args[0] {
            "help" => {println!("helping")},
            "new" => {
                if args.len() == 4 {
                    let name = args[1].clone();
                    if let Ok(width) = args[2].parse::<usize>() {
                        if let Ok(height) = args[3].parse::<usize>() {
                            let new_level = LevelData::new(name.to_string(), width, height, Vec::new());
                            new_level.save();
                            levels.push(new_level);
                        } else {
                            println!("invalid height")
                        }
                    } else {
                        println!("invalid width")
                    }
                } else {
                    println!("invalid number of args to new");
                }
            },
            "tadd" => {
                if args.len() == 3 {
                    let name = args[1].clone();
                    if let Some(mut level) = levels.iter_mut().find(|x| x.name == name) {
                        let tile_str = args[2].clone();
                        if tile_str.len() == 4 {
                            let mut tile = Vec::new();
                            for c in tile_str.chars() {
                                if let Some(colour) = colours.get(&c) {
                                    tile.push(colour);
                                } else {
                                    println!("bad colour: {}", c);
                                    continue;
                                }
                            }
                            level.tile_choices.push([*tile[0], *tile[1], *tile[2], *tile[3]]);
                            level.save();
                        }

                    } else {
                        println!("level {} not found", name);
                    }
                }
            },
            "trem" => {
                if args.len() == 3 {
                    let name = args[1].clone();
                    if let Some(mut level) = levels.iter_mut().find(|x| x.name == name) {
                        let tile_str = args[2].clone();
                        if tile_str.len() == 4 {
                            let mut tile = Vec::new();
                            for c in tile_str.chars() {
                                if let Some(colour) = colours.get(&c) {
                                    tile.push(colour);
                                } else {
                                    println!("bad colour: {}", c);
                                    continue;
                                }
                            }
                            let t = [*tile[0], *tile[1], *tile[2], *tile[3]];
                            if let Some(idx) = level.tile_choices.iter().position(|x| *x == t) {
                                level.tile_choices.remove(idx);
                            } else {
                                println!("tile not found");
                            }
                            level.save();
                        }

                    } else {
                        println!("level {} not found", name);
                    }
                }
            },
            _ => {println!("invalid command: {}", args[0])},
        }
        




        print!("> ");
        io::stdout().flush().unwrap();
    }
}