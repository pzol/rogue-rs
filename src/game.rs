extern crate collections;
use std::rand::Rng;

use mob;
use world::{ World, Direction, TileKind, Pos };
use input;
use input::Command;


pub struct Game {
    pub world: World,
    pub mobs: Vec<mob::Mob>
}

/// Combined information about what's going on on a tile
/// which Mob, Items, etc
///
/// its state is frozen during creation and contains no references, so it can be easily copied around
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
        monsters.iter().skip(1).find(|mob| mob.pos() == pos).map(|mob| mob.clone())
    }
}

enum Movement {
    Goto(Pos),
    Attack(Pos),
    Obstacle(TileKind)
}

impl Game {
    pub fn new() -> Game {
        Game { world: World::new(), mobs: vec![] }
    }

    pub fn add_mob(&mut self, mob: mob::Mob) {
        self.mobs.push(mob)
    }

    fn monsters(&self) -> &[mob::Mob] {
        self.mobs.slice_from(1)
    }

    pub fn update(&mut self, cmd: input::Command) {
        self.act(&cmd);
        self.update_monsters();
    }

    pub fn update_monsters(&self) {
        let hero_pos = self.hero().pos();

        for m in self.monsters().iter() {
            let dir = m.behavior.act(m, hero_pos);
            let dst = World::destination(m.pos(), dir);

            match self.try_move(dst) {
                Movement::Goto(dst)      => m.goto(dst),
                Movement::Attack(dst)    => println!("I have not learned to attack, yet."),
                Movement::Obstacle(kind) => println!("There is a {}.", kind)
            }
        }
    }

    fn try_move(&self, dst: Pos) -> Movement {
        let tile = self.tile_info(dst);

        match tile {
            TileInfo { is_walkable: true, mob: Some(_), ..} => Movement::Attack(dst),
            TileInfo { is_walkable: true, mob: None, ..}    => Movement::Goto(dst),
            _ => Movement::Obstacle(tile.kind)
        }
    }

    pub fn act(&mut self, cmd: &Command) {
        println!("{}", cmd);

        match *cmd {
            Command::Walk(direction)  => self.walk(direction),
            Command::Open(direction)  => self.open_close(direction),
            Command::Close(direction) => self.open_close(direction),
            Command::Rest             => self.hero_mut().dec_hp(1),
            Command::Auto             => self.auto(),
            Command::Look             => self.look(),
            _  => ()
        }
    }


    fn walk(&mut self, dir: Direction) {
        let dst  = World::destination(self.hero().pos(), dir);
        let tile = self.tile_info(dst);

        match tile {
            TileInfo { is_walkable: true, mob: Some(_), ..} => self.attack(dst),
            TileInfo { is_walkable: true, mob: None, ..}    => self.hero_mut().goto(dst),
            _ => println!("There is a {}", tile.kind)
        }
    }

    fn tile_info(&self, pos: Pos) -> TileInfo {
        TileInfo::new(self, pos)
    }

    fn attack(&mut self, pos: Pos) {
        let dmg = Game::dmg(self.hero());

        let mut victim = self.mob_at_mut(pos);
        victim.dec_hp(dmg);

        println!("You hit {}, the {} for {}/{} hp!", victim.name, victim.kind, dmg, victim.hp);
        if victim.hp < 0 {
            println!("{}, the {} should die!", victim.name, victim.kind);
        }
        
    }

    fn mob_at_mut(&mut self, pos: Pos) -> &mut mob::Mob {
        self.mobs.iter_mut().find(|m| m.pos() == pos).expect("expected a mob at pos")
    }

    fn hero(&self) -> &mob::Mob {
        &self.mobs[0]
    }

    fn hero_mut(&mut self) -> &mut mob::Mob {
        &mut self.mobs[0]
    }

    fn dmg(mob: &mob::Mob) -> uint {
        let mut rng = ::std::rand::thread_rng();
        rng.gen_range(1u, mob.str as uint)
    }

    fn open_close(&mut self, dir: Direction) {
        let dst = World::destination(self.hero().pos(), dir);
        let tile = self.world.at(dst).kind;
        match tile {
            TileKind::DoorClosed => self.world.set(dst, TileKind::DoorOpen),
            TileKind::DoorOpen   => self.world.set(dst, TileKind::DoorClosed),
            _ => ()
        }
    }

    fn auto(&mut self) {
        let pos = self.hero().pos();
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

    fn look(&mut self) {
        println!("Look around");
    }
}
