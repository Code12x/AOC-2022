use std::fs;
use std::io::ErrorKind;

const DONE_MARK: char = '*';

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

    fn get_poi(&self, y: usize, x: usize) -> &Poi {
        &self.map[y][x]
    }

    fn get_possible_moves(&self, poi: &Poi) -> [bool;4] {
        let mut results = [false, false, false, false];

        let mut i = 0;
        while i<4 {
            results[i] = match i {

                // Check if the position above poi is possible
                0 => {
                    if poi.y > 0 && (self.get_poi(poi.y-1, poi.x).height as i32 - poi.height as i32 == 1 ||
                                     self.get_poi(poi.y-1, poi.x).height as i32 - poi.height as i32 == 0) &&
                    self.get_poi(poi.y-1, poi.x).mark != DONE_MARK {
                        true
                    } else {
                        false
                    }
                },
                
                // Check if the position to the right of poi is possible
                1 => {
                    if poi.y > 0 && (self.get_poi(poi.y, poi.x+1).height as i32 - poi.height as i32 == 1 ||
                                     self.get_poi(poi.y, poi.x+1).height as i32 - poi.height as i32 == 0) &&
                    self.get_poi(poi.y, poi.x+1).mark != DONE_MARK {
                        true
                    } else {
                        false
                    }
                },

                // Check if the position below poi is possible
                2 => {
                    if poi.y > 0 && (self.get_poi(poi.y+1, poi.x).height as i32 - poi.height as i32 == 1 ||
                                     self.get_poi(poi.y+1, poi.x).height as i32 - poi.height as i32 == 0) && 
                    self.get_poi(poi.y+1, poi.x).mark != DONE_MARK {
                        true
                    } else {
                        false
                    }
                },
                
                // Check if the position to the left of poi is possible
                3 => {
                    if poi.y > 0 && (self.get_poi(poi.y, poi.x-1).height as i32 - poi.height as i32 == 1 ||
                                     self.get_poi(poi.y, poi.x-1).height as i32 - poi.height as i32 == 0) && 
                        self.get_poi(poi.y, poi.x-1).mark != DONE_MARK {
                        true
                    } else {
                        false
                    }
                },

                // Any other number would be an error
                _ => panic!("I smell a bug here >:)"),
            };

            i += 1;
        }
        results
    }
}

fn get_input() -> Vec<Vec<Poi>> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut map = Vec::<Vec<Poi>>::new();
    
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            map[y][x] = Poi { y, x, height: character, mark: character };
        }
    }
    map
}

#[derive(Clone)]
struct Path<'a> {
    poi: &'a Poi,
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

    // Target
    let target_cords = match map.find_mark('E') {
        Ok(cords) => cords,
        Err(e) => panic!("Error finding target {}", e),
    };
    map.map[target_cords.0][target_cords.1].height = 'z';

    // Setup the paths and give it the inital value of santa's path from santa_cords
    let mut paths: Vec<Path> = Vec::new();
    paths.push(Path {poi: map.get_poi(santa_cords.0, santa_cords.1), moves_count: 0});
    
    // The main loop... go through the paths at the time of the loop and check for new paths and
    // add them to the paths... if a path reaches the target, output the moves_count
    // Somewhere loop through paths and set each one's mark to DONE_MARK
    let mut is_found = false;
    'is_found_loop: while !is_found {
        let mut current_paths = paths.clone();
        paths.clear();

        for current_path in current_paths {

        }
    }
}
