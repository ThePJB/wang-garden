TODO
----

add a 'q, e to rotate tile' somehow
order
better editor lol
fix warnings



Gwan

stretch goal: voxels and tiles fever dream version of the witness
deduce the tile score by the illumination of the boy

maximize an area u just fill ay, payload boy
but bootstrapping can be tricky, least area

YYYY
then Y--- but that are incompatible or barely compatible
or compatible for certain shapes of YYYYs eg 3 in a row

make it isotropic with nonsquare grid

reject invalid placement obviously

make some super atmospheric pixely fog and shaders and stuff

make cool tiled murals of trees and waves etc
try to evoke nature in it
can have hints. wonder how hard it will be...

can manipulate the world, open door closed door etc like in the witness. pattern with very specific level of ambiguity
hardcoded rings, make em fit nicely into the lectern thing
arrange environment to make it obvious
cool voxcellular world

ghost
rotation
aspect ratio correction device / figure out the layout thing
it is actually real annoying maintaining separate draw and click things

layout: good way to abstract it out? eg call a function(rect, type) for each thing

fn layout(F) {
  F(screen_rect, UI::screen)

  menu rect = screen.child1
  F(menu_rect, UI::menu)

  game = screen.child2
  F(game_rect, UI::game)

  for i, tile in tiles
    tile_rect = maths
    F(tile_rect, Tile(i))
  }
}

draw() {
  layout(|r, type| {
    match type {
      tile(i) => renderer.draw(r, level.tiles[i])
      placeytile(i) => renderer.draw(r, level.placeytiles[i])
      scene => renderer.draw(r, grey)
      game => renderer.draw(r, different grey)
    }
  })
}

handle_click(pos p) {
  layout(|r, type| {
    if r.contains(p) {
      match type {
        tile(i) => select i
        placey(i) => place i
        scene |
        game => {},
      }
    }
  })
}

vs vec of gui elements (r, type), kinda needs recalculating if stuff changes
but muh mutability, rust will probably be a silly stuff about that
and supreme flexibility


indicate edge colours
have changing between puzzles
make a bunch of puzzles
implement the aspect ratio thing
transparency for mr rollover

if i show 'just' what tiles ur using then it will be real easy to design levels

// could probably work backwards to create arbitrary patterns
eg life glider
computer assisted checking probably a good idea
existence + uniqueness


ultimately i might just introduce the concept of baked on tiles

rainbow is real pretty i can do it once that is in place

maybe i could have auto symmetry? more satisfy less click bigger grids


// just draw score as white squares or red squares
// stuff font rendering lol
// layout is real good, it could just use like hsplit or split in 2 or something, as well as lock square aspect or something. square child that fits.
// draw grid or something
// draw ghost
// could make it look nice by having kinda 3d ish or stylish tiles, procedurally cracked
// maybe magic theme or something
// nice sounds and maybe it unlocks something
// could be a the witness puzzle
// score display
// dragging is cooler
// position tiles around the board, more unique

// what if there was no numbers but just level of glow of a thing

// you could potentially collect tiles tile types for puzzles in the world... locks and keys with a few extra steps

// stone grinding sound, satisfying rotation etc. drag and scroll, release: go back and snap into place

// level ideas
// x and y: chainy boys with payoff at the end. long payoff and short payoff options. obviously fit the orientation

//start doing it to lay out the rooms
// funny how theres real vague and real not vague ideas eg with policy as well

