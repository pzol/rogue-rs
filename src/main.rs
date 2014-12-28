#![feature(globs)]
extern crate tcod;
use tcod::{Console, BackgroundFlag, Color};
use tcod::Map;
use tcod::Key::Special;
use tcod::Key::Printable;
use tcod::KeyCode::{Up, Down, Left, Right, Escape, Spacebar};
pub use mob::Mob;
pub use world::{ World, Action, Point, Tile };
pub use world::Direction::*;

pub mod mob;
pub mod world;

fn render(con: &mut Console, world: &World, hero: &Mob) {
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
            match *tile {
                Tile::Wall => {
                    con.set_char_background(x, y,  dark_wall, BackgroundFlag::Set);
                    map.set(x, y, false, false)
                }
                _  => {
                    con.set_char_background(x, y,  dark_ground, BackgroundFlag::Set);
                    con.put_char(x, y, tile.to_char(), BackgroundFlag::None);
                    map.set(x, y, true, true)
                }
            }

            x += 1;
        }
        y += 1;
    }

    con.put_char(hero.point.x as int, hero.point.y as int, '@', BackgroundFlag::Set);
    Console::flush();
}

fn main() {
    let mut world = World::new();
    let mut con = Console::init_root(world.max_x as int + 1, world.max_y as int + 1, "Rogue", false);
    let mut exit = false;
    let mut mobs : Vec<Mob> = vec![Mob::new(40, 25)];
    let hero = &mut mobs[0];

    render(&mut con, &world, hero);
    while !(Console::window_closed() || exit) {
        let keypress = Console::wait_for_keypress(true);
        match keypress.key {
            Special(Escape) => exit = true,
            Special(Up)    | Printable('w') => hero.act(&mut world, Action::Walk(N)),
                             Printable('q') => hero.act(&mut world, Action::Walk(NW)),
                             Printable('e') => hero.act(&mut world, Action::Walk(NE)),
            Special(Down)  | Printable('s') => hero.act(&mut world, Action::Walk(S)),
                             Printable('z') => hero.act(&mut world, Action::Walk(SW)),
                             Printable('c') => hero.act(&mut world, Action::Walk(SE)),
            Special(Left)  | Printable('a') => hero.act(&mut world, Action::Walk(W)),
            Special(Right) | Printable('d') => hero.act(&mut world, Action::Walk(E)),

                             Printable('R') => hero.act(&mut world, Action::Rest),
            Special(Spacebar)               => hero.act(&mut world, Action::Auto),
            _ => println!("Unmapped key {}", keypress.key)
        }

        render(&mut con, &world, hero);
    }
}
