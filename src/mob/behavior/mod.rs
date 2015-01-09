use input::Command;
use game::MobInfo;
use mob;
use dice;
use geo;
use geo::Pos;
use world::TileKind;

use std::num::FromPrimitive;
use std::num::SignedInt;

#[derive(Copy, Clone, Show)]
pub enum Kind {
    Heroic,
    Animalic
}

pub enum Action {
    TryMove(geo::Dir),
    Nothing,
    OpenClose(geo::Dir),
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
                println!("    sees a {} the {}", monster.name, monster.kind);
            }
        }

        match info.input {
            Command::Walk(dir)  => Action::TryMove(dir),
            // Command::Open(direction)  => self.open_close(direction),
            // Command::Close(direction) => self.open_close(direction),
            Command::Rest             => { mob.inc_hp(1); Action::Nothing },
            Command::Auto             => {
                for &(dir, tile_kind) in info.adjacent().iter() {
                    match tile_kind {
                        TileKind::DoorClosed | TileKind::DoorOpen => return Action::OpenClose(dir),
                        _ => ()
                    }
                }

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
        for tile in info.tiles.iter() {
            if let Some(ref monster) = tile.mob {
                let from = mob.pos.get();
                let Pos(from_x, from_y) = from; 
                let Pos(to_x, to_y)     = monster.pos.get();

                let dx = (to_x as i32 - from_x as i32).signum();
                let dy = (to_y as i32 - from_y as i32).signum();

                let dst = geo::Pos(from_x + dx, from_y + dy);
                println!("  sees {}, dst {}, moves towards it", monster.kind, dst);
                return Action::TryMove(from.dir(dst));
            }
        }

        let idir = dice::rand(geo::Dir::NW as uint, geo::Dir::SE as uint);
        println!("  wanders around aimlessly");

        match FromPrimitive::from_int(idir as int) {
            Some(dir) => Action::TryMove(dir),
            None      => Action::Nothing
        }
    }
}
