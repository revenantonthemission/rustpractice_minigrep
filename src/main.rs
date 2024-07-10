//커맨드 라인 인수로 전달된 값들을 읽기 위해 std::env::args 함수를 사용한다.
use std::env;
use std::fs;

fn main() {
    //cargo run [options] [-- args]이기 때문에 --는 따로 처리하지 않아도 env::args().collect()를 통해 수집될 수 있다.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    //fs::read_to_string 함수를 사용해 파일을 열고 std::io::Result<String>을 반환한다. 실패시 expect를 사용한다.
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
