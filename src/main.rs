use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("test.txt")?;

    // 每次寫入約 1MB 的數據，重複 300 次來達到大約 300MB
    for _ in 0..300 {
        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1_000_000) // 生成約 1MB 的數據
            .map(char::from)
            .collect::<Vec<_>>()
            .chunks(30)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        file.write_all(data.as_bytes())?;
    }

    Ok(())
}
