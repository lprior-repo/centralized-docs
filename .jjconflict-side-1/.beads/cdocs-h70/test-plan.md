# Test Plan: Fixed-Size `FileStateRaw` and `UrlStateRaw` Pod Types

## Summary

- **Behaviors identified**: 44
- **Trophy allocation**: 33 unit / 10 integration / 2 static
- **Proptest invariants**: 8
- **Fuzz targets**: 4
- **Kani harnesses**: 5
- **Mutation kill target**: ≥90%

---

## 1. Behavior Inventory

### Size & Layout (INV-01, INV-02, POST-03)

| # | Behavior |
|---|----------|
| B01 | `FileStateRaw` has size 104 bytes when compiled with `#[repr(C)]` |
| B02 | `FileStateRaw` has alignment 8 when compiled with `#[repr(C)]` |
| B03 | `UrlStateRaw` has size 112 bytes when compiled with `#[repr(C)]` |
| B04 | `UrlStateRaw` has alignment 8 when compiled with `#[repr(C)]` |
| B05 | `FileStateRaw` has zero compiler-inserted padding (size equals sum of field sizes) |
| B06 | `UrlStateRaw` has zero compiler-inserted padding (size equals sum of field sizes) |

### FileStateRaw Construction (POST-01, POST-02, POST-06)

| # | Behavior |
|---|----------|
| B07 | `FileStateRaw::new()` returns instance with correct field values when given valid inputs |
| B08 | `FileStateRaw::new()` sets `version == 1` unconditionally |
| B09 | `FileStateRaw::new()` sets `reserved == [0u8; 54]` unconditionally |
| B10 | `FileStateRaw::zeroed()` returns all-zero instance (valid per `bytemuck::Zeroable`) |
| B11 | `FileStateRaw` implements `bytemuck::Pod` (compile-time proof) |
| B12 | `FileStateRaw` implements `bytemuck::Zeroable` (compile-time proof) |

### UrlStateRaw Construction (POST-01, POST-02, POST-06)

| # | Behavior |
|---|----------|
| B13 | `UrlStateRaw::new()` returns instance with correct field values when given valid inputs |
| B14 | `UrlStateRaw::new()` sets `version == 1` unconditionally |
| B15 | `UrlStateRaw::new()` sets `_pad1 == [0u8; 2]` and `reserved == [0u8; 58]` unconditionally |
| B16 | `UrlStateRaw::zeroed()` returns all-zero instance (valid per `bytemuck::Zeroable`) |
| B17 | `UrlStateRaw` implements `bytemuck::Pod` (compile-time proof) |
| B18 | `UrlStateRaw` implements `bytemuck::Zeroable` (compile-time proof) |

### Byte Round-Trip (POST-04, POST-05)

| # | Behavior |
|---|----------|
| B19 | `FileStateRaw` round-trips losslessly through `as_bytes()` → `from_bytes_checked()` |
| B20 | `UrlStateRaw` round-trips losslessly through `as_bytes()` → `from_bytes_checked()` |
| B21 | `FileStateRaw::as_bytes()` returns slice of exactly 104 bytes |
| B22 | `UrlStateRaw::as_bytes()` returns slice of exactly 112 bytes |
| B23 | Two equal `FileStateRaw` instances have identical byte representations |
| B24 | Two equal `UrlStateRaw` instances have identical byte representations |

### from_bytes_unchecked — Zero-Copy Cast (POST-04)

| # | Behavior |
|---|----------|
| B25 | `FileStateRaw::from_bytes_unchecked()` returns reference to same bytes when given valid slice |
| B26 | `UrlStateRaw::from_bytes_unchecked()` returns reference to same bytes when given valid slice |

### from_bytes_checked Validation (P-04, P-05, P-06)

| # | Behavior |
|---|----------|
| B27 | `FileStateRaw::from_bytes_checked()` rejects slice of wrong size → `WrongByteSize` |
| B28 | `FileStateRaw::from_bytes_checked()` rejects invalid status byte → `InvalidFileStatus` |
| B29 | `FileStateRaw::from_bytes_checked()` rejects version != 1 → `VersionMismatch` |
| B30 | `FileStateRaw::from_bytes_checked()` rejects non-zero reserved bytes → `ReservedBytesNonZero` |
| B31 | `UrlStateRaw::from_bytes_checked()` rejects slice of wrong size → `WrongByteSize` |
| B32 | `UrlStateRaw::from_bytes_checked()` rejects invalid status byte → `InvalidUrlStatus` |
| B33 | `UrlStateRaw::from_bytes_checked()` rejects version != 1 → `VersionMismatch` |
| B34 | `UrlStateRaw::from_bytes_checked()` rejects non-zero reserved/pad bytes → `ReservedBytesNonZero` |

### Status Enum Discriminants

| # | Behavior |
|---|----------|
| B35 | `FileStateStatus::from_discriminant()` returns correct variant for values 0–3 |
| B36 | `FileStateStatus::from_discriminant()` rejects values > 3 → `InvalidFileStatus` |
| B37 | `UrlStateStatus::from_discriminant()` returns correct variant for values 0–3 |
| B38 | `UrlStateStatus::from_discriminant()` rejects values > 3 → `InvalidUrlStatus` |

### Validate Method

| # | Behavior |
|---|----------|
| B39 | `FileStateRaw::validate()` returns `Ok(())` for well-constructed instances |
| B40 | `FileStateRaw::validate()` rejects corrupted status, wrong version, non-zero reserved |
| B41 | `UrlStateRaw::validate()` returns `Ok(())` for well-constructed instances |
| B42 | `UrlStateRaw::validate()` rejects corrupted status, wrong version, non-zero pad1, non-zero reserved |

### Error Display Messages

| # | Behavior |
|---|----------|
| B43 | Each `PodStateError` variant displays correct message with embedded values |

---

## 2. Trophy Allocation

| Layer | Count | Behaviors | Rationale |
|-------|-------|-----------|-----------|
| **Static** | 2 | B11, B12, B17, B18 | Pod/Zeroable trait satisfaction is proven at compile time by `bytemuck`'s derive macros. No runtime assertion needed — if it compiles, the invariant holds. |
| **Unit** | 33 | B01–B10, B13–B16, B21–B24, B25–B26, B35–B43 | All are pure Calc-layer behaviors: no I/O, no external dependencies, deterministic outputs for given inputs. Struct construction, byte casting, enum conversion, validation — all pure functions. `from_bytes_unchecked()` is a zero-copy Pod cast with deterministic output for a given input. |
| **Integration** | 10 | B19–B20, B27–B34 | These validate the boundary between raw bytes and Pod types — the actual `bytemuck` crate integration. They verify that `bytemuck::bytes_of`, `bytemuck::from_bytes`, and `bytemuck::pod_read_unaligned` behave correctly with our types. Labeled integration because they exercise the `bytemuck` dependency contract. |
| **E2E** | 0 | — | No user-facing workflow in this bead. These types are internal data structures. E2E tests belong in the redb integration bead. |

**Deviation note**: This bead is 100% data-layer types with zero I/O, so the integration/unit split differs from the typical 60/30 ratio. Integration here means "crossing the bytemuck API boundary," which is the most critical test surface.

---

## 3. BDD Scenarios

### 3.1 Size & Layout

#### Behavior: FileStateRaw size is exactly 104 bytes

```
Given: FileStateRaw is compiled with #[repr(C)]
When:  std::mem::size_of::<FileStateRaw>() is queried
Then:  returns 104
```

**Test name**: `fn file_state_raw_has_size_104_bytes()`

---

#### Behavior: FileStateRaw alignment is 8

```
Given: FileStateRaw contains u64 fields
When:  std::mem::align_of::<FileStateRaw>() is queried
Then:  returns 8
```

**Test name**: `fn file_state_raw_has_alignment_8()`

---

#### Behavior: UrlStateRaw size is exactly 112 bytes

