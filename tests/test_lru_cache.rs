use lru_cache::traits::CacheTrait;
use lru_cache::Cache;

/// Test d'intégration pour un cache String/String et i32/i32
#[test]
fn integration_test_cache_lru() {
    // --- Test String/String ---
    let mut cache_str: Cache<String, String> = Cache::new(3);

    cache_str.put("A".into(), "value_a".into());
    cache_str.put("B".into(), "value_b".into());
    cache_str.put("C".into(), "value_c".into());
    cache_str.put("D".into(), "value_d".into()); // Cache == [B, C, D]

    assert_eq!(cache_str.get(&"A".into()), None);
    assert_eq!(cache_str.get(&"D".into()), Some("value_d".into()));
    assert_eq!(cache_str.get(&"B".into()), Some("value_b".into()));

    let path_str = "integration_cache_str.txt";
    cache_str.save_as_strings(path_str);

    let mut restored_str: Cache<String, String> = Cache::new(3);
    restored_str.load_from_strings(path_str);

    assert_eq!(restored_str.get(&"B".into()), Some("value_b".into()));
    assert_eq!(restored_str.get(&"C".into()), Some("value_c".into()));
    assert_eq!(restored_str.get(&"D".into()), Some("value_d".into()));

    // Nettoyage du fichier
    let _ = std::fs::remove_file(path_str);

    // --- Test i32/i32 ---
    let mut cache_i32: Cache<i32, i32> = Cache::new(3);

    cache_i32.put(1, 10);
    cache_i32.put(2, 20);
    cache_i32.put(3, 30);
    cache_i32.put(4, 40); // Cache == [2, 3, 4]

    assert_eq!(cache_i32.get(&1), None);
    assert_eq!(cache_i32.get(&4), Some(40));
    assert_eq!(cache_i32.get(&2), Some(20));

    let path_i32 = "integration_cache_i32.txt";
    cache_i32.save_as_strings(path_i32);

    let mut restored_i32: Cache<i32, i32> = Cache::new(3);
    restored_i32.load_from_strings(path_i32);

    assert_eq!(restored_i32.get(&2), Some(20));
    assert_eq!(restored_i32.get(&3), Some(30));
    assert_eq!(restored_i32.get(&4), Some(40));

    // Nettoyage du fichier
    let _ = std::fs::remove_file(path_i32);
}
