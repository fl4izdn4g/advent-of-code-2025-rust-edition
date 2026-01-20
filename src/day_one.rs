use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_DATA: &str = "src/data/input.txt";
const START_POSITION: i16 = 50;
fn get_rotations() -> BufReader<File> {
    let file = File::open(INPUT_DATA).expect("couldn't open file");
    BufReader::new(file)
}

struct Rotation {
    pub direction: String,
    pub move_by: i16
}

impl Rotation {
    pub fn new(operation: &String) -> Rotation {
        let direction = String::from(operation.get(0..1).unwrap_or("L"));
        let move_by = operation.get(1..).unwrap_or("0").parse().unwrap_or(0);
        Rotation {
            direction,
            move_by
        }
    }

    pub fn direction(&self) -> &String {
        &self.direction
    }

    pub fn move_by(&self) -> &i16 {
        &self.move_by
    }
}

trait LockingMechanism {
    fn new(starting_position: i16) -> Self;
    fn rotate(&mut self, rotation: &Rotation);
    fn compute_password(&self) -> i16;
}

struct StandardMechanism {
    position: i16,
    password: i16,
}

struct SecuredMechanism {
    position: i16,
    password: i16
}

impl LockingMechanism for StandardMechanism {
    fn new(starting_position: i16) -> StandardMechanism {
        StandardMechanism {
            position: starting_position,
            password: 0
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        let current_move_by = rotation.move_by() % 100;

        if rotation.direction() == "L" {
            // minus
            if self.position - current_move_by < 0 {
                let rest = current_move_by - self.position;
                self.position = 100 - rest;
            } else {
                self.position -= current_move_by;
            }
        } else {
            // plus
            if self.position + current_move_by > 100 {
                let rest = self.position + current_move_by - 100;
                self.position = rest;
            } else {
                self.position += current_move_by;
            }
        }

        if (100 - self.position) == 0 || self.position == 0 {
            self.position = 0;
            self.password += 1;
        }

        println!("Rolling for {}: {} ({}) ==> {}", rotation.direction(), rotation.move_by(), current_move_by, self.position);
    }

    fn compute_password(&self) -> i16 {
        self.password
    }
}

impl LockingMechanism for SecuredMechanism {
    fn new(starting_position: i16) -> SecuredMechanism {
        SecuredMechanism {
            position: starting_position,
            password: 0i16
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        let full_rotations_counter = rotation.move_by().wrapping_div(100);
        if full_rotations_counter > 0 {
            println!("<-- full rotation -->");
            self.password += full_rotations_counter;
        }
        println!("Full rotations -> {} => Password after rotation: {}", full_rotations_counter, self.password);
        let current_move_by = rotation.move_by() % 100;

        if rotation.direction() == "L" {
            // minus
            if self.position - current_move_by < 0 {
                let rest = current_move_by - self.position;
                self.position = 100 - rest;
                self.password += 1;
            } else {
                self.position -= current_move_by;
            }
        } else {
            // plus
            if self.position + current_move_by > 100 {
                let rest = self.position + current_move_by - 100;
                self.position = rest;
                self.password += 1;
            } else {
                self.position += current_move_by;
            }
        }

        if (100 - self.position) == 0 || self.position == 0 {
            self.position = 0;
            self.password += 1;
        }

        println!("Rolling for {}: {} ({}) ==> {}", rotation.direction(), rotation.move_by(), current_move_by, self.position);
    }

    fn compute_password(&self) -> i16 {
        self.password
    }
}


pub fn secret_entrance_v1() {
    // number of times the dial is pointing at 0 after any rotation in sequence
    let data = get_rotations();

    let mut standard_lock = StandardMechanism::new(START_POSITION);
    for line in data.lines() {
        let rotation_operation = line.unwrap();
        // println!("For operation: {}", rotation_operation);

        let rotation = Rotation::new(&rotation_operation);
        standard_lock.rotate(&rotation);
    }
    let found_password = standard_lock.compute_password();
    println!("Found password: {}", found_password)
}

pub fn second_entrance_v2() {
    // number of times the dial is passing 0 after any rotation in sequence
    let data = get_rotations();

    let mut secured_lock = SecuredMechanism::new(START_POSITION);
    for line in data.lines() {
        let rotation_operation = line.unwrap();
        // println!("For operation: {}", rotation_operation);

        let rotation = Rotation::new(&rotation_operation);
        secured_lock.rotate(&rotation);
    }
    let found_password = secured_lock.compute_password();
    println!("Found password: {}", found_password)
}
