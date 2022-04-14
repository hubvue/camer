use std::error::Error;
use std::fs;
use std::path::Path;

const CAMRC: &str = "~/.camrc";

pub fn exist() -> bool {
    let path = Path::new(CAMRC);
    path.exists()
}

pub fn read() -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(CAMRC)?;
    Ok(content)
}

pub fn delete() -> Result<(), Box<dyn Error>> {
    let result = fs::remove_file(CAMRC)?;
    Ok(result)
}

pub fn append(value: String) -> Result<(), Box<dyn Error>> {
    let file_res = read()?;

    match file_res {
        Ok(content) => {}
        Err(error) => Err(error),
    }
    if let Ok(content) = file_content {
        let write_res = fs::write(CAMRC, content)?;
        return Ok(write_res);
    } else {
        return Err(file_content);
    }
}
