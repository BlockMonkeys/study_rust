// 스코프가 벗어낫음에도 s1을 빌려서 사용함으로써 에러가 나지 않는다.
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 빌려줘 s1을.. 소유권을 주지 않음.
    let s2 = &s1; // 빌려줘 s1을.. 소유권을 주지 않음.
    println!("The length of '{}' is {}.", s1, s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 가변참조자 &mut를 통한 가변 참조자는 스코프 내 1번만 사용이 가능하다.
// 가변을 허용하긴 하지만 매우 통제된 형식으로 허용
// fn main() {
//     let mut s1 = String::from("hello");
//     change(&mut s1); // 빌려줘 s1을..
//     println!("The length of '{}'", s1);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// 댕글링
// fn main() {
//     let reference_to_nothing = dangle(); // 2. 여기서 소유권을 받을려고 하면 소유권이 해제되지 않음으로 에러
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     // &s // 1. 소유권의 종료가 이루어져야하는데 Brrow를 다시해서 리턴하게 되면,
//     s
// }
