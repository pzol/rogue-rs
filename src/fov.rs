use world::Pos;

use std::cmp::{ min, max };
use std::num::SignedInt;

pub fn fov(pos: Pos, radius: u32) -> Vec<Pos> {
    let Pos { x: cx, y: cy } = pos;

    let mut ns : Vec<Pos> = vec![];
    for i in range(0i32, 80) { // FIXME: use real world size
        for j in range(0i32, 50) {

            let x = (i - cx as i32).abs();
            let y = (j - cy as i32).abs();

            // outer bounds is enough if you use ray casting afterwards, 
            // those will cover the inner area anyway
            if max(x, y) + (min(x, y) / 2 ) == radius as i32 { // == for bounds only <= for all inside 
                ns.push(Pos { x: i as uint, y: j as uint })
            }
        }
    }
    
    ns
}


// Bresenham's line algorithm
pub fn ray(from: Pos, to: Pos, visit: |Pos| -> bool) -> Vec<Pos> {
    let mut ray = vec![];

    let mut dx = (to.x as i32 - from.x as i32).abs();
    let mut dy = (to.y as i32 - from.y as i32).abs();
    let mut x = from.x as i32;
    let mut y = from.y as i32;
    let mut n = 1 + dx + dy;
    let x_inc : i32 = if to.x > from.x { 1 } else { -1 };
    let y_inc : i32 = if to.y > from.y { 1 } else { -1 };

    let mut error = dx - dy;
    dx *= 2;
    dy *= 2;

    while n > 0 {
        n -= 1;

        let pos = Pos { x: x as uint, y: y as uint};

        if visit(pos) {
            ray.push(pos);
        } else {
            break;
        }

        if error > 0 {
            x += x_inc;
            error -= dy;
        } else {
            y += y_inc;
            error += dx;
        }
    }

    ray
}
