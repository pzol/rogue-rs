#![feature(globs)]
extern crate tcod;
use tcod::{Console, BackgroundFlag, Color};
use tcod::Map;
use tcod::Key::Special;
use tcod::Key::Printable;
use tcod::KeyCode::{Up, Down, Left, Right, Escape, Spacebar};
use std::cell::Ref;
pub use mob::Mob;
use world::{ World, Cell, TileKind };
use world::Direction::*;
use game::Action;

pub mod mob;
pub mod world;
pub mod game;

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
        let action = match keypress.key {
            Special(Escape) => Action::Exit,
            Special(Up)    | Printable('w') => Action::Mob(mob::Action::Walk(N)),
                             Printable('q') => Action::Mob(mob::Action::Walk(NW)),
                             Printable('e') => Action::Mob(mob::Action::Walk(NE)),
            Special(Down)  | Printable('s') => Action::Mob(mob::Action::Walk(S)),
                             Printable('z') => Action::Mob(mob::Action::Walk(SW)),
                             Printable('c') => Action::Mob(mob::Action::Walk(SE)),
            Special(Left)  | Printable('a') => Action::Mob(mob::Action::Walk(W)),
            Special(Right) | Printable('d') => Action::Mob(mob::Action::Walk(E)),

                             Printable('R') => Action::Mob(mob::Action::Rest),
            Special(Spacebar)               => Action::Mob(mob::Action::Auto),
            _ => Action::Unknown(format!("Unmapped key {}", keypress.key).to_string())
        };

        match action {
            Action::Exit        => exit = true,
            Action::Unknown(s)  => println!("{}", s),
            Action::Mob(moba)   => {
                hero.act(moba);
                render(&mut con, world.borrow(), hero);
            }
        }
    }
}
