use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tempfile::NamedTempFile;

static CONFIG_FILE: &'static str = r#"
[web]
host = "127.0.0.1"
port = 8002
"#;

#[test]
fn test_web_server_without_config() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8080");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 200);
}

#[test]
fn test_web_server_with_arguments() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--host", "127.0.0.1", "--port", "8001"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8001");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 200);
}

#[test]
fn test_web_server_with_config() {
    let mut file = NamedTempFile::new().unwrap();

    write!(file, "{}", CONFIG_FILE).unwrap();

    let path = file.path().as_os_str().to_str().unwrap();
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--config", path])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8002");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 200);
}

#[test]
fn test_web_server_404() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--host", "127.0.0.1", "--port", "8003"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8003/404");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 404);
}
