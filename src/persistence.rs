use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;
use crate::structs::Cache;
use crate::traits::CacheTrait;

/// Sauvegarde le contenu d’un cache LRU dans un fichier.
///
/// Chaque paire clé → valeur est écrite sur une ligne au format `clé=valeur`.
///
/// # Exemple
/// ```rust
/// use lru_cache::structs::Cache;
/// use lru_cache::persistence;
/// use lru_cache::traits::CacheTrait;
///
/// let mut cache: Cache<String, String> = Cache::new(3);
/// cache.put("A".to_string(), "value_a".to_string());
/// cache.put("B".to_string(), "value_b".to_string());
///
/// persistence::save("mon_cache.txt", &cache);
///
/// ```
pub fn save<K: ToString + Clone + std::fmt::Display, V: ToString + Clone>(path: &str, cache: &Cache<K, V>) where K: Eq, K: Hash {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("cannot open file");

    for key in &cache.order {
        if let Some(value) = cache.map.get(key) {
            let _ = writeln!(file, "{}={}", key.to_string(), value.to_string());
        }
    }
}

/// Charge le contenu d’un fichier dans un cache LRU.
///
/// Lit chaque ligne du fichier au format `clé=valeur` et insère les éléments dans le cache.
///
/// # Exemple
/// ```rust
/// use lru_cache::structs::Cache;
/// use lru_cache::persistence;
///
/// let mut cache: Cache<String, String> = Cache::new(3);
/// persistence::load("mon_cache.txt", &mut cache);
///
/// // Nettoyage du fichier
/// let _ = std::fs::remove_file("mon_cache.txt");
/// ```
pub fn load<K, V>(path: &str, cache: &mut Cache<K, V>)
where
    K: FromStr + Eq + Hash + Clone + std::fmt::Display,
    V: FromStr + Clone, V: ToString
{
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return, // Si le fichier est inexistant alors on ne fait rien
    };

    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut parts = line.splitn(2, '=');
            if let (Some(k_str), Some(v_str)) = (parts.next(), parts.next()) {
                if let (Ok(k), Ok(v)) = (K::from_str(k_str), V::from_str(v_str)) {
                    cache.put(k, v);
                }
            }
        }
    }
}

