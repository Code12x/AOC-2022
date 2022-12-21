use std::{fs, usize, marker};
use std::io::ErrorKind;

#[derive(Debug)]
#[derive(Clone)]
struct Poi {
    y: usize,
    x: usize,
    height: char,
    mark: Option<char>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Path<'a> {
    poi: &'a Poi,
    moves_count: i32,
}

struct InputMap {
    map: Vec<Vec<char>>,
}

impl InputMap {
    fn get_poi(&self, y: &usize, x: &usize, mark: &Option<char>) -> Poi {
        let height = self.map[*y][*x];

        Poi {
            y: *y,
            x: *x,
            height,
            mark: match mark {
                Some(marker) => Some(*marker),
                None => None,
            },
        }
    }
}

struct Map {
    map: Vec<Vec<Poi>>,
}

impl Map {
    fn get_poi(&mut self, y: &usize, x: &usize) -> &mut Poi {
        &mut self.map[*y][*x]
    }

    fn find_char(&mut self, character: &char) -> Result<&mut Poi, ErrorKind> {
        let mut y = 0;
        while y < self.map.len() {
            let mut x = 0;
            while x < self.map[y].len() {
                if &self.map[y][x].height == character {
                    return Ok(self.get_poi(&y, &x))
                }
                x += 1;
            }
            y += 1;
        }
        Err(ErrorKind::NotFound)
    }

    fn get_possible_moves(&mut self, poi: &Poi) -> Vec<bool> {
        let mut possible_moves = Vec::new();

        // Top side
        if poi.y > 0 && (self.get_poi(&(poi.y-1), &(poi.x)).height as i32 - poi.height as i32 == 1 ||
                         self.get_poi(&(poi.y-1), &(poi.x)).height as i32 - poi.height as i32 == 0 ) {
            possible_moves.push(true);
        } else {
            possible_moves.push(false);
        }

        // Right side
        if poi.x < self.map[0].len()-1 && (self.get_poi(&(poi.y), &(poi.x+1)).height as i32 - poi.height as i32 == 1 || 
                                        self.get_poi(&(poi.y), &(poi.x+1)).height as i32 - poi.height as i32 == 0) {
            possible_moves.push(true);
        }else {
            possible_moves.push(false);
        }

        // Bottom side
        if poi.y < self.map.len()-1 && (self.get_poi(&(poi.y+1), &(poi.x)).height as i32 - poi.height as i32 == 1 ||
                         self.get_poi(&(poi.y+1), &(poi.x)).height as i32 - poi.height as i32 == 0 ) {
            possible_moves.push(true);
        }else {
            possible_moves.push(false);
        }

        // Left side
        if poi.x > 0 && (self.get_poi(&(poi.y), &(poi.x-1)).height as i32 - poi.height as i32 == 1 || 
                         self.get_poi(&(poi.y), &(poi.x-1)).height as i32 - poi.height as i32 == 0) {
            possible_moves.push(true);
        }else {
            possible_moves.push(false);
        }

        possible_moves
    }

}

fn get_input() -> Vec<Vec<Poi>> {
    let input = fs::read_to_string("input.txt").expect("Error opening input.txt");

    let mut map: Vec<Vec<Poi>> = Vec::<Vec<Poi>>::new();

    for (y, line_as_str) in input.lines().enumerate() {
        let mut row: Vec<Poi> = Vec::new();
        for (x, col) in line_as_str.chars().enumerate() {
            row.push(Poi { y, x, height: col, mark: None });
        }
        map.push(row);
    }

    map
}

fn main() {
    let mut map = Map { map: get_input() };
    let mut santa = match map.find_char(&'S') {
        Ok(poi) => poi,
        Err(e) => panic!("Error, santa could not be found! {}", e),
    };
    santa.height = 'a';
    santa.mark = Some('S');

    let mut target = match map.find_char(&'E') {
        Ok(poi) => poi,
        Err(e) => panic!("Error finding the target! {e}"),
    };
    target.height = 'z';
    target.mark = Some('E');

    let mut paths: Vec<Path> = Vec::new();
    let santa_path = Path {poi: &santa, moves_count: 0};
    paths.push(santa_path);

    let mut path_is_found = false;

    'is_found_loop: while !path_is_found {
        println!("{}", &paths.len());
        let current_paths: Vec<Path> = paths.clone();
        paths.clear();
        for path in &current_paths {
            let possible_moves = map.get_possible_moves(&path.poi);
            for i in [0, 1, 2, 3] {
                let possible_move = possible_moves[i];
                if possible_move {
                    let new_path = match i {
                        0 => {
                            Path {
                                poi: map.get_poi(&(&path.poi.y-1), &path.poi.x),
                                moves_count: path.moves_count+1
                            }
                        },
                        1 => {
                            Path {
                                poi: map.get_poi(&path.poi.y, &(&path.poi.x+1)),
                                moves_count: path.moves_count+1
                            }
                        },
                        2 => {
                            Path {
                                poi: map.get_poi(&(&path.poi.y+1), &path.poi.x),
                                moves_count: path.moves_count+1
                            }
                        },
                        3 => {
                            Path {
                                poi: map.get_poi(&path.poi.y, &(&path.poi.x-1)),
                                moves_count: path.moves_count+1
                            }
                        },
                        _ => panic!("Uh Oh! I smell a bug here!"),
                    };
                    println!("{:?}", &new_path);

                    let mut is_accounted_for = false;
                    for check_path in &paths {
                        if get_distance_from_x_to_y(&new_path.poi, &check_path.poi) != 0 as f64 {
                            is_accounted_for = true;
                        }
                    }

                    path.poi.mark = Some('.');

                    if !is_accounted_for {
                        paths.push(new_path);
                    }

                    if get_distance_from_x_to_y(&paths[paths.len()-1].poi, &target) == 0 as f64 {
                        path_is_found = true;
                        continue 'is_found_loop;
                    }
                }
            }
        }
    }

    let last_path = &paths[paths.len()-1];
    let santa = &paths[0].poi;

    println!("The shortest path from y:{} x: {} (santa) to y: {} x: {} (target) is a total of {} moves!", santa.y, santa.x, &target.y, &target.x, last_path.moves_count);
}

fn get_distance_from_x_to_y(source: &Poi, target: &Poi) -> f64 {
    (((source.x as i32 - target.x as i32).pow(2) - (source.y as i32 - target.y as i32).pow(2)) as f64).sqrt()
}
