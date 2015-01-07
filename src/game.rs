extern crate collections;
use std::rand::Rng;

use mob;
use fov;
use mob::behavior as bhvr;
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
#[derive(Clone, Show)]
struct TileInfo {
    is_walkable: bool,
    kind: TileKind,
    pub mob: Option<mob::Mob>,
    pos: Pos
}

impl TileInfo {
    fn new(game: &Game, pos: Pos) -> TileInfo {
        let tile = game.world.at(pos);
        let mob  = TileInfo::mob_at(&game.mobs, pos);
        TileInfo { pos: pos, is_walkable: tile.is_walkable(), mob: mob, kind: tile.kind }
    }

    fn mob_at(monsters: &Vec<mob::Mob>, pos: Pos) -> Option<mob::Mob> {
        monsters.iter().find(|mob| mob.pos() == pos).map(|mob| mob.clone())
    }
}

enum Event {
    Goto(Pos),
    Attack(Pos),
    Obstacle(TileKind)
}

#[derive(Show)]
pub struct MobInfo {
    pub pos: Pos,
    pub tiles: Box<[TileInfo]>,
    pub input: input::Command
}

impl MobInfo {
    pub fn new(game: &Game, pos: Pos, input: input::Command) -> MobInfo {
        let mut los = fov::los(pos, &game.world);
        los.retain(|t| *t != pos);
        let tiles : Vec<TileInfo> = los.iter().map(|pos| TileInfo::new(game, *pos)).collect();

        MobInfo { tiles: tiles.into_boxed_slice(), input: input, pos: pos }
    }

    pub fn adjacent(&self) -> Vec<(Direction, TileKind)> {
        (*self.tiles).iter().
            filter(|p| (**p).pos.dist(self.pos) == 1).
            map(|p| (self.pos.dir(p.pos), p.kind)).collect()
    }
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
        println!("----------------------------------------------------------");
        let mut corpses = vec![];
        for (idx, mob) in self.mobs.iter().enumerate() {
            if mob.hp.get() == 0 {
                println!("{} is dead", mob.kind);
                corpses.push(idx);
                continue;
            }

            let mob_info = MobInfo::new(self, mob.pos(), cmd.clone());

            match mob.behavior.act(mob, mob_info) {
                bhvr::Action::TryMove(dir) => {
                    let dst = World::destination(mob.pos(), dir);
                    match self.try_move(dst) {
                        Event::Goto(dst)      => mob.goto(dst),
                        Event::Attack(dst)    => {
                            println!("attack");
                            let other = self.mob_at(dst);
                                let dmg = Game::dmg(mob);
                                println!("{} hits {} for {}", mob.kind, other.kind, dmg);
                                self.mob_at(dst).dec_hp(dmg);
                            },
                        Event::Obstacle(kind) => println!("There is a {}.", kind)
                    }
                },
                _ => println!("nop")
            }
        }

        self.mobs.retain(|mob| mob.hp.get() > 0);

        println!("hero {}", self.hero());

        self.create_monster();
    }

    fn create_monster(&mut self) {
        let newbie = mob::rnd();
        if self.world.at(newbie.pos.get()).is_walkable() {
            println!("{} is born at {}", newbie.kind, newbie.pos.get());
            self.add_mob(newbie);
        }
    }

    fn try_move(&self, dst: Pos) -> Event {
        let tile = self.tile_info(dst);

        match tile {
            TileInfo { is_walkable: true, mob: Some(_), ..} => Event::Attack(dst),
            TileInfo { is_walkable: true, mob: None, ..}    => Event::Goto(dst),
            _ => Event::Obstacle(tile.kind)
        }
    }

    // fn walk(&mut self, dir: Direction) {
    //     let dst  = World::destination(self.hero().pos(), dir);
    //     let tile = self.tile_info(dst);

    //     match tile {
    //         TileInfo { is_walkable: true, mob: Some(_), ..} => self.attack(dst),
    //         TileInfo { is_walkable: true, mob: None, ..}    => self.hero_mut().goto(dst),
    //         _ => println!("There is a {}", tile.kind)
    //     }
    // }

    fn tile_info(&self, pos: Pos) -> TileInfo {
        TileInfo::new(self, pos)
    }

    fn mob_at(&self, pos: Pos) -> &mob::Mob {
        self.mobs.iter().find(|m| m.pos() == pos).expect("expected a mob at pos")
    }

    fn hero(&self) -> &mob::Mob {
        self.mobs.iter().find(|m| m.kind == mob::Kind::Hero).expect("The hero is dead!")
    }

    fn dmg(mob: &mob::Mob) -> u32 {
        let mut rng = ::std::rand::thread_rng();
        rng.gen_range(1u, mob.str as uint) as u32
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
}
