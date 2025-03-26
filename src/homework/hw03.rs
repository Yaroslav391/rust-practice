fn envelope(w: u32, h: u32) {
    for y in 0..h {
        for x in 0..w {
            let is_edge = y == 0 || y == h - 1 || x == 0 || x == w - 1;
            let diag1 = (y as f64) * ((w - 1) as f64) / ((h - 1) as f64);
            let diag2 = ((w - 1) as f64) - (y as f64) * ((w - 1) as f64) / ((h - 1) as f64);
            let threshold = 0.55;
            let on_diag1 = ((x as f64) - diag1).abs() < threshold;
            let on_diag2 = ((x as f64) - diag2).abs() < threshold;
            
            let c = if is_edge || on_diag1 || on_diag2 {
                "*"
            } else {
                " "
            };
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let w = 30;
    let h = 15;
    envelope(w, h);
}
