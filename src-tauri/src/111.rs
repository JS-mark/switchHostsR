use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let path = "/etc/hosts"; // hosts 文件路径
    let mut hosts = BufReader::new(File::open(path).unwrap());

    let mut buffer = String::new();
    let mut found = false;

    // 在 hosts 文件中查找需要替换的行
    for line in hosts.lines() {
        let line = line.unwrap();
        if line.contains("example.com") {
            buffer.push_str("# This is a comment.\n");
            found = true;
        } else {
            buffer.push_str(&line);
            buffer.push_str("\n");
        }
    }

    // 如果没有找到需要替换的行，则添加一行新的记录
    if !found {
        buffer.push_str("127.0.0.1 example.com\n");
    }

    // 将修改后的 hosts 文件写回磁盘
    let mut file = File::create(path).unwrap();
    file.write_all(buffer.as_bytes()).unwrap();
}
