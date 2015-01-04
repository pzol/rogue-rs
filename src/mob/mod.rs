use world::Pos;
use std::cell::Cell;
pub mod behavior;

#[allow(dead_code)]
#[deriving(Clone)]
pub struct Mob {
    pub pos: Cell<Pos>,
    pub display_char: char,
    pub name: String,
    pub kind: Kind,
    pub str: uint,
    int: uint,
    con: uint,
    dex: uint,
    ap: uint,
    pub hp: uint,
    pub behavior: behavior::Kind
}

impl<'a> Mob {
    pub fn new(name: &'a str, kind: Kind, x: uint, y: uint, behavior: behavior::Kind) -> Mob {
        Mob {
            name: name.to_string(),
            pos: Cell::new(Pos { x: x, y: y }),
            kind: kind,
            ap: 1, 
            hp: 10,
            str: 7,
            int: 7,
            con: 7,
            dex: 7,
            display_char: kind.to_char(),
            behavior: behavior
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos.get()
    }

    pub fn goto(&self, pos: Pos) {
        self.pos.set(pos);
    }

    pub fn inc_hp(&mut self, hp: uint) {
        self.hp += hp;
    }

    pub fn dec_hp(&mut self, hp: uint) {
        self.hp -= hp;
    }
}

#[deriving(Copy, Clone, Show)]
pub enum Kind {
    Hero   = '@' as int,
    Canine = 'C' as int,
    Hobgoblin = 'h' as int
}

impl Kind {
    pub fn to_char(&self) -> char {
        (*self).clone() as int as u8 as char
    }
}
