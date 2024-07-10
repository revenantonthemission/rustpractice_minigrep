//커맨드 라인 인수로 전달된 값들을 읽기 위해 std::env::args 함수를 사용한다.
use std::env;
use std::process;

//라이브러리 크레이트의 코드를 가져온다.
use minigrep::Config;

fn main() {
    //cargo run [options] [-- args]이기 때문에 --는 따로 처리하지 않아도 env::args().collect()를 통해 수집될 수 있다.
    let args: Vec<String> = env::args().collect();

    //unwrap_or_else를 사용하면 Result가 Ok일 때 Ok 안의 값을 반환하고, Err일 때 클로저 안의 코드를 호출한다.
    let config = Config::build(&args).unwrap_or_else(|err| {
        //표준 에러 출력을 위한 eprintln! 매크로
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //실행 파트를 run으로 분리.
    if let Err(e) = minigrep::run(config) {
        //표준 에러 출력을 위한 eprintln! 매크로
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
