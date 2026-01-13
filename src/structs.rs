use std::collections::{HashMap, VecDeque};

/// Structure Cache **LRU (Least Recently Used)** générique.
///
/// Rappel du principe : Stocke les éléments les plus récemment utilisés. Lorsque la capacité est dépassée, l’élément le moins récent est supprimé.
///
/// Accès rapide grâce à la `HashMap`. L’ordre d’utilisation est suivi via `VecDeque`.
/// (VecDeque est une double-ended queue, on peut ajouter ou retirer des éléments à l’avant ou à l’arrière. Dans notre cache LRU, on l’utilise pour garder la trace de l’ordre d’utilisation des clés)
pub struct Cache<K, V> where K: std::fmt::Display {
    /// Capacité maximale du cache.
    pub(crate) capacity: usize,
    /// Stockage clé → valeur.
    pub(crate) map: HashMap<K, V>,
    /// Ordre d’utilisation (front = Least Recently Used, back = Most Recently Used).
    pub(crate) order: VecDeque<K>,
}