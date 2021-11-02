pub fn cocktail_sort(numbers: &mut [i32]) {
    println!("cocktail sort start");
    let mut start = 0;
    let mut end = numbers.len() - 1;
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in start..end {
            if numbers[i] > numbers[i + 1] {
                let tmp = numbers[i];
                numbers[i] = numbers[i + 1];
                numbers[i + 1] = tmp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        end -= 1;
        swapped = false;
        for i in (start..end).rev() {
            if numbers[i] > numbers[i + 1] {
                let tmp = numbers[i];
                numbers[i] = numbers[i + 1];
                numbers[i + 1] = tmp;
                swapped = true;
            }
        }
        start += 1;
    }
    println!("cocktail sort end");
}
