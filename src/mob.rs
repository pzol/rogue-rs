use world::Pos;

#[allow(dead_code)]
#[deriving(Clone, Show)]
pub struct Mob {
    pub pos: Pos,
    pub display_char: char,
    pub name: String,
    pub kind: MobKind,
    pub str: uint,
    int: uint,
    con: uint,
    dex: uint,
    ap: uint,
    pub hp: uint
}

impl Mob {
    pub fn new(name: &str, kind: MobKind, x: uint, y: uint) -> Mob {
        Mob {
            name: name.to_string(),
            pos: Pos { x: x, y: y },
            kind: kind,
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

    pub fn inc_hp(&mut self, hp: uint) {
        self.hp += hp;
    }

    pub fn dec_hp(&mut self, hp: uint) {
        self.hp -= hp;
    }
}

#[deriving(Copy, Clone, Show)]
pub enum MobKind {
    Hero   = '@' as int,
    Canine = 'C' as int
}

impl MobKind {
    pub fn to_char(&self) -> char {
        (*self).clone() as int as u8 as char
    }
}
