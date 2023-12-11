use std::{collections::BTreeMap, fmt::Display};

use anyhow::Result;

const TABLE: &[Pos; 4] = &[
    Pos { x: 0, y: -1 },
    Pos { x: 1, y: 0 },
    Pos { x: 0, y: 1 },
    Pos { x: -1, y: 0 },
];

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pos: Pos,
    tile: Tile,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    NS,
    EW,
    NE90,
    NW90,
    SW90,
    SE90,
    G,
    Start,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    fn dir(&self) -> &Dir {
        match self {
            Pos { x, y } if x == &0 && y == &-1 => &Dir::N,
            Pos { x, y } if x == &1 && y == &0 => &Dir::E,
            Pos { x, y } if x == &0 && y == &1 => &Dir::S,
            Pos { x, y } if x == &-1 && y == &0 => &Dir::W,
            _ => unreachable!("not a vaild coord pair"),
        }
    }
}

pub fn build_maze(input: &str) -> (Node, Vec<Vec<Node>>) {
    let mut start: Node = Node {
        pos: Pos::new(0, 0),
        tile: Tile::G,
    };
    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);
                    match tile {
                        Tile::Start => {
                            let node = Node {
                                pos: Pos::new(x as i32, y as i32),
                                tile,
                            };
                            start = node.clone();
                            node
                        }
                        _ => Node {
                            pos: Pos::new(x as i32, y as i32),
                            tile,
                        },
                    }
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    assert!(start.tile != Tile::G);

    (start, tiles)
}

// 6773 = just right
pub fn process(input: &str) -> Result<String> {
    let maze: (Node, Vec<Vec<Node>>) = build_maze(input);

    let mut map = BTreeMap::<Pos, Pos>::new();

    get_connections(&maze.0, &maze.1, &mut map);
    let mut pos = maze.0.pos.clone();
    let mut count = 1;

    while let Some(curr) = map.get(&pos) {
        if curr == &maze.0.pos {
            break;
        }
        count += 1;
        pos = curr.clone();
    }

    Ok((count as f32 / 2.0).round().to_string())
}

pub fn get_connections(start_node: &Node, maze: &Vec<Vec<Node>>, map: &mut BTreeMap<Pos, Pos>) {
    TABLE.iter().for_each(|loc| {
        let pos = Pos::new(start_node.pos.x + loc.x, start_node.pos.y + loc.y);
        if let Some(row) = maze.get(pos.y as usize) {
            if let Some(node) = row.get(pos.x as usize) {
                let can_connect = start_node.tile.can_connect(loc.dir(), &node.tile);

                let node_is_visisted_from_start_node = map
                    .get(&node.pos)
                    .is_some_and(|node_pos| node_pos == &start_node.pos);

                if node.tile == Tile::Start && can_connect && !node_is_visisted_from_start_node {
                    map.insert(start_node.pos.clone(), node.pos.clone());
                    return;
                }

                if map.get(&start_node.pos).is_none()
                    && can_connect
                    && !node_is_visisted_from_start_node
                {
                    map.insert(start_node.pos.clone(), node.pos.clone());
                    get_connections(node, maze, map);
                }
            }
        }
    });
}

pub fn connected_pipe(start_node: &Node, check: &Pos, maze: &[Vec<Node>]) -> bool {
    if let Some(j_tiles) = maze.get((start_node.pos.y + check.y) as usize) {
        if let Some(ij_node) = j_tiles.get((start_node.pos.x + check.x) as usize) {
            return start_node.tile.can_connect(check.dir(), &ij_node.tile);
        }
    }
    false
}

impl Tile {
    fn can_connect(&self, dir: &Dir, other: &Tile) -> bool {
        use Tile::*;

        //  N
        // W+E
        //  S
        match self {
            NS => match dir {
                Dir::N => vec![NS, SE90, SW90, Start].contains(other),
                Dir::S => vec![NS, NE90, NW90, Start].contains(other),
                Dir::E => false,
                Dir::W => false,
            },
            EW => match dir {
                Dir::N => false,
                Dir::S => false,
                Dir::E => vec![EW, SW90, NW90, Start].contains(other),
                Dir::W => vec![EW, SE90, NE90, Start].contains(other),
            },
            NE90 => match dir {
                Dir::N => vec![NS, SW90, SE90, Start].contains(other),
                Dir::S => false,
                Dir::E => vec![EW, NW90, SW90, Start].contains(other),
                Dir::W => false,
            },
            NW90 => match dir {
                Dir::N => vec![NS, SW90, SE90, Start].contains(other),
                Dir::S => false,
                Dir::E => false,
                Dir::W => vec![EW, SE90, NE90, Start].contains(other),
            },
            SW90 => match dir {
                Dir::N => false,
                Dir::S => vec![NS, NE90, NW90, Start].contains(other),
                Dir::E => false,
                Dir::W => vec![EW, NE90, SE90, Start].contains(other),
            },
            SE90 => match dir {
                Dir::N => false,
                Dir::S => vec![NS, NE90, NW90, Start].contains(other),
                Dir::E => vec![EW, NW90, SW90, Start].contains(other),
                Dir::W => false,
            },
            G => match dir {
                Dir::N => false,
                Dir::S => false,
                Dir::E => false,
                Dir::W => false,
            },
            Start => match dir {
                Dir::N => vec![NS, SW90, SE90].contains(other),
                Dir::S => vec![NS, NW90, NE90].contains(other),
                Dir::E => vec![EW, NW90, SW90].contains(other),
                Dir::W => vec![EW, NE90, SE90].contains(other),
            },
        }
    }

    fn to_char(&self) -> char {
        use Tile::*;
        match self {
            NS => '|',
            EW => '-',
            NE90 => 'L',
            NW90 => 'J',
            SW90 => '7',
            SE90 => 'F',
            G => '.',
            Start => 'S',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        use Tile::*;
        match input {
            '|' => NS,
            '-' => EW,
            'L' => NE90,
            'J' => NW90,
            '7' => SW90,
            'F' => SE90,
            '.' => G,
            'S' => Start,
            tile => unreachable!("this tile type is not implemented: {tile}."),
        }
    }
}

pub enum Dir {
    N,
    S,
    E,
    W,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case((Tile::NS, Dir::N, Tile::NS), true)]
    #[case((Tile::NS, Dir::N, Tile::EW), false)]
    #[case((Tile::NS, Dir::N, Tile::Start), true)]
    #[case((Tile::Start, Dir::N, Tile::NE90), false)]
    #[case((Tile::Start, Dir::W, Tile::SW90), false)]
    fn test_tile_can_connect(
        #[case] input: (Tile, Dir, Tile),
        #[case] expected: bool,
    ) -> Result<()> {
        assert_eq!(input.0.can_connect(&input.1, &input.2), expected);
        Ok(())
    }

    #[test]
    fn test_process_complex() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(process(input)?, "8");
        Ok(())
    }

    #[test]
    fn test_process_loop() -> Result<()> {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(process(input)?, "4");
        Ok(())
    }
}
