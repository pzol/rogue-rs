use world::{ Action, Cell, TileKind };
use world::Direction;

pub struct Mob {
    pub cell: Cell,
    str: u32,
    int: u32,
    con: u32,
    dex: u32,
    ap: u32,
    hp: u32
}

impl Mob {
    pub fn new(cell: Cell) -> Mob {
        Mob { cell: cell, ap: 1, hp: 10, str: 7, int: 7, con: 7, dex: 7 }
    }

    fn walk(&mut self, dir: Direction) {
        let n = self.cell.neighbor(dir);

        if n.is_walkable() {
            self.cell = n;
        } else {
            println!("There is a {}", n.kind());
        }
    }

    fn rest(&mut self) {
        self.hp += 1;
    }

    fn auto(&mut self) {
        let ns = self.cell.adjacent();
        for n in ns.iter() {
            let (dir, ref cell) = *n;

            match cell.kind() {
                TileKind::DoorClosed => self.open_close(dir),
                TileKind::DoorOpen   => self.open_close(dir),
                _ => ()
            }
            println!("{} -> {}", dir, cell.kind())
        }
    }

    fn open_close(&mut self, dir: Direction) {
        let mut n = self.cell.neighbor(dir);

        match n.kind() {
            TileKind::DoorClosed => n.set(TileKind::DoorOpen),
            TileKind::DoorOpen   => n.set(TileKind::DoorClosed),
            _ => ()
        }
    }

    pub fn act(&mut self, action: Action) {
        println!("{}", action);

        match action {
            Action::Walk(direction)  => self.walk(direction),
            Action::Open(direction)  => self.open_close(direction),
            Action::Close(direction) => self.open_close(direction),
            Action::Rest             => self.rest(),
            Action::Auto             => self.auto()
        }
    }
}
