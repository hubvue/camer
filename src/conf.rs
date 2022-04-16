use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const CAMRC: &str = ".camrc";

fn get_home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

fn get_file_path(filename: &str) -> String {
    get_home_dir()
        .map(|home| home.join(filename))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn exist(path: String) -> bool {
    let path = Path::new(&path);
    path.exists()
}

// .camrc
pub fn exist_camrc() -> bool {
    let camrc_path = get_file_path(CAMRC);
    exist(camrc_path)
}

pub fn create_camrc() -> Result<(), Box<dyn Error>> {
    let camrc_path = get_file_path(CAMRC);
    let _file = fs::File::create(camrc_path)?;
    Ok(())
}

pub fn read_camrc() -> Result<String, Box<dyn Error>> {
    let camrc_path = get_file_path(CAMRC);
    let content = fs::read_to_string(camrc_path)?;
    Ok(content)
}

pub fn delete_camrc() -> Result<(), Box<dyn Error>> {
    let camrc_path = get_file_path(CAMRC);
    let result = fs::remove_file(camrc_path)?;
    Ok(result)
}

pub fn append_camrc(content: String) -> Result<(), Box<dyn Error>> {
    let file_content = read_camrc()?;
    let append_content = file_content + "\n" + &content;

    let camrc_path = get_file_path(CAMRC);
    let write_res = fs::write(camrc_path, append_content)?;
    Ok(write_res)
}

pub fn write_camrc(content: String) -> Result<(), Box<dyn Error>> {
    let camrc_path = get_file_path(CAMRC);
    let write_res = fs::write(camrc_path, content)?;
    Ok(write_res)
}

// env
fn get_env_path() -> Option<String> {
    let filenames = [".zshrc", ".bashrc", ".bash_profile"];
    let env_path = filenames
        .iter()
        .map(|&filename| get_file_path(filename))
        .filter(|filepath| exist(filepath.to_string()))
        .nth(0);

    return env_path;
}

const CAMRC_ENV_CONTENT: &str = "
# ====camrc====> 
source ~/.camrc
# ========>
";
pub fn mount_camrc_env() -> Result<(), Box<dyn Error>> {
    let res = match get_env_path() {
        Some(evn_path) => {
            let path = Path::new(evn_path.as_str());
            let env_content = fs::read_to_string(path)?;
            let write_res = fs::write(path, env_content + CAMRC_ENV_CONTENT)?;
            write_res
        }
        None => (),
    };
    Ok(res)
}

pub fn unmount_camrc_env() -> Result<(), Box<dyn Error>> {
    let res = match get_env_path() {
        Some(evn_path) => {
            let path = Path::new(evn_path.as_str());
            let env_content = fs::read_to_string(path)?;
            let replace_res = env_content.replace(CAMRC_ENV_CONTENT, "");
            let write_res = fs::write(path, replace_res)?;
            write_res
        }
        None => (),
    };
    Ok(res)
}
