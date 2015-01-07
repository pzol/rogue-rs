use world::Pos;

use std::cell::Cell;
use std::rand::Rng;
use std::num::FromPrimitive;

pub mod behavior;

pub fn rnd() -> Mob {
    let mut rng = ::std::rand::thread_rng();
    let rnd = rng.gen_range(1, 4);
    let rx  = rng.gen_range(1, 80); //FIXME
    let ry  = rng.gen_range(1, 50);

    match FromPrimitive::from_int(rnd as int) {
        Some(kind) => match kind {
            Kind::Canine => Mob {
                name: "Fido".to_string(),
                pos: Cell::new(Pos { x: rx, y: ry }),
                kind: kind,
                hp: Cell::new(7), str: 5,
                ap: 1, int: 7, con: 7, dex: 7, display_char: kind.to_char(), behavior: behavior::Kind::Animalic
            },
            Kind::Hobgoblin => Mob {
                name: "Gardhur".to_string(),
                pos: Cell::new(Pos { x: rx, y: ry }),
                kind: kind,
                hp: Cell::new(12), str: 12,
                ap: 1, int: 7, con: 7, dex: 7, display_char: kind.to_char(), behavior: behavior::Kind::Animalic
            },
            Kind::Orc => Mob {
                name: "Gardhur".to_string(),
                pos: Cell::new(Pos { x: rx, y: ry }),
                kind: kind,
                hp: Cell::new(15), str: 14,
                ap: 1, int: 7, con: 7, dex: 7, display_char: kind.to_char(), behavior: behavior::Kind::Animalic
            },
            _ => panic!("Can't spawn a hero")
        },
        None       => panic!("Invalid monster kind")
    }
}

#[allow(dead_code)]
#[derive(Clone, Show)]
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
    pub hp: Cell<u32>,
    pub behavior: behavior::Kind
}

impl<'a> Mob {
    pub fn new(name: &'a str, kind: Kind, x: uint, y: uint, behavior: behavior::Kind) -> Mob {
        Mob {
            name: name.to_string(),
            pos: Cell::new(Pos { x: x, y: y }),
            kind: kind,
            ap: 1, 
            hp: Cell::new(10),
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

    pub fn inc_hp(&self, inc: u32) {
        self.hp.set(self.hp.get() + inc);
    }

    pub fn dec_hp(&self, dec: u32) {
        let hp = self.hp.get();

        if dec > hp {
            self.hp.set(0)
        } else {
            self.hp.set(hp - dec);    
        }
        
    }
}

#[derive(Copy, Clone, Show, PartialEq, FromPrimitive)]
pub enum Kind {
    Hero,      //= '@' as int,
    Canine,    //= 'C' as int,
    Hobgoblin,  //= 'h' as int
    Orc
}

impl Kind {
    pub fn to_char(&self) -> char {
        match *self {
            Kind::Hero => '@',
            Kind::Canine => 'C',
            Kind::Hobgoblin => 'h',
            Kind::Orc => 'O'
        }
    }
}
