use std::{
  fs::{self, File},
  io::BufReader,
  path::PathBuf,
};

use anyhow::Result;
use serde_json::{json, Value};

pub fn render_template(src: &PathBuf, dest: &PathBuf) -> Result<()> {
  if src.is_dir() {
    if src.file_name().unwrap().to_str().unwrap() == "node_modules" {
      return Ok(());
    }

    match fs::create_dir(dest) {
      Ok(_) => {}
      _ => {}
    };
    let read_dir = fs::read_dir(src).unwrap();

    for file in read_dir {
      if let Ok(entry) = file {
        let path = entry.path();
        let dest_path = dest.join(path.file_name().unwrap());
        render_template(&path, &dest_path).unwrap();
      }
    }
    return Ok(());
  }

  let file_name = src.file_name().unwrap();

  if file_name == "package.json" {
    let file1 = File::open(&src)?;
    let reader1 = BufReader::new(file1);

    let pkg = serde_json::from_reader(reader1)?;

    let file2 = File::open(&dest)?;
    let reader2 = BufReader::new(file2);
    let pkg2 = serde_json::from_reader(reader2)?;

    let value = merge(&pkg, &pkg2).unwrap();
    let value = sort_pkg(&value).unwrap();
    let content = serde_json::to_string_pretty(&value).unwrap();

    fs::write(dest, content).unwrap();
    return Ok(());
  }

  let mut dest = dest.clone();
  if file_name.to_str().unwrap().starts_with("_") {
    dest = dest.to_str().unwrap().replace("_", ".").into();
  }


  if file_name == "_gitignore" && dest.is_file() {
    let old_ignore = fs::read_to_string(&dest).unwrap();
    let new_ignore = fs::read_to_string(&src).unwrap();

    let contents = format!("{}\n{}", old_ignore, new_ignore);
    fs::write(&dest, contents)?;
    
    return Ok(());
  }

  println!("file_name {:?}!!!!,{:?},{:?}",file_name,dest,dest.is_file());

  fs::copy(src, dest).unwrap();

  Ok(())
}

pub fn merge(obj1: &Value, obj2: &Value) -> Result<Value> {
  let mut obj1 = obj1.clone();
  let obj2 = obj2.clone();

  if obj1.is_object() && obj2.is_object() {
    let obj1 = obj1.as_object_mut().unwrap();
    let obj2 = obj2.as_object().unwrap();

    for (key, value) in obj2 {
      if obj1.contains_key(key) {
        obj1.insert(key.clone(), merge(&obj1[key], value)?);
      } else {
        obj1.insert(key.clone(), value.clone());
      }
    }
  } else if obj1.is_array() && obj2.is_array() {
    let obj1 = obj1.as_array_mut().unwrap();
    let obj2 = obj2.as_array().unwrap();

    for value in obj2 {
      obj1.push(value.clone());
    }
  } else {
    return Ok(obj2);
  }

  Ok(obj1)
}

fn sort_pkg(pkg: &Value) -> Result<Value> {
  let mut res = json!({});

  let dep_keys = vec![
    "dependencies",
    "devDependencies",
    "peerDependencies",
    "optionalDependencies",
  ];

  for key in pkg.as_object().unwrap().keys() {
    if !dep_keys.contains(&key.as_str()) {
      res[key] = pkg[key].clone()
    }
  }

  for key in dep_keys {
    if pkg.get(key).is_some() {
      res[key] = json!({});

      let mut keys = pkg[key].as_object().unwrap().keys().collect::<Vec<_>>();

      keys.sort();
      for k in keys {
        res[key][k] = pkg[key][k].clone();
      }
    }
  }
  Ok(res)
}
