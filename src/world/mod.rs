use std::rc::Rc;
use std::cell::{ RefCell, Ref };
use self::Direction::*;

pub type WorldRef<'a> = Ref<'a, World>;
pub type WorldRc = Rc<RefCell<World>>;

#[deriving(Copy, Clone)]
pub struct Tile {
    pub kind: TileKind
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        match self.kind {
            TileKind::Wall | TileKind::DoorClosed => false,
            _ => true
        }
    }
}

#[deriving(PartialEq, Show, FromPrimitive, Clone, Copy)]
pub enum TileKind {
    Floor       = ' ' as int,
    Wall        = '#' as int,
    DoorClosed  = '+' as int,
    DoorOpen    = ',' as int,
    StairsUp    = '<' as int,
    StairsDown  = '>' as int
}

impl TileKind {
    fn from_char(stile: char) -> TileKind {
        match FromPrimitive::from_int(stile as int) {
            Some(tile) => tile,
            None       => TileKind::Floor
        }
    }

    pub fn to_char(&self) -> char {
        (*self).clone() as int as u8 as char
    }
}

pub struct World {
    pub max_x: uint,
    pub max_y: uint,
    pub map: Vec<Vec<Tile>>
}

const DEMO_MAP : [&'static str, ..51] = [
    "#################################################################################",
    "#######################      ##################################       ###########",
    "#####################    #     ############################     ##### ###########",
    "######################  ###        #################         ######## ###########",
    "##################      #####             ##########         #######  ######   ##",
    "################       ########    ###### ##########         ###### #######     #",
    "###############      #################### ############## ######      ######     #",
    "################    ######                  ############ ######                 #",
    "########   #######  ######   #     #     #  ############  #####      ############",
    "########   ######      ###                  ############  #######################",
    "########                                                                #########",
    "####       ######      ###   #     #     #  #####################################",
    "#### ###   ########## ####                  ####    ######## ## ### ## ##########",
    "#### ###   ##########   ###########,############    ######             ##########",
    "#### ##################   #####          #######    ######             ##########",
    "#### ###             #### #####          ######## ########             ##########",
    "####           #     ####                +                             ##########",
    "########       #     #### #####          ######################## ###############",
    "########       #####      ##########,############################ ###############",
    "#################################### #########################    ###############",
    "#################################### ######################### ##################",
    "#################################### #########################        ###########",
    "#################################### #########################        ###########",
    "####################################        #####################################",
    "#################################### ###  #######################################",
    "#################################### ###  #######################################",
    "###################### ############# #           ################################",
    "#################################### ####                   #####################",
    "#################################### ####        ########## #####################",
    "#################################### ###################### #####################",
    "###################################   ##################### #####################",
    "#                                     ###############       #####################",
    "## ################################ ################# ###########################",
    "## ################################ ################# ###########################",
    "## ###########################      #################         ###################",
    "## ########################### ############################## ###################",
    "## ########################### #############################       ##############",
    "## #####         ############# ##############################      ##############",
    "## ##  # # # # # #############      ##########################    ###############",
    "##               ################## ###########################  ################",
    "#####  # # # # # ################## ############################ ################",
    "########         ################## ############################    #############",
    "################################### ##########      ############### #############",
    "###########################         ########                        #############",
    "########################### ##################      #############################",
    "########################### #######                 #############################",
    "########################### ####### #############################################",
    "########################### ####### #############################################",
    "#######################     ##         ##########################################",
    "###########################            ##########################################",
    "#################################################################################",
];

impl World {
    pub fn new() -> World {
        let mut map = Vec::new();

        for sline in DEMO_MAP.iter() {
            let mut line = Vec::new();
            for stile in sline.chars() {
                let kind = TileKind::from_char(stile);
                let tile = Tile { kind: kind };
                line.push(tile);
            }
            map.push(line);
        }
        World { max_x: 80u, max_y: 50u, map: map }
    }

    pub fn new_ref() -> Rc<RefCell<World>> {
        Rc::new(RefCell::new(World::new()))
    }

    /// return a tile a x, y of the map
    pub fn at(&self, x: uint, y: uint) -> &Tile{
        &self.map[y][x]
    }

    pub fn set(&mut self, x: uint, y: uint, kind: TileKind) {
        self.map[y][x].kind = kind
    }
}

pub struct Cell {
    world: Rc<RefCell<World>>,
    pub x: uint,
    pub y: uint
}

impl Cell {
    pub fn new(world: Rc<RefCell<World>>, x: uint, y: uint) -> Cell {
        Cell { world: world, x: x, y: y }
    }

    pub fn to_char(&self) -> char {
        self.kind().to_char()
    }

    pub fn is_walkable(&self) -> bool {
        self.world.borrow().at(self.x, self.y).is_walkable()
    }

    pub fn kind(&self) -> TileKind {
        self.world.borrow().at(self.x, self.y).kind
    }

    pub fn set(&mut self, kind: TileKind) {
        self.world.borrow_mut().set(self.x, self.y, kind)
    }

    fn destination(&self, dir: Direction) -> (uint, uint) {
        let Cell { x, y, .. } = *self;
        match dir {
            H  => (x,     y),
            NW => (x - 1, y - 1),
            N  => (x,     y - 1),
            NE => (x + 1, y - 1),
            W  => (x - 1, y),
            E  => (x + 1, y),
            SW => (x - 1, y + 1),
            S  => (x,     y + 1),
            SE => (x + 1, y + 1),
        }
    }

    pub fn neighbor(&self, dir: Direction) -> Cell {
        let (x, y) = self.destination(dir);
        Cell::new(self.world.clone(), x, y)
    }

    pub fn adjacent(&self) -> Vec<(Direction, Cell)> {
        let dirs = [H, NW, N, NE, W, E, SW, S, SE];
        let mut ts = vec![];
        for dir in dirs.iter() {
            let n = self.neighbor(*dir);

            match n.kind() {
                TileKind::Floor | TileKind::Wall => (),
                _ => ts.push((*dir, n))
            }
        }
        ts
    }
}

#[deriving(Clone, Copy, Show)]
pub enum Direction {
    NW, N, NE,
     W, H,  E,
    SW, S, SE
}

