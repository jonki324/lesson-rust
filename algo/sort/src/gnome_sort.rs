pub fn gnome_sort(numbers: &mut [i32]) {
    println!("gnome sort start");
    let len_nums = numbers.len();
    let mut index = 0;
    while index < len_nums {
        if index == 0 {
            index += 1;
        }
        if numbers[index] >= numbers[index - 1] {
            index += 1;
        } else {
            let tmp = numbers[index];
            numbers[index] = numbers[index - 1];
            numbers[index - 1] = tmp;
            index -= 1;
        }
    }
    println!("gnome sort end");
}
