use lru_cache::traits::CacheTrait;
use lru_cache::Cache;

fn main() {
    println!("--- Création d'un cache LRU String/String de taille 3 ---");
    let mut cache: Cache<String, String> = Cache::new(3);

    println!("Ajout des éléments A, B, C");
    cache.put("A".into(), "value_a".into());
    cache.put("B".into(), "value_b".into());
    cache.put("C".into(), "value_c".into());

    println!("\n--- Accès au cache pour mettre à jour l'ordre LRU ---");
    println!("get(B) = {:?}", cache.get(&"B".to_string()));

    println!("\n--- Ajout d'un nouvel élément D ---");
    cache.put("D".into(), "value_d".into());
    println!("get(A) = {:?}", cache.get(&"A".to_string()));

    println!("\n--- État du cache avant sauvegarde ---");
    for key in &["B", "C", "D"] {
        println!("{} = {:?}", key, cache.get(&key.to_string()));
    }

    println!("\n--- Sauvegarde du cache String/String sur disque ---");
    cache.save_as_strings("cache.txt");

    println!("\n--- Rechargement du cache depuis le fichier ---");
    let mut restored_cache: Cache<String, String> = Cache::new(3);
    restored_cache.load_from_strings("cache.txt");

    println!("État du cache restauré :");
    for key in &["B", "C", "D"] {
        println!("get({}) = {:?}", key, restored_cache.get(&key.to_string()));
    }

    // ------------------------------------------
    println!("\n--- Création d'un cache LRU i32/i32 de taille 3 ---");
    let mut cache_i32: Cache<i32, i32> = Cache::new(3);

    println!("Ajout des éléments 1->10, 2->20, 3->30, 4->40");
    cache_i32.put(1, 10);
    cache_i32.put(2, 20);
    cache_i32.put(3, 30);
    cache_i32.put(4, 40);

    println!("\n--- Sauvegarde du cache i32/i32 sur disque ---");
    cache_i32.save_as_strings("cache_i32.txt");

    println!("\n--- Rechargement du cache i32/i32 depuis le fichier ---");
    let mut restored_i32: Cache<i32, i32> = Cache::new(3);
    restored_i32.load_from_strings("cache_i32.txt");

    println!("État du cache i32 restauré :");
    for key in &[2, 3, 4] {
        println!("get({}) = {:?}", key, restored_i32.get(key));
    }
}
