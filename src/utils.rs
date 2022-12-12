use anyhow::{anyhow, Result};
use regex::Regex;
use std::{env, fs, path::PathBuf};

pub fn can_skip_emptying(target_dir: &String) -> Result<bool> {
  let root_path = env::current_dir().unwrap();
  let target_dir: PathBuf = target_dir.into();
  let target_path = root_path.join(&target_dir);
  if !target_path.is_dir() & !target_path.is_file() {
    return Ok(true);
  } else if target_path.is_file() {
    return Err(anyhow!("not a directory, scandir {:?}", target_dir));
  }
  let dir = fs::read_dir(target_path).unwrap();
  let mut count = 0;
  for x in dir {
    if let Ok(path) = x {
      if path.file_name().to_str().unwrap() != ".git" {
        count += 1;
      }
    }
  }
  return Ok(count == 0);
}

pub fn is_valid_package_name(package_name: &String) -> Result<bool> {
  let reg = Regex::new(r"^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$")?;
  Ok(reg.is_match(package_name))
}

pub fn to_valid_package_name(package_name: &String) -> Result<String> {
  let result = package_name.clone();

  let result = Regex::new(r"\s+")?
    .replace_all(&result.trim(), "_")
    .to_string();

  let result = Regex::new(r"^[._]")?.replace(&result, "").to_string();
  let result = Regex::new(r"[^a-z0-9-~]+")?
    .replace_all(&result, "-")
    .to_string();
  Ok(result)
}


pub fn get_path_from_cwd(path: &String) -> Result<PathBuf> {
  let root_path = env::current_dir().unwrap();
  let path: PathBuf = path.into();
  let path = root_path.join(&path);
  Ok(path)
}

pub fn empty_dir(target_dir: &String) -> Result<()> {
  
  let target_path = get_path_from_cwd(target_dir)?;
  if !target_path.is_dir() & !target_path.is_file() {
    return Ok(());
  } else if target_path.is_file() {
    return Err(anyhow!("not a directory, scandir {:?}", target_dir));
  }
  let dir = fs::read_dir(target_path).unwrap();
  for x in dir {
    if let Ok(path) = x {
      if path.file_name().to_str().unwrap() != ".git" {
        let path = path.path();
        if path.is_dir() {
          fs::remove_dir_all(path)?;
        } else {
          fs::remove_file(path)?;
        }
      }
    }
  }
  Ok(())
}


pub fn mkdir(target_dir: &String) -> Result<()> {
  let target_path = get_path_from_cwd(&target_dir)?;
  if !target_path.is_dir() & !target_path.is_file() {
    fs::create_dir_all(target_path)?;
  }
  Ok(())
}


#[test]
fn test_is_valid_pageage_name() {
  assert_eq!(
    is_valid_package_name(&String::from("_dsdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from(".dsdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from(r"d\sdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from("d!sdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from("d`sdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from("d(sdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from("d)sdsd")).unwrap(),
    false
  );
  assert_eq!(
    is_valid_package_name(&String::from("ds'dsd")).unwrap(),
    false
  );
}

#[test]
fn test_to_valid_package_name() {
  assert_eq!(
    to_valid_package_name(&String::from(" dsdsd")).unwrap(),
    "dsdsd".to_string()
  );
  assert_eq!(
    to_valid_package_name(&String::from("d sdsd")).unwrap(),
    "d-sdsd".to_string()
  );

  assert_eq!(
    to_valid_package_name(&String::from("d%%%sdsd")).unwrap(),
    "d-sdsd".to_string()
  );
}

#[test]
fn test_emoty_dir(){
  match empty_dir(&String::from("lang")){
    Ok(_) => {},
    _ => {}
  };
}