#![feature(globs)]
extern crate tcod;

pub use mob::Mob;
use world::{ World, Cell };
use game::{ Action, Game };

mod io;
pub mod mob;
pub mod world;
pub mod game;

fn main() {
    // let mut game = Game::new();

    let world    = World::new_ref();
    let mut io   = io::Io::new(world.clone());
    
    let tile = Cell::new(world.clone(), 40, 25);
    let mut mobs : Vec<Mob> = vec![Mob::new('@', tile)];

    loop {
        io.render(&mobs);
        match io.wait_for_action() {
            Action::Exit        => break,
            Action::Unknown(s)  => println!("{}", s),
            Action::Mob(moba)   => mobs[0].act(moba)
        }
    }
}