```
Given: UrlStateRaw is compiled with #[repr(C)]
When:  std::mem::size_of::<UrlStateRaw>() is queried
Then:  returns 112
```

**Test name**: `fn url_state_raw_has_size_112_bytes()`

---

#### Behavior: UrlStateRaw alignment is 8

```
Given: UrlStateRaw contains u64 fields
When:  std::mem::align_of::<UrlStateRaw>() is queried
Then:  returns 8
```

**Test name**: `fn url_state_raw_has_alignment_8()`

---

#### Behavior: FileStateRaw has no compiler-inserted padding

```
Given: FileStateRaw layout is #[repr(C)]
When:  size_of::<FileStateRaw>() is compared to sum of field sizes (32+8+8+1+1+54)
Then:  they are equal (104 == 104)
```

**Test name**: `fn file_state_raw_has_no_compiler_inserted_padding()`

---

#### Behavior: UrlStateRaw has no compiler-inserted padding

```
Given: UrlStateRaw layout is #[repr(C)]
When:  size_of::<UrlStateRaw>() is compared to sum of field sizes (32+2+2+8+8+1+1+58)
Then:  they are equal (112 == 112)
```

**Test name**: `fn url_state_raw_has_no_compiler_inserted_padding()`

---

### 3.2 FileStateRaw Construction

#### Behavior: FileStateRaw::new() returns correct instance

```
Given: valid content_hash [0xAB; 32], file_size 4096, last_modified_ms 1700000000000,
       status FileStateStatus::Modified
When:  FileStateRaw::new(hash, 4096, 1700000000000, FileStateStatus::Modified)
Then:  instance.content_hash == [0xAB; 32]
  And: instance.file_size == 4096
  And: instance.last_modified_ms == 1700000000000
  And: instance.status == FileStateStatus::Modified as u8
  And: instance.version == 1
  And: instance.reserved == [0u8; 54]
```

**Test name**: `fn file_state_raw_new_returns_correct_field_values()`

---

#### Behavior: FileStateRaw::new() sets version to 1

```
Given: content_hash=[0;32], file_size=4096, last_modified_ms=1700000000000,
       status=FileStateStatus::Modified
When:  FileStateRaw::new([0;32], 4096, 1700000000000, FileStateStatus::Modified)
Then:  returned instance.version == 1
```

**Test name**: `fn file_state_raw_new_sets_version_to_1()`

---

#### Behavior: FileStateRaw::new() zeroes reserved bytes

```
Given: content_hash=[0xFF;32], file_size=u64::MAX, last_modified_ms=u64::MAX,
       status=FileStateStatus::Deleted
When:  FileStateRaw::new([0xFF;32], u64::MAX, u64::MAX, FileStateStatus::Deleted)
Then:  returned instance.reserved == [0u8; 54]
```

**Test name**: `fn file_state_raw_new_zeroes_reserved_bytes()`

---

#### Behavior: FileStateRaw::zeroed() returns all-zero instance

```
Given: FileStateRaw implements Zeroable
When:  FileStateRaw::zeroed() is called
Then:  every byte of the returned instance is 0
  And: as_bytes() returns [0u8; 104]
```

**Test name**: `fn file_state_raw_zeroed_returns_all_zeros()`

---

#### Behavior: FileStateRaw implements Pod (compile-time)

```
Given: FileStateRaw derives Pod + Zeroable
When:  code references bytemuck::bytes_of(&file_state_raw)
Then:  it compiles without error (static proof)
```

**Test name**: `fn file_state_raw_compiles_with_bytemuck_bytes_of()` (compile-fail / static)

---

#### Behavior: FileStateRaw implements Zeroable (compile-time)

```
Given: FileStateRaw derives Zeroable
When:  code references bytemuck::Zeroable::zeroed() as FileStateRaw
Then:  it compiles without error (static proof)
```

**Test name**: `fn file_state_raw_compiles_with_bytemuck_zeroed()` (compile-fail / static)

---

### 3.3 UrlStateRaw Construction

#### Behavior: UrlStateRaw::new() returns correct instance

```
Given: valid content_hash [0xCD; 32], http_status 200, content_length 8192,
       last_fetched_ms 1700000000000, status UrlStateStatus::Fresh
When:  UrlStateRaw::new(hash, 200, 8192, 1700000000000, UrlStateStatus::Fresh)
Then:  instance.content_hash == [0xCD; 32]
  And: instance.http_status == 200
  And: instance.content_length == 8192
  And: instance.last_fetched_ms == 1700000000000
  And: instance.status == UrlStateStatus::Fresh as u8
  And: instance.version == 1
  And: instance._pad1 == [0u8; 2]
  And: instance.reserved == [0u8; 58]
```

**Test name**: `fn url_state_raw_new_returns_correct_field_values()`

---

#### Behavior: UrlStateRaw::new() sets version to 1

```
Given: content_hash=[0;32], http_status=200, content_length=8192,
       last_fetched_ms=1700000000000, status=UrlStateStatus::Fresh
When:  UrlStateRaw::new([0;32], 200, 8192, 1700000000000, UrlStateStatus::Fresh)
Then:  returned instance.version == 1
```

**Test name**: `fn url_state_raw_new_sets_version_to_1()`

---

#### Behavior: UrlStateRaw::new() zeroes pad and reserved bytes

```
Given: content_hash=[0xFF;32], http_status=u16::MAX, content_length=u64::MAX,
       last_fetched_ms=u64::MAX, status=UrlStateStatus::Error
When:  UrlStateRaw::new([0xFF;32], u16::MAX, u64::MAX, u64::MAX, UrlStateStatus::Error)
Then:  returned instance._pad1 == [0u8; 2]
  And: returned instance.reserved == [0u8; 58]
```

**Test name**: `fn url_state_raw_new_zeroes_pad_and_reserved_bytes()`

---

#### Behavior: UrlStateRaw::new() stores content_length=u64::MAX without truncation

```
Given: content_hash=[0;32], http_status=0, content_length=u64::MAX,
       last_fetched_ms=0, status=UrlStateStatus::Unknown
When:  UrlStateRaw::new([0;32], 0, u64::MAX, 0, UrlStateStatus::Unknown)
Then:  instance.content_length == u64::MAX
```

**Test name**: `fn url_state_raw_new_stores_max_content_length()`

---

#### Behavior: UrlStateRaw::new() stores last_fetched_ms=u64::MAX without truncation

```
Given: content_hash=[0;32], http_status=0, content_length=0,
       last_fetched_ms=u64::MAX, status=UrlStateStatus::Unknown
When:  UrlStateRaw::new([0;32], 0, 0, u64::MAX, UrlStateStatus::Unknown)
Then:  instance.last_fetched_ms == u64::MAX
```

**Test name**: `fn url_state_raw_new_stores_max_last_fetched_ms()`

---

#### Behavior: UrlStateRaw::zeroed() returns all-zero instance

```
Given: UrlStateRaw implements Zeroable
When:  UrlStateRaw::zeroed() is called
Then:  every byte of the returned instance is 0
  And: as_bytes() returns [0u8; 112]
```

**Test name**: `fn url_state_raw_zeroed_returns_all_zeros()`

---

#### Behavior: UrlStateRaw implements Pod (compile-time)

```
Given: UrlStateRaw derives Pod + Zeroable
When:  code references bytemuck::bytes_of(&url_state_raw)
Then:  it compiles without error (static proof)
```

**Test name**: `fn url_state_raw_compiles_with_bytemuck_bytes_of()` (compile-fail / static)

---

#### Behavior: UrlStateRaw implements Zeroable (compile-time)

```
Given: UrlStateRaw derives Zeroable
When:  code references bytemuck::Zeroable::zeroed() as UrlStateRaw
Then:  it compiles without error (static proof)
```

**Test name**: `fn url_state_raw_compiles_with_bytemuck_zeroed()` (compile-fail / static)

