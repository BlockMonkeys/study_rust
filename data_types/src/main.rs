fn main() {
    // # Scala Types : 하나의 값
    let x : u64 = 128; // 양수 (Unsigned-Int)
    let y : i64 = -128; // 음수 (Signed)
    let a : f32 = 3.0; // 부동소수점 ()
    let b : bool = false; // Boolean;
    let c : char = 'ㅋ'; // Characters;

    // # Compound Types : 복합값 타입.
    // Tuple : 다른타입
    let tup : (u32, u8, f64) = (500, 6, 5.5);
    let (d, e, f) = tup;

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("{}",third);

    // Array : 고정크기, 같은 타입
    let ary = [1,2,3,4,5];
    let first_ary = ary[0];
    let second_ary = ary[1];
    let third_ary = ary[2];
}
