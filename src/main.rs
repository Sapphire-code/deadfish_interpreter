use std::io;
//use std::io::Write;

fn custom_pow(base: i32, exponent: i32) -> i32 {
    if exponent == 0 {
        return 1;
    }
    if exponent == 1 {
        return base;
    }
    if exponent % 2 == 0 {
        return custom_pow(base * base, exponent / 2);
    } else {
        return base * custom_pow(base, exponent - 1);
    }
}
fn main() {
    let mut program = String::new();
    io::stdin().read_line(&mut program).expect("Failed to read input");

    let mut accumulator = 0;

    for instruction in program.trim().chars() {
        match instruction {
            'i' => accumulator += 1,
            'd' => accumulator -= 1,
            's' => {accumulator = custom_pow(accumulator, 2);},
            'o' => println!("{}", accumulator),
            _ => {println!("Syntax Error")}

        }
        if accumulator > 255 || accumulator < 0 {
            accumulator = 0;
        }
    }
}