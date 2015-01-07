extern crate collections;

use mob;
use fov;
use mob::behavior as bhvr;
use world::{ World, TileKind};
use geo::{ Dir, Pos };
use input;
use input::Command;
use dice;

pub struct Game {
    pub world: World,
    pub mobs: Vec<mob::Mob>,
    pub turn: uint
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
    OpenDoor(Pos),
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
        let mut los = fov::los(pos, |pos| game.world.at(pos).is_translucent());
        los.retain(|t| *t != pos);
        let tiles : Vec<TileInfo> = los.iter().map(|pos| TileInfo::new(game, *pos)).collect();

        MobInfo { tiles: tiles.into_boxed_slice(), input: input, pos: pos }
    }

    pub fn adjacent(&self) -> Vec<(Dir, TileKind)> {
        (*self.tiles).iter().
            filter(|p| (**p).pos.dist(self.pos) == 1).
            map(|p| (self.pos.dir(p.pos), p.kind)).collect()
    }
}

impl Game {
    pub fn new() -> Game {
        Game { world: World::new(), mobs: vec![], turn: 0 }
    }

    pub fn add_mob(&mut self, mob: mob::Mob) {
        self.mobs.push(mob)
    }

    fn monsters(&self) -> &[mob::Mob] {
        self.mobs.slice_from(1)
    }

    pub fn update(&mut self, cmd: input::Command) {
        let mut event_q = vec![];

        self.turn += 1;
        println!("[{}] ----------------------------------------------------------", self.turn);
        for mob in self.mobs.iter() {
            println!("+ {} the {}, hp: {}", mob.name, mob.kind, mob.hp.get());
            if mob.hp.get() == 0 {
                println!("  has died");
                continue;
            }

            let mob_info = MobInfo::new(self, mob.pos(), cmd.clone());

            match mob.behavior.act(mob, mob_info) {
                bhvr::Action::TryMove(dir) => {
                    let dst = mob.pos().dest(dir);
                    match self.try_move(dst) {
                        Event::Goto(dst)      => mob.goto(dst),
                        Event::Attack(dst)    => {
                            let other = self.mob_at(dst);
                                let dmg = Game::dmg(mob);
                                println!("  hits {} for {}", other.kind, dmg);
                                self.mob_at(dst).dec_hp(dmg);
                            },
                        Event::Obstacle(kind) => println!("  bumps into a {}", kind),
                        e @ _ => event_q.push(e)
                    }
                },
                bhvr::Action::OpenClose(dir) => {
                    let dst = mob.pos().dest(dir); event_q.push(Event::OpenDoor(dst));
                }
                _ => println!("  does nothing")
            }
        }

        for e in event_q.iter() {
            match *e {
                Event::OpenDoor(dst) => self.open_close(dst),
                _ => unreachable!()
            }
        }

        self.mobs.retain(|mob| mob.hp.get() > 0);
        self.create_monster();
    }

    fn create_monster(&mut self) {
        let newbie = mob::rnd();
        if self.world.at(newbie.pos.get()).is_walkable() {
            println!("+ a {} is born at {}", newbie.kind, newbie.pos.get());
            self.add_mob(newbie);
        }
    }

    fn try_move(&self, dst: Pos) -> Event {
        let tile = self.tile_info(dst);

        match tile {
            TileInfo { kind: TileKind::DoorClosed, .. }     => Event::OpenDoor(dst),
            TileInfo { is_walkable: true, mob: Some(_), ..} => Event::Attack(dst),
            TileInfo { is_walkable: true, mob: None, ..}    => Event::Goto(dst),
            _ => Event::Obstacle(tile.kind)
        }
    }

    fn tile_info(&self, pos: Pos) -> TileInfo {
        TileInfo::new(self, pos)
    }

    fn mob_at(&self, pos: Pos) -> &mob::Mob {
        self.mobs.iter().find(|m| m.pos() == pos).expect("expected a mob at pos")
    }

    fn hero(&self) -> &mob::Mob {
        self.mobs.iter().find(|m| m.kind == mob::Kind::Hero).expect("You have died!")
    }

    fn dmg(mob: &mob::Mob) -> u32 {
        dice::Dice(1, 6).roll() as u32 + mob.str as u32
    }

    fn open_close(&mut self, dst: Pos) {
        let tile = self.world.at(dst).kind;
        match tile {
            TileKind::DoorClosed => self.world.set(dst, TileKind::DoorOpen),
            TileKind::DoorOpen   => self.world.set(dst, TileKind::DoorClosed),
            _ => ()
        }
    }

    // fn auto(&mut self) {
    //     let pos = self.hero().pos();
    //     let ns  = self.world.adjacent(pos);

    //     for n in ns.iter() {
    //         let (ref dir, ref tile_kind) = *n;

    //         match *tile_kind {
    //             TileKind::DoorClosed => self.open_close(*dir),
    //             TileKind::DoorOpen   => self.open_close(*dir),
    //             _ => ()
    //         }
    //         println!("{} -> {}", dir.clone(), tile_kind)
    //     }
    // }
}
