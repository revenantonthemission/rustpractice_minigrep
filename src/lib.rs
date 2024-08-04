use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

//Result를 통해 에러 처리
impl Config {
    pub fn build(mut args: impl Iterator<Item=String>,) -> Result<Config, &'static str> {

        //인덱싱 대신 Iterator의 트레이트 메서드를 사용함.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        //환경 변수 검사
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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
    //반복자를 이용하면 중간에 가변 벡터를 만들 필요가 없다.
    //함수형 프로그래밍에서는 더 명확한 코드를 만들기 위해 변경 가능한 상태의 양을 최소화하는 편을 선호한다.
    
    /*let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results*/

    contents.lines().filter(|line| line.contains(query)).collect()
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