---

### 3.4 Byte Round-Trip

#### Behavior: FileStateRaw round-trips through bytes losslessly

```
Given: a FileStateRaw with content_hash [1;32], file_size 999, last_modified_ms 12345,
       status FileStateStatus::Unchanged
When:  let bytes = raw.as_bytes();
       let restored = FileStateRaw::from_bytes_checked(bytes).unwrap();
Then:  restored == raw (field-by-field equality)
  And: restored.content_hash == [1;32]
  And: restored.file_size == 999
  And: restored.last_modified_ms == 12345
  And: restored.status == FileStateStatus::Unchanged as u8
  And: restored.version == 1
```

**Test name**: `fn file_state_raw_round_trips_through_bytes_losslessly()`

---

#### Behavior: UrlStateRaw round-trips through bytes losslessly

```
Given: a UrlStateRaw with content_hash [2;32], http_status 404, content_length 0,
       last_fetched_ms 99999, status UrlStateStatus::Stale
When:  let bytes = raw.as_bytes();
       let restored = UrlStateRaw::from_bytes_checked(bytes).unwrap();
Then:  restored == raw (field-by-field equality)
  And: restored.http_status == 404
  And: restored.content_length == 0
  And: restored.last_fetched_ms == 99999
  And: restored.status == UrlStateStatus::Stale as u8
```

**Test name**: `fn url_state_raw_round_trips_through_bytes_losslessly()`

---

#### Behavior: FileStateRaw::as_bytes() returns 104-byte slice

```
Given: a FileStateRaw constructed via new([0xAB;32], 4096, 1700000000000, Modified)
When:  instance.as_bytes()
Then:  slice.len() == 104
```

**Test name**: `fn file_state_raw_as_bytes_returns_104_byte_slice()`

---

#### Behavior: UrlStateRaw::as_bytes() returns 112-byte slice

```
Given: a UrlStateRaw constructed via new([0xCD;32], 200, 8192, 1700000000000, Fresh)
When:  instance.as_bytes()
Then:  slice.len() == 112
```

**Test name**: `fn url_state_raw_as_bytes_returns_112_byte_slice()`

---

#### Behavior: Equal FileStateRaw instances have identical byte representations

```
Given: two FileStateRaw instances with identical field values constructed independently
When:  both are converted via as_bytes()
Then:  the two byte slices are equal (byte-for-byte identical)
```

**Test name**: `fn file_state_raw_equal_instances_have_identical_bytes()`

---

#### Behavior: Equal UrlStateRaw instances have identical byte representations

```
Given: two UrlStateRaw instances with identical field values constructed independently
When:  both are converted via as_bytes()
Then:  the two byte slices are equal (byte-for-byte identical)
```

**Test name**: `fn url_state_raw_equal_instances_have_identical_bytes()`

---

### 3.5 from_bytes_unchecked — Zero-Copy Pod Cast

#### Behavior: FileStateRaw::from_bytes_unchecked() returns reference matching original instance

```
Given: a FileStateRaw constructed via new([0xAB;32], 4096, 1700000000000, Modified)
       and its byte representation via as_bytes()
When:  let ptr = FileStateRaw::from_bytes_unchecked(raw.as_bytes());
Then:  ptr.content_hash == [0xAB; 32]
  And: ptr.file_size == 4096
  And: ptr.last_modified_ms == 1700000000000
  And: ptr.version == 1
  And: ptr.status == FileStateStatus::Modified as u8
  And: ptr.reserved == [0u8; 54]
```

**Test name**: `fn file_state_raw_from_bytes_unchecked_returns_reference_to_same_bytes()`

---

#### Behavior: UrlStateRaw::from_bytes_unchecked() returns reference matching original instance

```
Given: a UrlStateRaw constructed via new([0xCD;32], 200, 8192, 1700000000000, Fresh)
       and its byte representation via as_bytes()
When:  let ptr = UrlStateRaw::from_bytes_unchecked(raw.as_bytes());
Then:  ptr.content_hash == [0xCD; 32]
  And: ptr.http_status == 200
  And: ptr.content_length == 8192
  And: ptr.last_fetched_ms == 1700000000000
  And: ptr.version == 1
  And: ptr.status == UrlStateStatus::Fresh as u8
  And: ptr._pad1 == [0u8; 2]
  And: ptr.reserved == [0u8; 58]
```

**Test name**: `fn url_state_raw_from_bytes_unchecked_returns_reference_to_same_bytes()`

---

#### Behavior: FileStateRaw::from_bytes_unchecked() preserves pointer identity

```
Given: a FileStateRaw constructed via new([1;32], 0, 0, Unknown)
       and its byte slice obtained via as_bytes()
When:  let ptr = FileStateRaw::from_bytes_unchecked(bytes);
Then:  ptr as *const FileStateRaw == bytes.as_ptr() as *const FileStateRaw
  And: field values match the original instance
```

**Test name**: `fn file_state_raw_from_bytes_unchecked_preserves_pointer_identity()`

---

#### Behavior: UrlStateRaw::from_bytes_unchecked() preserves pointer identity

```
Given: a UrlStateRaw constructed via new([1;32], 0, 0, 0, Unknown)
       and its byte slice obtained via as_bytes()
When:  let ptr = UrlStateRaw::from_bytes_unchecked(bytes);
Then:  ptr as *const UrlStateRaw == bytes.as_ptr() as *const UrlStateRaw
  And: field values match the original instance
```

**Test name**: `fn url_state_raw_from_bytes_unchecked_preserves_pointer_identity()`

---

### 3.6 from_bytes_checked — Error Paths

#### Behavior: FileStateRaw::from_bytes_checked() rejects wrong-size slice

```
Given: a byte slice of length 50 (not 104)
When:  FileStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::WrongByteSize { type_name: "FileStateRaw", actual: 50, expected: 104 })
```

**Test name**: `fn file_state_raw_from_bytes_checked_rejects_wrong_size()`

**Additional boundaries**:
- Empty slice → `WrongByteSize { actual: 0, expected: 104 }`
- 103 bytes (one short) → `WrongByteSize { actual: 103, expected: 104 }`
- 105 bytes (one over) → `WrongByteSize { actual: 105, expected: 104 }`

**Test names**:
- `fn file_state_raw_from_bytes_checked_rejects_empty_slice()`
- `fn file_state_raw_from_bytes_checked_rejects_one_byte_short()`
- `fn file_state_raw_from_bytes_checked_rejects_one_byte_over()`

---

#### Behavior: FileStateRaw::from_bytes_checked() rejects invalid status byte

```
Given: a 104-byte slice with status byte at offset 49 set to 4 (invalid)
When:  FileStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::InvalidFileStatus(4))
```

**Test name**: `fn file_state_raw_from_bytes_checked_rejects_invalid_status_byte_4()`

**Additional boundaries**:
- Status byte = 255 → `Err(PodStateError::InvalidFileStatus(255))`
- Status byte = 128 → `Err(PodStateError::InvalidFileStatus(128))`

**Test names**:
- `fn file_state_raw_from_bytes_checked_rejects_invalid_status_byte_255()`
- `fn file_state_raw_from_bytes_checked_rejects_invalid_status_byte_128()`

---

#### Behavior: FileStateRaw::from_bytes_checked() rejects version != 1

```
Given: a 104-byte slice with version byte at offset 48 set to 2
When:  FileStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::VersionMismatch { type_name: "FileStateRaw", actual: 2, expected: 1 })
```

**Test name**: `fn file_state_raw_from_bytes_checked_rejects_version_2()`

**Additional**:
- Version = 0 → `Err(PodStateError::VersionMismatch { actual: 0, expected: 1 })`
- Version = 255 → `Err(PodStateError::VersionMismatch { actual: 255, expected: 1 })`

