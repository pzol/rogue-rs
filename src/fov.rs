use world::Pos;

use std::cmp::{ min, max };
use std::num::SignedInt;

pub fn fov(pos: Pos, radius: u32) -> Vec<Pos> {
    let Pos { x: cx, y: cy } = pos;

    let mut ns : Vec<Pos> = vec![];
    for i in range(0i32, 80) {
        for j in range(0i32, 50) {

            let x = (i - cx as i32).abs();
            let y = (j - cy as i32).abs();

            if max(x, y) + (min(x, y) / 2 ) <= radius as i32 {
                ns.push(Pos { x: i as uint, y: j as uint })
            }
        }
    }
    
    ns
}
