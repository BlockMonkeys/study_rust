fn main() {
    const MAX_COUNT : u32 = 100_000;
    let mut x = 5;
    println!("The Value of x is : {}", x);
    x = 6;
    println!("The Value of x is : {}", x);
    println!("This is Max Value : {}", MAX_COUNT);

    let y = 1;
    let y = x + 1;
    println!("X + Y : {}", y);


    // 타입변경이 허용됨 재할당이므로.
    let z = "   ";
    let z = z.len();
    println!("z Length : {}", z);

    // // Shadowing은 타입변경이 불가.
    // let mut p = "   ";
    // p = p.len();
}
