use std::ops::Deref;
use std::{fmt::Display, marker::PhantomData, path::PathBuf};

use cacache::Integrity;
use chashmap::CHashMap;
use color_eyre::owo_colors::OwoColorize;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct SerdeCacache<K, V> {
    name: PathBuf,
    test: CHashMap<String, Vec<u8>>,
    test_serde: CHashMap<String, V>,
    _phantom_data: PhantomData<V>,
    _phantom_key: PhantomData<K>,
}

impl<K, V> SerdeCacache<K, V>
where
    V: Serialize + DeserializeOwned + Clone + PartialEq + Eq,
    K: Display,
{
    pub fn new(name: PathBuf) -> Self {
        Self {
            name,
            test: CHashMap::new(),
            test_serde: CHashMap::new(),
            _phantom_data: PhantomData,
            _phantom_key: PhantomData,
        }
    }

    /// Set an item in the cache
    pub async fn set(&self, key: &K, data: &V) -> color_eyre::Result<Integrity> {
        println!("Writting {} to cache: {}", key, self.name.to_string_lossy());
        //self.set_cache_intergrity(key, data);
        let serialized = rmp_serde::to_vec(data)?;
        Ok(cacache::write(&self.name, key.to_string(), serialized).await?)
    }

    /// Get an item and return an option if it isn't found. This is more akin to a [`HashMap`](std::collections::HashMap)
    pub async fn get(&self, key: &K) -> color_eyre::Result<Option<V>> {
        let read = cacache::read(&self.name, key.to_string()).await;


        match read {
            Ok(val) => {
                //self.check_cache_integrity(key, &val);
                //self.check_cache_integrity_v(key, &val);
                Ok(Some(rmp_serde::from_slice(&val)?))},
            Err(cacache::Error::EntryNotFound(_, _)) => Ok(None),
            val => {
                val?;
                Ok(None)
            }
        }
    }

    fn set_cache_intergrity(&self,key: &K, expected: &V) {
        println!("{}", "Setting intergrity...".to_string().purple());
        self.test_serde.insert(key.to_string(), expected.clone());
        self.test.insert(key.to_string(), rmp_serde::to_vec(&expected).unwrap());
    }

    fn check_cache_integrity(&self, key: &K, expected: &Vec<u8>) {
        match self.test.get(&key.to_string()) {
            Some(integrity) => {
                if &(integrity.deref()) == &expected {
                    println!("{}", "Integrity match!".to_string().green());
                } else {
                    println!("{}", "Integrity fail!".to_string().red());
                }
            },
            None => {println!("{}", "Integrity uncached!".to_string().yellow());}
        }
    }

    fn check_cache_integrity_v(&self, key: &K, expected: &Vec<u8>) {
        match self.test_serde.get(&key.to_string()) {
            Some(integrity) => {
                let convert: V  = rmp_serde::from_slice(&expected).unwrap();
                if *integrity == convert {
                    println!("{}", "Serde Integrity match!".to_string().green());
                } else {
                    println!("{}", "Serde Integrity fail!".to_string().red());
                }
            },
            None => {println!("{}", "Serde Integrity uncached!".to_string().yellow());}
        }
    }

    /// Get an item from the cache.
    pub async fn get_as_result(&self, key: &K) -> color_eyre::Result<V> {
        let read: Vec<u8> = cacache::read(&self.name, key.to_string()).await?;
        Ok(rmp_serde::from_slice(&read)?)
    }
}
