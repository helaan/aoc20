use bit_set::BitSet;
use rustc_hash::FxHashMap;
use std::arch::x86_64::_popcnt64;

const TILE_SIZE: usize = 10;
const TILEWB_SIZE: usize = TILE_SIZE - 2;
const TILEWB_SIZE_SQ: usize = TILEWB_SIZE * TILEWB_SIZE;

const NESSIE: [(u8, u8); 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    (16, 2),
    (17, 1),
    (18, 1),
    (19, 1),
    (18, 0),
];
// ANTINESSIE: X := NESSIE_W - X ...
const NESSIE_H: usize = 3;
const NESSIE_W: usize = 20;

fn atoi(data: &[u8], p: &mut usize, stop: u8) -> u16 {
    let mut r: u16 = (data[*p] - b'0') as u16;
    *p += 1;
    while data[*p] != stop {
        //dbg!(data[*p]);
        r *= 10;
        r += (data[*p] - b'0') as u16;
        *p += 1;
    }
    r
}

#[derive(Clone, Debug)]
struct Tile {
    id: u16,
    border_cw: [u16; 4],
    border_ccw: [u16; 4],
    adj: [u8; 4],
    content: u64,
    notmon: u64,
}

#[derive(Clone, Debug)]
struct TilePlacement {
    index: u8,
    rotation: u8,
    flipped: bool,
}

fn build_border(data: &[u8], base: usize, stride: usize) -> (u16, u16) {
    let mut cw = 0;
    let mut ccw = 0;
    for i in 0..TILE_SIZE {
        //dbg!(data[base + i * stride] as char);
        if data[base + i * stride] == b'#' {
            cw |= 1 << (TILE_SIZE - i - 1);
            ccw |= 1 << i;
        } else if data[base + i * stride] != b'.' {
            unreachable!(data[base + i * stride] as char)
        }
    }
    (cw, ccw)
}

fn nessie_check(
    tiles: &mut [Tile],
    grid: &[TilePlacement],
    x: usize,
    y: usize,
    dx: i8,
    dy: i8,
    xyswap: bool,
    grid_size: usize,
) -> bool {
    for part in &NESSIE {
        //dbg!(x, dx, xyswap, part);
        let px =
            (x as isize + dx as isize * if !xyswap { part.0 } else { part.1 } as isize) as usize;
        //dbg!(y, dy, xyswap, part);
        let py =
            (y as isize + dy as isize * if !xyswap { part.1 } else { part.0 } as isize) as usize;
        let gx = px / (TILEWB_SIZE);
        let gy = py / (TILEWB_SIZE);
        let g = gy * grid_size + gx;
        //dbg!(px, py, gx, gy, g);
        let gridentr = &grid[g];
        let tile = &tiles[gridentr.index as usize];
        let mut cx = px % (TILEWB_SIZE);
        let cy = py % (TILEWB_SIZE);
        if gridentr.flipped {
            cx = TILEWB_SIZE - cx - 1;
        }

        let cmask = 1
            << match grid[g].rotation {
                0 => cy * (TILEWB_SIZE) + cx,
                3 => (TILEWB_SIZE - cx - 1) * (TILEWB_SIZE) + cy,
                2 => TILEWB_SIZE_SQ - cy * TILEWB_SIZE - cx - 1,
                1 => TILEWB_SIZE_SQ - (TILEWB_SIZE - cx - 1) * TILEWB_SIZE - cy - 1,
                _ => unreachable!(),
            };
        //if x == 1 {
        //    dbg!(tile.content & cmask);
        //}
        if (tile.content & cmask) == 0 {
            return false;
        }
    }
    //println!("Monster at {}, {}", x, y);
    for part in &NESSIE {
        //dbg!(x, dx, xyswap, part);
        let px =
            (x as isize + dx as isize * if !xyswap { part.0 } else { part.1 } as isize) as usize;
        //dbg!(y, dy, xyswap, part);
        let py =
            (y as isize + dy as isize * if !xyswap { part.1 } else { part.0 } as isize) as usize;
        let gx = px / (TILEWB_SIZE);
        let gy = py / (TILEWB_SIZE);
        let g = gy * grid_size + gx;
        //dbg!(px, py, gx, gy, g);
        let gridentr = &grid[g];
        let tile = &mut tiles[gridentr.index as usize];
        let mut cx = px % (TILEWB_SIZE);
        let cy = py % (TILEWB_SIZE);
        if gridentr.flipped {
            cx = TILEWB_SIZE - cx - 1;
        }

        let cmask = 1
            << match grid[g].rotation {
                0 => cy * (TILEWB_SIZE) + cx,
                3 => (TILEWB_SIZE - cx - 1) * (TILEWB_SIZE) + cy,
                2 => TILEWB_SIZE_SQ - cy * TILEWB_SIZE - cx - 1,
                1 => TILEWB_SIZE_SQ - (TILEWB_SIZE - cx - 1) * TILEWB_SIZE - cy - 1,
                _ => unreachable!(),
            };
        //if x == 1 {
        //    dbg!(tile.content & cmask);
        //}
        tile.notmon &= !cmask;
    }
    true
}

