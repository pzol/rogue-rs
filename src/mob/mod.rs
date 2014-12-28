use world::{ World, Action, Point, Tile };
use world::Direction;
use world::Direction::*;

pub struct Mob {
    pub point: Point,
    pub str: u32,
    pub int: u32,
    pub con: u32,
    pub dex: u32,
    pub ap: u32,
    pub hp: u32
}

impl Mob {
    pub fn new(x: uint, y: uint) -> Mob {
        Mob { point: Point { x: x, y: y}, ap: 1, hp: 10, str: 7, int: 7, con: 7, dex: 7 }
    }

    fn destination(&self, dir: Direction) -> (uint, uint) {
        let Point { x, y } = self.point;
        match dir {
            H  => (x,     y),
            NW => (x - 1, y - 1),
            N  => (x,     y - 1),
            NE => (x + 1, y - 1),
            W  => (x - 1, y),
            E  => (x + 1, y),
            SW => (x - 1, y + 1),
            S  => (x,     y + 1),
            SE => (x + 1, y + 1),
        }
    }

    fn walk(&mut self, world: &World, dir: Direction) {
        let (x, y) = self.destination(dir);

        if world.at(x, y).is_walkable() {
            self.point.x = x;
            self.point.y = y;
        } else {
            println!("There is a {}", world.at(x, y));
        }
    }

    fn rest(&mut self) {
        self.hp += 1;
    }

    fn adjacent<'a>(&self, world: &'a World) -> Vec<(Direction, Tile)> {
        let dirs = [H, NW, N, NE, W, E, SW, S, SE];
        let mut ts = vec![];
        for dir in dirs.iter() {
            let (x, y) = self.destination(*dir);
            let tile   = world.at(x, y);

            match *tile {
                Tile::Floor | Tile::Wall => (),
                _ => ts.push((*dir, tile.clone()))
            }
        }
        ts
    }

    fn auto(&mut self, world: &mut World) {
        let ns = self.adjacent(world);
        for n in ns.iter() {
            let (dir, ref tile) = *n;

            match *tile {
                Tile::DoorClosed => self.open_close(world, dir),
                Tile::DoorOpen   => self.open_close(world, dir),
                _ => ()
            }
            println!("{} -> {}", dir, tile)
        }
    }

    fn open_close(&mut self, world: &mut World, dir: Direction) {
        let (x, y) = self.destination(dir);

        match *world.at(x, y) {
            Tile::DoorClosed => world.set(x, y, Tile::DoorOpen),
            Tile::DoorOpen   => world.set(x, y, Tile::DoorClosed),
            _ => ()
        }
    }

    pub fn act(&mut self, world: &mut World, action: Action) {
        println!("{}", action);

        match action {
            Action::Walk(direction)  => self.walk(world, direction),
            Action::Open(direction)  => self.open_close(world, direction),
            Action::Close(direction) => self.open_close(world, direction),
            Action::Rest             => self.rest(),
            Action::Auto             => self.auto(world)
        }
    }
}
