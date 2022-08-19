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
const DEFAULT_ALIASS: &str = "
## === git ===
alias gs='git status'
alias ga='git add'
alias gaa='git add -A'
alias gd='git diff'
alias gdc='git diff --cached'
alias gb='git branch'
alias gba='git branch -a'
alias gc='git commit -m'
alias gca='git commit -am'
alias gco='git checkout'
alias gcb='git checkout -b'
alias gcl='git clone'
alias gcp='git cherry-pick'
alias gm='git merge'
alias gma='git merge --abort'
alias gpl='git pull'
alias gps='git push'
alias gpsa='git push --all'
alias gpsu='git push --set-upstream origin'
alias grh='git reset HEAD'
alias grhh='git reset HEAD --hard'
alias grm='git rm'
alias grma='git rm --cached'
alias grc='git rebase --continue'
alias grs='git rebase --skip'
alias gst='git stash'
alias gsta='git stash apply'
alias gstd='git stash drop'
alias gstl='git stash list'
alias gstp='git stash pop'
alias gsh='git show'
alias gshh='git show HEAD'
alias gss='git show --stat'
alias gsl='git shortlog'
alias gslc='git shortlog -sn'
alias gt='git tag'
alias gta='git tag -a'
alias gtl='git tag -l'
alias gmv='git mv'
## === npm ===
alias ni='npm install'
alias ya='yarn add'
alias pa='pnpm add'
alias nr='npm run'
alias s='npm run start'
alias t='npm run test'
alias p='npm run prod'
alias d='npm run dev'
alias b='npm run build'
";
pub fn create_camrc() -> Result<(), Box<dyn Error>> {
    let camrc_path = get_file_path(CAMRC);
    let path = Path::new(camrc_path.as_str());
    let _file = fs::File::create(path)?;
    let write_res = fs::write(path, DEFAULT_ALIASS)?;
    Ok(write_res)
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

pub fn remove_camrc(content: &str) -> Result<(), Box<dyn Error>> {
    let camrc_path = get_file_path(CAMRC);
    let file_content = read_camrc()?;
    let replace_res = file_content.replace(content, "");
    let write_res = fs::write(camrc_path, replace_res)?;
    Ok(write_res)
}

// env
pub fn get_env_path() -> Option<String> {
    let filenames = [".zshrc", ".bashrc", ".bash_profile"];
    let env_path = filenames
        .iter()
        .map(|&filename| get_file_path(filename))
        .find(|filepath| exist(filepath.to_string()));

    env_path
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
            fs::write(path, env_content + CAMRC_ENV_CONTENT)?
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
            fs::write(path, replace_res)?
        }
        None => (),
    };
    Ok(res)
}
