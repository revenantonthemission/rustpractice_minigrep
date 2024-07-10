//커맨드 라인 인수로 전달된 값들을 읽기 위해 std::env::args 함수를 사용한다.
use std::env;

fn main() {
    //cargo run [options] [-- args]이기 때문에 --는 따로 처리하지 않아도 env::args().collect()를 통해 수집될 수 있다.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