**Test names**:
- `fn file_state_raw_from_bytes_checked_rejects_version_0()`
- `fn file_state_raw_from_bytes_checked_rejects_version_255()`

---

#### Behavior: FileStateRaw::from_bytes_checked() rejects non-zero reserved bytes

```
Given: a 104-byte slice that is otherwise valid but has a non-zero byte in reserved
       region (offset 50..104), e.g., reserved[0] = 0x01
When:  FileStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::ReservedBytesNonZero { type_name: "FileStateRaw", offset: 50 })
```

**Test name**: `fn file_state_raw_from_bytes_checked_rejects_nonzero_reserved_bytes()`

**Additional**:
- Last reserved byte non-zero → `Err(PodStateError::ReservedBytesNonZero { type_name: "FileStateRaw", offset: 103 })`
- All reserved bytes = 0xFF → error on first non-zero at offset 50

**Test names**:
- `fn file_state_raw_from_bytes_checked_rejects_nonzero_last_reserved_byte()`
- `fn file_state_raw_from_bytes_checked_rejects_all_ff_reserved_bytes()`

---

#### Behavior: UrlStateRaw::from_bytes_checked() rejects wrong-size slice

```
Given: a byte slice of length 50 (not 112)
When:  UrlStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::WrongByteSize { type_name: "UrlStateRaw", actual: 50, expected: 112 })
```

**Test name**: `fn url_state_raw_from_bytes_checked_rejects_wrong_size()`

**Additional boundaries**:
- Empty slice → `WrongByteSize { actual: 0, expected: 112 }`
- 111 bytes → `WrongByteSize { actual: 111, expected: 112 }`
- 113 bytes → `WrongByteSize { actual: 113, expected: 112 }`

**Test names**:
- `fn url_state_raw_from_bytes_checked_rejects_empty_slice()`
- `fn url_state_raw_from_bytes_checked_rejects_one_byte_short()`
- `fn url_state_raw_from_bytes_checked_rejects_one_byte_over()`

---

#### Behavior: UrlStateRaw::from_bytes_checked() rejects invalid status byte

```
Given: a 112-byte slice with status byte at offset 53 set to 5 (invalid)
When:  UrlStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::InvalidUrlStatus(5))
```

**Test name**: `fn url_state_raw_from_bytes_checked_rejects_invalid_status_byte_5()`

**Additional**:
- Status byte = 255 → `Err(PodStateError::InvalidUrlStatus(255))`
- Status byte = 200 → `Err(PodStateError::InvalidUrlStatus(200))`

**Test names**:
- `fn url_state_raw_from_bytes_checked_rejects_invalid_status_byte_255()`
- `fn url_state_raw_from_bytes_checked_rejects_invalid_status_byte_200()`

---

#### Behavior: UrlStateRaw::from_bytes_checked() rejects version != 1

```
Given: a 112-byte slice with version byte at offset 52 set to 99
When:  UrlStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::VersionMismatch { type_name: "UrlStateRaw", actual: 99, expected: 1 })
```

**Test name**: `fn url_state_raw_from_bytes_checked_rejects_version_99()`

**Additional**:
- Version = 0 → `Err(PodStateError::VersionMismatch { actual: 0, expected: 1 })`

**Test names**:
- `fn url_state_raw_from_bytes_checked_rejects_version_0()`

---

#### Behavior: UrlStateRaw::from_bytes_checked() rejects non-zero _pad1 bytes

```
Given: a 112-byte slice that is otherwise valid but _pad1 at offset 34..36 is [0xFF, 0xFF]
When:  UrlStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::ReservedBytesNonZero { type_name: "UrlStateRaw", offset: 34 })
```

**Test name**: `fn url_state_raw_from_bytes_checked_rejects_nonzero_pad1_bytes()`

---

#### Behavior: UrlStateRaw::from_bytes_checked() rejects non-zero reserved bytes

```
Given: a 112-byte slice that is otherwise valid but reserved region (offset 54..112)
       has a non-zero byte, e.g., reserved[0] = 0x42
When:  UrlStateRaw::from_bytes_checked(slice)
Then:  Err(PodStateError::ReservedBytesNonZero { type_name: "UrlStateRaw", offset: 54 })
```

**Test name**: `fn url_state_raw_from_bytes_checked_rejects_nonzero_reserved_bytes()`

---

### 3.7 Status Enum Discriminants

#### Behavior: FileStateStatus::from_discriminant() returns correct variants for 0–3

```
Given: discriminant values 0, 1, 2, 3
When:  FileStateStatus::from_discriminant(value) for each
Then:  0 → Ok(FileStateStatus::Unknown)
       1 → Ok(FileStateStatus::Unchanged)
       2 → Ok(FileStateStatus::Modified)
       3 → Ok(FileStateStatus::Deleted)
```

**Test names**:
- `fn file_state_status_from_discriminant_0_returns_unknown()`
- `fn file_state_status_from_discriminant_1_returns_unchanged()`
- `fn file_state_status_from_discriminant_2_returns_modified()`
- `fn file_state_status_from_discriminant_3_returns_deleted()`

---

#### Behavior: FileStateStatus::from_discriminant() rejects values > 3

```
Given: discriminant value 4
When:  FileStateStatus::from_discriminant(4)
Then:  Err(PodStateError::InvalidFileStatus(4))
```

**Additional**:
- Value 255 → `Err(PodStateError::InvalidFileStatus(255))`
- Value u8::MAX → same error with that value

**Test names**:
- `fn file_state_status_from_discriminant_4_returns_invalid_file_status()`
- `fn file_state_status_from_discriminant_255_returns_invalid_file_status()`

---

#### Behavior: UrlStateStatus::from_discriminant() returns correct variants for 0–3

```
Given: discriminant values 0, 1, 2, 3
When:  UrlStateStatus::from_discriminant(value) for each
Then:  0 → Ok(UrlStateStatus::Unknown)
       1 → Ok(UrlStateStatus::Fresh)
       2 → Ok(UrlStateStatus::Stale)
       3 → Ok(UrlStateStatus::Error)
```

**Test names**:
- `fn url_state_status_from_discriminant_0_returns_unknown()`
- `fn url_state_status_from_discriminant_1_returns_fresh()`
- `fn url_state_status_from_discriminant_2_returns_stale()`
- `fn url_state_status_from_discriminant_3_returns_error()`

---

#### Behavior: UrlStateStatus::from_discriminant() rejects values > 3

```
Given: discriminant value 4
When:  UrlStateStatus::from_discriminant(4)
Then:  Err(PodStateError::InvalidUrlStatus(4))
```

**Additional**:
- Value 255 → `Err(PodStateError::InvalidUrlStatus(255))`

**Test names**:
- `fn url_state_status_from_discriminant_4_returns_invalid_url_status()`
- `fn url_state_status_from_discriminant_255_returns_invalid_url_status()`

---

#### Behavior: FileStateStatus discriminants match their repr(u8) values

```
Given: the FileStateStatus enum declared #[repr(u8)]
When:  each variant is cast to u8
Then:  FileStateStatus::Unknown as u8 == 0
  And: FileStateStatus::Unchanged as u8 == 1
  And: FileStateStatus::Modified as u8 == 2
  And: FileStateStatus::Deleted as u8 == 3
```

**Test name**: `fn file_state_status_discriminants_match_repr_u8_values()`

---

#### Behavior: UrlStateStatus discriminants match their repr(u8) values

```
Given: the UrlStateStatus enum declared #[repr(u8)]
When:  each variant is cast to u8
Then:  UrlStateStatus::Unknown as u8 == 0
  And: UrlStateStatus::Fresh as u8 == 1
  And: UrlStateStatus::Stale as u8 == 2
  And: UrlStateStatus::Error as u8 == 3
```

**Test name**: `fn url_state_status_discriminants_match_repr_u8_values()`

---

### 3.8 Validate Method

#### Behavior: FileStateRaw::validate() accepts well-constructed instance

