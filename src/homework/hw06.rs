fn main() {
    let triangles = 5;
    draw_tree(triangles);
}
fn draw_tree(triangles: usize) {
    let mut max_width = 1; 
    for i in 0..triangles {
    }

    let mut current_width = 1; 
    for i in 0..triangles {
        for _j in 0..(i + 2) {
            let spaces = " ".repeat((max_width - current_width) / 2);
            let stars = "*".repeat(current_width);
            println!("{}{}", spaces, stars);
            current_width += 2; 
        }
        current_width -= 2 * (i + 2); 
    }


    let trunk_width = triangles / 2 + 1; 
    let trunk_spaces = " ".repeat((max_width - trunk_width) / 2); 
    let trunk = "*".repeat(trunk_width); 

    for _ in 0..(triangles / 2 + 1) {
        println!("{}{}", trunk_spaces, trunk);
    }
}
