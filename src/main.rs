#![feature(globs)]
extern crate tcod;
use tcod::{Console, BackgroundFlag, Color};
use tcod::Map;
use tcod::Key::Special;
use tcod::Key::Printable;
use tcod::KeyCode::{Up, Down, Left, Right, Escape, Spacebar};
use std::rc::Rc;
use std::cell::{ Ref, RefCell };
pub use mob::Mob;
pub use world::{ World, Action, Cell, TileKind };
pub use world::Direction::*;

pub mod mob;
pub mod world;

fn render<'a>(con: &mut Console, world: Ref<'a, World>, hero: &Mob) {
    con.clear();

    let dark_wall    = Color { r: 0,   g: 0,   b: 100 };
    // let light_wall   = Color { r: 130, g: 110, b: 50  };
    let dark_ground  = Color { r: 50,  g: 50,  b: 150 };
    // let light_ground = Color { r: 200, g: 180, b: 50  };

    let mut map = Map::new(80, 50);
    let mut y = 0i;

    for line in world.map.iter() {
        let mut x = 0i;
        for tile in line.iter() {
            match tile.kind {
                TileKind::Wall => {
                    con.set_char_background(x, y,  dark_wall, BackgroundFlag::Set);
                    map.set(x, y, false, false)
                }
                _  => {
                    con.set_char_background(x, y,  dark_ground, BackgroundFlag::Set);
                    con.put_char(x, y, tile.kind.to_char(), BackgroundFlag::None);
                    map.set(x, y, true, true)
                }
            }

            x += 1;
        }
        y += 1;
    }

    con.put_char(hero.cell.x as int, hero.cell.y as int, '@', BackgroundFlag::Set);
    Console::flush();
}

fn main() {
    let mut exit  = false;
    let world     = World::new();
    let mut con = Console::init_root(world.borrow().max_x as int + 1, world.borrow().max_y as int + 1, "Rogue", false);
    let tile = Cell::new(world.clone(), 40, 25);
    let mut mobs : Vec<Mob> = vec![Mob::new(tile)];
    let hero = &mut mobs[0];

    render(&mut con, world.borrow(), hero);
    while !(Console::window_closed() || exit) {
        let keypress = Console::wait_for_keypress(true);
        match keypress.key {
            Special(Escape) => exit = true,
            Special(Up)    | Printable('w') => hero.act(Action::Walk(N)),
                             Printable('q') => hero.act(Action::Walk(NW)),
                             Printable('e') => hero.act(Action::Walk(NE)),
            Special(Down)  | Printable('s') => hero.act(Action::Walk(S)),
                             Printable('z') => hero.act(Action::Walk(SW)),
                             Printable('c') => hero.act(Action::Walk(SE)),
            Special(Left)  | Printable('a') => hero.act(Action::Walk(W)),
            Special(Right) | Printable('d') => hero.act(Action::Walk(E)),

                             Printable('R') => hero.act(Action::Rest),
            Special(Spacebar)               => hero.act(Action::Auto),
            _ => println!("Unmapped key {}", keypress.key)
        }

        render(&mut con, world.borrow(), hero);
    }
}
