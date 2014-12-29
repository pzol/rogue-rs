#![feature(globs)]
extern crate tcod;

use game::{ Action, Game };

mod io;
pub mod mob;
pub mod world;
pub mod game;

fn main() {
    let mut game = Game::new();
    let hero = mob::Mob::new("Hero", mob::Mobs::Hero, 40, 25);
    let dog  = mob::Mob::new("Fido", mob::Mobs::Canine, 42, 26);
    game.add_mob(hero);
    game.add_mob(dog);
    let mut io   = io::Io::new(&game.world);
    
    loop {
        io.render(&game.mobs, &game.world);
        match io.wait_for_action() {
            Action::Exit        => break,
            Action::Unknown(s)  => println!("{}", s),
            action @ _          => game.act(action)
        }
    }
}
