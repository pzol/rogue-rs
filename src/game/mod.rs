use mob;

#[deriving(Clone, Show)]
pub enum Action {
    Exit,
    Mob(mob::Action),
    Unknown(String)
}
