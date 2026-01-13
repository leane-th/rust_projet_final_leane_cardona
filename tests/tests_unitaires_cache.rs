use lru_cache::traits::CacheTrait;
use lru_cache::Cache;

#[test]
fn test_lru_cache_o1() {
    let mut cache: Cache<String, String> = Cache::new(3); // Taille de 3

    cache.put("A".to_string(), String::from("value_a"));
    cache.put("B".to_string(), String::from("value_b"));
    cache.put("C".to_string(), String::from("value_c"));
    cache.put("D".to_string(), String::from("value_d")); // Cache == [B, C, D]

    assert_eq!(cache.get(&"A".to_string()), None);
    assert_eq!(cache.get(&"D".to_string()), Some(String::from("value_d"))); // Cache == [B, C, D]
    assert_eq!(cache.get(&"B".to_string()), Some(String::from("value_b"))); // Cache == [C, D, B]
    assert_eq!(cache.get(&"C".to_string()), Some(String::from("value_c"))); // Cache == [D, B, C]

    assert_eq!(cache.get(&"X".to_string()), None); // Cache == [D, B, C]

    cache.put("A".to_string(), String::from("value_a")); // Cache == [B, C, A]
    cache.put("X".to_string(), String::from("value_x")); // Cache == [C, A, X]
    assert_eq!(cache.get(&"B".to_string()), None); // Cache == [C, A, X]
    assert_eq!(cache.get(&"D".to_string()), None); // Cache == [C, A, X]

    let path = "test_cache.txt";
    cache.save_as_strings(path);

    let mut restored: Cache<String, String> = Cache::new(3);
    restored.load_from_strings(path);

    assert_eq!(restored.get(&"A".to_string()), Some("value_a".to_string()));
    assert_eq!(restored.get(&"X".to_string()), Some("value_x".to_string()));
    assert_eq!(restored.get(&"C".to_string()), Some("value_c".to_string()));

    // Nettoyage du fichier
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_lru_cache_o1_i32() {
    let mut cache: Cache<i32, i32> = Cache::new(3);

    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);
    cache.put(4, 40); // Cache == [2, 3, 4]

    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&4), Some(40)); // Cache == [2, 3, 4]
    assert_eq!(cache.get(&2), Some(20)); // Cache == [3, 4, 2]
    assert_eq!(cache.get(&3), Some(30)); // Cache == [4, 2, 3]

    cache.put(5, 50); // Cache == [2, 3, 5]
    assert_eq!(cache.get(&4), None);

    let path = "test_cache_i32.txt";
    cache.save_as_strings(path);

    let mut restored: Cache<i32, i32> = Cache::new(3);
    restored.load_from_strings(path);

    assert_eq!(restored.get(&2), Some(20));
    assert_eq!(restored.get(&3), Some(30));
    assert_eq!(restored.get(&5), Some(50));

    let _ = std::fs::remove_file(path);
}