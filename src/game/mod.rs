use io;
use mob;
use world::World;

#[deriving(Clone, Show)]
pub enum Action {
    Exit,
    Mob(mob::Action),
    Unknown(String) 
}

pub struct Game {
    world: Box<World>,
    mobs:  Vec<mob::Mob>
}

impl Game {
    pub fn new() -> Game {
        Game { world: box World::new(), mobs: vec![] }
    }

    pub fn add_mob(&mut self, mob: mob::Mob) {
        self.mobs.push(mob)
    }
}
