pub fn comb_sort(numbers: &mut [i32]) {
    println!("comb_sort start");
    let len_numbers = numbers.len();
    let mut gap = len_numbers;
    let mut swapped = true;
    while gap != 1 || swapped {
        gap = (gap as f64 / 1.3) as usize;
        if gap < 1 {
            gap = 1;
        }
        swapped = false;
        for i in 0..len_numbers - gap {
            if numbers[i] > numbers[i + gap] {
                let tmp = numbers[i];
                numbers[i] = numbers[i + gap];
                numbers[i + gap] = tmp;
                swapped = true;
            }
        }
    }
    println!("comb_sort end");
}
