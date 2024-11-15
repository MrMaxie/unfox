use serde::{Deserialize, Serialize};
use std::{hash::Hasher, ops::Add};
use std::hash::Hash;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

enum DeltaPos {
    Plus,
    None,
    Minus,
}

impl Add<usize> for DeltaPos {
    type Output = usize;

    fn add(self, other: usize) -> usize {
        match self {
            DeltaPos::Plus => other.wrapping_add(1),
            DeltaPos::None => other,
            DeltaPos::Minus => other.wrapping_sub(1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StepEntry {
    pub board: Board,
    pub pawn_at: (usize, usize),
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum TileType {
    Empty,
    Goal,
    Wall,
}

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, PartialEq, Hash)]
#[repr(u8)]
enum Pawn {
    Fox,
    Monster,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct Tile {
    tile_type: TileType,
    pawn: Option<Pawn>,
    edges: u8,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    fn get_tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.x == x && tile.y == y)
    }

    pub fn is_solved(&self) -> bool {
        self.tiles.iter().all(|tile| match (tile.tile_type, tile.pawn) {
            (TileType::Goal, Some(Pawn::Fox)) => true,
            (TileType::Goal, _) => false,
            _ => true,
        })
    }

    pub fn fulfill_board_with_walls(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_tile_at(x, y).is_some() {
                    continue;
                }

                self.tiles.push(Tile {
                    tile_type: TileType::Wall,
                    pawn: None,
                    edges: 0,
                    x,
                    y,
                });
            }
        }
    }

    fn can_step_at(&self, x: usize, y: usize) -> bool {
        let tile = match self.get_tile_at(x, y) {
            Some(tile) => tile,
            None => return false,
        };

        match (tile.tile_type, tile.pawn) {
            (TileType::Empty, None) => true,
            (TileType::Goal, None) => true,
            _ => false,
        }
    }

    fn replace_tile_at(&mut self, x: usize, y: usize, tile: Tile) {
        let index = self.tiles.iter().position(|t| t.x == x && t.y == y).unwrap();
        self.tiles[index] = tile;
    }

    fn move_pawn(&mut self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        let tile = self.get_tile_at(x, y).unwrap();

        let (pawn, tile) = match tile.pawn {
            Some(pawn) => (pawn, tile),
            None => return None,
        };

        if (tile.edges & direction.clone() as u8) == 0 {
            return None;
        }

        let (deltax, deltay) = match direction {
            Direction::Up => (DeltaPos::None, DeltaPos::Minus),
            Direction::Down => (DeltaPos::None, DeltaPos::Plus),
            Direction::Left => (DeltaPos::Minus, DeltaPos::None),
            Direction::Right => (DeltaPos::Plus, DeltaPos::None),
        };

        let (targetx, targety) = (deltax + x, deltay + y);

        if !self.can_step_at(targetx, targety) {
            return None;
        }

        let mut target_tile = self.get_tile_at(targetx, targety).unwrap().clone();
        target_tile.pawn = Some(pawn);
        let mut source_tile = self.get_tile_at(x, y).unwrap().clone();
        source_tile.pawn = None;

        self.replace_tile_at(targetx, targety, target_tile);
        self.replace_tile_at(x, y, source_tile);

        Some((targetx, targety))
    }

    fn push_pawn_until_blocked(&mut self, x: usize, y: usize, direction: Direction) -> usize {
        let mut moved_times = 0;
        let mut x = x;
        let mut y = y;

        while let Some((new_x, new_y)) = self.move_pawn(x, y, direction) {
            x = new_x;
            y = new_y;
            moved_times += 1;
        }

        moved_times
    }

    pub fn get_possible_steps(&self) -> Vec<StepEntry> {
        let mut new_boards: Vec<StepEntry> = Vec::new();

        // find each pawn:
        for tile in &self.tiles {
            if tile.pawn.is_none() {
                continue;
            };

            let pawn_x = tile.x;
            let pawn_y = tile.y;

            // find all possible moves by edge + checking if there are no obstacles:
            for edge in 0..4 {
                let dir = match edge {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Right,
                    _ => panic!("Invalid edge"),
                };

                let mut new_board = self.clone();

                if new_board.push_pawn_until_blocked(pawn_x, pawn_y, dir) > 0 {
                    new_boards.push(StepEntry {
                        board: new_board,
                        pawn_at: (pawn_x, pawn_y),
                        direction: dir,
                    });
                }
            }
        }

        new_boards
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for tile in &self.tiles {
            if let Some(pawn) = tile.pawn {
                tile.x.hash(state);
                tile.y.hash(state);
                pawn.hash(state);
            }
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.tiles
            .iter()
            .filter(|tile| tile.pawn.is_some()) // tylko płytki, które mają pionki
            .all(|tile| {
                let other_tile = other.get_tile_at(tile.x, tile.y);
                match other_tile {
                    Some(other_tile) => tile.pawn == other_tile.pawn,
                    None => false,
                }
            })
    }
}

impl Eq for Board {}
