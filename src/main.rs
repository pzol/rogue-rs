#![feature(globs)]
extern crate tcod;

use game::{ Action, Game };
use tcod_adapter as io;

mod tcod_adapter;
pub mod mob;
pub mod world;
pub mod game;

fn main() {
    let mut game = Game::new();
    let hero = mob::Mob::new("Hero", mob::MobKind::Hero, 40, 25);
    let dog  = mob::Mob::new("Fido", mob::MobKind::Canine, 42, 26);
    game.add_mob(hero);
    game.add_mob(dog);
    let inp     = io::input::Input::new();
    let mut out = io::output::Output::new(&game.world);
    
    loop {
        out.render(&game.mobs, &game.world);
        match inp.wait_for_action() {
            Action::Exit        => break,
            Action::Unknown(s)  => println!("{}", s),
            action @ _          => game.act(action)
        }
    }
}