/*fn print(tiles: &[Tile], grid: &[TilePlacement], grid_size: usize) -> String {
    let mut s = String::new();
    for x in 0..grid_size * TILEWB_SIZE {
        if x % TILEWB_SIZE == 0 {
            s.push('\n')
        };
        for y in 0..grid_size * TILEWB_SIZE {
            let px = x; //+ dx * if xyswap { part.0 } else { part.1 } as usize;
            let py = y; //+ dy * if xyswap { part.1 } else { part.0 } as usize;
            let gx = px / (TILEWB_SIZE);
            let gy = py / (TILEWB_SIZE);
            let g = gy * grid_size + gx;
            let gridentr = &grid[g];
            let tile = &tiles[gridentr.index as usize];
            let mut cx = px % (TILEWB_SIZE);
            let mut cy = py % (TILEWB_SIZE);
            if gridentr.flipped {
                //cy = TILEWB_SIZE - cy - 1;
                cx = TILEWB_SIZE - cx - 1;
            }

            let cmask = 1
                << match grid[g].rotation {
                    0 => cy * (TILEWB_SIZE) + cx,
                    3 => (TILEWB_SIZE - cx - 1) * (TILEWB_SIZE) + cy,
                    2 => TILEWB_SIZE_SQ - cy * TILEWB_SIZE - cx - 1,
                    1 => TILEWB_SIZE_SQ - (TILEWB_SIZE - cx - 1) * TILEWB_SIZE - cy - 1,
                    _ => unreachable!(),
                };
            if y % TILEWB_SIZE == 0 {
                s.push(' ')
            };
            s.push(if (tile.content & cmask) == 0 {
                '.'
            } else {
                '#'
            });
        }

        s.push('\n');
    }
    s
}*/

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 5;
    let mut tiles = Vec::new();

    while p < data.len() {
        let id = atoi(data, &mut p, b':');
        p += 2;
        let b1 = build_border(data, p, 1);
        //dbg!(b1);
        let b2 = build_border(data, p + TILE_SIZE - 1, TILE_SIZE + 1);
        //dbg!(b2);
        let b3 = build_border(data, p + (TILE_SIZE + 1) * (TILE_SIZE - 1), 1);
        //dbg!(b3);
        let b4 = build_border(data, p, TILE_SIZE + 1);
        //dbg!(b4);
        let mut content = 0;
        for y in 0..TILEWB_SIZE {
            for x in 0..TILEWB_SIZE {
                if data[p + (y + 1) * (TILE_SIZE + 1) + x + 1] == b'#' {
                    content |= 1 << (y * TILEWB_SIZE + x);
                }
            }
        }
        tiles.push(Tile {
            id,
            border_cw: [b1.0, b2.0, b3.1, b4.1],
            border_ccw: [b1.1, b2.1, b3.0, b4.0],
            adj: [255; 4],
            content,
            notmon: content,
        });
        p += (TILE_SIZE + 1) * TILE_SIZE + 6;
    }

    let grid_size = (tiles.len() as f32).sqrt() as usize;

    /*let mut tiles_edges = Vec::with_capacity(4);
    let mut tiles_border = Vec::with_capacity(4 * grid_size - 8);
    let mut tiles_inner = Vec::with_capacity(tiles.len()); //- 4 * grid_size + 4);

    for tile in &tiles {
        let mut neighboring = 0;
        dbg!(tile);
        for (x, y) in tile.border_cw.iter().zip(tile.border_ccw.iter()) {
            if let Some(other) = tiles.iter().find(|t| {
                tile.id != t.id && t.border_cw.iter().find(|z| x == *z || y == *z).is_some()
            }) {
                println!("{} -> {}", tile.id, other.id);
                neighboring += 1;
            }
        }
        match neighboring {
            2 => tiles_edges.push(tile),
            3 => tiles_border.push(tile),
            4 => tiles_inner.push(tile),
            _ => unreachable!(neighboring),
        }
    }
    dbg!(&tiles_edges);
    let p1 = tiles_edges.iter().fold(1, |acc, x| acc * x.id as usize);
    dbg!(p1);*/

    let mut sidesmap = FxHashMap::default();

    for x in 0..tiles.len() {
        for s in 0..4 {
            if let Some(oid) = sidesmap.get(&tiles[x].border_cw[s]) {
                tiles[x].adj[s] = *oid;
                let oix = (0..4)
                    .find(|i| tiles[*oid as usize].border_cw[*i] == tiles[x].border_cw[s])
                    .unwrap()
                    .clone();
                tiles[*oid as usize].adj[oix] = x as u8;
            } else if let Some(oid) = sidesmap.get(&tiles[x].border_ccw[s]) {
                tiles[x].adj[s] = *oid;
                let oix = (0..4)
                    .find(|i| tiles[*oid as usize].border_cw[*i] == tiles[x].border_ccw[s])
                    .unwrap()
                    .clone();
                tiles[*oid as usize].adj[oix] = x as u8;
            } else {
                sidesmap.insert(tiles[x].border_cw[s], x as u8);
            }
        }
    }
    //dbg!(&tiles);

    let corner = tiles
        .iter()
        .enumerate()
        .find(|(_i, t)| t.adj.iter().filter(|x| **x == 255).eq([255, 255].iter()))
        .unwrap();

    let mut grid = Vec::with_capacity(grid_size * grid_size);
    let mut used = BitSet::with_capacity(tiles.len());

    //dbg!(corner);
    grid.push(TilePlacement {
        index: corner.0 as u8,
        rotation: (0..4)
            .find(|i| {
                corner.1.adj[*i as usize] == 255 && corner.1.adj[(*i as usize + 1) % 4] != 255
            })
            .unwrap(),
        flipped: false,
    });
    //dbg!(&grid);
    // Check if side 0 and 3 are empty
    assert_eq!(
        tiles[grid[0].index as usize].adj[grid[0].rotation as usize],
        255
    );
    assert_eq!(
        tiles[grid[0].index as usize].adj[(grid[0].rotation as usize + 3) % 4],
        255
    );
    used.insert(corner.0);
    // top row
    for i in 1..grid_size * grid_size {
        //dbg!(i);
        if i % grid_size != 0 {
            let left_grid = &grid[i - 1];
            //dbg!(left_grid);
            let left_tile = &tiles[left_grid.index as usize];
            //dbg!(left_tile);
            let this_adj_index = (left_grid.rotation + if left_grid.flipped { 3 } else { 1 }) % 4;
            //dbg!(this_adj_index);
            let index = left_tile.adj[this_adj_index as usize];
            //dbg!(index);
            let tile = &tiles[index as usize];
            //dbg!(tile);
            let mut flipped = tile
                .border_cw
                .contains(&left_tile.border_cw[this_adj_index as usize]);
            let adj_index = if flipped {
                tile.border_cw
            } else {
                tile.border_ccw
            }
            .iter()
            .enumerate()
            .find(|(_i, x)| **x == left_tile.border_cw[this_adj_index as usize])
            .unwrap()
            .0;
            flipped ^= left_grid.flipped;
            let rotation = (adj_index as u8 + if flipped { 3 } else { 1 }) % 4;
            //dbg!(flipped, rotation);
            assert_eq!(used.insert(index as usize), true);
            grid.push(TilePlacement {
                index,
                rotation,
                flipped,
            })
        } else {
            let up_grid = &grid[i - grid_size];
            let up_tile = &tiles[up_grid.index as usize];
            let this_adj_index = (up_grid.rotation + 2) % 4;
            let index = up_tile.adj[this_adj_index as usize];
            let tile = &tiles[index as usize];
            let mut flipped = tile
                .border_cw
                .contains(&up_tile.border_cw[this_adj_index as usize]);
            let adj_index = if flipped {
                tile.border_cw
            } else {
                tile.border_ccw
            }
            .iter()
            .enumerate()
            .find(|(_i, x)| **x == up_tile.border_cw[this_adj_index as usize])
            .unwrap()
            .0;
            flipped ^= up_grid.flipped;
            let rotation = adj_index as u8;

            assert_eq!(used.insert(index as usize), true);
            grid.push(TilePlacement {
                index,
                rotation,
                flipped,
            })
        }
    }
    //grid[7].flipped = true;
    //grid[6].flipped = false;
    //dbg!(&grid);

    let p1 = [0, grid_size - 1, grid.len() - grid_size, grid.len() - 1]
        .iter()
        .fold(1, |acc, x| acc * tiles[grid[*x].index as usize].id as usize);
    //dbg!(p1);

    //println!("{}", print(&tiles, &grid, grid_size));

    for y in 0..grid_size * TILEWB_SIZE - NESSIE_H - 1 {
        for x in 0..grid_size * TILEWB_SIZE - NESSIE_W - 1 {
            nessie_check(&mut tiles, &grid, x, y, 1, 1, false, grid_size);
            nessie_check(&mut tiles, &grid, x + NESSIE_W, y, -1, 1, false, grid_size);
            nessie_check(&mut tiles, &grid, x, y + NESSIE_H, 1, -1, false, grid_size);
            nessie_check(
                &mut tiles,
                &grid,
                x + NESSIE_W,
                y + NESSIE_H,
                -1,
                -1,
                false,
                grid_size,
            );
        }
    }
    for y in 0..grid_size * TILEWB_SIZE - NESSIE_W - 1 {
        for x in 0..grid_size * TILEWB_SIZE - NESSIE_H - 1 {
            nessie_check(&mut tiles, &grid, x, y, 1, 1, true, grid_size);
            nessie_check(&mut tiles, &grid, x + NESSIE_H, y, -1, 1, true, grid_size);
            nessie_check(&mut tiles, &grid, x, y + NESSIE_W, 1, -1, true, grid_size);
            nessie_check(
                &mut tiles,
                &grid,
                x + NESSIE_H,
                y + NESSIE_W,
                -1,
                -1,
                true,
                grid_size,
            );
        }
    }
    //dbg!(&tiles);
    let mut p2 = 0;
    for tile in &tiles {
        let r = unsafe { _popcnt64(tile.notmon as i64) };
        p2 += r;
    }
    // dbg!(p2);

    format!("{} {}\n", p1, p2)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build() {
        let data = b"#...##.#..";
        let data2 = b"..#.##...#";
        let r = build_border(data, 0, 1);
        let r2 = build_border(data2, 0, 1);
        dbg!(r, r2);
        assert_eq!(r.0, r2.1);
        assert_eq!(r.1, r2.0);
    }
}
