use tcod::{ Console, BackgroundFlag, Color, Map };
use geo;
use world;
use fov;
use mob;
use world::TileKind;

pub struct Output {
    con: Console
}

impl Output {
    pub fn new(world: &world::World) -> Output {
        let con = Console::init_root(world.max_x as int + 1, world.max_y as int + 1, "Rogue", false);
        Output { con: con }
    }

    pub fn render(&mut self, mobs: &Vec<mob::Mob>, world: &world::World) {
        let pos = mobs[0].pos.get();
        let los = fov::los(pos, |pos| world.at(pos).is_translucent());

        self.con.clear();

        let dark_wall    = Color { r: 0,   g: 0,   b: 100 };
        let light_wall   = Color { r: 130, g: 110, b: 50  };
        let dark_ground  = Color { r: 50,  g: 50,  b: 150 };
        let light_ground = Color { r: 200, g: 180, b: 50  };

        let mut map = Map::new(80, 50);
        let mut y = 0i;

        for line in world.map.iter() {
            let mut x = 0i;
            for tile in line.iter() {
                let cpos = geo::Pos(x as i32, y as i32);

                match tile.kind {
                    TileKind::Wall => {
                        let color = if los.contains(&cpos) { light_wall } else { dark_wall };
                        self.con.set_char_background(x, y,  color, BackgroundFlag::Set);
                        map.set(x, y, false, false)
                    }
                    _  => {
                        let color = if los.contains(&cpos) { light_ground } else { dark_ground };
                        self.con.set_char_background(x, y,  color, BackgroundFlag::Set);
                        self.con.put_char(x, y, tile.kind.to_char(), BackgroundFlag::None);
                        map.set(x, y, true, true)
                    }
                }

                x += 1;
            }
            y += 1;
        }

        for (i, mob) in mobs.iter().enumerate() {
            if i == 0 || los.contains(&mob.pos()) {
                let geo::Pos(x, y) = mob.pos();
                self.con.put_char(x as int, y as int, mob.display_char, BackgroundFlag::Set);
            }
        }
        Console::flush();
    }
}
