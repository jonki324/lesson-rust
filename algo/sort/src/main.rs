use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

fn in_order(numbers: &mut [i32]) -> bool {
    for i in 0..numbers.len() - 1 {
        if &numbers[i] > &numbers[i+1] {
            return false;
        }
    }
    true
}

fn bogo_sort(numbers: &mut [i32]) {
    println!("bogo sort start");
    let mut rng = thread_rng();
    while !in_order(numbers) {
        numbers.shuffle(&mut rng);
    }
    println!("bogo sort end");
}

fn create_numbers() -> [i32; 10] {
    let mut numbers = [0; 10];
    let mut rng = thread_rng();
    for n in &mut numbers {
        *n = rng.gen_range(1..1000);
    }
    numbers
}

fn main() {
    let mut numbers = create_numbers();
    println!("Before: {:?}", &numbers);

    bogo_sort(&mut numbers);

    println!("After: {:?}", numbers);
}
