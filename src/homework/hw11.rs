use rand::Rng;
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}
pub fn min_adjacent_sum(data: &[i32]) -> (usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_index + 1)
}
pub fn pretty_print(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:>3}.", i)).collect::<Vec<_>>().join(" "));
    println!("data:    [{}]", data.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(", "));

    let (i1, i2) = min_adjacent_sum(data);

    // Маркуємо обрану пару
    let mut marker_line = String::from("indexes:");
    for i in 0..data.len() {
        if i == i1 {
            marker_line.push_str(&format!("{:>width$}", "\\__", width = 4));
        } else if i == i2 {
            marker_line.push_str(&format!("{:>width$}", "__/", width = 3));
        } else {
            marker_line.push_str(&format!("{:>width$}", "", width = 4));
        }
    }
    println!("{}", marker_line);

    let sum = data[i1] + data[i2];
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i1], data[i2], sum, i1, i2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64,
                        90, 86, 60, 54, 78, 72, 83, 44, 89, 22];
        assert_eq!(min_adjacent_sum(&data), (5, 6));
    }
}
