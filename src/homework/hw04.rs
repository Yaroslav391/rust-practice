const HEIGHT: usize = 5; 

fn main() {
    for i in 0..HEIGHT {
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    for i in (0..HEIGHT - 1).rev() {
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}
