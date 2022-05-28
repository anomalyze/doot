use anyhow::{Result, Context};
use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::Deserialize;
use std::{
    env, 
    fs::create_dir_all, 
    os::unix::fs, 
    path::Path
};

#[derive(Debug, Default, Deserialize)]
struct DotFile {
    name: String,
    src_path: String,
    dst_path: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    dotfiles: Vec<DotFile>,
}

fn main() -> Result<()> {
    banner();
    let home = env::var("HOME")?;
    let config: Config = Figment::new()
    .merge(Yaml::file(&format!("{}/.doot.yaml", home)))
    .merge(Yaml::file(&format!("{}/.doot.yml", home)))
    .merge(Yaml::file(".doot.yaml"))
    .merge(Yaml::file(".doot.yaml"))
    .extract().context("missing config file")?;

    for d in config.dotfiles.into_iter() {
        linker(&d)?;
    }

    Ok(())
}

fn linker(df: &DotFile) -> Result<()> {
    let home = env::var("HOME")?;
    let mut src = String::default();
    let mut dst = String::default();

    if df.src_path.contains("$HOME") {
        src = df.src_path.replace("$HOME", home.as_str());
    } else {
        src = df.src_path.clone()
    }

    if df.dst_path.contains("$HOME") {
        dst = df.dst_path.replace("$HOME", home.as_str());
    } else {
        dst = df.dst_path.clone()
    }

    println!("[*] Creating directory structure...");
    let parent = Path::new(&dst).parent().unwrap();
    match create_dir_all(parent) {
        Err(e) => println!("[-] Error: {}",  e),
        Ok(_) => println!("[+] Created: {}",parent.display())
    }

    println!("[*] Attempting link for {}...", &df.name);
    match fs::symlink(src, dst) {
        Err(e) => println!("[-] Error: {}", e),
        Ok(_) => println!("[+] Linked: {}",df.name),
    }
    Ok(())
}

fn banner() {
    println!("
        ><<                      ><<  
        ><<                      ><<  
        ><<   ><<       ><<    ><>< ><
    ><< ><< ><<  ><<  ><<  ><<   ><<  
   ><   ><<><<    ><<><<    ><<  ><<  
   ><   ><< ><<  ><<  ><<  ><<   ><<  
    ><< ><<   ><<       ><<       ><< 
                dotfile linker v0.1.0\n");
}
