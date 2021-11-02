pub fn bubble_sort(numbers: &mut [i32]) {
    println!("bubble sort start");
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - 1 - i {
            if numbers[j] > numbers[j + 1] {
                let tmp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = tmp;
            }
        }
    }
    println!("bubble sort end");
}
