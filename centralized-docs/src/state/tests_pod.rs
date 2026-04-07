//! Pod type unit tests and proptests.

use super::*;
use std::mem::size_of;

#[test]
fn file_state_raw_size_is_200_bytes() {
    assert_eq!(size_of::<FileStateRaw>(), 200);
}

#[test]
fn url_state_raw_size_is_120_bytes() {
    assert_eq!(size_of::<UrlStateRaw>(), 120);
}

#[test]
fn file_state_raw_satisfies_required_traits() {
    fn assert_traits<T: Copy + Clone + std::fmt::Debug + PartialEq + Eq>() {}
    assert_traits::<FileStateRaw>();
}

#[test]
fn url_state_raw_satisfies_required_traits() {
    fn assert_traits<T: Copy + Clone + std::fmt::Debug + PartialEq + Eq>() {}
    assert_traits::<UrlStateRaw>();
}

#[test]
fn file_state_raw_zeroed_is_valid() {
    let zeroed = FileStateRaw::zeroed();
    assert_eq!(zeroed.content_hash, [0u8; 32]);
    assert_eq!(zeroed.config_hash, [0u8; 32]);
    assert_eq!(zeroed.analysis_hash, [0u8; 32]);
    assert_eq!(zeroed.transform_hash, [0u8; 32]);
    assert_eq!(zeroed.chunk_hash, [0u8; 32]);
    assert_eq!(zeroed.last_processed_secs, 0);
    assert_eq!(zeroed.reserved, [0u8; 32]);
    let bytes = zeroed.to_bytes();
    let restored = FileStateRaw::from_bytes(&bytes).unwrap();
    assert_eq!(restored, zeroed);
}

#[test]
fn url_state_raw_zeroed_is_valid() {
    let zeroed = UrlStateRaw::zeroed();
    assert_eq!(zeroed.content_hash, [0u8; 32]);
    assert_eq!(zeroed.url_hash, [0u8; 32]);
    assert_eq!(zeroed.last_fetched_secs, 0);
    assert_eq!(zeroed.status_code, 0);
    assert_eq!(zeroed.reserved, [0u8; 46]);
    let bytes = zeroed.to_bytes();
    let restored = UrlStateRaw::from_bytes(&bytes).unwrap();
    assert_eq!(restored, zeroed);
}

#[test]
fn file_state_raw_pod_roundtrip_returns_original() {
    let state = FileStateRaw {
        content_hash: [0xAA; 32],
        config_hash: [0xBB; 32],
        analysis_hash: [0xCC; 32],
        transform_hash: [0xDD; 32],
        chunk_hash: [0xEE; 32],
        last_processed_secs: 1_700_000_000,
        reserved: [0xFF; 32],
    };
    let bytes = state.to_bytes();
    assert_eq!(bytes.len(), 200);
    let restored = FileStateRaw::from_bytes(&bytes).unwrap();
    assert_eq!(restored, state);
}

#[test]
fn url_state_raw_pod_roundtrip_returns_original() {
    let state = UrlStateRaw {
        content_hash: [0x11; 32],
        url_hash: [0x22; 32],
        last_fetched_secs: 1_700_000_001,
        status_code: 200,
        reserved: [0x33; 46],
    };
    let bytes = state.to_bytes();
    assert_eq!(bytes.len(), 120);
    let restored = UrlStateRaw::from_bytes(&bytes).unwrap();
    assert_eq!(restored, state);
}

#[test]
fn file_state_wrong_value_size_returns_pod_size_mismatch() {
    let result = read_file_state_raw(&[0u8; 199]);
    assert!(matches!(
        result,
        Err(StateError::PodSizeMismatch {
            table: "file_state",
            expected: 200,
            actual: 199
        })
    ));
    let result = read_file_state_raw(&[0u8; 201]);
    assert!(matches!(
        result,
        Err(StateError::PodSizeMismatch {
            table: "file_state",
            expected: 200,
            actual: 201
        })
    ));
}

#[test]
fn url_state_wrong_value_size_returns_pod_size_mismatch() {
    let result = read_url_state_raw(&[0u8; 119]);
    assert!(matches!(
        result,
        Err(StateError::PodSizeMismatch {
            table: "url_state",
            expected: 120,
            actual: 119
        })
    ));
    let result = read_url_state_raw(&[0u8; 121]);
    assert!(matches!(
        result,
        Err(StateError::PodSizeMismatch {
            table: "url_state",
            expected: 120,
            actual: 121
        })
    ));
}

#[test]
fn file_state_raw_byte_layout_matches_offsets() {
    let state = FileStateRaw {
        content_hash: [0x0A; 32],
        config_hash: [0x0B; 32],
        analysis_hash: [0x0C; 32],
        transform_hash: [0x0D; 32],
        chunk_hash: [0x0E; 32],
        last_processed_secs: 0x1122_3344_5566_7788,
        reserved: [0xFF; 32],
    };
    let bytes = state.to_bytes();
    assert_eq!(&bytes[0..32], state.content_hash.as_slice());
    assert_eq!(&bytes[32..64], state.config_hash.as_slice());
    assert_eq!(&bytes[64..96], state.analysis_hash.as_slice());
    assert_eq!(&bytes[96..128], state.transform_hash.as_slice());
    assert_eq!(&bytes[128..160], state.chunk_hash.as_slice());
    assert_eq!(&bytes[160..168], state.last_processed_secs.to_le_bytes());
    assert_eq!(&bytes[168..200], state.reserved.as_slice());
}

