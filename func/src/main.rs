fn main() {
    let x = plus(5,6);
    println!("result is {}", x);

    let y = senior(18);
    println!("Are You Senior? {}", y);

    // thisloop();
    // thisWhile();
    // thisfor();
    revfor();
}

fn plus(x : u64, y : u64) -> u64 {
    let result : u64 = x + y;
    result
}
// 조건문
fn senior(age : u64) -> bool {
    if age < 19 {
        false
    } else {
        true
    }
}
// 반복문 loop
fn thisloop() {
    loop {
        println!("lloooooopp!");
    }
}

// 반복문 while
fn thisWhile() {
    let a = [1,2,3,4,5];
    let mut idx = 0;
    while idx < 5 {
        println!("What the While {}", a[idx]);
        idx = idx +1;
    }
}

//반복문 for
fn thisfor() {
    let a = [1,2,3,4,5];
    for elements in a.iter() {
        println!("For loopppp {}", elements);
    }
}

// 반복문 for 2
fn revfor() {
    for nums in (1..45).rev() {
        println!("Rev For loppp {}", nums);
    }
}