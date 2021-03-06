fn main() {
    let s_3 = sum_of_multiples(3, 999);
    println!("Sum of multiples of 3: {}", s_3);
    let s_5 = sum_of_multiples(5, 999);
    println!("Sum of multiples of 5: {}", s_5);
    let s_15 = sum_of_multiples(15, 999);
    println!("Sum of multiples of 15: {}", s_15);
    println!(
        "Sum of multiples of 3 or 15 below 1000: {}",
        s_3 + s_5 - s_15
    );
}

fn sum_of_multiples(multiple: u32, upper_bound: u32) -> u32 {
    let max_occurrences = upper_bound / multiple;
    (max_occurrences * (2 * multiple + (max_occurrences - 1) * multiple)) / 2
}