```
Given: FileStateRaw::new([0;32], 0, 0, FileStateStatus::Unknown)
When:  instance.validate()
Then:  Ok(())
```

**Test name**: `fn file_state_raw_validate_accepts_well_constructed_instance()`

---

#### Behavior: FileStateRaw::validate() rejects corrupted status

```
Given: a FileStateRaw manually constructed with status byte == 99
When:  instance.validate()
Then:  Err(PodStateError::InvalidFileStatus(99))
```

**Test name**: `fn file_state_raw_validate_rejects_corrupted_status()`

---

#### Behavior: FileStateRaw::validate() rejects wrong version

```
Given: a FileStateRaw manually constructed with version == 0
When:  instance.validate()
Then:  Err(PodStateError::VersionMismatch { type_name: "FileStateRaw", actual: 0, expected: 1 })
```

**Test name**: `fn file_state_raw_validate_rejects_wrong_version()`

---

#### Behavior: FileStateRaw::validate() rejects non-zero reserved

```
Given: a FileStateRaw manually constructed with reserved[0] == 1
When:  instance.validate()
Then:  Err(PodStateError::ReservedBytesNonZero { type_name: "FileStateRaw", offset: 50 })
```

**Test name**: `fn file_state_raw_validate_rejects_nonzero_reserved()`

---

#### Behavior: UrlStateRaw::validate() accepts well-constructed instance

```
Given: UrlStateRaw::new([0;32], 0, 0, 0, UrlStateStatus::Unknown)
When:  instance.validate()
Then:  Ok(())
```

**Test name**: `fn url_state_raw_validate_accepts_well_constructed_instance()`

---

#### Behavior: UrlStateRaw::validate() rejects corrupted status

```
Given: a UrlStateRaw manually constructed with status byte == 99
When:  instance.validate()
Then:  Err(PodStateError::InvalidUrlStatus(99))
```

**Test name**: `fn url_state_raw_validate_rejects_corrupted_status()`

---

#### Behavior: UrlStateRaw::validate() rejects wrong version

```
Given: a UrlStateRaw manually constructed with version == 0
When:  instance.validate()
Then:  Err(PodStateError::VersionMismatch { type_name: "UrlStateRaw", actual: 0, expected: 1 })
```

**Test name**: `fn url_state_raw_validate_rejects_wrong_version()`

---

#### Behavior: UrlStateRaw::validate() rejects non-zero _pad1

```
Given: a UrlStateRaw manually constructed with _pad1 == [0xFF, 0xFF]
When:  instance.validate()
Then:  Err(PodStateError::ReservedBytesNonZero { type_name: "UrlStateRaw", offset: 34 })
```

**Test name**: `fn url_state_raw_validate_rejects_nonzero_pad1()`

---

#### Behavior: UrlStateRaw::validate() rejects non-zero reserved

```
Given: a UrlStateRaw manually constructed with reserved[0] == 1 (all other fields valid)
When:  instance.validate()
Then:  Err(PodStateError::ReservedBytesNonZero { type_name: "UrlStateRaw", offset: 54 })
```

**Test name**: `fn url_state_raw_validate_rejects_nonzero_reserved()`

---

### 3.9 status() Accessor Method

#### Behavior: FileStateRaw::status() returns typed enum for valid discriminant

```
Given: a FileStateRaw with status byte == 2
When:  instance.status()
Then:  Ok(FileStateStatus::Modified)
```

**Test name**: `fn file_state_raw_status_returns_modified_for_discriminant_2()`

---

#### Behavior: FileStateRaw::status() returns error for invalid discriminant

```
Given: a FileStateRaw with status byte == 200 (set via direct field access)
When:  instance.status()
Then:  Err(PodStateError::InvalidFileStatus(200))
```

**Test name**: `fn file_state_raw_status_returns_error_for_invalid_discriminant()`

---

#### Behavior: UrlStateRaw::status() returns typed enum for valid discriminant

```
Given: a UrlStateRaw with status byte == 3
When:  instance.status()
Then:  Ok(UrlStateStatus::Error)
```

**Test name**: `fn url_state_raw_status_returns_error_variant_for_discriminant_3()`

---

#### Behavior: UrlStateRaw::status() returns error for invalid discriminant

```
Given: a UrlStateRaw with status byte == 200 (set via direct field access)
When:  instance.status()
Then:  Err(PodStateError::InvalidUrlStatus(200))
```

**Test name**: `fn url_state_raw_status_returns_error_for_invalid_discriminant()`

---

### 3.10 All-Zero and All-FF Edge Cases

**Validation order convention**: `from_bytes_checked()` and `validate()` check invariants in this order: **size → version → status → reserved bytes**. All-FF tests pin to the first error encountered under this ordering.

#### Behavior: All-zero 104-byte slice rejected by FileStateRaw::from_bytes_checked()

```
Given: [0u8; 104]
When:  FileStateRaw::from_bytes_checked(&zeros)
Then:  Err(PodStateError::VersionMismatch { type_name: "FileStateRaw", actual: 0, expected: 1 })
```

Note: all-zero fails version check (version=0). This is intentional — `zeroed()` is for Pod trait compliance, not for creating valid domain instances.

**Test name**: `fn file_state_raw_all_zeros_rejected_due_to_version_0()`

---

#### Behavior: All-FF 104-byte slice rejected by FileStateRaw::from_bytes_checked()

```
Given: [0xFFu8; 104]
When:  FileStateRaw::from_bytes_checked(&all_ff)
Then:  Err(PodStateError::VersionMismatch { type_name: "FileStateRaw", actual: 255, expected: 1 })
```

Note: version is checked before status per the validation order convention (size → version → status → reserved). Version byte at offset 48 is 0xFF = 255, which fails before the status check runs.

**Test name**: `fn file_state_raw_all_ff_bytes_rejected_with_version_mismatch()`

---

#### Behavior: All-zero 112-byte slice rejected by UrlStateRaw::from_bytes_checked()

```
Given: [0u8; 112]
When:  UrlStateRaw::from_bytes_checked(&zeros)
Then:  Err(PodStateError::VersionMismatch { type_name: "UrlStateRaw", actual: 0, expected: 1 })
```

**Test name**: `fn url_state_raw_all_zeros_rejected_due_to_version_0()`

---

#### Behavior: All-FF 112-byte slice rejected by UrlStateRaw::from_bytes_checked()

```
Given: [0xFFu8; 112]
When:  UrlStateRaw::from_bytes_checked(&all_ff)
Then:  Err(PodStateError::VersionMismatch { type_name: "UrlStateRaw", actual: 255, expected: 1 })
```

Note: version is checked before status per the validation order convention. Version byte at offset 52 is 0xFF = 255, which fails before the status check runs.

**Test name**: `fn url_state_raw_all_ff_bytes_rejected_with_version_mismatch()`

---

### 3.11 Error Display Messages

#### Behavior: PodStateError::InvalidFileStatus displays correct message

```
Given: PodStateError::InvalidFileStatus(42)
When:  error.to_string()
Then:  contains "42" and "valid: 0-3"
```

**Test name**: `fn pod_state_error_invalid_file_status_displays_correct_message()`

---

#### Behavior: PodStateError::InvalidUrlStatus displays correct message

```
Given: PodStateError::InvalidUrlStatus(99)
When:  error.to_string()
Then:  contains "99" and "valid: 0-3"
```

**Test name**: `fn pod_state_error_invalid_url_status_displays_correct_message()`

---

#### Behavior: PodStateError::WrongByteSize displays correct message

```
Given: PodStateError::WrongByteSize { type_name: "FileStateRaw", actual: 50, expected: 104 }
When:  error.to_string()
Then:  contains "FileStateRaw", "50", "104"
```

**Test name**: `fn pod_state_error_wrong_byte_size_displays_correct_message()`

---

#### Behavior: PodStateError::VersionMismatch displays correct message

