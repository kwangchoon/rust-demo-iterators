use std::collections::HashSet;

/**
 * Mini project
 */

struct Worm {
    position: Point2d,
    length: i32,
}

struct Map {
    cells: Vec<Vec<MapCell>>,
}

#[derive(Debug, Copy, Clone)]
enum MapCell {
    Air,
    Dirt,
}

impl Map {
    // fn at(&self, target: <Point2d as Add<Point2d>>::Output) -> Option<MapCell> {
    fn at(&self, target: Point2d) -> Option<MapCell> {
        if target.x < 0 || target.y < 0 {
            return None;
        }
        let x = target.x as usize;
        let y = target.y as usize;
        if x >= self.cells.len() || y >= self.cells[x].len() {
            return None;
        }
        Some(self.cells[x][y])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point2d {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point2d {
    type Output = Point2d;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Point2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
enum Action {
    Move(Point2d),
    Dig(Point2d),
}

enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    const ALL: [Direction; 8] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest,
    ];

    fn as_vec(&self) -> Point2d {
        match self {
            Direction::North => Point2d { x: 0, y: -1 },
            Direction::South => Point2d { x: 0, y: 1 },
            Direction::East => Point2d { x: 1, y: 0 },
            Direction::West => Point2d { x: -1, y: 0 },
            Direction::NorthEast => Point2d { x: 1, y: -1 },
            Direction::NorthWest => Point2d { x: -1, y: -1 },
            Direction::SouthEast => Point2d { x: 1, y: 1 },
            Direction::SouthWest => Point2d { x: -1, y: 1 },
        }
    }
}

#[test]
fn case_study_imperative_style() {
    fn valid_moves_for_worm_imperative(
        worm: &Worm,
        map: &Map,
        occupied_cells: HashSet<Point2d>,
    ) -> Vec<Action> {
        let mut valid_moves = Vec::new();

        // Direction::ALL is a constant list of all directions a worm
        // could walk, including diagonals.
        for dir in &Direction::ALL {
            // 1. Target is one cell in a direction from the worm
            let target = worm.position + Direction::as_vec(dir);
            // 2. We check that the target is free
            if !occupied_cells.contains(&target) {
                // 3. and 4. Air cells are moved to, Dirt cells are dug,
                // out of bounds cells do nothing.
                match map.at(target) {
                    Some(MapCell::Air) => {
                        valid_moves.push(Action::Move(target));
                    }
                    Some(MapCell::Dirt) => {
                        valid_moves.push(Action::Dig(target));
                    }
                    _ => {}
                }
            }
        }

        // By the end of the loop, this Vec is the result I want, but
        // you have to read through to the most nested part to see where
        // it's being modified.
        valid_moves
    }

    let map = Map {
        cells: vec![
            vec![MapCell::Air, MapCell::Air, MapCell::Dirt],
            vec![MapCell::Air, MapCell::Dirt, MapCell::Air],
            vec![MapCell::Dirt, MapCell::Air, MapCell::Air],
        ],
    };
    let worm = Worm {
        position: Point2d { x: 0, y: 0 },
        length: 3,
    };
    let occupied_cells = HashSet::new();

    let actions = valid_moves_for_worm_imperative(&worm, &map, occupied_cells);
    println!("{:?}", actions);
}

#[test]
fn case_study_transform_to_functional_style() {
    fn valid_moves_for_worm(
        worm: &Worm,
        map: &Map,
        occupied_cells: HashSet<Point2d>,
    ) -> Vec<Action> {
        let mut valid_moves = Vec::new();

        // Direction::ALL is a constant list of all directions a worm
        // could walk, including diagonals.
        for dir in &Direction::ALL {
            // 1. Target is one cell in a direction from the worm
            let target = worm.position + Direction::as_vec(dir);
            // 2. We check that the target is free
            if !occupied_cells.contains(&target) {
                // 3. and 4. Air cells are moved to, Dirt cells are dug,
                // out of bounds cells do nothing.
                match map.at(target) {
                    Some(MapCell::Air) => {
                        valid_moves.push(Action::Move(target));
                    }
                    Some(MapCell::Dirt) => {
                        valid_moves.push(Action::Dig(target));
                    }
                    _ => {}
                }
            }
        }

        valid_moves
    }

    let map = Map {
        cells: vec![
            vec![MapCell::Air, MapCell::Air, MapCell::Dirt],
            vec![MapCell::Air, MapCell::Dirt, MapCell::Air],
            vec![MapCell::Dirt, MapCell::Air, MapCell::Air],
        ],
    };
    let worm = Worm {
        position: Point2d { x: 0, y: 0 },
        length: 3,
    };
    let occupied_cells = HashSet::new();

    let actions = valid_moves_for_worm(&worm, &map, occupied_cells);
    println!("{:?}", actions);
}
