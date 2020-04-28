use std::process::Command;
use std::fs::File;
use std::str::from_utf8;

/// e2e test
/// clean the redo log before this
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
        .map(|it| if it.starts_with('(') {
            &it[5..]
        } else {
            it
        })
        .filter(|it| it != &"" && !it.starts_with("waring"))
        .collect();
    assert_eq!(result[0], "cursor: 0");
    assert!([&result[1], &result[2], &result[3]].contains(&&&"a"));
    assert!([&result[1], &result[2], &result[3]].contains(&&&"b"));
    assert!([&result[1], &result[2], &result[3]].contains(&&&"c"));
    assert_eq!(result[4], "cursor: 0");
    assert!([&result[5], &result[6]].contains(&&&"c"));
    assert!([&result[5], &result[6]].contains(&&&"b"));
    assert_eq!(result[7], "5");
    assert_eq!(result[8], "3");
}

