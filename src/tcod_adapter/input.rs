use tcod::{ Console, Key };
use tcod::Key::Special;
use tcod::Key::Printable;
use tcod::KeyCode::{Up, Down, Left, Right, Escape, Spacebar};

use input;
use input::Command;
use world::Direction::*;

pub struct Input;

impl Input {
    pub fn new() -> Input {
        Input
    }

    fn wait_for_keypress() -> Key {
        let keypress = Console::wait_for_keypress(true);
        keypress.key
    }

    fn is_window_closed() -> bool {
        Console::window_closed()
    }
}

impl input::Input for Input {
    fn wait_for_action(&self) -> input::Command {
        if Input::is_window_closed() {
            return Command::Exit
        }

        let key = Input::wait_for_keypress();
        match key {
            Special(Escape) => Command::Exit,
            Special(Up)    | Printable('w') => Command::Walk(N),
                             Printable('q') => Command::Walk(NW),
                             Printable('e') => Command::Walk(NE),
            Special(Down)  | Printable('s') => Command::Walk(S),
                             Printable('z') => Command::Walk(SW),
                             Printable('c') => Command::Walk(SE),
            Special(Left)  | Printable('a') => Command::Walk(W),
            Special(Right) | Printable('d') => Command::Walk(E),

                             Printable('R') => Command::Rest,
                             Printable('l') => Command::Look,
            Special(Spacebar)               => Command::Auto,
            _ => Command::Unknown(format!("Unmapped key {}", key).to_string())
        }
    }
}
