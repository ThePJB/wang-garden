use crate::level::*;
use crate::kmath::*;

#[test]
fn test_level() {
    let r = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 0.0, 0.0);
    let plenty_of_tiles = vec![[r,r,b,b], [r,r,r,r], [b,b,b,b]];
    let ld = LevelData::new(69, "test level".to_string(), 4, 4, plenty_of_tiles);
    ld.save();
}

/*
pub fn levels_vec() -> Vec<Level> {
    let mut levels = Vec::new();

    let b = Vec3::new(0.0, 0.0, 0.0);
    let u = Vec3::new(0.2, 0.2, 1.0);
    let r = Vec3::new(1.0, 0.2, 0.2);
    let y = Vec3::new(1.0, 1.0, 0.2);
    let g = Vec3::new(0.2, 0.9, 0.2);

    let mut level1 = Level::new(
        "new level 1", 4, 4, &[
            [b,b,r,r],
        ]
    );

    let allb = [b,b,b,b];
    let rrrr = [r,r,r,r];
    let brrb = [b,r,r,b];
    let rbbr = [r,b,b,r];

    level1.set_tile(0, 0, allb);
    level1.set_tile(0, 1, allb);
    level1.set_tile(0, 2, allb);
    level1.set_tile(0, 3, allb);
    level1.set_tile(1, 0, allb);
    level1.set_tile(2, 0, allb);
    level1.set_tile(3, 0, allb);
    level1.set_tile(3, 3, allb);
    level1.set_tile(3, 2, allb);
    level1.set_tile(3, 1, allb);
    level1.set_tile(1, 3, allb);
    level1.set_tile(2, 3, allb);
    level1.set_tile(3, 3, allb);
    levels.push(level1);

    let mut level2 = Level::new(
        "new level 2", 5, 5, &[
            [b,b,r,r],
        ]
    );

    level2.set_tile(0, 0, allb);
    level2.set_tile(0, 1, allb);
    level2.set_tile(0, 2, allb);
    level2.set_tile(0, 3, allb);
    level2.set_tile(0, 4, brrb);
    level2.set_tile(1, 0, allb);
    level2.set_tile(2, 0, allb);
    level2.set_tile(3, 0, allb);
    level2.set_tile(4, 0, brrb);
    level2.set_tile(4, 3, rrrr);
    level2.set_tile(4, 3, rrrr);
    level2.set_tile(4, 2, rrrr);
    level2.set_tile(4, 1, rrrr);
    level2.set_tile(1, 4, rrrr);
    level2.set_tile(2, 4, rrrr);
    level2.set_tile(3, 4, rrrr);
    level2.set_tile(4, 4, rrrr);
    levels.push(level2);

    let mut level2 = Level::new(
        "new level 3", 5, 5, &[
            [b,b,r,r],
        ]
    );

    level2.set_tile(0, 0, brrb.rotate_cw());
    level2.set_tile(0, 1, allb);
    level2.set_tile(0, 2, allb);
    level2.set_tile(0, 3, allb);
    level2.set_tile(0, 4, brrb);
    level2.set_tile(1, 0, allb);
    level2.set_tile(2, 0, allb);
    level2.set_tile(3, 0, allb);
    level2.set_tile(4, 0, brrb.rotate_ccw());
    level2.set_tile(4, 3, rrrr);
    level2.set_tile(4, 3, rrrr);
    level2.set_tile(4, 2, rrrr);
    level2.set_tile(4, 1, rrrr);
    level2.set_tile(1, 4, rrrr);
    level2.set_tile(2, 4, rrrr);
    level2.set_tile(3, 4, rrrr);
    level2.set_tile(4, 4, brrb.rotate_cw());
    levels.push(level2);

    /*

    // super easy
    levels.push(Level::new(
        "level1 -- ezez",
        4, 4, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,b,r,r], 1),
    ]));
    // pretty easy
    levels.push(Level::new(
        "level 2 -- pretty easy 3x4",
        3, 4, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,b,r], 0),
        ([b,b,r,r], 1),
    ]));



    // pretty easy
    levels.push(Level::new(
        "level 4 -- possible, tricky, ass soln. s soln. multi 3+", 5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,b,r], 0),
        ([b,b,r,r], 1),
        ([b,r,r,r], 1),
    ]));

    // pretty hard, symmetry might not work
    levels.push(Level::new(
        "level 5 -- soln",
        5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,b,r], 0),
        ([b,b,r,r], 1),
        ([r,r,r,r], 1),
    ]));
    


    levels.push(Level::new(
        "level 6 -- quite easy might not even need X tile, oh nah you do",
        5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,r,r], 1),
        ([r,b,b,b], 1), // or single red?
        ([b,r,b,r], 0), // one of these, multiple is trivial
    ]));
    levels.push(Level::new(
        "level 7 -- multi diamond, easy good intro",
        4, 4, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,u,b], 1),
        ([b,u,r,b], 0), 
    ]));
    levels.push(Level::new(
        "level 8 -- 4 arrows cool not too hard ",
        4, 4, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,u,b], 1),
        ([b,r,u,r], 0), 
        ([b,u,u,r], 0), 
        ([r,r,u,u], 0), 
    ]));
    levels.push(Level::new(
        "level 9 -- like prev but 5x5 quickly its tiling a 3x3 u square",
        5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,u,b], 1),
        ([b,r,u,r], 0), 
        ([b,u,u,r], 0), 
        ([r,r,u,u], 0), 
        ([r,u,r,u], 0), 
        ([u,u,u,u], 0), 
    ]));
    levels.push(Level::new(
        "level 10 -- pretty easy",
        3, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,r,u,b], 1),
        ([b,r,u,r], 0),
        ([b,u,u,r], 0),
        ([r,r,u,u], 0),
        ([u,u,u,u], 0),
    ]));
    levels.push(Level::new(
        "level 11 -- tricky, very yellow indeed ",
        5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,b,y,g], 1),
        ([b,y,y,y], 0),
        ([b,y,b,g], 0),
        ([g,b,g,b], 0),
        ([g,y,y,y], 0),
    ]));

    // solvable
    levels.push(Level::new(
        "level 12 -- solvable, not too hard, maybe first in this series",
        5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,b,b,y], 1),
        ([b,y,y,y], 0),
        ([g,y,g,b], 0),
        ([g,b,g,b], 0),
        ([g,y,y,y], 0),
    ]));

    levels.push(Level::new(
        "level 13 -- quite easy",
        4, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,b,y,g], 1),
        ([b,y,y,y], 0),
        ([b,y,b,g], 0),
    ]));

    let c = Vec3::new(0.0, 1.0, 1.0);
    let m = Vec3::new(1.0, 0.0, 1.0);

    levels.push(Level::new(
        "level 14 -- cmyk solvable",
        7, 7, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,b,c,c], 1),
        ([b,b,m,m], 1),
        ([b,c,y,m], 1),
        ([b,m,y,c], 1),
        ([c,c,m,m], 1),

        // ([c,c,c,c], 1), //check this doesnt break it
        // ([m,m,m,m], 1),
        ([y,y,y,y], 1), // sufficient but a bit silly, corners op
    ]));



    levels.push(Level::new(
        "level 15 -- cmyk",
        5, 5, Vec3::new(0.0, 0.0, 0.0), &[
        ([b,b,y,y], 1),
        ([y,b,c,c], 1),
        ([b,y,m,m], 1),
        ([b,m,y,c], 1),
        ([c,c,m,m], 1),
        ([c,y,m,y], 1),
        ([y,y,y,y], 1),

    ]));*/


    levels

}

// maybe quantities would be good
// asymmetry only would require I suppose asymmetric tiles

// i feel like theres some theorem that would capture whether you actually can or not
// 5x5 is hard you need a real symmetrical bit as the capstone
// or to be able to make a +
*/