```
Given: PodStateError::VersionMismatch { type_name: "UrlStateRaw", actual: 5, expected: 1 }
When:  error.to_string()
Then:  contains "UrlStateRaw", "5", "1"
```

**Test name**: `fn pod_state_error_version_mismatch_displays_correct_message()`

---

#### Behavior: PodStateError::ReservedBytesNonZero displays correct message

```
Given: PodStateError::ReservedBytesNonZero { type_name: "FileStateRaw", offset: 50 }
When:  error.to_string()
Then:  contains "FileStateRaw", "50"
```

**Test name**: `fn pod_state_error_reserved_bytes_nonzero_displays_correct_message()`

---

### 3.12 Trait Proofs

#### Behavior: FileStateRaw is Copy + Send + Sync

```
Given: FileStateRaw type
When:  compile-time trait bounds are checked
Then:  FileStateRaw: Copy + Send + Sync (static proof)
```

**Test name**: `fn file_state_raw_is_copy_send_sync()` (compile-time / static)

---

#### Behavior: UrlStateRaw is Copy + Send + Sync

```
Given: UrlStateRaw type
When:  compile-time trait bounds are checked
Then:  UrlStateRaw: Copy + Send + Sync (static proof)
```

**Test name**: `fn url_state_raw_is_copy_send_sync()` (compile-time / static)

---

## 4. Proptest Invariants

### Proptest: FileStateRaw round-trip

```
Invariant: For any FileStateRaw constructed via new() with arbitrary valid inputs,
           as_bytes() → from_bytes_checked() → Ok(original)
Strategy:  content_hash: any [u8; 32]
           file_size: any u64
           last_modified_ms: any u64
           status: proptest::sample::select(vec![0u8, 1, 2, 3]).prop_map(|d| FileStateStatus::from_discriminant(d).unwrap())
Anti:      N/A (construction always succeeds for valid status enum)
```

### Proptest: UrlStateRaw round-trip

```
Invariant: For any UrlStateRaw constructed via new() with arbitrary valid inputs,
           as_bytes() → from_bytes_checked() → Ok(original)
Strategy:  content_hash: any [u8; 32]
           http_status: any u16
           content_length: any u64
           last_fetched_ms: any u64
           status: proptest::sample::select(vec![0u8, 1, 2, 3]).prop_map(|d| UrlStateStatus::from_discriminant(d).unwrap())
Anti:      N/A (construction always succeeds for valid status enum)
```

### Proptest: FileStateRaw byte length invariant

```
Invariant: For any FileStateRaw, as_bytes().len() == 104
Strategy:  same as round-trip strategy above
Anti:      N/A (structurally guaranteed by type)
```

### Proptest: UrlStateRaw byte length invariant

```
Invariant: For any UrlStateRaw, as_bytes().len() == 112
Strategy:  same as round-trip strategy above
Anti:      N/A (structurally guaranteed by type)
```

### Proptest: FileStateStatus from_discriminant validity

```
Invariant: from_discriminant(d) returns Ok for d in 0..=3, Err for d in 4..=255
Strategy:  any u8 value
Anti:      Any value >= 4
```

### Proptest: UrlStateStatus from_discriminant validity

```
Invariant: from_discriminant(d) returns Ok for d in 0..=3, Err for d in 4..=255
Strategy:  any u8 value
Anti:      Any value >= 4
```

### Proptest: FileStateRaw validate catches all corruptions

```
Invariant: For any FileStateRaw with status byte >= 4, validate() returns Err
Strategy:  valid FileStateRaw + override status to any u8 in 4..=255
Anti:      Status 0-3 are the only valid values
```

### Proptest: UrlStateRaw validate catches all corruptions

```
Invariant: For any UrlStateRaw with status byte >= 4, validate() returns Err
Strategy:  valid UrlStateRaw + override status to any u8 in 4..=255
Anti:      Status 0-3 are the only valid values
```

---

## 5. Fuzz Targets

### Fuzz Target: `file_state_raw_from_bytes`

```
Input type: &[u8] (arbitrary-length byte slice)
Risk:       Panic on wrong-size slice passed to from_bytes_unchecked (unsafe path)
            Logic error in validation order in from_bytes_checked
Corpus seeds:
  - [0u8; 104] (all zeros)
  - [0xFFu8; 104] (all ones)
  - Valid FileStateRaw::new([1;32], 0, 0, Unknown).as_bytes()
  - Valid bytes with status byte = 4 at offset 49
  - Valid bytes with version byte = 2 at offset 48
  - Valid bytes with reserved[0] = 1 at offset 50
  - Empty slice
  - 103-byte slice
  - 105-byte slice
```

### Fuzz Target: `url_state_raw_from_bytes`

```
Input type: &[u8] (arbitrary-length byte slice)
Risk:       Panic on wrong-size slice in from_bytes_unchecked
            Logic error in validation (pad1, reserved, version, status order)
Corpus seeds:
  - [0u8; 112] (all zeros)
  - [0xFFu8; 112] (all ones)
  - Valid UrlStateRaw::new([1;32], 200, 0, 0, Fresh).as_bytes()
  - Valid bytes with status byte = 5 at offset 53
  - Valid bytes with _pad1 = [0xFF, 0xFF] at offset 34
  - Valid bytes with reserved[0] = 1 at offset 54
  - Empty slice
  - 111-byte slice
  - 113-byte slice
```

### Fuzz Target: `file_state_status_from_discriminant`

```
Input type: u8
Risk:       Panic if match is non-exhaustive (shouldn't be with #[repr(u8)])
Corpus seeds:
  - 0, 1, 2, 3 (valid discriminants)
  - 4, 128, 255 (invalid)
```

### Fuzz Target: `url_state_status_from_discriminant`

```
Input type: u8
Risk:       Panic if match is non-exhaustive
Corpus seeds:
  - 0, 1, 2, 3 (valid discriminants)
  - 4, 128, 255 (invalid)
```

---

## 6. Kani Harnesses

### Kani Harness: file_state_raw_no_padding

```
Property: size_of::<FileStateRaw>() == 32 + 8 + 8 + 1 + 1 + 54
Bound:    N/A (compile-time constant)
Rationale: Padding would cause silent data corruption when bytes are written to redb
           and read back on a different alignment. This must be PROVEN, not just tested.
```

### Kani Harness: url_state_raw_no_padding

```
Property: size_of::<UrlStateRaw>() == 32 + 2 + 2 + 8 + 8 + 1 + 1 + 58
Bound:    N/A (compile-time constant)
Rationale: Same as above. The _pad1 field must be the ONLY padding in UrlStateRaw.
           If the compiler inserts additional hidden padding, data corruption follows.
```

### Kani Harness: file_state_raw_alignment

```
Property: align_of::<FileStateRaw>() == 8
Bound:    N/A (compile-time constant)
Rationale: redb stores values as byte slices but may return aligned references.
           Misalignment causes UB on some platforms.
```

### Kani Harness: file_state_raw_field_offsets

```
Property: offset_of!(FileStateRaw, content_hash) == 0
  And:    offset_of!(FileStateRaw, file_size) == 32
  And:    offset_of!(FileStateRaw, last_modified_ms) == 40
  And:    offset_of!(FileStateRaw, version) == 48
  And:    offset_of!(FileStateRaw, status) == 49
  And:    offset_of!(FileStateRaw, reserved) == 50
Bound:    N/A (compile-time constants)
Rationale: If field offsets drift (e.g., due to a refactor changing field order or type),
           all bytemuck byte casting will silently read wrong fields. This is the
           most critical invariant to formally verify.
```

### Kani Harness: url_state_raw_field_offsets

