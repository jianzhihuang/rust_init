use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("test.txt")?;

    // 每次寫入約 1MB 的數據，重複 300 次來達到大約 300MB
    for _ in 0..2 {
        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1_000_000) // 生成約 1MB 的數據
            .map(char::from)
            .collect::<Vec<_>>()
            .chunks(30)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        for (i, c) in data.chars().enumerate() {
            match i {
                5 => {
                    // 在第 5 個元素的位置寫入 "我"
                    file.write_all('我'.encode_utf8(&mut [0; 4]).as_bytes())?;
                }
                10 => {
                    // 在第 10 個元素的位置寫入 "你"
                    file.write_all('你'.encode_utf8(&mut [0; 4]).as_bytes())?;
                }
                _ => {
                    file.write_all(c.encode_utf8(&mut [0; 4]).as_bytes())?;
                }
            }
        }

        file.write_all(data.as_bytes())?;
    }

    Ok(())
}
