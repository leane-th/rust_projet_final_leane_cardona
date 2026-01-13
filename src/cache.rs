use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::str::FromStr;
use crate::persistence;
pub(crate) use crate::structs::Cache;
use crate::traits::CacheTrait;

/// Implémentation du cache LRU générique
impl<K: Eq + Hash + Clone + std::fmt::Display, V: Clone> Cache<K, V> {
    /// Crée un nouveau cache de capacité donnée
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Déplace une clé à la fin de la VecDeque (MRU)
    fn move_to_back(&mut self, key: &K) {
        // Retire la clé de sa position actuelle
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        // La remet à la fin (plus récemment utilisée)
        self.order.push_back(key.clone());
    }
}

/// Implémentation du trait générique pour LRU
///
/// # Exemple d'utilisation avec String/String
/// ```rust
/// use lru_cache::structs::Cache;
/// use lru_cache::traits::CacheTrait;
///
/// let mut cache: Cache<String, String> = Cache::new(3);
/// cache.put("A".into(), "value_a".into());
/// cache.put("B".into(), "value_b".into());
/// cache.put("C".into(), "value_c".into());
///
/// // Accès met à jour l'ordre LRU
/// assert_eq!(cache.get(&"B".to_string()), Some("value_b".to_string()));
///
/// // Sauvegarde et rechargement
/// cache.save_as_strings("cache-exemple.txt");
/// let mut restored: Cache<String, String> = Cache::new(3);
/// restored.load_from_strings("cache-exemple.txt");
/// assert_eq!(restored.get(&"B".to_string()), Some("value_b".to_string()));
///
/// // Nettoyage du fichier
/// let _ = std::fs::remove_file("cache-exemple.txt");
/// ```
///
/// # Exemple avec i32/i32
/// ```rust
/// use lru_cache::structs::Cache;
/// use lru_cache::traits::CacheTrait;
///
/// let mut cache: Cache<i32, i32> = Cache::new(3);
/// cache.put(1, 10);
/// cache.put(2, 20);
/// cache.put(3, 30);
///
/// // Sauvegarde
/// cache.save_as_strings("cache-exemple-i32.txt");
///
/// // Rechargement
/// let mut restored: Cache<i32, i32> = Cache::new(3);
/// restored.load_from_strings("cache-exemple-i32.txt");
/// assert_eq!(restored.get(&2), Some(20));
///
/// // Nettoyage du fichier
/// let _ = std::fs::remove_file("cache-exemple-i32.txt");
/// ```
impl<K: FromStr + ToString + Eq + Hash + Clone + std::fmt::Display, V: FromStr + ToString + Clone> CacheTrait<K, V> for Cache<K, V> {

    // O(1) pour l'accès à la valeur (HashMap)
    fn get(&mut self, key: &K) -> Option<V> {
        let value = self.map.get(key)?.clone();
        self.move_to_back(key);
        Some(value)
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        let inserted = self.map.insert(key.clone(), value);

        self.move_to_back(&key);

        if self.map.len() > self.capacity {
            if let Some(lru_key) = self.order.pop_front() {
                self.map.remove(&lru_key);
            }
        }

        inserted
    }

    fn save_as_strings(&self, path: &str) {
        // Transforme le cache générique en Cache<String, String> pour ensuite le persister
        let mut string_cache = crate::structs::Cache::<String, String>::new(self.order.len());
        for key in &self.order {
            if let Some(value) = self.map.get(key) {
                string_cache.put(key.to_string(), value.to_string());
            }
        }

        persistence::save(path, &string_cache);
    }

    fn load_from_strings(&mut self, path: &str) {
        // Convertit le cache générique en Cache<String, String> intermédiaire
        let mut string_cache: crate::structs::Cache<String, String> = crate::structs::Cache::new(self.capacity);
        crate::persistence::load(path, &mut string_cache);

        self.map.clear();
        self.order.clear();

        // Reconvertit les String en types K et V via FromStr
        for key in &string_cache.order {
            if let (Ok(k), Ok(v)) = (K::from_str(key), V::from_str(&string_cache.map[key])) {
                self.put(k, v);
            }
        }
    }
}