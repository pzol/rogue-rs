use std::rand::Rng;
use world;
use mob;

#[deriving(Copy, Clone)]
pub enum Kind {
    Heroic,
    Animalic
}

pub enum Action {
    TryMove(world::Direction),
    Nothing,
}

impl Kind {
    pub fn act(&self, mob: &mob::Mob, hero_pos: world::Pos) -> Action {
        let behvr = match *self {
            Kind::Animalic => Random,
            _        => panic!("{} has no defined behavior")
        };

        behvr.act(mob, hero_pos)
    }   
}

pub trait Behavior {
    fn act(&self, mob: &mob::Mob, hero_pos: world::Pos) -> Action ;
}

pub struct Random;

impl Behavior for Random {

    fn act(&self, mob: &mob::Mob, hero_pos: world::Pos) -> Action {
        let mut rng = ::std::rand::thread_rng();
        let idir = rng.gen_range(world::Direction::NW as uint, world::Direction::SE as uint);

        match FromPrimitive::from_int(idir as int) {
            Some(dir) => Action::TryMove(dir),
            None      => Action::Nothing
        }
    }
}
