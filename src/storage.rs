use std::collections::HashMap;
use std::error::Error;
use std::env::var;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub data: HashMap<String, String>,
    folder_path: String,
    file_path: String,
}

impl Storage {
    pub fn new() -> Result<Storage, Box<dyn Error>> {
        let folder_path = var("FOLDER_PATH")?;
        let file_path = var("FILE_PATH")?;

        if !Path::new(&folder_path).exists() {
            fs::create_dir(&folder_path)?;
        }

        if !Path::new(&file_path).exists() {
            Storage::open_file(&file_path)?;
        }

        let mut file = Storage::open_file(&file_path)?;

        let filesize = file.metadata()?.size();
        let mut buffer = String::with_capacity(filesize as usize);
        let read_bytes = file.read_to_string(&mut buffer)?;

        if read_bytes != filesize as usize {
            return Err(format!("error, read {} bytes from {} bytes", read_bytes, filesize).into());
        }

        if read_bytes == 0 {
            return Ok(Storage {
                data: HashMap::with_capacity(1),
                file_path: file_path,
                folder_path: folder_path,
            });
        }

        let data = serde_json::from_str::<HashMap<String, String>>(&buffer)?;
        return Ok(Storage { data: data, file_path: file_path, folder_path: folder_path });
    }

    fn open_file(path: &str) -> io::Result<File> {
        return File::options()
            .append(false)
            .create(true)
            .read(true)
            .write(true)
            .mode(0o644)
            .open(path);
    }

    pub fn add(&mut self, k: &str, v: &str) {
        match self.data.get(k) {
            Some(_) => return,
            None => {
                self.data.insert(k.to_string(), v.to_string());
            }
        }
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.data.get(k)
    }

    pub fn update(&mut self, k: &str, v: &str) {
        match self.data.get(k) {
            Some(val) => {
                if v != val {
                    self.data.remove(k);
                    self.data.insert(k.to_string(), v.to_string());
                }
            }
            None => {}
        }
    }

    pub fn remove(&mut self, k: &str) {
        self.data.remove(k);
    }

    fn write_to_disc(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        fs::remove_file(filename)?;

        let marshalled = serde_json::to_string(&self.data)?;
        let mut file = Storage::open_file(filename)?;
        file.write_all(marshalled.as_bytes())?;

        return Ok(());
    }
}

// need to skip store test data to disc
#[cfg(not(debug_assertions))]
impl Drop for Storage {
    fn drop(&mut self) {
        match self.write_to_disc(&self.file_path) {
            Ok(_) => {}
            Err(err) => println!("error while saving data: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;
    use std::collections::HashMap;

    #[test]
    fn test_add_values() {
        let mut storage = Storage {
            data: HashMap::with_capacity(1),
            folder_path: "".to_string(),
            file_path: "".to_string(),
        };
        assert_eq!(0, storage.data.len());

        storage.add("key", "hehe");
        assert_eq!(3, storage.data.capacity());
        assert_eq!(1, storage.data.len());

        let val = storage.data.get("key").unwrap();
        assert_eq!("hehe", val);

        storage.add("key", "hehe1");
        assert_eq!(3, storage.data.capacity());
        assert_eq!(1, storage.data.len());

        let val = storage.data.get("key").unwrap();
        assert_eq!("hehe", val);

        storage.add("key3", "hehe3");
        assert_eq!(3, storage.data.capacity());
        assert_eq!(2, storage.data.len());

        let val = storage.data.get("key3").unwrap();
        assert_eq!("hehe3", val);
    }

    #[test]
    fn test_get_values() {
        let mut storage = Storage {
            data: HashMap::with_capacity(1),
            folder_path: "".to_string(),
            file_path: "".to_string(),
        };
        assert_eq!(0, storage.data.len());

        storage.add("key0", "hehe0");
        storage.add("key1", "hehe1");
        storage.add("key2", "hehe2");
        storage.add("key3", "hehe3");

        assert_eq!(7, storage.data.capacity());
        assert_eq!(4, storage.data.len());

        let val0 = storage.get("key0").unwrap();
        assert_eq!("hehe0", val0);

        let val1 = storage.get("key1").unwrap();
        assert_eq!("hehe1", val1);

        let val2 = storage.get("key2").unwrap();
        assert_eq!("hehe2", val2);

        let val3 = storage.get("key3").unwrap();
        assert_eq!("hehe3", val3);

        match storage.get("key4") {
            Some(_) => panic!("not hehe"),
            None => {}
        }
    }

    #[test]
    fn test_update_values() {
        let mut storage = Storage {
            data: HashMap::with_capacity(1),
            folder_path: "".to_string(),
            file_path: "".to_string(),
        };
        assert_eq!(0, storage.data.len());

        storage.add("key0", "hehe0");
        storage.add("key1", "hehe1");

        assert_eq!(3, storage.data.capacity());
        assert_eq!(2, storage.data.len());

        let val0 = storage.get("key0").unwrap();
        assert_eq!("hehe0", val0);

        let val1 = storage.get("key1").unwrap();
        assert_eq!("hehe1", val1);

        storage.update("key0", "hehe");
        assert_eq!(3, storage.data.capacity());
        assert_eq!(2, storage.data.len());

        let val0 = storage.get("key0").unwrap();
        assert_eq!("hehe", val0);

        let val1 = storage.get("key1").unwrap();
        assert_eq!("hehe1", val1);

        storage.update("notexists", "nothehe");
        assert_eq!(3, storage.data.capacity());
        assert_eq!(2, storage.data.len());

        let val0 = storage.get("key0").unwrap();
        assert_eq!("hehe", val0);

        let val1 = storage.get("key1").unwrap();
        assert_eq!("hehe1", val1);
    }

    #[test]
    fn test_remove_values() {
        let mut storage = Storage {
            data: HashMap::with_capacity(1),
            folder_path: "".to_string(),
            file_path: "".to_string(),
        };
        assert_eq!(0, storage.data.len());

        storage.add("key0", "hehe0");
        storage.add("key1", "hehe1");

        assert_eq!(3, storage.data.capacity());
        assert_eq!(2, storage.data.len());

        storage.remove("key0");
        match storage.get("key0") {
            Some(_) => panic!("not hehe"),
            None => {}
        }

        let val1 = storage.get("key1").unwrap();
        assert_eq!("hehe1", val1);

        assert_eq!(3, storage.data.capacity());
        assert_eq!(1, storage.data.len());

        storage.remove("notexists");
        let val1 = storage.get("key1").unwrap();
        assert_eq!("hehe1", val1);

        assert_eq!(3, storage.data.capacity());
        assert_eq!(1, storage.data.len());
    }
}
