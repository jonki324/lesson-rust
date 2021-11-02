use rand::seq::SliceRandom;
use rand::thread_rng;

fn in_order(numbers: &mut [i32]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] > numbers[i + 1] {
            return false;
        }
    }
    true
}

pub fn bogo_sort(numbers: &mut [i32]) {
    println!("bogo sort start");
    let mut rng = thread_rng();
    while !in_order(numbers) {
        numbers.shuffle(&mut rng);
    }
    println!("bogo sort end");
}
