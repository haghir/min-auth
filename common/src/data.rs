use fs2::FileExt;
use serde::de::DeserializeOwned;
use std::{
    collections::LinkedList,
    fs::{read_dir, File, ReadDir},
    io::BufReader,
    marker::PhantomData,
    path::Path,
};

pub mod credentials;
pub mod requests;
pub mod users;

pub trait DataLoader
where
    Self: DeserializeOwned,
{
    fn load<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        file.lock_shared()?;

        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}

pub struct DataFinder<T>
where
    T: DataLoader,
{
    readers: LinkedList<ReadDir>,
    _x: PhantomData<T>,
}

impl<T> DataFinder<T>
where
    T: DataLoader,
{
    pub fn new<P>(data_dir: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let data_dir = data_dir.as_ref();
        let reader = read_dir(data_dir)?;
        let mut readers = LinkedList::new();
        readers.push_back(reader);
        Ok(Self {
            readers,
            _x: Default::default(),
        })
    }
}

impl<T> Iterator for DataFinder<T>
where
    T: DataLoader,
{
    type Item = Result<T, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let reader = match self.readers.back_mut() {
                Some(item) => item,
                None => return None,
            };

            let entry = match reader.next() {
                Some(item) => item,
                None => {
                    self.readers.pop_back();
                    continue;
                }
            };

            let entry = match entry {
                Ok(item) => item,
                Err(e) => return Some(Err(e.into())),
            };

            let ftype = match entry.file_type() {
                Ok(item) => item,
                Err(e) => return Some(Err(e.into())),
            };

            let path = entry.path();
            if ftype.is_dir() {
                self.readers.push_back(match read_dir(&path) {
                    Ok(item) => item,
                    Err(e) => return Some(Err(e.into())),
                });
                continue;
            }

            if let Some(x) = path.extension() {
                if x != "json" {
                    continue;
                }
            } else {
                continue;
            }

            return Some(T::load(path));
        }
    }
}

pub fn load_all<T, P>(data_dir: P) -> Result<Vec<T>, std::io::Error>
where
    P: AsRef<Path>,
    T: DataLoader,
{
    let data_dir = data_dir.as_ref();
    let mut ret = Vec::new();
    for data in DataFinder::new(&data_dir)? {
        match data {
            Ok(item) => ret.push(item),
            Err(e) => return Err(e),
        }
    }

    Ok(ret)
}
