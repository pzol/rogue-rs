use mob;
use std::rand::Rng;
use world::{ World, Direction, TileKind, Pos };

#[deriving(Clone, Show)]
pub enum Action {
    Exit,
    Unknown(String),

    Walk(Direction),
    Open(Direction),
    Close(Direction),
    Rest,
    Auto
}

pub struct Game {
    pub world: World,
    pub mobs:  Vec<mob::Mob>
}

#[deriving(Clone)]
struct TileInfo {
    is_walkable: bool,
    kind: TileKind,
    mob: Option<mob::Mob>,
    pos: Pos
}

impl TileInfo {
    fn new(game: &Game, pos: Pos) -> TileInfo {
        let tile = game.world.at(pos);
        let mob  = TileInfo::mob_at(&game.mobs, pos);
        TileInfo { pos: pos, is_walkable: tile.is_walkable(), mob: mob, kind: tile.kind }
    }

    fn mob_at(monsters: &Vec<mob::Mob>, pos: Pos) -> Option<mob::Mob> {
        monsters.iter().skip(1).find(|mob| mob.pos == pos).map(|mob| mob.clone())
    }
}

enum Walk<'a> {
    CanWalk(Pos),
    CannotWalk(TileKind),
    OtherMob(Pos, &'a mob::Mob)
}

impl Game {
    pub fn new() -> Game {
        Game { world: World::new(), mobs: vec![] }
    }

    pub fn add_mob(&mut self, mob: mob::Mob) {
        self.mobs.push(mob)
    }

    pub fn act(&mut self, action: Action) {
        println!("{}", action);

        match action {
            Action::Walk(direction)  => self.walk(direction),
            Action::Open(direction)  => self.open_close(direction),
            Action::Close(direction) => self.open_close(direction),
            Action::Rest             => self.mobs[0].dec_hp(1),
            Action::Auto             => self.auto(),
            _  => ()
        }
    }

    fn open_close(&mut self, dir: Direction) {
        let pos = self.mobs[0].pos;
        let dst = World::destination(pos, dir);
        let tile = self.world.at(dst).kind;
        match tile {
            TileKind::DoorClosed => self.world.set(dst, TileKind::DoorOpen),
            TileKind::DoorOpen   => self.world.set(dst, TileKind::DoorClosed),
            _ => ()
        }
    }

    fn current_pos(&mut self) -> Pos {
        self.mobs[0].pos
    }

    fn tile_info(&mut self, pos: Pos) -> TileInfo {
        TileInfo::new(self, pos)
    }

    fn walk(&mut self, dir: Direction) {
        let dst  = World::destination(self.current_pos(), dir);
        let tile = self.tile_info(dst);

        match tile {
            TileInfo { is_walkable: true, mob: Some(_), ..} => self.attack(dst),
            TileInfo { is_walkable: true, mob: None, ..}    => self.move_hero(dst),
            _ => println!("There is a {}", tile.kind)
        }
    }

    fn hero_mut(&mut self) -> &mut mob::Mob {
        &mut self.mobs[0]
    }

    fn move_hero(&mut self, pos: Pos) {
        self.hero_mut().goto(pos);
    }

    fn dmg(mob: &mob::Mob) -> uint {
        let mut rng = ::std::rand::task_rng();
        rng.gen_range(1u, mob.str as uint)
    }

    fn attack(&mut self, pos: Pos) {
        let (hero, mob) = {
            let (heroes, mobs) = self.mobs.split_at_mut(1);
            let idx = mobs.iter().position(|m| m.pos == pos).unwrap();
            (&mut heroes[0], &mut mobs[idx])
        };

        let dmg = Game::dmg(hero);
        mob.dec_hp(dmg);
        println!("You hit {}, the {} for {}/{} hp!", mob.name, mob.kind, dmg, mob.hp);
    }

    fn auto(&mut self) {
        let pos = self.mobs[0].pos;
        let ns  = self.world.adjacent(pos);

        for n in ns.iter() {
            let (ref dir, ref tile_kind) = *n;

            match *tile_kind {
                TileKind::DoorClosed => self.open_close(*dir),
                TileKind::DoorOpen   => self.open_close(*dir),
                _ => ()
            }
            println!("{} -> {}", dir.clone(), tile_kind)
        }
    }
}
