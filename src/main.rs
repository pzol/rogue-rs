#![feature(globs)]
extern crate tcod;

use game::Game;
use input::Input;
use tcod_adapter as io;

mod tcod_adapter;
mod input;
mod mob;
mod world;
mod game;

fn main() {
    let mut game = Game::new();
    let hero = mob::Mob::new("Hero", mob::Kind::Hero, 40, 25, mob::Behavior::Heroic);
    let dog  = mob::Mob::new("Fido", mob::Kind::Canine, 42, 26, mob::Behavior::Animalic);
    game.add_mob(hero);
    game.add_mob(dog);
    let inp     = io::input::Input::new();
    let mut out = io::output::Output::new(&game.world);
    
    loop {
        out.render(&game.mobs, &game.world);
        let cmd = inp.wait_for_action();

        match cmd {
            input::Command::Exit => break,
            _ => game.update(cmd)
        }
    }
}
