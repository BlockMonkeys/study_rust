fn main() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let he = &s[..2];
    let rld = &s[8..];
    let mut slice = &s[..];
    slice = "Changed";
    println!("slice : {} , s : {}", slice, s);
    
    let ary = [1, 2, 3, 4, 5];
    let ary_slice = &ary[1..3];
    println!("Array Slice : {:#?}", ary_slice);
}
