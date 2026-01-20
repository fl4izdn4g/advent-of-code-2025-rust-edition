use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_DATA: &str = "src/data/input.txt";
const START_POSITION: i16 = 50;
fn get_rotations() -> BufReader<File> {
    let file = File::open(INPUT_DATA).expect("couldn't open file");
    BufReader::new(file)
}

pub struct Rotation {
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
}

pub fn secret_entrance_v1() {
    // number of times the dial is pointing at 0 after any rotation in sequence
    let mut password: u16 = 0;
    let mut current_dial_position: i16 = START_POSITION;

    let data = get_rotations();

    for line in data.lines() {
        let rotation_operation = line.unwrap();
        let rotation = Rotation::new(&rotation_operation);

        let current_move_by = &rotation.move_by % 100;

        if &rotation.direction == "L" {
            // minus
            if current_dial_position - current_move_by < 0 {
                let rest = current_move_by - current_dial_position;
                current_dial_position = 100 - rest;
            } else {
                current_dial_position -= current_move_by;
            }
        } else {
            // plus
            if current_dial_position + current_move_by > 100 {
                let rest = current_dial_position + current_move_by - 100;
                current_dial_position = rest;
            } else {
                current_dial_position += current_move_by;
            }
        }

        if (100 - current_dial_position) == 0 || current_dial_position == 0 {
            current_dial_position = 0;
            password += 1;
        }

        println!("Rolling for {} -> {}: {} ({}) ==> {}", rotation_operation, &rotation.direction, &rotation.move_by, current_move_by, current_dial_position);


    }
    println!("Password is {}", password)

}