#[test]
fn url_state_raw_byte_layout_matches_offsets() {
    let state = UrlStateRaw {
        content_hash: [0xAA; 32],
        url_hash: [0xBB; 32],
        last_fetched_secs: 0x8877_6655_4433_2211,
        status_code: 404,
        reserved: [0xCC; 46],
    };
    let bytes = state.to_bytes();
    assert_eq!(&bytes[0..32], state.content_hash.as_slice());
    assert_eq!(&bytes[32..64], state.url_hash.as_slice());
    assert_eq!(&bytes[64..72], state.last_fetched_secs.to_le_bytes());
    assert_eq!(&bytes[72..74], state.status_code.to_le_bytes());
    assert_eq!(&bytes[74..120], state.reserved.as_slice());
}

#[test]
fn proptest_file_state_raw_roundtrip() {
    use proptest::prelude::*;
    proptest!(|(
        content_hash in proptest::array::uniform32(0u8..=255u8),
        config_hash in proptest::array::uniform32(0u8..=255u8),
        analysis_hash in proptest::array::uniform32(0u8..=255u8),
        transform_hash in proptest::array::uniform32(0u8..=255u8),
        chunk_hash in proptest::array::uniform32(0u8..=255u8),
        last_processed_secs: u64,
        reserved in proptest::array::uniform32(0u8..=255u8),
    )| {
        let state = FileStateRaw {
            content_hash, config_hash, analysis_hash, transform_hash,
            chunk_hash, last_processed_secs, reserved,
        };
        let bytes = state.to_bytes();
        prop_assert_eq!(bytes.len(), 200);
        let restored = FileStateRaw::from_bytes(&bytes)
            .expect("round-trip should succeed for any FileStateRaw");
        prop_assert_eq!(restored, state);
    });
}

#[test]
fn proptest_url_state_raw_roundtrip() {
    use proptest::prelude::*;
    proptest!(|(
        content_hash in proptest::array::uniform32(0u8..=255u8),
        url_hash in proptest::array::uniform32(0u8..=255u8),
        last_fetched_secs: u64,
        status_code: u16,
        reserved in any::<[u8; 46]>(),
    )| {
        let state = UrlStateRaw {
            content_hash, url_hash, last_fetched_secs, status_code, reserved,
        };
        let bytes = state.to_bytes();
        prop_assert_eq!(bytes.len(), 120);
        let restored = UrlStateRaw::from_bytes(&bytes)
            .expect("round-trip should succeed for any UrlStateRaw");
        prop_assert_eq!(restored, state);
    });
}

#[test]
fn proptest_file_state_raw_byte_layout() {
    use proptest::prelude::*;
    proptest!(|(
        content_hash in proptest::array::uniform32(0u8..=255u8),
        config_hash in proptest::array::uniform32(0u8..=255u8),
        analysis_hash in proptest::array::uniform32(0u8..=255u8),
        transform_hash in proptest::array::uniform32(0u8..=255u8),
        chunk_hash in proptest::array::uniform32(0u8..=255u8),
        last_processed_secs: u64,
        reserved in proptest::array::uniform32(0u8..=255u8),
    )| {
        let state = FileStateRaw {
            content_hash, config_hash, analysis_hash, transform_hash,
            chunk_hash, last_processed_secs, reserved,
        };
        let bytes = state.to_bytes();
        prop_assert_eq!(&bytes[0..32], content_hash.as_slice());
        prop_assert_eq!(&bytes[32..64], config_hash.as_slice());
        prop_assert_eq!(&bytes[64..96], analysis_hash.as_slice());
        prop_assert_eq!(&bytes[96..128], transform_hash.as_slice());
        prop_assert_eq!(&bytes[128..160], chunk_hash.as_slice());
        prop_assert_eq!(&bytes[160..168], last_processed_secs.to_le_bytes());
        prop_assert_eq!(&bytes[168..200], reserved.as_slice());
    });
}

#[test]
fn proptest_url_state_raw_byte_layout() {
    use proptest::prelude::*;
    proptest!(|(
        content_hash in proptest::array::uniform32(0u8..=255u8),
        url_hash in proptest::array::uniform32(0u8..=255u8),
        last_fetched_secs: u64,
        status_code: u16,
        reserved in any::<[u8; 46]>(),
    )| {
        let state = UrlStateRaw {
            content_hash, url_hash, last_fetched_secs, status_code, reserved,
        };
        let bytes = state.to_bytes();
        prop_assert_eq!(&bytes[0..32], content_hash.as_slice());
        prop_assert_eq!(&bytes[32..64], url_hash.as_slice());
        prop_assert_eq!(&bytes[64..72], last_fetched_secs.to_le_bytes());
        prop_assert_eq!(&bytes[72..74], status_code.to_le_bytes());
        prop_assert_eq!(&bytes[74..120], reserved.as_slice());
    });
}
