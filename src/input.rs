use geo::Dir;

pub trait Input {
    fn wait_for_action(&self) -> Command;
}

#[derive(Clone, Show)]
pub enum Command {
    Exit,
    Unknown(String),

    Walk(Dir),
    Open(Dir),
    Close(Dir),
    Rest,
    Auto,
    Look
}
