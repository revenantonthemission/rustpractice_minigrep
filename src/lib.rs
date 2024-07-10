use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

//Result를 통해 에러 처리
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

//1. 유닛 타입()을 반환하는 함수에서 Result<(), Box<dyn Error>>를 반환하는 함수로 변경한다. 이때 ()는 Ok(())다.
//2. Box<dyn Error>는 트레이트 객체로, Error 트레이트를 구현한 어떤 타입을 반환한다.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //fs::read_to_string 함수를 사용해 파일을 열고 std::io::Result<String>을 반환한다. ? 연산자를 통해 현재의 함수로부터 에러 값을 받을 수 있다.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

//테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
