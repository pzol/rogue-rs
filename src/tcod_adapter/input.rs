use tcod::{ Console, BackgroundFlag, Color, Map, Key };
use tcod::Key::Special;
use tcod::Key::Printable;
use tcod::KeyCode::{Up, Down, Left, Right, Escape, Spacebar};

use game;
use game::Action;
use world;
use world::Direction::*;

pub struct Input;

impl Input {
    pub fn new() -> Input {
        Input
    }
    pub fn wait_for_action(&self) -> game::Action {
        if self.is_window_closed() {
            return Action::Exit
        }

        let key = self.wait_for_keypress();
        match key {
            Special(Escape) => Action::Exit,
            Special(Up)    | Printable('w') => Action::Walk(N),
                             Printable('q') => Action::Walk(NW),
                             Printable('e') => Action::Walk(NE),
            Special(Down)  | Printable('s') => Action::Walk(S),
                             Printable('z') => Action::Walk(SW),
                             Printable('c') => Action::Walk(SE),
            Special(Left)  | Printable('a') => Action::Walk(W),
            Special(Right) | Printable('d') => Action::Walk(E),

                             Printable('R') => Action::Rest,
                             Printable('l') => Action::Look,
            Special(Spacebar)               => Action::Auto,
            _ => Action::Unknown(format!("Unmapped key {}", key).to_string())
        }
    }

    fn wait_for_keypress(&self) -> Key {
        let keypress = Console::wait_for_keypress(true);
        keypress.key
    }

    pub fn is_window_closed(&self) -> bool {
        Console::window_closed()
    }
}
