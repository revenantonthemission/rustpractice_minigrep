use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

//Result를 통해 에러 처리
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        //환경 변수 검사
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case, })
    }
}

//1. 유닛 타입()을 반환하는 함수에서 Result<(), Box<dyn Error>>를 반환하는 함수로 변경한다. 이때 ()는 Ok(())다.
//2. Box<dyn Error>는 트레이트 객체로, Error 트레이트를 구현한 어떤 타입을 반환한다.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //fs::read_to_string 함수를 사용해 파일을 열고 std::io::Result<String>을 반환한다. ? 연산자를 통해 현재의 함수로부터 에러 값을 받을 수 있다.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
      search_case_insensitive(&config.query, &contents)
    } else {
      search(&config.query, &contents)
    };

    for line in results {
      println!("{line}");
    }

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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
