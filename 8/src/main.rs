use std::thread;

const NEIGHBORS_FOR_ALIVE: [usize; 1] = [3];
const NEIGHBORS_FOR_DIE: [usize; 7] = [0, 1, 4, 5, 6, 7, 8];

fn main() {
    let points: Vec<(usize, usize)> = vec![
        (1,10),
        (2,8),
        (2,10),
        (3,9),
        (3,10),
    ];

    let mut world = World::new(50, 25, points);
    world.run(300);
}

struct World {
    points: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl World {
    // Constructor.
    fn new(
        width: usize,
        height: usize,
        alive_points: Vec<(usize, usize)>,
    ) -> World {
        let mut world = World {
            points: vec![vec![false; height]; width],
            width: width,
            height: height,
        };

        for (x, y) in alive_points {
            world.set_alive(x, y, true);
        }

        world
    }

    fn run(&mut self, pause: u32) {
        while !self.is_empty() {
            self.print();
            thread::sleep_ms(pause);
            self.tic();
        }
    }

    /// Returns true is world contains any alive points.
    fn is_empty(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.points[x][y] {
                    return false;
                }
            }
        }

        true
    }

    /// Update state of point.
    fn tic(&mut self) {
        let mut new_points = vec![vec![false; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
            let count_alive_neighbors = self.count_alive_neighbors(x, y);
                if self.is_alive(x, y) {
                    new_points[x][y] = !NEIGHBORS_FOR_DIE.contains(&count_alive_neighbors)
                } else{
                    new_points[x][y] = NEIGHBORS_FOR_ALIVE.contains(&count_alive_neighbors)
                }
            }
        }
        self.points = new_points;
    }

    /// Returns count of alive neighbors of poin [x,y].
    fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        let width = self.width as isize;
        let height = self.height as isize;
        let mut result = 0;

        for i in (x - 1)..(x + 2) {
            for j in (y - 1)..(y + 2) {
                if (i == x && j == y) || i < 0 || i >= width || j < 0 || j >= height {
                    continue;
                }
                if self.is_alive(i as usize, j as usize) {
                    result += 1;
                }
            }
        }
        result
    }

    /// Sets state for point [x,y].
    fn set_alive(&mut self, x: usize, y: usize, value: bool) {
        self.points[x][y] = value;
    }

    /// Returns state of point [x,y].
    fn is_alive(&self, x: usize, y: usize) -> bool {
        self.points[x][y]
    }

    /// Prints current state of points.
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.is_alive(x, y) {"*"} else {"·"});
            }
            println!("");
        }
        println!("");
    }
}
