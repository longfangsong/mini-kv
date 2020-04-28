use std::process::Command;
use std::fs::File;
use std::str::from_utf8;

/// e2e test
#[test]
fn test_server() {
    let file = File::open("./src/test/test_case.txt").unwrap();
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("client")
        .stdin(file)
        .output()
        .expect("Failed to execute command");
    let result = from_utf8(&output.stdout).unwrap();
    let result: Vec<_> = result
        .split(">>>")
        .map(|it| it.split('\n'))
        .flatten()
        .map(|it| it.trim())
        .map(|it| it.trim_matches('\u{0}'))
        .filter(|it| it != &"" && !it.starts_with("waring"))
        .collect();
    assert_eq!(result[0], "cursor: 0");
    assert_eq!(result[1], "(0): a");
    assert_eq!(result[2], "(1): b");
    assert_eq!(result[3], "(2): c");
    assert_eq!(result[4], "cursor: 0");
    assert_eq!(result[5], "(0): b");
    assert_eq!(result[6], "(1): c");
    assert_eq!(result[7], "5");
    assert_eq!(result[8], "3");
}