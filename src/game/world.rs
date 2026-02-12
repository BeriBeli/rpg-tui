use rand::Rng;

use crate::game::model::{MAP_H, MAP_W, Tile};

pub fn generate_map(rng: &mut impl Rng) -> Vec<Vec<Tile>> {
    let mut map = vec![vec![Tile::Floor; MAP_W]; MAP_H];

    for x in 0..MAP_W {
        map[0][x] = Tile::Wall;
        map[MAP_H - 1][x] = Tile::Wall;
    }
    for row in &mut map {
        row[0] = Tile::Wall;
        row[MAP_W - 1] = Tile::Wall;
    }

    for (y, row) in map.iter_mut().enumerate().take(MAP_H - 1).skip(1) {
        for (x, tile) in row.iter_mut().enumerate().take(MAP_W - 1).skip(1) {
            if rng.random_range(0..100) < 10 {
                *tile = Tile::Wall;
            }
            if y == 2 || x == MAP_W - 3 {
                *tile = Tile::Floor;
            }
        }
    }

    for row in map.iter_mut().take(4).skip(1) {
        for tile in row.iter_mut().take(4).skip(1) {
            *tile = Tile::Floor;
        }
    }

    map[2][2] = Tile::Town;
    map[MAP_H - 2][MAP_W - 2] = Tile::Lair;
    map
}
