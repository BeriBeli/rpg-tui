use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::{Chest, MAP_H, MAP_W, NpcKind, NpcPoint, Position, Tile, WorldObjects};

pub fn generate_world(seed: u64) -> (Vec<Vec<Tile>>, WorldObjects) {
    let mut rng = StdRng::seed_from_u64(seed);
    let map = generate_map(&mut rng);
    let objects = generate_world_objects(&map, &mut rng);
    (map, objects)
}

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

fn generate_world_objects(map: &[Vec<Tile>], rng: &mut impl Rng) -> WorldObjects {
    let mut candidates = floor_candidates(map);

    let mut chests = Vec::new();
    for _ in 0..5 {
        if candidates.is_empty() {
            break;
        }
        let position = pop_random_position(&mut candidates, rng);
        chests.push(Chest {
            position,
            opened: false,
            gold: rng.random_range(8..=24),
            potion: if rng.random_range(0..100) < 45 { 1 } else { 0 },
            ether: if rng.random_range(0..100) < 35 { 1 } else { 0 },
        });
    }

    let npc_kinds = [NpcKind::Traveler, NpcKind::Scout, NpcKind::Sage];
    let mut npcs = Vec::new();
    for kind in npc_kinds {
        if candidates.is_empty() {
            break;
        }
        let position = pop_random_position(&mut candidates, rng);
        npcs.push(NpcPoint {
            position,
            kind,
            interacted: false,
            reward_gold: rng.random_range(10..=20),
        });
    }

    WorldObjects::new(chests, npcs)
}

fn floor_candidates(map: &[Vec<Tile>]) -> Vec<Position> {
    let mut positions = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile != Tile::Floor {
                continue;
            }
            if x <= 3 && y <= 3 {
                continue;
            }
            if x >= MAP_W.saturating_sub(3) && y >= MAP_H.saturating_sub(3) {
                continue;
            }
            positions.push(Position { x, y });
        }
    }
    positions
}

fn pop_random_position(candidates: &mut Vec<Position>, rng: &mut impl Rng) -> Position {
    let idx = rng.random_range(0..candidates.len());
    candidates.swap_remove(idx)
}

#[cfg(test)]
mod tests {
    use super::generate_world;

    #[test]
    fn world_generation_places_objects_deterministically_for_seed() {
        let (_, objects_a) = generate_world(99);
        let (_, objects_b) = generate_world(99);
        assert_eq!(objects_a.chests.len(), objects_b.chests.len());
        assert_eq!(objects_a.npcs.len(), objects_b.npcs.len());
        assert_eq!(
            objects_a.chests[0].position.x,
            objects_b.chests[0].position.x
        );
        assert_eq!(objects_a.npcs[0].position.y, objects_b.npcs[0].position.y);
    }
}
