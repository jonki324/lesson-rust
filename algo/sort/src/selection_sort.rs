pub fn selection_sort(numbers: &mut [i32]) {
    println!("selection sort start");
    let len_nums = numbers.len();
    for i in 0..len_nums {
        let mut min_idx = i;
        for j in (i + 1)..len_nums {
            if numbers[min_idx] > numbers[j] {
                min_idx = j;
            }
        }
        let tmp = numbers[i];
        numbers[i] = numbers[min_idx];
        numbers[min_idx] = tmp;
    }
    println!("selection sort end");
}
