fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let num1: u32 = 56;
    let num2: u32 = 98;
    let result = gcd(num1, num2);

    println!("Найбільший спільний дільник чисел {} і {} дорівнює {}", num1, num2, result);
}
