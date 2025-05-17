use rand::Rng;
pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();
    if total as usize % n != 0 {
        return -1;
    }
    let average = total / n as u32;
    let mut moves = 0;
    let mut cumulative_diff = 0;

    for &ship in shipments.iter() {
        let diff = ship as i32 - average as i32;
        cumulative_diff += diff;
        moves += cumulative_diff.abs();
    }
    (moves / 2) as isize
}
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let average = rng.gen_range(1..=10);
    let mut shipments = vec![average; n];

    for _ in 0..(n / 2) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] >= 1 {
            shipments[i] -= 1;
            shipments[j] += 1;
        }
    }

    shipments
}
fn main() {
    let samples = vec![
        vec![1, 1, 1, 1, 6], 
        vec![9, 3, 7, 2, 9], 
        vec![1, 2, 3],  
    ];
    for (i, shipment) in samples.iter().enumerate() {
        println!("Sample {}: {:?} => Moves: {}", i + 1, shipment, count_permutation(shipment));
    }
    let generated = gen_shipments(10);
    println!("\nGenerated: {:?} => Moves: {}", generated, count_permutation(&generated));
}
