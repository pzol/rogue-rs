use mob;
use world::{ World, Direction, TileKind };

#[deriving(Clone, Show)]
pub enum Action {
    Exit,
    Unknown(String),

    Walk(Direction),
    Open(Direction),
    Close(Direction),
    Rest,
    Auto
}

pub struct Game {
    pub world: World,
    pub mobs:  Vec<mob::Mob>
}

impl Game {
    pub fn new() -> Game {
        Game { world: World::new(), mobs: vec![] }
    }

    pub fn add_mob(&mut self, mob: mob::Mob) {
        self.mobs.push(mob)
    }

    pub fn act(&mut self, action: Action) {
        println!("{}", action);

        match action {
            Action::Walk(direction)  => self.walk(direction),
            Action::Open(direction)  => self.open_close(direction),
            Action::Close(direction) => self.open_close(direction),
            Action::Rest             => self.mobs[0].rest(),
            Action::Auto             => self.auto(),
            _  => ()
        }
    }

    fn open_close(&mut self, dir: Direction) {
        let pos = self.mobs[0].pos;
        let destination = World::destination(pos, dir);
        let tile = self.world.at(destination).kind;
        match tile {
            TileKind::DoorClosed => self.world.set(destination, TileKind::DoorOpen),
            TileKind::DoorOpen   => self.world.set(destination, TileKind::DoorClosed),
            _ => ()
        }
    }

    fn walk(&mut self, dir: Direction) {
        let mut mob = &mut self.mobs[0];
        let destination = World::destination(mob.pos, dir);
        let tile = self.world.at(destination);

        if tile.is_walkable() {
            mob.goto(destination);
        } else {
            println!("There is a {}", tile.kind);
        }
    }

    fn auto(&mut self) {
        let pos = self.mobs[0].pos;
        let ns = self.world.adjacent(pos);

        for n in ns.iter() {
            let (ref dir, ref tile_kind) = *n;

            match *tile_kind {
                TileKind::DoorClosed => self.open_close(*dir),
                TileKind::DoorOpen   => self.open_close(*dir),
                _ => ()
            }
            println!("{} -> {}", dir.clone(), tile_kind)
        }
    }
}
