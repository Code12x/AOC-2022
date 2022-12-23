use std::fs;
use std::io::ErrorKind;

const DONE_MARK: char = '*';

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
}

#[derive(Debug)]
#[derive(Clone)]
struct Poi {
    y: usize,
    x: usize,
    height: char,
    mark: char,
}

struct Map {
    map: Vec<Vec<Poi>>,
}

impl Map {
    fn find_mark(&self, character: char) -> Result<(usize, usize), ErrorKind> {
        for row in &self.map {
            for poi in row {
                if poi.mark == character {
                    return Ok((poi.y, poi.x));
                }
            }
        }
        Err(ErrorKind::NotFound)
    }

    fn get_poi(&self, y: usize, x: usize) -> Poi {
        self.map[y][x].clone()
    }

    fn get_possible_moves(&self, poi: &Poi) -> [bool;4] {
        let mut results = [false, false, false, false];

        let mut i = 0;
        while i<4 {
            results[i] = match i {

                // Check if the position above poi is possible
                0 => {
                    println!("top");
                    if poi.y > 0 && self.get_poi(poi.y-1, poi.x).height as i32 - poi.height as i32 <= 1 &&
                    self.get_poi(poi.y-1, poi.x).mark != DONE_MARK {
                        println!("True");
                        true
                    } else {
                        println!("False");
                        false
                    }
                },
                
                // Check if the position to the right of poi is possible
                1 => {
                    println!("right");
                    if poi.x < self.map[0].len()-1 && self.get_poi(poi.y, poi.x+1).height as i32 - poi.height as i32 <= 1 &&
                    self.get_poi(poi.y, poi.x+1).mark != DONE_MARK {
                        println!("True");
                        true
                    } else {
                        println!("False");
                        false
                    }
                },

                // Check if the position below poi is possible
                2 => {
                    println!("bottom");
                    if poi.y < self.map.len()-1 && self.get_poi(poi.y+1, poi.x).height as i32 - poi.height as i32 <= 1 &&
                    self.get_poi(poi.y+1, poi.x).mark != DONE_MARK {
                        println!("True");
                        true
                    } else {
                        println!("False");
                        false
                    }
                },
                
                // Check if the position to the left of poi is possible
                3 => {
                    println!("left");
                    if poi.x > 0 && self.get_poi(poi.y, poi.x-1).height as i32 - poi.height as i32 <= 1 &&
                    self.get_poi(poi.y, poi.x-1).mark != DONE_MARK {
                        println!("True");
                        true
                    } else {
                        println!("False");
                        false
                    }
                },

                // Any other number would be an error
                _ => panic!("I smell a bug here >:)"),
            };

            i += 1;
        }
        println!("results: {:?}", results);
        results
    }
    
    fn set_mark(&mut self, y: usize, x: usize, mark: char) {
        self.map[y][x].mark = mark;
    }
}

fn get_input() -> Vec<Vec<Poi>> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut map = Vec::<Vec<Poi>>::new();
    
    for (y, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (x, character) in line.chars().enumerate() {
            map[y].push(Poi { y, x, height: character, mark: character });
        }
    }
    map
}

#[derive(Clone)]
#[derive(Debug)]
struct Path {
    poi: Poi,
    moves_count: i32,
}

fn main() {
    let mut map = Map { map: get_input() };

    // Santa
    let santa_cords = match map.find_mark('S') {
        Ok(cords) => cords,
        Err(e) => panic!("Error finding santa {}", e),
    };
    map.map[santa_cords.0][santa_cords.1].height = 'a';
    println!("Santa setup {:?}", santa_cords);

    // Target
    let target_cords = match map.find_mark('E') {
        Ok(cords) => cords,
        Err(e) => panic!("Error finding target {}", e),
    };
    map.map[target_cords.0][target_cords.1].height = 'z';
    println!("Target setup {:?}", target_cords);

    // Setup the paths and give it the inital value of santa's path from santa_cords
    let mut paths: Vec<Path> = Vec::new();
    paths.push(Path {poi: map.get_poi(santa_cords.0, santa_cords.1), moves_count: 0});
    println!("Paths setup {:?}", paths);
    println!("******************************************************************");
    
    // The main loop... go through the paths at the time of the loop and check for new paths and
    // add them to the paths... if a path reaches the target, output the moves_count, else add the
    // DONE_MARK
    // Somewhere loop through paths and set each one's mark to DONE_MARK
    let mut is_found = false;
    'is_found_loop: while !is_found {
        for row in &map.map {
            let mut row_str = String::new();
            for character in row {
                row_str.push(character.mark);
            }
            println!("{}", row_str);
        }

        if paths.len() == 0 {
            panic!("paths.len == 0 :O either there's a bug... or there is no more moves possible and the map is impossible!");
        }

        for path in &paths {
            map.set_mark(path.poi.y, path.poi.x, DONE_MARK);
            println!("Set mark DONE_MARK for {:?}", path);
            println!("{:?}", map.map[20][0]);
        }
        let current_paths = paths.clone();
        println!("current_paths = {:?}", current_paths);

        paths.clear();

        for current_path in current_paths {
            println!("current_path: {:?}", current_path);
            let possible_moves = map.get_possible_moves(&current_path.poi);
            println!("possible_moves for &current_path.poi: {:?}", &current_path.poi);

            for i in [0 as usize, 1 as usize, 2 as usize, 3 as usize] {
                let mut y_op: Option<Operation> = None;
                let mut x_op: Option<Operation> = None;

                match i {
                    0 => y_op = Some(Operation::Subtract),
                    1 => x_op = Some(Operation::Add),
                    2 => y_op = Some(Operation::Add),
                    3 => x_op = Some(Operation::Subtract),
                    _ => panic!("Error here and impossible to occur :D"),
                }

                println!("y_op for possible_moves[{}]: {:?}", i, y_op);
                println!("x_op for possible_moves[{}]: {:?}", i, x_op);

                if possible_moves[i] {
                    let y: usize = match y_op {
                        Some(operation) => {
                            match operation {
                                Operation::Add => current_path.poi.y + 1,
                                Operation::Subtract => current_path.poi.y - 1,
                            }
                        },
                        None => current_path.poi.y,
                    };

                    let x: usize = match x_op {
                        Some(operation) => {
                            match operation {
                                Operation::Add => current_path.poi.x + 1,
                                Operation::Subtract => current_path.poi.x - 1,
                            }
                        }
                        None => current_path.poi.x,
                    };

                    let mut is_accounted_for = false;
                    for path in &paths {
                        if x == path.poi.x && y == path.poi.y {
                            is_accounted_for = true;
                        }
                    }

                    println!("is {}, {} accounted for? {}", y, x, is_accounted_for);

                    if !is_accounted_for {
                        let new_poi = map.get_poi(y, x);
                        let new_path = Path { poi: new_poi, moves_count: current_path.moves_count+1 };
                        paths.push(new_path);

                        if y == target_cords.0 && x == target_cords.1 {
                            is_found = true;
                            continue 'is_found_loop;
                        } 
                    }
                }
            }
            println!("=============================================================");
        }
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }

    println!("The shortest path is {:?} moves long", paths[paths.len()-1]);
}
