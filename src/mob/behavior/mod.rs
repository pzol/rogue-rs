use world;
use input::Command;
use game::MobInfo;
use mob;

use std::rand::Rng;
use std::num::FromPrimitive;
use std::num::SignedInt;

#[derive(Copy, Clone, Show)]
pub enum Kind {
    Heroic,
    Animalic
}

pub enum Action {
    TryMove(world::Direction),
    Nothing,
    Rest
}

impl Kind {
    pub fn act(&self, mob: &mob::Mob, info: MobInfo) -> Action {
        let behvr = match *self {
            Kind::Animalic => box Random as Box<Behavior>,
            Kind::Heroic   => box Hero as Box<Behavior>,
            // _        => panic!("{} has no defined behavior")
        };

        behvr.act(mob, info)
    }   
}

pub trait Behavior {
    fn act(&self, mob: &mob::Mob, info: MobInfo) -> Action ;
}

pub struct Hero;

impl Behavior for Hero {
    fn act(&self, mob: &mob::Mob, info: MobInfo) -> Action {
        for tile in info.tiles.iter() {
            if let Some(ref monster) = tile.mob {
                println!("There is a {} the {}", monster.name, monster.kind);
            }
        }

        match info.input {
            Command::Walk(dir)  => Action::TryMove(dir),
            // Command::Open(direction)  => self.open_close(direction),
            // Command::Close(direction) => self.open_close(direction),
            Command::Rest             => { mob.inc_hp(1); Action::Nothing },
            Command::Auto             => {
                println!("{}", info.adjacent());

                Action::Nothing
            },
            // Command::Look             => self.look(),
            _  => Action::Nothing
        }
    }    
}

pub struct Random;

impl Behavior for Random {

    fn act(&self, mob: &mob::Mob, info: MobInfo) -> Action {
        println!("+ {}", mob.name);
        for tile in info.tiles.iter() {
            if let Some(ref monster) = tile.mob {
                let from = mob.pos.get();
                let to   = monster.pos.get();

                let dx = (to.x as i32 - from.x as i32).signum();
                let dy = (to.y as i32 - from.y as i32).signum();

                let dst = world::Pos { x: from.x + dx as uint, y: from.y + dy as uint };
                println!("{} the {}, sees a {}, dst {}", mob.name, mob.kind, monster.kind, dst);
                return Action::TryMove(from.dir(dst));

            }
        }

        let mut rng = ::std::rand::thread_rng();
        let idir = rng.gen_range(world::Direction::NW as uint, world::Direction::SE as uint);

        match FromPrimitive::from_int(idir as int) {
            Some(dir) => Action::TryMove(dir),
            None      => Action::Nothing
        }
    }
}
