//커맨드 라인 인수로 전달된 값들을 읽기 위해 std::env::args 함수를 사용한다.
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    //cargo run [options] [-- args]이기 때문에 --는 따로 처리하지 않아도 env::args().collect()를 통해 수집될 수 있다.
    let args: Vec<String> = env::args().collect();

    //unwrap_or_else를 사용하면 Result가 Ok일 때 Ok 안의 값을 반환하고, Err일 때 클로저 안의 코드를 호출한다.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //실행 파트를 run으로 분리.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

//1. 유닛 타입()을 반환하는 함수에서 Result<(), Box<dyn Error>>를 반환하는 함수로 변경한다. 이때 ()는 Ok(())다.
//2. Box<dyn Error>는 트레이트 객체로, Error 트레이트를 구현한 어떤 타입을 반환한다.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //fs::read_to_string 함수를 사용해 파일을 열고 std::io::Result<String>을 반환한다. ? 연산자를 통해 현재의 함수로부터 에러 값을 받을 수 있다.
    let contents =
        fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

//Result를 통해 에러 처리
impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
