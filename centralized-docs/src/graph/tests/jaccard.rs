use proptest::prelude::*;

use super::*;

// Property 1: Commutativity - jaccard(a, b) == jaccard(b, a)
proptest! {
    #[test]
    fn prop_jaccard_commutativity(
        tags1 in prop::collection::vec(".*", 0..20),
        tags2 in prop::collection::vec(".*", 0..20)
    ) {
        let vec1: Vec<String> = tags1.into_iter().map(|s| s.clone()).collect();
        let vec2: Vec<String> = tags2.into_iter().map(|s| s.clone()).collect();

        let result1 = jaccard_similarity(&vec1, &vec2);
        let result2 = jaccard_similarity(&vec2, &vec1);

        prop_assert_eq!(result1, result2);
    }
}

// Property 2: Reflexivity - jaccard(a, a) == 1.0
proptest! {
    #[test]
    fn prop_jaccard_reflexivity(tags in prop::collection::vec(".*", 0..20)) {
        let vec: Vec<String> = tags.into_iter().map(|s| s.clone()).collect();
        let result = jaccard_similarity(&vec, &vec);

        prop_assert_eq!(result, 1.0);
    }
}

// Property 3: Bounds - result always in [0.0, 1.0]
proptest! {
    #[test]
    fn prop_jaccard_bounds(
        tags1 in prop::collection::vec(".*", 0..20),
        tags2 in prop::collection::vec(".*", 0..20)
    ) {
        let vec1: Vec<String> = tags1.into_iter().map(|s| s.clone()).collect();
        let vec2: Vec<String> = tags2.into_iter().map(|s| s.clone()).collect();

        let result = jaccard_similarity(&vec1, &vec2);

        prop_assert!(result >= 0.0);
        prop_assert!(result <= 1.0);
    }
}

// Property 4: Empty sets - jaccard([], []) == 1.0
#[test]
fn prop_jaccard_both_empty() {
    let empty: Vec<String> = vec![];
    let result = jaccard_similarity(&empty, &empty);

    assert_eq!(result, 1.0);
}

// Property 5: Disjoint sets - jaccard(a, b) == 0.0 when no shared elements
proptest! {
    #[test]
    fn prop_jaccard_disjoint_sets(
        prefix1 in "[a-m]{1,5}",
        prefix2 in "[n-z]{1,5}",
        count in 1..10usize
    ) {
        let set1: Vec<String> = (0..count)
            .map(|i| format!("{prefix1}{i}"))
            .collect();
        let set2: Vec<String> = (0..count)
            .map(|i| format!("{prefix2}{i}"))
            .collect();

        let result = jaccard_similarity(&set1, &set2);

        prop_assert_eq!(result, 0.0);
    }
}

#[test]
fn test_jaccard_similarity() {
    let tags1 = vec!["rust".to_string(), "cue".to_string()];
    let tags2 = vec!["rust".to_string(), "tour".to_string()];

    let similarity = jaccard_similarity(&tags1, &tags2);
    // Intersection: ["rust"] = 1
    // Union: ["rust", "cue", "tour"] = 3
    // Jaccard = 1/3 ≈ 0.333
    assert!((similarity - 0.333).abs() < 0.01);
}
