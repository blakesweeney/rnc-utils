use std::io::prelude::*;
use std::error::Error;

pub fn each_json_line(
    mut reader: Box<dyn BufRead>,
    f: impl Fn(serde_json::Value) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {

    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf)? {
            0 => break,
            _ => {
                let cleaned = buf.replace("\\\\", "\\");
                let data: serde_json::Value = serde_json::from_str(&cleaned)?;
                f(data)?;
                buf.clear();
            }
        }
    }

    Ok(())
}
