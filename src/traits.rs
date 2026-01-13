/// Trait générique pour un **cache clé → valeur**.
///
/// Permet de définir un comportement standard pour différents types de caches
pub trait CacheTrait<K, V> {
    /// Récupère la valeur associée à une clé.
    fn get(&mut self, key: &K) -> Option<V>;

    /// Ajoute une valeur dans le cache.
    fn put(&mut self, key: K, value: V) -> Option<V>;
    
    /// Sauvegarde le cache dans un fichier
    fn save_as_strings(&self, path: &str);
    
    /// Charge le cache depuis un fichier en convertissant les clés/valeurs en types K et V
    fn load_from_strings(&mut self, path: &str);
}