use tcod::{ Console, BackgroundFlag, Color, Map, Key };
use tcod::Key::Special;
use tcod::Key::Printable;
use tcod::KeyCode::{Up, Down, Left, Right, Escape, Spacebar};

use game;
use game::Action;
use world;
use world::TileKind;
use world::Direction::*;
use mob;

pub struct Io {
    con: Console,
}

impl Io {
    pub fn new(world: &world::World) -> Io {
        let con = Console::init_root(world.max_x as int + 1, world.max_y as int + 1, "Rogue", false);
        Io { con: con }
    }

    pub fn wait_for_action(&self) -> game::Action {
        if self.is_window_closed() {
            return Action::Exit
        }

        let key = self.wait_for_keypress();
        match key {
            Special(Escape) => Action::Exit,
            Special(Up)    | Printable('w') => Action::Walk(N),
                             Printable('q') => Action::Walk(NW),
                             Printable('e') => Action::Walk(NE),
            Special(Down)  | Printable('s') => Action::Walk(S),
                             Printable('z') => Action::Walk(SW),
                             Printable('c') => Action::Walk(SE),
            Special(Left)  | Printable('a') => Action::Walk(W),
            Special(Right) | Printable('d') => Action::Walk(E),

                             Printable('R') => Action::Rest,
                             Printable('l') => Action::Look,
            Special(Spacebar)               => Action::Auto,
            _ => Action::Unknown(format!("Unmapped key {}", key).to_string())
        }
    }

    fn wait_for_keypress(&self) -> Key {
        let keypress = Console::wait_for_keypress(true);
        keypress.key
    }

    pub fn is_window_closed(&self) -> bool {
        Console::window_closed()
    }

    pub fn render(&mut self, mobs: &Vec<mob::Mob>, world: &world::World) {
        self.con.clear();

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
                        self.con.set_char_background(x, y,  dark_wall, BackgroundFlag::Set);
                        map.set(x, y, false, false)
                    }
                    _  => {
                        self.con.set_char_background(x, y,  dark_ground, BackgroundFlag::Set);
                        self.con.put_char(x, y, tile.kind.to_char(), BackgroundFlag::None);
                        map.set(x, y, true, true)
                    }
                }

                x += 1;
            }
            y += 1;
        }

        for mob in mobs.iter() {
            self.con.put_char(mob.pos.x as int, mob.pos.y as int, mob.display_char, BackgroundFlag::Set);
        }
        Console::flush();
    }
}
