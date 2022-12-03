use rand::Rng;

fn main() {
    let result :[u32;6] = lottery_result();
    let userInput : [u32;6] = [1,3,6,28,32,55];
    let mut answerCount = 0;

    for userVal in userInput.iter() {
        for resultVal in result.iter() {
            if userVal == resultVal {
                answerCount = answerCount + 1;
            }
        }
    }
    println!("정답 갯수 : {}", answerCount);
}

fn lottery_result() -> [u32; 6] {
    let mut result_ary :[u32;6] = [0,0,0,0,0,0];
    let mut idx = 0;
    while idx < result_ary.len() {
        let secret = rand::thread_rng().gen_range(1..=45);
        result_ary[idx] = secret;
        idx = idx + 1;
    }

    result_ary
}