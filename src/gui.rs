use std::{env, fs::remove_dir_all, path::{Path, PathBuf}};
use npm_rs::NpmEnv;

static GUI_DIR: &str = "gui";
static DIST_DIR: &str = "dist";
static URL_ENV: &str = "VITE_SERVER_DOMAIN";

fn remove_node_modules() {
    let node_modules_path = Path::new(GUI_DIR).join("node_modules");

    if !node_modules_path.exists() || !node_modules_path.is_dir() {
        return;
    }

    if let Err(e) = remove_dir_all(node_modules_path) {
        eprintln!("Failed to remove node_modules folder: {}, but we can continue as is", e);
    }
}

pub fn ensure_gui_ready() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dist_dir = Path::new(GUI_DIR).join(DIST_DIR);

    let is_gui_ready = dist_dir.exists() && dist_dir.is_dir() && dist_dir.join("index.html").exists();

    if is_gui_ready {
        return Ok(dist_dir);
    }

    println!("Building GUI...");

    let original_dir = env::current_dir()?;
    env::set_current_dir(GUI_DIR)?;

    NpmEnv::default()
        .with_env(URL_ENV, "")
        .init_env()
        .install(None)
        .run("build")
        .exec()?;

    env::set_current_dir(original_dir)?;

    remove_node_modules();

    let is_gui_ready = dist_dir.exists() && dist_dir.is_dir() && dist_dir.join("index.html").exists();

    if !is_gui_ready {
        return Err("GUI is not ready".into());
    }

    Ok(dist_dir)
}
