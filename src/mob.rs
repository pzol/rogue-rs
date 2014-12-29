use world::Pos;

#[allow(dead_code)]
pub struct Mob {
    pub pos: Pos,
    pub display_char: char,
    pub name: String,
    str: u32,
    int: u32,
    con: u32,
    dex: u32,
    ap: u32,
    hp: u32
}

impl Mob {
    pub fn new(name: &str, kind: Mobs, x: uint, y: uint) -> Mob {
        Mob {
            name: name.to_string(),
            pos: Pos { x: x, y: y }, 
            ap: 1, 
            hp: 10,
            str: 7,
            int: 7,
            con: 7,
            dex: 7,
            display_char: kind.to_char()
        }
    }

    pub fn goto(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn rest(&mut self) {
        self.hp += 1;
    }
}

#[deriving(Copy, Clone)]
pub enum Mobs {
    Hero   = '@' as int,
    Canine = 'C' as int
}

impl Mobs {
    pub fn to_char(&self) -> char {
        (*self).clone() as int as u8 as char
    }
}
