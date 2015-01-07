use world::Direction;

pub trait Input {
    fn wait_for_action(&self) -> Command;
}

#[derive(Clone, Show)]
pub enum Command {
    Exit,
    Unknown(String),

    Walk(Direction),
    Open(Direction),
    Close(Direction),
    Rest,
    Auto,
    Look
}
