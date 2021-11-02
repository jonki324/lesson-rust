use rand::{thread_rng, Rng};
// mod bogo_sort;
// mod bubble_sort;
mod cocktail_sort;

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

    // bogo_sort::bogo_sort(&mut numbers);
    // bubble_sort::bubble_sort(&mut numbers);
    cocktail_sort::cocktail_sort(&mut numbers);

    println!("After: {:?}", numbers);
}