```
Property: offset_of!(UrlStateRaw, content_hash) == 0
  And:    offset_of!(UrlStateRaw, http_status) == 32
  And:    offset_of!(UrlStateRaw, _pad1) == 34
  And:    offset_of!(UrlStateRaw, content_length) == 36
  And:    offset_of!(UrlStateRaw, last_fetched_ms) == 44
  And:    offset_of!(UrlStateRaw, version) == 52
  And:    offset_of!(UrlStateRaw, status) == 53
  And:    offset_of!(UrlStateRaw, reserved) == 54
Bound:    N/A (compile-time constants)
Rationale: Same as FileStateRaw — field offset drift = silent data corruption.
           The _pad1 gap at offset 34..36 is especially critical to verify.
```

---

## 7. Mutation Testing Checkpoints

**Target: ≥90% mutation kill rate**

### Critical Mutations to Catch

| # | Mutation | Caught By Test |
|---|----------|---------------|
| 1 | Change `FileStateRaw::new()` to set `version = 0` | `file_state_raw_new_sets_version_to_1` |
| 2 | Change `FileStateRaw::new()` to set `version = 2` | `file_state_raw_new_sets_version_to_1` |
| 3 | Change `FileStateRaw::new()` to NOT zero `reserved` | `file_state_raw_new_zeroes_reserved_bytes` |
| 4 | Change `UrlStateRaw::new()` to NOT zero `_pad1` | `url_state_raw_new_zeroes_pad_and_reserved_bytes` |
| 5 | Change `UrlStateRaw::new()` to NOT zero `reserved` | `url_state_raw_new_zeroes_pad_and_reserved_bytes` |
| 6 | Remove `from_bytes_checked()` size check | `file_state_raw_from_bytes_checked_rejects_wrong_size` |
| 7 | Remove `from_bytes_checked()` version check | `file_state_raw_from_bytes_checked_rejects_version_2` |
| 8 | Remove `from_bytes_checked()` status check | `file_state_raw_from_bytes_checked_rejects_invalid_status_byte_4` |
| 9 | Remove `from_bytes_checked()` reserved bytes check | `file_state_raw_from_bytes_checked_rejects_nonzero_reserved_bytes` |
| 10 | Change `from_discriminant()` to accept value 4 | `file_state_status_from_discriminant_4_returns_invalid_file_status` |
| 11 | Change `from_discriminant()` to accept value 255 | `file_state_status_from_discriminant_255_returns_invalid_file_status` |
| 12 | Swap `FileStateStatus::Unchanged` and `::Modified` discriminants | `file_state_status_from_discriminant_1_returns_unchanged` + `file_state_status_from_discriminant_2_returns_modified` |
| 13 | Remove `FileStateRaw::validate()` status check | `file_state_raw_validate_rejects_corrupted_status` |
| 14 | Remove `FileStateRaw::validate()` version check | `file_state_raw_validate_rejects_wrong_version` |
| 15 | Remove `FileStateRaw::validate()` reserved bytes check | `file_state_raw_validate_rejects_nonzero_reserved` |
| 16 | Change `as_bytes()` to return wrong length | `file_state_raw_as_bytes_returns_104_byte_slice` |
| 17 | Change `WrongByteSize` error to report wrong `expected` | `pod_state_error_wrong_byte_size_displays_correct_message` |
| 18 | Change `VersionMismatch` error to report wrong `actual` | `pod_state_error_version_mismatch_displays_correct_message` |
| 19 | Remove `UrlStateRaw::from_bytes_checked()` `_pad1` check | `url_state_raw_from_bytes_checked_rejects_nonzero_pad1_bytes` |
| 20 | **Delete `FileStateRaw::from_bytes_unchecked()` body** | `file_state_raw_from_bytes_unchecked_returns_reference_to_same_bytes` |
| 21 | **`from_bytes_unchecked()` returns reference to wrong bytes** | `file_state_raw_from_bytes_unchecked_preserves_pointer_identity` |
| 22 | **Delete `UrlStateRaw::from_bytes_unchecked()` body** | `url_state_raw_from_bytes_unchecked_returns_reference_to_same_bytes` |
| 23 | **`UrlStateRaw::from_bytes_unchecked()` returns reference to wrong bytes** | `url_state_raw_from_bytes_unchecked_preserves_pointer_identity` |
| 24 | **Remove `UrlStateRaw::validate()` version check** | `url_state_raw_validate_rejects_wrong_version` |
| 25 | **Remove `UrlStateRaw::validate()` reserved-field check** | `url_state_raw_validate_rejects_nonzero_reserved` |
| 26 | Swap `content_length` and `last_fetched_ms` in `UrlStateRaw::new()` | `url_state_raw_new_returns_correct_field_values` |
| 27 | Truncate `content_length` on construction | `url_state_raw_new_stores_max_content_length` |
| 28 | Truncate `last_fetched_ms` on construction | `url_state_raw_new_stores_max_last_fetched_ms` |
| 29 | Reorder validation checks in `from_bytes_checked()` | `file_state_raw_all_ff_bytes_rejected_with_version_mismatch` (pins check order: version before status) |

### Mutation Kill Coverage Assessment

- **Struct size assertions**: Any mutation to struct layout (adding/removing fields, changing types) → caught by B01–B06.
- **Constructor field assignments**: Any mutation to field initialization → caught by B07–B09, B13–B15.
- **Constructor max-boundary**: Truncation of u64 fields → caught by `url_state_raw_new_stores_max_content_length`, `url_state_raw_new_stores_max_last_fetched_ms`.
- **from_bytes_unchecked body**: Deletion or incorrect reference return → caught by B25, B26, plus pointer identity tests.
- **Validation branching**: Any removed branch in `from_bytes_checked()` or `validate()` → caught by B27–B34, B39–B42.
- **Validation ordering**: Reordering checks in `from_bytes_checked()` → caught by all-FF tests that pin to `VersionMismatch`.
- **Enum discriminant mapping**: Any swapped/removed arm in `from_discriminant()` → caught by B35–B38.
- **Error message construction**: Any mutation to format string or field values → caught by display tests.

**All 29 listed mutations are killed. Kill rate: 100% of identified mutations.**

---

## 8. Combinatorial Coverage Matrix

### FileStateRaw::new()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| all fields populated | hash=[1;32], size=4096, mod_ms=1000, status=Modified | instance with exact field values | unit |
| zero file_size | hash=[0;32], size=0, mod_ms=0, status=Unknown | instance.file_size == 0 | unit |
| max u64 file_size | hash=[0xFF;32], size=u64::MAX, mod_ms=u64::MAX, status=Deleted | instance.file_size == u64::MAX | unit |
| each status variant | status in {Unknown, Unchanged, Modified, Deleted} | instance.status == variant as u8 | unit |
| version always 1 | hash=[0;32], size=4096, mod_ms=1700000000000, status=Modified | instance.version == 1 | unit |
| reserved always zero | hash=[0xFF;32], size=u64::MAX, mod_ms=u64::MAX, status=Deleted | instance.reserved == [0u8; 54] | unit |

### UrlStateRaw::new()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| all fields populated | hash=[1;32], http=200, len=8192, fetch_ms=1000, status=Fresh | instance with exact field values | unit |
| zero content_length | hash=[0;32], http=0, len=0, fetch_ms=0, status=Unknown | instance.content_length == 0 | unit |
| max u16 http_status | hash=[0;32], http=u16::MAX, len=0, fetch_ms=0, status=Error | instance.http_status == u16::MAX | unit |
| max u64 content_length | hash=[0;32], http=0, len=u64::MAX, fetch_ms=0, status=Unknown | instance.content_length == u64::MAX | unit |
| max u64 last_fetched_ms | hash=[0;32], http=0, len=0, fetch_ms=u64::MAX, status=Unknown | instance.last_fetched_ms == u64::MAX | unit |
| each status variant | status in {Unknown, Fresh, Stale, Error} | instance.status == variant as u8 | unit |
| version always 1 | hash=[0;32], http=200, len=8192, fetch_ms=1700000000000, status=Fresh | instance.version == 1 | unit |
| pad1 always zero | hash=[0xFF;32], http=u16::MAX, len=u64::MAX, fetch_ms=u64::MAX, status=Error | instance._pad1 == [0u8; 2] | unit |
| reserved always zero | hash=[0xFF;32], http=u16::MAX, len=u64::MAX, fetch_ms=u64::MAX, status=Error | instance.reserved == [0u8; 58] | unit |

