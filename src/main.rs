mod day_one;

fn main() {
    println!("Advent of Code 2025");
    println!("-------------");
    println!("Day One");
    let rotations = day_one::vectorize_input();
    day_one::secret_entrance_v1(&rotations);
    day_one::secret_entrance_v2(&rotations);
}
