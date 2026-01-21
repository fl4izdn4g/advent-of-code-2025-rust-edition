use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_DATA: &str = "src/data/input.txt";
const START_POSITION: i16 = 50;
fn get_input() -> BufReader<File> {
    let file = File::open(INPUT_DATA).expect("couldn't open file");
    BufReader::new(file)
}

pub fn vectorize_input() -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    let input = get_input();
    for line in input.lines() {
        match line {
            Ok(l) => vector.push(l),
            Err(_) => continue,
        }
    }

    vector
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
            self.password += full_rotations_counter;
        }
        let current_move_by = rotation.move_by() % 100;

        if rotation.direction() == "L" {
            // minus
            if self.position - current_move_by < 0 {
                if self.position != 0 {
                    self.password += 1;
                }
                let rest = current_move_by - self.position;
                self.position = 100 - rest;
                // position = 0 cmb -1 => 99

            } else {
                self.position -= current_move_by;
            }
        } else {
            // plus
            if self.position + current_move_by > 100 {
                if self.position != 0 {
                    self.password += 1;
                }
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

        println!("Rolling for {}: {} ({}) ==> {}; password: {}", rotation.direction(), rotation.move_by(), current_move_by, self.position, self.password);
    }

    fn compute_password(&self) -> i16 {
        self.password
    }
}



pub fn secret_entrance_v1(rotations: &Vec<String>) -> i16 {
    // number of times the dial is pointing at 0 after any rotation in sequence
    let mut standard_lock = StandardMechanism::new(START_POSITION);
    for rotation_operation in rotations {
        let rotation = Rotation::new(&rotation_operation);
        standard_lock.rotate(&rotation);
    }
    let found_password = standard_lock.compute_password();
    println!("Found password: {}", found_password);

    found_password
}

pub fn secret_entrance_v2(rotations: &Vec<String>) -> i16 {
    // number of times the dial is passing 0 after any rotation in sequence
    let mut secured_lock = SecuredMechanism::new(START_POSITION);
    for rotation_operation in rotations {
        let rotation = Rotation::new(&rotation_operation);
        secured_lock.rotate(&rotation);
    }
    let found_password = secured_lock.compute_password();
    println!("Found password: {}", found_password);

    found_password
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_entrance_v2_passing_zero_from_left_to_right_increase_password() {
        let rotations = vec![String::from("R52")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 1);
    }

    #[test]
    fn second_entrance_v2_passing_zero_from_right_to_left_increase_password() {
        let rotations = vec![String::from("L52")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 1);
    }

    #[test]
    fn second_entrance_v2_stopping_on_zero_from_right_increase_password() {
        let rotations = vec![String::from("R50")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 1);
    }

    #[test]
    fn second_entrance_v2_stopping_on_zero_from_left_increase_password() {
        let rotations = vec![String::from("L50")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 1);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_one_pass_from_right_increase_password() {
        let rotations = vec![String::from("R240")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 2);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_one_pass_from_right_stops_on_zero_increase_password() {
        let rotations = vec![String::from("R250")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 3);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_one_pass_from_left_increase_password() {
        let rotations = vec![String::from("L240")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 2);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_one_pass_from_left_stops_on_zero_increase_password() {
        let rotations = vec![String::from("L250")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 3);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_multiple_pass_from_right_increase_password() {
        let rotations = vec![String::from("R240"), String::from("R140")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 4);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_multiple_pass_from_left_increase_password() {
        let rotations = vec![String::from("L260"), String::from("L220")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 5);
    }

    #[test]
    fn second_entrance_v2_passing_multiple_times_zero_on_multiple_pass_and_revert_increase_password() {
        let rotations = vec![String::from("L260"), String::from("R260")];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 6);
    }

    #[test]
    fn testing_example() {
        let rotations = vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ];
        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 6);
    }

    #[test]
    fn add_to_password_only_when_really_passed_0() {
        let rotations = vec![
            String::from("R50"),
            String::from("L1"),
        ];

        let password = secret_entrance_v2(&rotations);
        assert_eq!(password, 1);
    }
}