### FileStateRaw::from_bytes_unchecked()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid bytes from new() | as_bytes() of new([0xAB;32], 4096, 1700000000000, Modified) | reference with matching field values | unit |
| pointer identity | as_bytes() of any valid instance | returned pointer == input slice pointer | unit |

### UrlStateRaw::from_bytes_unchecked()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid bytes from new() | as_bytes() of new([0xCD;32], 200, 8192, 1700000000000, Fresh) | reference with matching field values | unit |
| pointer identity | as_bytes() of any valid instance | returned pointer == input slice pointer | unit |

### FileStateRaw::from_bytes_checked()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid bytes | 104-byte valid slice | Ok(FileStateRaw matching input) | integration |
| empty slice | `&[]` | Err(WrongByteSize { actual: 0, expected: 104 }) | integration |
| one byte short | 103-byte slice | Err(WrongByteSize { actual: 103, expected: 104 }) | integration |
| one byte over | 105-byte slice | Err(WrongByteSize { actual: 105, expected: 104 }) | integration |
| invalid status 4 | valid slice, status=4 | Err(InvalidFileStatus(4)) | integration |
| invalid status 255 | valid slice, status=255 | Err(InvalidFileStatus(255)) | integration |
| version 0 | valid except version=0 | Err(VersionMismatch { actual: 0, expected: 1 }) | integration |
| version 2 | valid except version=2 | Err(VersionMismatch { actual: 2, expected: 1 }) | integration |
| non-zero reserved[0] | valid except reserved[0]=1 | Err(ReservedBytesNonZero { offset: 50 }) | integration |
| all-zero bytes | [0u8; 104] | Err(VersionMismatch { actual: 0, expected: 1 }) | integration |
| all-FF bytes | [0xFFu8; 104] | Err(VersionMismatch { actual: 255, expected: 1 }) | integration |

### UrlStateRaw::from_bytes_checked()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid bytes | 112-byte valid slice | Ok(UrlStateRaw matching input) | integration |
| empty slice | `&[]` | Err(WrongByteSize { actual: 0, expected: 112 }) | integration |
| one byte short | 111-byte slice | Err(WrongByteSize { actual: 111, expected: 112 }) | integration |
| one byte over | 113-byte slice | Err(WrongByteSize { actual: 113, expected: 112 }) | integration |
| invalid status 5 | valid slice, status=5 | Err(InvalidUrlStatus(5)) | integration |
| invalid status 255 | valid slice, status=255 | Err(InvalidUrlStatus(255)) | integration |
| version 0 | valid except version=0 | Err(VersionMismatch { actual: 0, expected: 1 }) | integration |
| non-zero _pad1 | valid except _pad1=[1,1] | Err(ReservedBytesNonZero { offset: 34 }) | integration |
| non-zero reserved[0] | valid except reserved[0]=1 | Err(ReservedBytesNonZero { offset: 54 }) | integration |
| all-zero bytes | [0u8; 112] | Err(VersionMismatch { actual: 0, expected: 1 }) | integration |
| all-FF bytes | [0xFFu8; 112] | Err(VersionMismatch { actual: 255, expected: 1 }) | integration |

### FileStateRaw::validate()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid instance | new([0;32], 0, 0, Unknown) | Ok(()) | unit |
| corrupted status=99 | manually constructed, status=99 | Err(InvalidFileStatus(99)) | unit |
| wrong version=0 | manually constructed, version=0 | Err(VersionMismatch { actual: 0, expected: 1 }) | unit |
| non-zero reserved[0] | manually constructed, reserved[0]=1 | Err(ReservedBytesNonZero { offset: 50 }) | unit |

### UrlStateRaw::validate()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid instance | new([0;32], 0, 0, 0, Unknown) | Ok(()) | unit |
| corrupted status=99 | manually constructed, status=99 | Err(InvalidUrlStatus(99)) | unit |
| wrong version=0 | manually constructed, version=0 | Err(VersionMismatch { actual: 0, expected: 1 }) | unit |
| non-zero _pad1 | manually constructed, _pad1=[0xFF,0xFF] | Err(ReservedBytesNonZero { offset: 34 }) | unit |
| non-zero reserved[0] | manually constructed, reserved[0]=1 | Err(ReservedBytesNonZero { offset: 54 }) | unit |

### FileStateStatus::from_discriminant()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| 0 | valid | Ok(Unknown) | unit |
| 1 | valid | Ok(Unchanged) | unit |
| 2 | valid | Ok(Modified) | unit |
| 3 | valid | Ok(Deleted) | unit |
| 4 | invalid | Err(InvalidFileStatus(4)) | unit |
| 255 | invalid | Err(InvalidFileStatus(255)) | unit |

### UrlStateStatus::from_discriminant()

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| 0 | valid | Ok(Unknown) | unit |
| 1 | valid | Ok(Fresh) | unit |
| 2 | valid | Ok(Stale) | unit |
| 3 | valid | Ok(Error) | unit |
| 4 | invalid | Err(InvalidUrlStatus(4)) | unit |
| 255 | invalid | Err(InvalidUrlStatus(255)) | unit |

### PodStateError variants (exhaustive)

| Error Variant | Test Scenario | Layer |
|---------------|---------------|-------|
| InvalidFileStatus(u8) | from_discriminant(4), from_bytes_checked with status=4, validate with status=99 | unit |
| InvalidUrlStatus(u8) | from_discriminant(4), from_bytes_checked with status=5, validate with status=99 | unit |
| WrongByteSize { type_name, actual, expected } | from_bytes_checked with wrong-size slice | integration |
| VersionMismatch { type_name, actual, expected } | from_bytes_checked with version=0 or 2, validate with version=0, all-FF/all-zero | integration |
| ReservedBytesNonZero { type_name, offset } | from_bytes_checked with non-zero reserved, validate with non-zero reserved/pad1 | integration |

---

## Open Questions

1. **Validation order convention**: This plan mandates that `from_bytes_checked()` and
   `validate()` check invariants in this order: **size → version → status → reserved bytes**.
   All-FF and all-zero tests pin to the first error under this ordering (VersionMismatch).
   The implementation MUST follow this ordering for the all-FF tests to pass.

2. **`from_bytes_unchecked()` safety**: This function is marked as performing a Pod cast
   without validation. The BDD scenarios exercise it with known-valid byte slices (produced
   by `as_bytes()` from a valid instance). Tests verify both field-level correctness and
   pointer identity (the returned reference points to the same memory as the input slice).
   Calling it with wrong-size or invalid bytes is UB by contract and is not tested.

3. **`offset_of!` macro**: Rust has no stable `offset_of!` macro. The Kani harness for field
   offsets may need to use `unsafe` pointer arithmetic or the `memoffset` crate. If
   `#![forbid(unsafe_code)]` prevents this, consider using a dedicated test module with
   `#![allow(unsafe_code)]` and a safety comment, or using `addr_of!` + casting.

4. **bytemuck + `#![forbid(unsafe_code)]`**: As noted in the contract, if bytemuck's derive
   macros emit `unsafe impl` in the consumer crate, compilation will fail. The test plan
   includes a compile-time test (B11, B12, B17, B18) that serves as a canary — if the types
   compile with `bytemuck::bytes_of`, the integration works.

5. **`#[non_exhaustive]` on `PodStateError`**: The contract specifies `#[non_exhaustive]`.
   Tests should NOT match on wildcard `_ =>` arms; they must match the specific variant
   being tested. This ensures new variants added later don't silently pass existing tests.
