# Test Plan: SCIP-inspired Semantic Domain Model

- **bead_id**: cdocs-7mf
- **bead_title**: schema: Define SCIP-inspired semantic domain model
- **phase**: STATE_1.5_RETRY_2
- **updated_at**: 2026-03-29T17:47:19Z

## Summary

- **Behaviors identified**: 95
- **Trophy allocation**: 82 unit / 19 integration / 2 static / 0 e2e
- **Proptest invariants**: 9
- **Fuzz targets**: 4
- **Kani harnesses**: 3
- **Mutation testing threshold**: ≥90% kill rate

### Ratio Justification

This bead defines a pure domain model (zero I/O, zero async, zero network). The Testing Trophy target is ~60% integration / ~30% unit, but that ratio assumes component boundaries with real dependencies. Here the only integration boundary is `serde` (Serialize/Deserialize). All constructor validation, accessor methods, bitmask operations, Display formatting, and equality/ordering are pure Calc-layer logic — the rightful domain of exhaustive unit tests. Deviation is justified by Fowler: "Test behaviors, not methods" — and every behavior here IS a pure computation. Integration tests cover serde round-trips (real `serde_json`) and Hash in real collections.

---

## 1. Behavior Inventory

### ScipSymbolId (37 behaviors)

1. ScipSymbolId constructs valid ID when scheme, module_path, and descriptor are all valid
2. ScipSymbolId rejects empty scheme
3. ScipSymbolId rejects whitespace-only scheme (spaces)
4. ScipSymbolId rejects scheme containing `/`
5. ScipSymbolId rejects scheme containing `#`
6. ScipSymbolId rejects empty module_path
7. ScipSymbolId rejects whitespace-only module_path (spaces)
8. ScipSymbolId rejects module_path with empty segment at any position (e.g., `auth//service`)
9. ScipSymbolId rejects module_path starting with `/`
10. ScipSymbolId rejects module_path ending with `/`
11. ScipSymbolId rejects module_path containing `#`
12. ScipSymbolId rejects empty descriptor
13. ScipSymbolId rejects whitespace-only descriptor (spaces)
14. ScipSymbolId rejects descriptor containing `/`
15. ScipSymbolId rejects descriptor containing `#`
16. ScipSymbolId::parse returns valid ID when input matches `<scheme>/<path>#<descriptor>`
17. ScipSymbolId::parse rejects input with no `#` separator
18. ScipSymbolId::parse rejects empty string input
19. ScipSymbolId::parse rejects when scheme portion is empty (e.g., `/path#desc`)
20. ScipSymbolId::parse rejects when descriptor portion is empty (e.g., `scheme/path#`)
21. ScipSymbolId::parse and ScipSymbolId::new produce equal values for equivalent components
22. ScipSymbolId::as_str returns canonical `<scheme>/<module_path>#<descriptor>` format
23. ScipSymbolId::scheme() extracts the scheme portion before the first `/`
24. ScipSymbolId::module_path() extracts the module path between first `/` and `#`
25. ScipSymbolId::descriptor() extracts the descriptor portion after `#`
26. ScipSymbolId::into_string consumes self and returns owned String equal to as_str()
27. ScipSymbolId equality holds for identical components
28. ScipSymbolId ordering is lexicographic on the canonical string
29. ScipSymbolId Display outputs the canonical string
30. ScipSymbolId Deref returns `&str` pointing to the canonical string
31. ScipSymbolId AsRef returns `&str` pointing to the canonical string
32. ScipSymbolId Borrow returns `&str` pointing to the canonical string
33. ScipSymbolId round-trips through JSON serialization/deserialization preserving value
34. ScipSymbolId rejects tab-only scheme (M4 fix)
35. ScipSymbolId rejects newline-only module_path (M4 fix)
36. ScipSymbolId rejects mixed-whitespace-only descriptor (`"\t \n"`) (M4 fix)
37. ScipSymbolId::parse rejects input with no `/` before `#` (m7 fix)

### SymbolRole (27 behaviors)

38. SymbolRole::DEFINITION has bits == 1
39. SymbolRole::READ has bits == 2
40. SymbolRole::WRITE has bits == 4
41. SymbolRole::GENERATED has bits == 8
42. SymbolRole::TEST has bits == 16
43. SymbolRole::ALL has bits == 31
44. SymbolRole::from_bits returns Ok for each individual valid flag
45. SymbolRole::from_bits returns Ok for valid combinations (e.g., 3 = DEFINITION | READ)
46. SymbolRole::from_bits returns Ok for zero (empty role)
47. SymbolRole::from_bits returns Err(UnknownBit) for bits outside the defined mask
48. SymbolRole::from_bits_truncate silently discards unknown bits (e.g., 33 → 1)
49. SymbolRole::from_bits_truncate returns empty for bits with no known flags
50. SymbolRole::empty() has bits == 0
51. SymbolRole::empty() is_empty returns true
52. SymbolRole::bits() returns the raw u32 value for arbitrary roles (m3 fix)
53. SymbolRole::contains returns true when the flag is set
54. SymbolRole::contains returns false when the flag is not set
55. SymbolRole::contains returns true for self (a role contains itself)
56. SymbolRole::is_empty returns false for non-zero roles
57. SymbolRole::union combines two roles via bitwise OR
58. SymbolRole::intersection keeps only shared flags via bitwise AND
59. SymbolRole Display for a single role outputs the lowercase name (e.g., `"definition"`)
60. SymbolRole Display for combined roles outputs sorted `+`-delimited names (e.g., `"definition+read"`)
61. SymbolRole Display for zero roles outputs `"none"`
62. SymbolRole BitOr produces the union of two roles
63. SymbolRole BitAnd produces the intersection of two roles
64. SymbolRole round-trips through JSON serialization as u32 bits value

### SymbolKind (8 behaviors)

65. SymbolKind Display outputs the lowercase variant name for all 12 variants
66. SymbolKind serializes as lowercase string for all 12 variants
67. SymbolKind deserializes from lowercase string for all 12 variants
68. SymbolKind deserialization rejects unknown strings with Err(UnknownKind)
69. SymbolKind round-trips through JSON for all 12 variants
70. SymbolKind equality and Hash are consistent (equal variants have equal hashes)
71. SymbolKind deserializes case-insensitively from uppercase/mixed-case strings (INV-6, M3 fix)
72. SymbolKind deserialization rejects empty string with Err(UnknownKind("")) (m6 fix)

### RelationshipKind (6 behaviors)

73. RelationshipKind Display outputs the lowercase variant name for all 6 variants
74. RelationshipKind serializes as lowercase string for all 6 variants
75. RelationshipKind deserializes from lowercase string for all 6 variants
76. RelationshipKind round-trips through JSON for all 6 variants
77. RelationshipKind equality and Hash are consistent
78. RelationshipKind deserialization rejects unknown strings with serde error (m10 fix)

### SymbolRelationship (5 behaviors)

79. SymbolRelationship equality compares both target and kind fields
80. SymbolRelationship Hash allows use in HashSet and HashMap
81. SymbolRelationship Hash/Eq consistency: equal hashes for equal relationships, unequal hashes for different kind (M2 fix)
82. SymbolRelationship serializes both target and kind to JSON
83. SymbolRelationship round-trips through JSON preserving both fields

### SymbolNode (8 behaviors)

84. SymbolNode equality compares all seven fields
85. SymbolNode Hash allows use as graph node keys in HashMap/HashSet
86. SymbolNode serializes all fields to JSON including optional None fields
87. SymbolNode deserializes from JSON with all fields present (m4 fix — independent of round-trip)
88. SymbolNode round-trips through JSON preserving all fields
89. SymbolNode with None signature and documentation round-trips through JSON correctly
90. SymbolNode with empty relationships Vec round-trips through JSON correctly
91. SymbolNode Hash/Eq consistency: different display_name but same scip_id produces equal hashes and unequal nodes (M1 fix)

### Error Types (4 behaviors)

92. All ScipSymbolIdError variants produce human-readable Display messages matching the contract patterns
93. ScipSymbolIdError variants carry correct payload data (e.g., `EmptyModuleSegment(pos)` preserves position)
94. SymbolRoleError::UnknownBit(bit) displays the bit value in the error message
95. SymbolKindError::UnknownKind(s) displays the string value in the error message

---

## 2. Trophy Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| 1 | ScipSymbolId new() valid construction | **Unit** | Pure validation logic, no I/O |
| 2 | ScipSymbolId new() EmptyScheme | **Unit** | Pure validation |
| 3 | ScipSymbolId new() whitespace scheme | **Unit** | Pure validation |
| 4 | ScipSymbolId new() scheme with `/` | **Unit** | Pure validation |
| 5 | ScipSymbolId new() scheme with `#` | **Unit** | Pure validation |
| 6 | ScipSymbolId new() EmptyModulePath | **Unit** | Pure validation |
| 7 | ScipSymbolId new() whitespace module_path | **Unit** | Pure validation |
| 8 | ScipSymbolId new() EmptyModuleSegment | **Unit** | Pure validation — must test specific position |
| 9 | ScipSymbolId new() LeadingSlash | **Unit** | Pure validation |
| 10 | ScipSymbolId new() TrailingSlash | **Unit** | Pure validation |
| 11 | ScipSymbolId new() HashInModulePath | **Unit** | Pure validation |
| 12 | ScipSymbolId new() EmptyDescriptor | **Unit** | Pure validation |
| 13 | ScipSymbolId new() whitespace descriptor | **Unit** | Pure validation |
| 14 | ScipSymbolId new() SlashInDescriptor | **Unit** | Pure validation |
| 15 | ScipSymbolId new() HashInDescriptor | **Unit** | Pure validation |
| 16 | ScipSymbolId parse() valid | **Unit** | Pure parsing |
| 17 | ScipSymbolId parse() no `#` | **Unit** | Pure parsing |
| 18 | ScipSymbolId parse() empty string | **Unit** | Pure parsing |
| 19 | ScipSymbolId parse() empty scheme | **Unit** | Pure parsing |
| 20 | ScipSymbolId parse() empty descriptor | **Unit** | Pure parsing |
| 21 | parse() == new() equivalence | **Unit** | Pure property |
| 22 | as_str() canonical format | **Unit** | Pure accessor |
| 23 | scheme() extraction | **Unit** | Pure accessor |
| 24 | module_path() extraction | **Unit** | Pure accessor |
| 25 | descriptor() extraction | **Unit** | Pure accessor |
| 26 | into_string() | **Unit** | Pure accessor |
| 27 | Eq symmetry | **Unit** | Pure trait |
| 28 | Ord lexicographic | **Unit** | Pure trait |
| 29 | Display | **Unit** | Pure formatting |
| 30 | Deref | **Unit** | Pure trait |
| 31 | AsRef | **Unit** | Pure trait |
| 32 | Borrow | **Unit** | Pure trait |
| 33 | JSON round-trip | **Integration** | Exercises real serde boundary |
| 34 | tab-only scheme | **Unit** | Pure validation |
| 35 | newline-only module_path | **Unit** | Pure validation |
| 36 | mixed-whitespace descriptor | **Unit** | Pure validation |
| 37 | parse no-slash | **Unit** | Pure parsing |
| 38–43 | SymbolRole constant bit values | **Unit** | Pure constants |
| 44 | from_bits valid single flags | **Unit** | Pure validation |
| 45 | from_bits valid combinations | **Unit** | Pure validation |
| 46 | from_bits zero | **Unit** | Pure validation |
| 47 | from_bits unknown bits | **Unit** | Pure validation |
| 48 | from_bits_truncate drops unknowns | **Unit** | Pure bitmask logic |
| 49 | from_bits_truncate all-unknown → empty | **Unit** | Pure bitmask logic |
| 50 | empty() bits == 0 | **Unit** | Pure constant |
| 51 | empty().is_empty() == true | **Unit** | Pure property |
| 52 | bits() returns raw value (arbitrary role) | **Unit** | Pure accessor |
| 53 | contains true | **Unit** | Pure bitmask logic |
| 54 | contains false | **Unit** | Pure bitmask logic |
| 55 | contains self | **Unit** | Pure property |
| 56 | is_empty false for non-zero | **Unit** | Pure property |
| 57 | union OR semantics | **Unit** | Pure bitmask logic |
| 58 | intersection AND semantics | **Unit** | Pure bitmask logic |
| 59 | Display single role | **Unit** | Pure formatting |
| 60 | Display combined sorted | **Unit** | Pure formatting |
| 61 | Display zero → "none" | **Unit** | Pure formatting |
| 62 | BitOr operator | **Unit** | Pure operator |
| 63 | BitAnd operator | **Unit** | Pure operator |
| 64 | JSON round-trip as u32 | **Integration** | Real serde boundary |
| 65 | SymbolKind Display 12 variants | **Unit** | Pure formatting |
| 66 | SymbolKind serialize lowercase | **Integration** | Real serde |
| 67 | SymbolKind deserialize lowercase | **Integration** | Real serde |
| 68 | SymbolKind reject unknown | **Integration** | Real serde |
| 69 | SymbolKind JSON round-trip | **Integration** | Real serde |
| 70 | SymbolKind Hash/Eq | **Unit** | Pure trait |
| 71 | SymbolKind case-insensitive deserialize | **Integration** | Real serde |
| 72 | SymbolKind reject empty string | **Integration** | Real serde |
| 73 | RelationshipKind Display 6 variants | **Unit** | Pure formatting |
| 74 | RelationshipKind serialize | **Integration** | Real serde |
| 75 | RelationshipKind deserialize | **Integration** | Real serde |
| 76 | RelationshipKind round-trip | **Integration** | Real serde |
| 77 | RelationshipKind Hash/Eq | **Unit** | Pure trait |
| 78 | RelationshipKind reject unknown | **Integration** | Real serde |
| 79 | SymbolRelationship Eq | **Unit** | Pure trait |
| 80 | SymbolRelationship Hash | **Unit** | Pure trait |
| 81 | SymbolRelationship Hash/Eq consistency | **Unit** | Pure trait |
| 82 | SymbolRelationship serialize | **Integration** | Real serde |
| 83 | SymbolRelationship round-trip | **Integration** | Real serde |
| 84 | SymbolNode Eq | **Unit** | Pure trait |
| 85 | SymbolNode Hash | **Unit** | Pure trait |
| 86 | SymbolNode serialize all fields | **Integration** | Real serde |
| 87 | SymbolNode deserialize (independent) | **Integration** | Real serde |
| 88 | SymbolNode round-trip | **Integration** | Real serde |
| 89 | SymbolNode None optionals round-trip | **Integration** | Real serde |
| 90 | SymbolNode empty Vec round-trip | **Integration** | Real serde |
| 91 | SymbolNode Hash/Eq consistency | **Unit** | Pure trait |
| 92 | ScipSymbolIdError Display messages | **Unit** | Pure formatting |
| 93 | ScipSymbolIdError payload data | **Unit** | Pure data |
| 94 | SymbolRoleError Display | **Unit** | Pure formatting |
| 95 | SymbolKindError Display | **Unit** | Pure formatting |
| — | clippy deny unwrap_used | **Static** | Compile-time gate |
| — | clippy deny expect_used | **Static** | Compile-time gate |

**Totals**: 82 unit, 19 integration, 2 static, 0 e2e.

---

## 3. BDD Scenarios

### 3.1 ScipSymbolId::new — Valid Construction

#### Behavior: ScipSymbolId constructs valid ID when all components valid

```
Given: scheme = "rust", module_path = "auth/AuthService", descriptor = "login()"
When:  ScipSymbolId::new("rust", "auth/AuthService", "login()")
Then:  Ok(id) where id.as_str() == "rust/auth/AuthService#login()"
```

Test: `fn scip_symbol_id_constructs_valid_id_when_all_components_valid()`

#### Behavior: ScipSymbolId constructs ID with method disambiguation in descriptor

```
Given: scheme = "rust", module_path = "auth/AuthService", descriptor = "Auth.my_method"
When:  ScipSymbolId::new("rust", "auth/AuthService", "Auth.my_method")
Then:  Ok(id) where id.as_str() == "rust/auth/AuthService#Auth.my_method"
```

Test: `fn scip_symbol_id_constructs_id_with_method_disambiguation_when_descriptor_contains_dot()`

#### Behavior: ScipSymbolId constructs ID with single-segment module path

```
Given: scheme = "python", module_path = "mymodule", descriptor = "MyClass"
When:  ScipSymbolId::new("python", "mymodule", "MyClass")
Then:  Ok(id) where id.as_str() == "python/mymodule#MyClass"
```

Test: `fn scip_symbol_id_constructs_with_single_segment_module_path_when_path_has_no_slashes()`

---

### 3.2 ScipSymbolId::new — Scheme Validation (2 error variants)

#### Behavior: ScipSymbolId rejects empty scheme

```
Given: scheme = ""
When:  ScipSymbolId::new("", "auth/AuthService", "login()")
Then:  Err(ScipSymbolIdError::EmptyScheme)
```

Test: `fn scip_symbol_id_returns_empty_scheme_error_when_scheme_is_empty()`

#### Behavior: ScipSymbolId rejects whitespace-only scheme (spaces)

```
Given: scheme = "   "
When:  ScipSymbolId::new("   ", "auth/AuthService", "login()")
Then:  Err(ScipSymbolIdError::EmptyScheme)
```

Test: `fn scip_symbol_id_returns_empty_scheme_error_when_scheme_is_whitespace_only()`

#### Behavior: ScipSymbolId rejects tab-only scheme (M4 fix)

```
Given: scheme = "\t"
When:  ScipSymbolId::new("\t", "auth/AuthService", "login()")
Then:  Err(ScipSymbolIdError::EmptyScheme)
```

Test: `fn scip_symbol_id_returns_empty_scheme_error_when_scheme_is_tab_only()`

#### Behavior: ScipSymbolId rejects scheme containing `/`

```
Given: scheme = "rust/core"
When:  ScipSymbolId::new("rust/core", "auth", "login()")
Then:  Err(ScipSymbolIdError::InvalidScheme(s)) where s == "/"
```

Test: `fn scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_slash()`

#### Behavior: ScipSymbolId rejects scheme containing `#`

```
Given: scheme = "rust#std"
When:  ScipSymbolId::new("rust#std", "auth", "login()")
Then:  Err(ScipSymbolIdError::InvalidScheme(s)) where s == "#"
```

Test: `fn scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_hash()`

#### Behavior: ScipSymbolId rejects scheme containing multiple invalid characters

```
Given: scheme = "ru/st#core"
When:  ScipSymbolId::new("ru/st#core", "auth", "login()")
Then:  Err(ScipSymbolIdError::InvalidScheme(_))
```

Test: `fn scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_multiple_invalid_chars()`

---

### 3.3 ScipSymbolId::new — Module Path Validation (5 error variants)

#### Behavior: ScipSymbolId rejects empty module path

```
Given: module_path = ""
When:  ScipSymbolId::new("rust", "", "login()")
Then:  Err(ScipSymbolIdError::EmptyModulePath)
```

Test: `fn scip_symbol_id_returns_empty_module_path_error_when_module_path_is_empty()`

#### Behavior: ScipSymbolId rejects whitespace-only module path (spaces)

```
Given: module_path = "  "
When:  ScipSymbolId::new("rust", "  ", "login()")
Then:  Err(ScipSymbolIdError::EmptyModulePath)
```

Test: `fn scip_symbol_id_returns_empty_module_path_error_when_module_path_is_whitespace_only()`

#### Behavior: ScipSymbolId rejects newline-only module path (M4 fix)

```
Given: module_path = "\n"
When:  ScipSymbolId::new("rust", "\n", "login()")
Then:  Err(ScipSymbolIdError::EmptyModulePath)
```

Test: `fn scip_symbol_id_returns_empty_module_path_error_when_module_path_is_newline_only()`

#### Behavior: ScipSymbolId rejects module path with empty segment (double slash)

```
Given: module_path = "auth//service"
When:  ScipSymbolId::new("rust", "auth//service", "login()")
Then:  Err(ScipSymbolIdError::EmptyModuleSegment(pos)) where pos == 5
```

Test: `fn scip_symbol_id_returns_empty_module_segment_error_when_path_has_double_slash()`

#### Behavior: ScipSymbolId rejects module path starting with `/`

```
Given: module_path = "/auth/service"
When:  ScipSymbolId::new("rust", "/auth/service", "login()")
Then:  Err(ScipSymbolIdError::LeadingSlash)
```

Test: `fn scip_symbol_id_returns_leading_slash_error_when_module_path_starts_with_slash()`

#### Behavior: ScipSymbolId rejects module path ending with `/`

```
Given: module_path = "auth/service/"
When:  ScipSymbolId::new("rust", "auth/service/", "login()")
Then:  Err(ScipSymbolIdError::TrailingSlash)
```

Test: `fn scip_symbol_id_returns_trailing_slash_error_when_module_path_ends_with_slash()`

#### Behavior: ScipSymbolId rejects module path containing `#`

```
Given: module_path = "auth#service"
When:  ScipSymbolId::new("rust", "auth#service", "login()")
Then:  Err(ScipSymbolIdError::HashInModulePath)
```

Test: `fn scip_symbol_id_returns_hash_in_module_path_error_when_path_contains_hash()`

---

### 3.4 ScipSymbolId::new — Descriptor Validation (2 error variants)

#### Behavior: ScipSymbolId rejects empty descriptor

```
Given: descriptor = ""
When:  ScipSymbolId::new("rust", "auth/AuthService", "")
Then:  Err(ScipSymbolIdError::EmptyDescriptor)
```

Test: `fn scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_empty()`

#### Behavior: ScipSymbolId rejects whitespace-only descriptor (spaces)

```
Given: descriptor = "  "
When:  ScipSymbolId::new("rust", "auth/AuthService", "  ")
Then:  Err(ScipSymbolIdError::EmptyDescriptor)
```

Test: `fn scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_whitespace_only()`

#### Behavior: ScipSymbolId rejects mixed-whitespace-only descriptor (M4 fix)

```
Given: descriptor = "\t \n"
When:  ScipSymbolId::new("rust", "auth/AuthService", "\t \n")
Then:  Err(ScipSymbolIdError::EmptyDescriptor)
```

Test: `fn scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_mixed_whitespace()`

#### Behavior: ScipSymbolId rejects descriptor containing `/`

```
Given: descriptor = "login/method"
When:  ScipSymbolId::new("rust", "auth/AuthService", "login/method")
Then:  Err(ScipSymbolIdError::SlashInDescriptor)
```

Test: `fn scip_symbol_id_returns_slash_in_descriptor_error_when_descriptor_contains_slash()`

#### Behavior: ScipSymbolId rejects descriptor containing `#` (m9 fix)

```
Given: descriptor = "login#extra"
When:  ScipSymbolId::new("rust", "auth/AuthService", "login#extra")
Then:  Err(ScipSymbolIdError::InvalidScheme(_)) or implementation rejects to preserve INV-1
       (single `#` separator invariant). The exact error variant depends on implementation
       strategy: reject `#` in descriptor, or reject via a dedicated variant.
```

Test: `fn scip_symbol_id_rejects_hash_in_descriptor_when_descriptor_contains_hash()`

> **Note on m9**: The contract PRE-S5 says descriptor "May include `.` for method disambiguation"
> but does not explicitly list `#` as forbidden. However, INV-1 states a ScipSymbolId always
> contains exactly one `#`. If `#` passes descriptor validation, `as_str()` would contain
> multiple `#`s. The test verifies this invariant is maintained regardless of which error
> variant is returned.

---

### 3.5 ScipSymbolId::parse — Valid and Invalid Formats

#### Behavior: ScipSymbolId::parse returns valid ID for well-formed input

```
Given: input = "rust/auth/AuthService#login()"
When:  ScipSymbolId::parse("rust/auth/AuthService#login()")
Then:  Ok(id) where id.as_str() == "rust/auth/AuthService#login()"
```

Test: `fn scip_symbol_id_parse_returns_valid_id_when_input_matches_format()`

#### Behavior: ScipSymbolId::parse rejects input with no `#` separator

```
Given: input = "rust/auth/AuthService"
When:  ScipSymbolId::parse("rust/auth/AuthService")
Then:  Err(ScipSymbolIdError::InvalidFormat(s)) where s == "rust/auth/AuthService"
```

Test: `fn scip_symbol_id_parse_returns_invalid_format_error_when_input_has_no_hash()`

#### Behavior: ScipSymbolId::parse rejects empty string (m2 fix — exact value)

```
Given: input = ""
When:  ScipSymbolId::parse("")
Then:  Err(ScipSymbolIdError::InvalidFormat(s)) where s == ""
```

Test: `fn scip_symbol_id_parse_returns_invalid_format_error_when_input_is_empty()`

#### Behavior: ScipSymbolId::parse rejects when scheme is empty (m2 fix — exact value)

```
Given: input = "/auth/AuthService#login()"
When:  ScipSymbolId::parse("/auth/AuthService#login()")
Then:  Err(ScipSymbolIdError::InvalidFormat(s)) where s == "/auth/AuthService#login()"
```

Test: `fn scip_symbol_id_parse_returns_invalid_format_error_when_scheme_is_empty()`

#### Behavior: ScipSymbolId::parse rejects when descriptor is empty (m2 fix — exact value)

```
Given: input = "rust/auth/AuthService#"
When:  ScipSymbolId::parse("rust/auth/AuthService#")
Then:  Err(ScipSymbolIdError::InvalidFormat(s)) where s == "rust/auth/AuthService#"
```

Test: `fn scip_symbol_id_parse_returns_invalid_format_error_when_descriptor_is_empty()`

#### Behavior: ScipSymbolId::parse rejects input with no `/` before `#` (m7 fix)

```
Given: input = "noslash#desc"
When:  ScipSymbolId::parse("noslash#desc")
Then:  Err(ScipSymbolIdError::InvalidFormat(s)) where s == "noslash#desc"
```

Test: `fn scip_symbol_id_parse_returns_invalid_format_error_when_input_has_no_slash()`

#### Behavior: ScipSymbolId::parse propagates module path validation errors (m2 fix — exact value)

```
Given: input = "rust/auth//service#login()"
When:  ScipSymbolId::parse("rust/auth//service#login()")
Then:  Err(ScipSymbolIdError::EmptyModuleSegment(pos)) where pos == 5
```

Test: `fn scip_symbol_id_parse_propagates_module_path_errors_when_path_has_empty_segments()`

#### Behavior: ScipSymbolId::parse propagates descriptor validation errors

```
Given: input = "rust/auth/AuthService#login/bad"
When:  ScipSymbolId::parse("rust/auth/AuthService#login/bad")
Then:  Err(ScipSymbolIdError::SlashInDescriptor)
```

Test: `fn scip_symbol_id_parse_propagates_descriptor_errors_when_descriptor_has_slash()`

---

### 3.6 ScipSymbolId — parse/new Equivalence

#### Behavior: ScipSymbolId::parse and ::new produce equal values for equivalent components

```
Given: scheme = "rust", module_path = "auth/AuthService", descriptor = "login()"
When:  let parsed = ScipSymbolId::parse("rust/auth/AuthService#login()")
       let constructed = ScipSymbolId::new("rust", "auth/AuthService", "login()")
Then:  parsed == constructed
```

Test: `fn scip_symbol_id_parse_equals_new_when_components_match()`

---

### 3.7 ScipSymbolId — Accessors

#### Behavior: ScipSymbolId::as_str returns canonical format

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  id.as_str()
Then:  "rust/auth/AuthService#login()"
```

Test: `fn scip_symbol_id_as_str_returns_canonical_format_when_id_is_valid()`

#### Behavior: ScipSymbolId::scheme() extracts scheme

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  id.scheme()
Then:  "rust"
```

Test: `fn scip_symbol_id_scheme_returns_scheme_portion_when_id_is_valid()`

#### Behavior: ScipSymbolId::module_path() extracts module path

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  id.module_path()
Then:  "auth/AuthService"
```

Test: `fn scip_symbol_id_module_path_returns_path_portion_when_id_is_valid()`

#### Behavior: ScipSymbolId::descriptor() extracts descriptor

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  id.descriptor()
Then:  "login()"
```

Test: `fn scip_symbol_id_descriptor_returns_descriptor_portion_when_id_is_valid()`

#### Behavior: ScipSymbolId::into_string consumes and returns owned String

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  let s = id.into_string()
Then:  s == "rust/auth/AuthService#login()"
```

Test: `fn scip_symbol_id_into_string_returns_owned_string_equal_to_as_str_when_consumed()`

---

### 3.8 ScipSymbolId — Trait Implementations

#### Behavior: ScipSymbolId equality holds for identical components

```
Given: id1 = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
       id2 = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  id1 == id2
Then:  true
```

Test: `fn scip_symbol_id_equality_holds_when_components_are_identical()`

#### Behavior: ScipSymbolId inequality holds for different components

```
Given: id1 = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
       id2 = ScipSymbolId::new("python", "auth/AuthService", "login()").unwrap()
When:  id1 != id2
Then:  true
```

Test: `fn scip_symbol_id_inequality_holds_when_schemes_differ()`

#### Behavior: ScipSymbolId ordering is lexicographic

```
Given: id_a = ScipSymbolId::new("python", "auth", "f").unwrap()
       id_b = ScipSymbolId::new("rust", "auth", "f").unwrap()
When:  id_a.cmp(&id_b)
Then:  std::cmp::Ordering::Less
```

Test: `fn scip_symbol_id_ordering_is_lexicographic_when_comparing_different_schemes()`

#### Behavior: ScipSymbolId Display outputs canonical string

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  format!("{id}")
Then:  "rust/auth/AuthService#login()"
```

Test: `fn scip_symbol_id_display_outputs_canonical_string_when_formatted()`

#### Behavior: ScipSymbolId Deref returns &str

```
Given: id = ScipSymbolId::new("rust", "auth", "f").unwrap()
When:  let s: &str = &*id
Then:  s == "rust/auth#f"
```

Test: `fn scip_symbol_id_deref_returns_str_reference_when_dereferenced()`

#### Behavior: ScipSymbolId AsRef returns &str

```
Given: id = ScipSymbolId::new("rust", "auth", "f").unwrap()
When:  let s: &str = id.as_ref()
Then:  s == "rust/auth#f"
```

Test: `fn scip_symbol_id_as_ref_returns_str_reference_when_called()`

#### Behavior: ScipSymbolId Borrow returns &str

```
Given: id = ScipSymbolId::new("rust", "auth", "f").unwrap()
When:  let s: &str = id.borrow()
Then:  s == "rust/auth#f"
```

Test: `fn scip_symbol_id_borrow_returns_str_reference_when_borrowed()`

---

### 3.9 ScipSymbolId — JSON Round-Trip

#### Behavior: ScipSymbolId round-trips through JSON

```
Given: id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap()
When:  let json = serde_json::to_string(&id)
       let reconstructed: ScipSymbolId = serde_json::from_str(&json).unwrap()
Then:  reconstructed == id
       && json == "\"rust/auth/AuthService#login()\""
```

Test: `fn scip_symbol_id_round_trips_through_json_when_serialized_and_deserialized()`

---

### 3.10 SymbolRole — Constants

#### Behavior: SymbolRole constants have correct bit values

```
Given: all five constants + ALL + empty
When:  [inspect each constant's bits()]
Then:  DEFINITION.bits() == 1
       READ.bits() == 2
       WRITE.bits() == 4
       GENERATED.bits() == 8
       TEST.bits() == 16
       ALL.bits() == 31
       empty().bits() == 0
```

Tests (one per constant):
- `fn symbol_role_definition_has_bits_one_when_inspected()`
- `fn symbol_role_read_has_bits_two_when_inspected()`
- `fn symbol_role_write_has_bits_four_when_inspected()`
- `fn symbol_role_generated_has_bits_eight_when_inspected()`
- `fn symbol_role_test_has_bits_sixteen_when_inspected()`
- `fn symbol_role_all_has_bits_thirty_one_when_inspected()`

---

### 3.11 SymbolRole — from_bits Validation

#### Behavior: SymbolRole::from_bits accepts each individual valid flag

```
Given: bits = 1, 2, 4, 8, 16
When:  SymbolRole::from_bits(bits) for each
Then:  Ok(role) where role.bits() == bits
```

Test: `fn symbol_role_from_bits_accepts_each_valid_single_flag_when_given_power_of_two()`

#### Behavior: SymbolRole::from_bits accepts valid combinations

```
Given: bits = 3 (DEFINITION|READ), 5 (DEFINITION|WRITE), 31 (ALL)
When:  SymbolRole::from_bits(bits)
Then:  Ok(role) where role.bits() == bits
       && role.contains(SymbolRole::DEFINITION)
```

Test: `fn symbol_role_from_bits_accepts_valid_combinations_when_bits_are_within_mask()`

#### Behavior: SymbolRole::from_bits accepts zero

```
Given: bits = 0
When:  SymbolRole::from_bits(0)
Then:  Ok(role) where role.bits() == 0 && role.is_empty()
```

Test: `fn symbol_role_from_bits_accepts_zero_when_given_empty_bits()`

#### Behavior: SymbolRole::from_bits rejects unknown bits

```
Given: bits = 32 (bit 5, outside defined mask)
When:  SymbolRole::from_bits(32)
Then:  Err(SymbolRoleError::UnknownBit(32))
```

Test: `fn symbol_role_from_bits_returns_unknown_bit_error_when_bits_exceed_mask()`

#### Behavior: SymbolRole::from_bits rejects combination with unknown bits

```
Given: bits = 33 (1 | 32)
When:  SymbolRole::from_bits(33)
Then:  Err(SymbolRoleError::UnknownBit(33))
```

Test: `fn symbol_role_from_bits_returns_unknown_bit_error_when_bits_include_unknown_flag()`

---

### 3.12 SymbolRole — from_bits_truncate

#### Behavior: SymbolRole::from_bits_truncate silently drops unknown bits

```
Given: bits = 33 (1 | 32)
When:  SymbolRole::from_bits_truncate(33)
Then:  role.bits() == 1
       && role.contains(SymbolRole::DEFINITION)
       && !role.contains(SymbolRole::TEST)
```

Test: `fn symbol_role_from_bits_truncate_drops_unknown_bits_when_bits_exceed_mask()`

#### Behavior: SymbolRole::from_bits_truncate returns empty for all-unknown bits

```
Given: bits = 64 (all unknown)
When:  SymbolRole::from_bits_truncate(64)
Then:  role.bits() == 0 && role.is_empty()
```

Test: `fn symbol_role_from_bits_truncate_returns_empty_when_all_bits_are_unknown()`

---

### 3.13 SymbolRole — Query Methods

#### Behavior: SymbolRole::bits() returns raw value for arbitrary role (m3 fix)

```
Given: role = SymbolRole::DEFINITION | SymbolRole::WRITE
When:  role.bits()
Then:  5
```

Test: `fn symbol_role_bits_returns_sum_of_flag_bits_when_role_is_combined()`

#### Behavior: SymbolRole::contains returns true for present flag

```
Given: role = SymbolRole::DEFINITION | SymbolRole::READ
When:  role.contains(SymbolRole::DEFINITION)
Then:  true
```

Test: `fn symbol_role_contains_returns_true_when_flag_is_set()`

#### Behavior: SymbolRole::contains returns false for absent flag

```
Given: role = SymbolRole::DEFINITION
When:  role.contains(SymbolRole::READ)
Then:  false
```

Test: `fn symbol_role_contains_returns_false_when_flag_is_not_set()`

#### Behavior: SymbolRole::contains returns true for self

```
Given: role = SymbolRole::DEFINITION | SymbolRole::WRITE
When:  role.contains(role)
Then:  true
```

Test: `fn symbol_role_contains_returns_true_when_comparing_role_to_itself()`

#### Behavior: SymbolRole::is_empty returns false for non-zero

```
Given: role = SymbolRole::DEFINITION
When:  role.is_empty()
Then:  false
```

Test: `fn symbol_role_is_empty_returns_false_when_any_flag_is_set()`

---

### 3.14 SymbolRole — Bitwise Operations

#### Behavior: SymbolRole::union combines flags

```
Given: a = SymbolRole::DEFINITION, b = SymbolRole::READ
When:  a.union(b)
Then:  result.bits() == 3
       && result.contains(SymbolRole::DEFINITION)
       && result.contains(SymbolRole::READ)
```

Test: `fn symbol_role_union_combines_flags_via_bitwise_or_when_called()`

#### Behavior: SymbolRole::intersection keeps shared flags

```
Given: a = SymbolRole::DEFINITION | SymbolRole::READ, b = SymbolRole::READ | SymbolRole::WRITE
When:  a.intersection(b)
Then:  result.bits() == 2
       && result.contains(SymbolRole::READ)
       && !result.contains(SymbolRole::DEFINITION)
```

Test: `fn symbol_role_intersection_keeps_shared_flags_via_bitwise_and_when_called()`

#### Behavior: SymbolRole BitOr operator produces union

```
Given: a = SymbolRole::DEFINITION, b = SymbolRole::READ
When:  a | b
Then:  result.bits() == 3
```

Test: `fn symbol_role_bitor_produces_union_when_operators_applied()`

#### Behavior: SymbolRole BitAnd operator produces intersection

```
Given: a = SymbolRole::DEFINITION | SymbolRole::READ, b = SymbolRole::READ | SymbolRole::WRITE
When:  a & b
Then:  result.bits() == 2
```

Test: `fn symbol_role_bitand_produces_intersection_when_operators_applied()`

---

### 3.15 SymbolRole — Display

#### Behavior: SymbolRole Display for single role outputs lowercase name

```
Given: role = SymbolRole::DEFINITION
When:  format!("{role}")
Then:  "definition"
```

Test (one per variant):
- `fn symbol_role_display_outputs_definition_when_single_definition_role()`
- `fn symbol_role_display_outputs_read_when_single_read_role()`
- `fn symbol_role_display_outputs_write_when_single_write_role()`
- `fn symbol_role_display_outputs_generated_when_single_generated_role()`
- `fn symbol_role_display_outputs_test_when_single_test_role()`

#### Behavior: SymbolRole Display for combined roles outputs sorted + delimited names

```
Given: role = SymbolRole::READ | SymbolRole::DEFINITION
When:  format!("{role}")
Then:  "definition+read"
```

Test: `fn symbol_role_display_outputs_sorted_plus_delimited_names_when_multiple_roles()`

#### Behavior: SymbolRole Display for all roles

```
Given: role = SymbolRole::ALL
When:  format!("{role}")
Then:  "definition+read+write+generated+test"
```

Test: `fn symbol_role_display_outputs_all_five_sorted_when_all_roles_set()`

#### Behavior: SymbolRole Display for zero roles outputs "none"

```
Given: role = SymbolRole::empty()
When:  format!("{role}")
Then:  "none"
```

Test: `fn symbol_role_display_outputs_none_when_no_roles_set()`

---

### 3.16 SymbolRole — JSON Round-Trip

#### Behavior: SymbolRole round-trips through JSON as u32

```
Given: role = SymbolRole::DEFINITION | SymbolRole::READ
When:  let json = serde_json::to_string(&role)
       let reconstructed: SymbolRole = serde_json::from_str(&json).unwrap()
Then:  reconstructed == role
       && json == "3"
```

Test: `fn symbol_role_round_trips_through_json_as_u32_when_serialized()`

---

### 3.17 SymbolKind — Display, Serialize, Deserialize

#### Behavior: SymbolKind Display outputs lowercase for all 12 variants

```
Given: each variant of SymbolKind
When:  format!("{kind}")
Then:  lowercase variant name string
```

Test: `fn symbol_kind_display_outputs_lowercase_name_for_each_variant()`

#### Behavior: SymbolKind serializes as lowercase string

```
Given: kind = SymbolKind::Struct
When:  serde_json::to_string(&kind)
Then:  "\"struct\""
```

Test: `fn symbol_kind_serializes_as_lowercase_string_when_all_variants_tested()`

#### Behavior: SymbolKind deserializes from lowercase string

```
Given: json = "\"function\""
When:  serde_json::from_str::<SymbolKind>(json)
Then:  Ok(SymbolKind::Function)
```

Test: `fn symbol_kind_deserializes_from_lowercase_string_for_each_variant()`

#### Behavior: SymbolKind deserialization rejects unknown string

```
Given: json = "\"unknown_kind\""
When:  serde_json::from_str::<SymbolKind>(json)
Then:  Err(SymbolKindError::UnknownKind(s)) where s == "unknown_kind"
```

Test: `fn symbol_kind_returns_unknown_kind_error_when_deserializing_invalid_string()`

#### Behavior: SymbolKind deserialization rejects empty string (m6 fix)

```
Given: json = "\"\""
When:  serde_json::from_str::<SymbolKind>(json)
Then:  Err(SymbolKindError::UnknownKind(s)) where s == ""
```

Test: `fn symbol_kind_returns_unknown_kind_error_when_deserializing_empty_string()`

#### Behavior: SymbolKind deserializes case-insensitively (M3 fix, INV-6)

```
Given: json = "\"Struct\""
When:  serde_json::from_str::<SymbolKind>(json)
Then:  Ok(SymbolKind::Struct)
```

```
Given: json = "\"FUNCTION\""
When:  serde_json::from_str::<SymbolKind>(json)
Then:  Ok(SymbolKind::Function)
```

```
Given: json = "\"Type_Alias\""
When:  serde_json::from_str::<SymbolKind>(json)
Then:  Ok(SymbolKind::TypeAlias)
```

Test: `fn symbol_kind_deserializes_case_insensitively_when_input_has_mixed_case()`

#### Behavior: SymbolKind round-trips through JSON for all variants

```
Given: kind = SymbolKind::Enum
When:  let json = serde_json::to_string(&kind)
       let back: SymbolKind = serde_json::from_str(&json).unwrap()
Then:  back == kind
```

Test: `fn symbol_kind_round_trips_through_json_for_each_variant()`

---

### 3.18 RelationshipKind — Display, Serialize, Deserialize

#### Behavior: RelationshipKind Display outputs lowercase for all 6 variants

```
Given: each variant of RelationshipKind
When:  format!("{kind}")
Then:  lowercase variant name string
```

Test: `fn relationship_kind_display_outputs_lowercase_name_for_each_variant()`

#### Behavior: RelationshipKind serializes as lowercase string

```
Given: kind = RelationshipKind::Implements
When:  serde_json::to_string(&kind)
Then:  "\"implements\""
```

Test: `fn relationship_kind_serializes_as_lowercase_string_for_each_variant()`

#### Behavior: RelationshipKind deserializes from lowercase string

```
Given: json = "\"calls\""
When:  serde_json::from_str::<RelationshipKind>(json)
Then:  Ok(RelationshipKind::Calls)
```

Test: `fn relationship_kind_deserializes_from_lowercase_string_for_each_variant()`

#### Behavior: RelationshipKind deserialization rejects unknown string (m10 fix)

```
Given: json = "\"FooBar\""
When:  serde_json::from_str::<RelationshipKind>(json)
Then:  serde_json::Error (deserialization error — serde produces an error for unknown enum variants)
```

Test: `fn relationship_kind_returns_deserialization_error_when_given_unknown_string()`

#### Behavior: RelationshipKind round-trips through JSON

```
Given: kind = RelationshipKind::Overrides
When:  let json = serde_json::to_string(&kind)
       let back: RelationshipKind = serde_json::from_str(&json).unwrap()
Then:  back == kind
```

Test: `fn relationship_kind_round_trips_through_json_for_each_variant()`

---

### 3.19 SymbolRelationship

#### Behavior: SymbolRelationship equality compares target and kind

```
Given: rel1 = SymbolRelationship { target: id1, kind: RelationshipKind::Calls }
       rel2 = SymbolRelationship { target: id1.clone(), kind: RelationshipKind::Calls }
       rel3 = SymbolRelationship { target: id2, kind: RelationshipKind::Calls }
When:  rel1 == rel2, rel1 == rel3
Then:  true, false
```

Test: `fn symbol_relationship_equality_compares_target_and_kind_when_compared()`

#### Behavior: SymbolRelationship Hash allows use in HashSet

```
Given: rel = SymbolRelationship { target: id, kind: RelationshipKind::Imports }
When:  HashSet::from([rel.clone()]).contains(&rel)
Then:  true
```

Test: `fn symbol_relationship_hash_allows_use_in_hashset_when_inserted()`

#### Behavior: SymbolRelationship Hash/Eq consistency (M2 fix)

```
Given: rel_calls = SymbolRelationship { target: id.clone(), kind: RelationshipKind::Calls }
       rel_refs = SymbolRelationship { target: id, kind: RelationshipKind::References }
       where rel_calls != rel_refs (same target, different kind)
When:  let hash_calls = HashSet::<u64>::from([compute_hash(&rel_calls)])
       let hash_refs = HashSet::<u64>::from([compute_hash(&rel_refs)])
Then:  rel_calls != rel_refs (verify they are unequal)
       && use_std_hash_to_verify: insert both into HashSet<SymbolRelationship>,
          assert set.len() == 2 (both stored as distinct entries)
       && insert rel_calls into HashMap<SymbolRelationship, i32> with value 1,
          then set rel_refs with value 2,
          assert map.len() == 2
```

Test: `fn symbol_relationship_hash_eq_consistency_when_same_target_different_kind()`

#### Behavior: SymbolRelationship round-trips through JSON

```
Given: rel with known target and kind
When:  serialize → deserialize
Then:  reconstructed == rel
```

Test: `fn symbol_relationship_round_trips_through_json_when_serialized()`

---

### 3.20 SymbolNode

#### Behavior: SymbolNode equality compares all fields

```
Given: node1 with full fields, node2 with identical fields, node3 with different display_name
When:  node1 == node2, node1 == node3
Then:  true, false
```

Test: `fn symbol_node_equality_compares_all_fields_when_compared()`

#### Behavior: SymbolNode Hash allows use as graph key

```
Given: node = SymbolNode { ... }
When:  HashMap::from([(node.scip_id.clone(), node.clone())]).get(&node.scip_id)
Then:  Some(&node)
```

Test: `fn symbol_node_hash_allows_use_as_graph_key_in_hashmap_when_inserted()`

#### Behavior: SymbolNode Hash/Eq consistency (M1 fix)

```
Given: node_a = SymbolNode { scip_id: id.clone(), display_name: "AuthService".into(), ... }
       node_b = SymbolNode { scip_id: id, display_name: "DifferentName".into(), ... }
       where node_a.scip_id == node_b.scip_id but node_a.display_name != node_b.display_name
       therefore node_a != node_b
When:  let mut set = HashSet::new();
       set.insert(node_a.clone());
       set.insert(node_b.clone());
Then:  node_a != node_b (verify they are unequal)
       && set.len() == 2 (both stored as distinct entries — Hash/Eq contract satisfied)
```

Test: `fn symbol_node_hash_eq_consistency_when_nodes_differ_in_non_id_fields()`

#### Behavior: SymbolNode serializes all fields

```
Given: node with all fields populated (Some signature, Some documentation, non-empty relationships)
When:  serde_json::to_string(&node)
Then:  JSON contains all field names: "scip_id", "kind", "roles", "display_name", "signature", "documentation", "relationships"
```

Test: `fn symbol_node_serializes_all_fields_when_all_fields_populated()`

#### Behavior: SymbolNode deserializes from JSON with all fields present (m4 fix — independent test)

```
Given: json = r#"{"scip_id":"rust/auth#f","kind":"struct","roles":1,"display_name":"Auth","signature":"fn()","documentation":"doc","relationships":[]}"#
When:  let node: SymbolNode = serde_json::from_str(json)
Then:  node.scip_id.as_str() == "rust/auth#f"
       && node.kind == SymbolKind::Struct
       && node.roles.bits() == 1
       && node.display_name == "Auth"
       && node.signature == Some("fn()".to_string())
       && node.documentation == Some("doc".to_string())
       && node.relationships.is_empty()
```

Test: `fn symbol_node_deserializes_from_json_when_all_fields_present()`

#### Behavior: SymbolNode round-trips with None optionals

```
Given: node with signature = None, documentation = None, relationships = vec![]
When:  serialize → deserialize
Then:  reconstructed.signature == None
       && reconstructed.documentation == None
       && reconstructed.relationships.is_empty()
```

Test: `fn symbol_node_round_trips_with_none_optionals_when_fields_are_empty()`

#### Behavior: SymbolNode round-trips with populated fields

```
Given: node with signature = Some("fn login() -> bool"), documentation = Some("Logs in user"), relationships = [rel]
When:  serialize → deserialize
Then:  reconstructed.signature == Some("fn login() -> bool")
       && reconstructed.documentation == Some("Logs in user")
       && reconstructed.relationships.len() == 1
```

Test: `fn symbol_node_round_trips_with_populated_optionals_when_all_fields_set()`

---

### 3.21 Error Types — Display Messages

#### Behavior: All ScipSymbolIdError variants produce correct Display messages

```
Given: each variant of ScipSymbolIdError constructed with sample data
When:  format!("{error}")
Then:  message matches the contract pattern exactly:
       EmptyScheme → "SCIP scheme cannot be empty"
       InvalidScheme("/") → "SCIP scheme contains invalid character: /"
       EmptyModulePath → "SCIP module path cannot be empty"
       EmptyModuleSegment(5) → "SCIP module path contains empty segment at position 5"
       LeadingSlash → "SCIP module path must not start with '/'"
       TrailingSlash → "SCIP module path must not end with '/'"
       HashInModulePath → "SCIP module path must not contain '#'"
       EmptyDescriptor → "SCIP descriptor cannot be empty"
       SlashInDescriptor → "SCIP descriptor must not contain '/'"
       InvalidFormat("bad") → "Invalid SCIP symbol format: bad"
```

Tests (one per variant):
- `fn scip_symbol_id_error_displays_empty_scheme_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_invalid_scheme_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_empty_module_path_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_empty_module_segment_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_leading_slash_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_trailing_slash_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_hash_in_module_path_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_empty_descriptor_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_slash_in_descriptor_message_when_variant_constructed()`
- `fn scip_symbol_id_error_displays_invalid_format_message_when_variant_constructed()`

#### Behavior: SymbolRoleError::UnknownBit displays bit value

```
Given: error = SymbolRoleError::UnknownBit(32)
When:  format!("{error}")
Then:  "Unknown SymbolRole bit: 32"
```

Test: `fn symbol_role_error_displays_unknown_bit_message_when_variant_constructed()`

#### Behavior: SymbolKindError::UnknownKind displays string value

```
Given: error = SymbolKindError::UnknownKind("Foo".to_string())
When:  format!("{error}")
Then:  "Unknown SymbolKind: Foo"
```

Test: `fn symbol_kind_error_displays_unknown_kind_message_when_variant_constructed()`

---

## 4. Proptest Invariants

### Proptest 1: ScipSymbolId new-then-parse round-trip

```
Invariant:  ScipSymbolId::new(scheme, module_path, descriptor) succeeds
            ⟹ ScipSymbolId::parse(id.as_str()) == ScipSymbolId::new(scheme, module_path, descriptor)
Strategy:   scheme = "[a-z]{1,10}" (no / or #)
            module_path = "[a-zA-Z0-9_]+(/[a-zA-Z0-9_]+){0,5}" (valid segments)
            descriptor = "[a-zA-Z0-9_.()]{1,30}" (no /)
Anti-invariant: scheme containing / or # → should fail
```

### Proptest 2: ScipSymbolId as_str always contains exactly one `#`

```
Invariant:  For any successfully constructed ScipSymbolId, as_str().matches('#').count() == 1
Strategy:   Same valid inputs as Proptest 1
Anti-invariant: N/A — if construction succeeds, this MUST hold
```

### Proptest 3: ScipSymbolId as_str never has empty segments (m8 fix — no unwrap)

```
Invariant:  For any successfully constructed ScipSymbolId, the module_path portion
            (between first '/' and '#') contains no empty segments.
            ⟹ as_str().split('#').next().unwrap_or("") does not contain "//"
            ⟹ module_path portion does not start with "/"
            ⟹ module_path portion does not end with "/"
Implementation: Use if-let or match on split result instead of unwrap().
            if let Some(pre_hash) = id.as_str().split('#').next() {
                assert!(!pre_hash.contains("//"));
            }
Strategy:   Same valid inputs as Proptest 1
```

### Proptest 4: SymbolRole from_bits then bits round-trip

```
Invariant:  For any u32 in [0..31], from_bits(b).unwrap().bits() == b
Strategy:   bits = 0u32..31u32
Anti-invariant: bits >= 32 → Err(UnknownBit)
```

### Proptest 5: SymbolRole union/intersection algebraic laws

```
Invariant 1 (commutativity):  a.union(b).bits() == b.union(a).bits()
Invariant 2 (identity):        a.union(SymbolRole::empty()).bits() == a.bits()
Invariant 3 (absorption):      a.intersection(a) == a
Invariant 4 (bounded):         a.intersection(b).bits() <= a.bits() && a.intersection(b).bits() <= b.bits()
Strategy:   a = 0u32..31u32, b = 0u32..31u32 (both valid)
```

### Proptest 6: SymbolRole contains is consistent with bits

```
Invariant:  For any valid role a and valid flag f:
            a.contains(f) == true ⟹ (a.bits() & f.bits()) == f.bits()
            a.contains(f) == false ⟹ (a.bits() & f.bits()) != f.bits()
Strategy:   a = 0u32..31u32, f = [1, 2, 4, 8, 16]
```

### Proptest 7: SymbolRole Display is deterministic and parseable

```
Invariant:  For any valid SymbolRole, Display output is deterministic:
            format!("{role}") is always the same string for the same bits
            AND for combined roles, "+" delimited names are sorted by bit value ascending
Strategy:   role = from_bits(0u32..31u32).unwrap()
```

### Proptest 8: SymbolNode serde round-trip preserves all fields

```
Invariant:  For any SymbolNode with arbitrary valid fields:
            serde_json::from_value::<SymbolNode>(serde_json::to_value(&node).unwrap()) == node
Strategy:   scip_id = valid ScipSymbolId
            kind = any SymbolKind variant
            roles = any valid SymbolRole (0..31)
            display_name = non-empty string
            signature = Option<String>
            documentation = Option<String>
            relationships = Vec<SymbolRelationship>
```

### Proptest 9: SymbolKind serde round-trip for all variants

```
Invariant:  For any SymbolKind variant:
            serde_json::from_str::<SymbolKind>(&serde_json::to_string(&kind).unwrap()) == kind
Strategy:   kind = any of the 12 SymbolKind variants
```

---

## 5. Fuzz Targets

### Fuzz Target 1: ScipSymbolId::parse

```
Input type:     &[u8] interpreted as &str
Risk class:     Logic error (accepting malformed input), panic on edge cases (empty, Unicode)
Corpus seeds:
  - "rust/auth/AuthService#login()"           (happy path)
  - ""                                         (empty)
  - "no_hash_here"                             (no separator)
  - "scheme/path#"                             (empty descriptor)
  - "/path#desc"                               (empty scheme)
  - "s/a//b#c"                                 (empty segment)
  - "s/path#/desc"                             (slash in descriptor)
  - "s/#desc"                                  (empty module path)
  - "s///#"                                    (all empty segments)
  - "rust/a/b/c/d/e/f/g/h#desc"               (deep path)
  - "x"                                        (single char, no hash)
  - "#"                                        (just hash)
  - "scheme with spaces/path#desc"             (spaces in scheme)
  - "\t\n"                                     (whitespace)
  - "日本語/パス#記述子"                         (Unicode)
  - "noslash#desc"                             (no slash before hash — m7 fix)
  - "rust/auth#desc#extra"                     (hash in descriptor — m9 fix)
```

### Fuzz Target 2: serde_json deserialize ScipSymbolId

```
Input type:     &[u8] interpreted as JSON string
Risk class:     Logic error (accepting malformed SCIP strings via parse), panic on invalid UTF-8 in JSON
Corpus seeds:
  - "\"rust/auth/AuthService#login()\""        (valid)
  - "\"\""                                      (empty string)
  - "\"no_hash\""                               (no separator)
  - "123"                                       (not a string)
  - "null"                                      (null)
  - "\"scheme/path#desc\""                      (valid)
  - "\"a/b#c\""                                 (minimal valid)
```

### Fuzz Target 3: serde_json deserialize SymbolRole

```
Input type:     &[u8] interpreted as JSON number
Risk class:     Logic error (accepting out-of-range bits), panic on non-number
Corpus seeds:
  - "0"                                        (empty)
  - "1"                                        (DEFINITION)
  - "31"                                       (ALL)
  - "32"                                       (unknown bit)
  - "2147483647"                               (large valid-ish u32)
  - "4294967295"                               (u32::MAX)
  - "\"definition\""                            (string, not u32)
  - "null"                                     (null)
  - "-1"                                       (negative)
```

### Fuzz Target 4: serde_json deserialize SymbolKind

```
Input type:     &[u8] interpreted as JSON string
Risk class:     Logic error (accepting invalid variant names), panic on non-string
Corpus seeds:
  - "\"struct\""                                (valid)
  - "\"Struct\""                                (case sensitivity test — INV-6)
  - "\"FUNCTION\""                              (uppercase — INV-6)
  - "\"unknown\""                               (invalid)
  - ""                                          (empty)
  - "123"                                       (not a string)
  - "null"                                      (null)
  - "\"\""                                      (empty string)
```

---

## 6. Kani Harnesses

### Kani Harness 1: ScipSymbolId::new never panics

```
Property:     ScipSymbolId::new(scheme, module_path, descriptor) returns either Ok or Err,
              never panics, for any &str inputs
Bound:        scheme.len() <= 256, module_path.len() <= 1024, descriptor.len() <= 256
Rationale:    This is a public API boundary accepting arbitrary strings. Formal verification
              proves the validation logic is exhaustive — no code path can panic regardless of
              input content (including empty strings, embedded nulls, Unicode, etc.)
Harness:      fn harness() {
                  let scheme: String = kani::any();
                  let module_path: String = kani::any();
                  let descriptor: String = kani::any();
                  let _ = ScipSymbolId::new(scheme, module_path, descriptor);
              }
```

### Kani Harness 2: SymbolRole::from_bits never panics

```
Property:     SymbolRole::from_bits(bits) returns either Ok or Err for any u32 input
Bound:        bits: u32 (full range, 0..2^32)
Rationale:    from_bits is the strict constructor for a bitmask type. It must handle the full
              u32 range without panicking. Formal proof is stronger than exhaustive enumeration.
Harness:      fn harness() {
                  let bits: u32 = kani::any();
                  let _ = SymbolRole::from_bits(bits);
              }
```

### Kani Harness 3: SymbolRole::from_bits_truncate result is always valid

```
Property:     For any u32 input, from_bits_truncate(bits) produces a SymbolRole whose bits
              are a subset of the defined mask (bits & 0x1F == bits)
Bound:        bits: u32 (full range)
Rationale:    from_bits_truncate is the lossy constructor. The contract says it silently
              ignores unknown bits. Kani proves no unknown bit survives truncation.
Harness:      fn harness() {
                  let bits: u32 = kani::any();
                  let role = SymbolRole::from_bits_truncate(bits);
                  assert!(role.bits() & !0x1Fu32 == 0);
              }
```

---

## 7. Mutation Testing Checkpoints

### Critical Mutations and Their Killers

| Mutation | Killer Test |
|----------|-------------|
| Remove `scheme.is_empty()` check in `new()` | `scip_symbol_id_returns_empty_scheme_error_when_scheme_is_empty()` |
| Remove `scheme.contains('/')` check | `scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_slash()` |
| Remove `scheme.contains('#')` check | `scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_hash()` |
| Remove `module_path.is_empty()` check | `scip_symbol_id_returns_empty_module_path_error_when_module_path_is_empty()` |
| Remove empty segment detection (`//`) | `scip_symbol_id_returns_empty_module_segment_error_when_path_has_double_slash()` |
| Remove leading `/` check | `scip_symbol_id_returns_leading_slash_error_when_module_path_starts_with_slash()` |
| Remove trailing `/` check | `scip_symbol_id_returns_trailing_slash_error_when_module_path_ends_with_slash()` |
| Remove `module_path.contains('#')` check | `scip_symbol_id_returns_hash_in_module_path_error_when_path_contains_hash()` |
| Remove `descriptor.is_empty()` check | `scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_empty()` |
| Remove `descriptor.contains('/')` check | `scip_symbol_id_returns_slash_in_descriptor_error_when_descriptor_contains_slash()` |
| Remove `#` check in descriptor (m9) | `scip_symbol_id_rejects_hash_in_descriptor_when_descriptor_contains_hash()` |
| Remove `#` split in `parse()` | `scip_symbol_id_parse_returns_invalid_format_error_when_input_has_no_hash()` |
| Remove `/` check in `parse()` (m7) | `scip_symbol_id_parse_returns_invalid_format_error_when_input_has_no_slash()` |
| Change `from_bits` to always return Ok | `symbol_role_from_bits_returns_unknown_bit_error_when_bits_exceed_mask()` |
| Change `from_bits_truncate` to return Err instead of truncating | `symbol_role_from_bits_truncate_drops_unknown_bits_when_bits_exceed_mask()` |
| Change Display to not sort roles | `symbol_role_display_outputs_sorted_plus_delimited_names_when_multiple_roles()` |
| Change Display zero-role from "none" to "" | `symbol_role_display_outputs_none_when_no_roles_set()` |
| Remove `#[serde(rename_all = "lowercase")]` from SymbolKind | `symbol_kind_serializes_as_lowercase_string_when_all_variants_tested()` |
| Change SymbolRole Serialize from u32 to string | `symbol_role_round_trips_through_json_as_u32_when_serialized()` |
| Swap equality comparison order in SymbolNode | `symbol_node_equality_compares_all_fields_when_compared()` |
| Change `contains` to always return true | `symbol_role_contains_returns_false_when_flag_is_not_set()` |
| Change `is_empty` to always return true | `symbol_role_is_empty_returns_false_when_any_flag_is_set()` |
| Change `bits()` to always return 0 for non-constants (m3) | `symbol_role_bits_returns_sum_of_flag_bits_when_role_is_combined()` |
| Change SymbolNode Hash to only hash `scip_id` (M1) | `symbol_node_hash_eq_consistency_when_nodes_differ_in_non_id_fields()` |
| Change SymbolRelationship Hash to only hash `target` (M2) | `symbol_relationship_hash_eq_consistency_when_same_target_different_kind()` |
| Remove case-insensitive deserialization from SymbolKind (M3) | `symbol_kind_deserializes_case_insensitively_when_input_has_mixed_case()` |
| Change `trim()` to `trim_start()` in scheme validation (M4) | `scip_symbol_id_returns_empty_scheme_error_when_scheme_is_tab_only()` |
| Change `trim()` to `trim_start()` in module_path validation (M4) | `scip_symbol_id_returns_empty_module_path_error_when_module_path_is_newline_only()` |
| Change `trim()` to `trim_start()` in descriptor validation (M4) | `scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_mixed_whitespace()` |

### Threshold

**Minimum mutation kill rate: 90%**

Run with: `cargo mutants --test-threads 1 -- --test-threads 1`

All 28 critical mutations listed above MUST be killed. Any surviving mutation indicates a missing test.

---

## 8. Combinatorial Coverage Matrix

### 8.1 ScipSymbolId::new — Scheme Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid scheme | `"rust"` | Ok(canonical string) | unit |
| empty scheme | `""` | Err(EmptyScheme) | unit |
| whitespace-only scheme (spaces) | `"   "` | Err(EmptyScheme) | unit |
| whitespace-only scheme (tab) | `"\t"` | Err(EmptyScheme) | unit |
| scheme with `/` | `"ru/st"` | Err(InvalidScheme("/")) | unit |
| scheme with `#` | `"ru#st"` | Err(InvalidScheme("#")) | unit |
| scheme with both | `"ru/st#nd"` | Err(InvalidScheme(_)) | unit |
| scheme with unicode | `"日本語"` | Ok(canonical with unicode) | unit |
| scheme with hyphens | `"c-sharp"` | Ok(canonical) | unit |

### 8.2 ScipSymbolId::new — Module Path Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid single segment | `"auth"` | Ok(canonical) | unit |
| valid multi-segment | `"auth/AuthService"` | Ok(canonical) | unit |
| empty path | `""` | Err(EmptyModulePath) | unit |
| whitespace-only (spaces) | `"  "` | Err(EmptyModulePath) | unit |
| whitespace-only (newline) | `"\n"` | Err(EmptyModulePath) | unit |
| double slash | `"auth//service"` | Err(EmptyModuleSegment(5)) | unit |
| leading slash | `"/auth"` | Err(LeadingSlash) | unit |
| trailing slash | `"auth/"` | Err(TrailingSlash) | unit |
| contains `#` | `"auth#mod"` | Err(HashInModulePath) | unit |
| triple slash | `"a///b"` | Err(EmptyModuleSegment(2)) | unit |
| only slashes | `"///"` | Err(LeadingSlash) | unit |

### 8.3 ScipSymbolId::new — Descriptor Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid descriptor | `"login()"` | Ok(canonical) | unit |
| with dots | `"Struct.method"` | Ok(canonical) | unit |
| with parens | `"fn(x) -> T"` | Ok(canonical) | unit |
| empty | `""` | Err(EmptyDescriptor) | unit |
| whitespace-only (spaces) | `"  "` | Err(EmptyDescriptor) | unit |
| whitespace-only (mixed) | `"\t \n"` | Err(EmptyDescriptor) | unit |
| contains `/` | `"a/b"` | Err(SlashInDescriptor) | unit |
| contains `#` | `"login#extra"` | Err(implementation-specific) | unit |
| unicode descriptor | `"日本語メソッド"` | Ok(canonical) | unit |

### 8.4 ScipSymbolId::parse — Format Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid format | `"rust/auth#f"` | Ok(canonical) | unit |
| no hash | `"rust/auth/f"` | Err(InvalidFormat("rust/auth/f")) | unit |
| empty string | `""` | Err(InvalidFormat("")) | unit |
| just hash | `"#"` | Err(InvalidFormat("#")) | unit |
| empty scheme | `"/auth#f"` | Err(InvalidFormat("/auth#f")) | unit |
| empty descriptor | `"rust/auth#"` | Err(InvalidFormat("rust/auth#")) | unit |
| no slash before hash | `"noslash#desc"` | Err(InvalidFormat("noslash#desc")) | unit |
| multiple hashes | `"ru/auth#desc#extra"` | Err(implementation-specific) | unit |
| whitespace in input | `"rust /auth#f"` | Err(InvalidScheme(_)) | unit |

### 8.5 SymbolRole — from_bits

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| 0 | zero | Ok(empty role) | unit |
| 1 | single flag | Ok(DEFINITION) | unit |
| 2 | single flag | Ok(READ) | unit |
| 4 | single flag | Ok(WRITE) | unit |
| 8 | single flag | Ok(GENERATED) | unit |
| 16 | single flag | Ok(TEST) | unit |
| 3 | combination | Ok(DEFINITION \| READ) | unit |
| 31 | ALL | Ok(ALL) | unit |
| 32 | bit 5 (unknown) | Err(UnknownBit(32)) | unit |
| 33 | 1 \| 32 | Err(UnknownBit(33)) | unit |
| u32::MAX | all bits set | Err(UnknownBit(u32::MAX)) | unit |

### 8.6 SymbolRole — from_bits_truncate

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| 0 | zero | empty (bits=0) | unit |
| 31 | ALL | ALL (bits=31) | unit |
| 33 | 1 \| 32 | DEFINITION (bits=1) | unit |
| 64 | all unknown | empty (bits=0) | unit |
| 255 | mix | bits=31 (ALL) | unit |
| u32::MAX | all bits set | bits=31 (ALL) | unit |

### 8.7 SymbolRole — Display

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| empty (0) | zero role | `"none"` | unit |
| single flag | DEFINITION | `"definition"` | unit |
| two flags | DEFINITION \| READ | `"definition+read"` | unit |
| all flags | ALL | `"definition+read+write+generated+test"` | unit |
| reverse order | TEST \| DEFINITION | `"definition+test"` (sorted) | unit |

### 8.8 SymbolKind — Serialize/Deserialize

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| struct variant | `SymbolKind::Struct` | `"struct"` → `Struct` | integration |
| function variant | `SymbolKind::Function` | `"function"` → `Function` | integration |
| trait variant | `SymbolKind::Trait` | `"trait"` → `Trait` | integration |
| enum variant | `SymbolKind::Enum` | `"enum"` → `Enum` | integration |
| module variant | `SymbolKind::Module` | `"module"` → `Module` | integration |
| constant variant | `SymbolKind::Constant` | `"constant"` → `Constant` | integration |
| method variant | `SymbolKind::Method` | `"method"` → `Method` | integration |
| field variant | `SymbolKind::Field` | `"field"` → `Field` | integration |
| interface variant | `SymbolKind::Interface` | `"interface"` → `Interface` | integration |
| typealias variant | `SymbolKind::TypeAlias` | `"type_alias"` → `TypeAlias` | integration |
| variable variant | `SymbolKind::Variable` | `"variable"` → `Variable` | integration |
| package variant | `SymbolKind::Package` | `"package"` → `Package` | integration |
| unknown string | `"FooBar"` | Err(UnknownKind("FooBar")) | integration |
| empty string | `""` | Err(UnknownKind("")) | integration |
| uppercase (INV-6) | `"Struct"` | Ok(Struct) | integration |
| mixed case (INV-6) | `"Type_Alias"` | Ok(TypeAlias) | integration |

### 8.9 RelationshipKind — Serialize/Deserialize

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| imports | `RelationshipKind::Imports` | `"imports"` → `Imports` | integration |
| contains | `RelationshipKind::Contains` | `"contains"` → `Contains` | integration |
| implements | `RelationshipKind::Implements` | `"implements"` → `Implements` | integration |
| calls | `RelationshipKind::Calls` | `"calls"` → `Calls` | integration |
| references | `RelationshipKind::References` | `"references"` → `References` | integration |
| overrides | `RelationshipKind::Overrides` | `"overrides"` → `Overrides` | integration |
| unknown string | `"FooBar"` | serde::Error | integration |

### 8.10 SymbolNode — Round-Trip Matrix

| Scenario | signature | documentation | relationships | Layer |
|----------|-----------|---------------|----------------|-------|
| all Some, non-empty | `Some("sig")` | `Some("doc")` | `[rel]` | integration |
| all None, empty | `None` | `None` | `[]` | integration |
| mixed | `Some("sig")` | `None` | `[]` | integration |
| mixed 2 | `None` | `Some("doc")` | `[rel]` | integration |
| multiple rels | `Some("sig")` | `Some("doc")` | `[rel1, rel2]` | integration |

---

## Static Analysis Checks

| Check | Tool | Rationale |
|-------|------|-----------|
| `#![deny(clippy::unwrap_used)]` | clippy | Enforced at crate level — INV-8 |
| `#![deny(clippy::expect_used)]` | clippy | Enforced at crate level — INV-8 |
| `#![deny(clippy::panic)]` | clippy | Belt-and-suspenders on INV-8 |
| No dead code warnings on public API | rustc | All public types must be reachable |
| Derive macros compile correctly | rustc | Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize on all types |

---

## Open Questions

1. **INV-6 case-insensitive deserialization**: ~~The contract states `SymbolKind` should accept `"Struct"` or `"struct"`. The `#[serde(rename_all = "lowercase")]` attribute handles serialization but deserialization may need a custom deserializer or `#[serde(alias)]` annotations. Confirm whether case-insensitive deserialization is in scope for this bead or deferred.~~ **RESOLVED**: INV-6 is a contract invariant. BDD scenario added (section 3.17). Implementation must use `#[serde(alias = "...")]` annotations or a custom deserializer.

2. **EmptyModuleSegment position calculation**: The contract specifies `EmptyModuleSegment(pos: usize)` with position. Confirm whether `pos` is the byte offset or the segment index (0-based) in the module path. The contract example `auth//service` → position 5 suggests byte offset.

3. **ScipSymbolId::parse with multiple `#`**: The contract's `InvalidFormat` is for inputs not matching `<scheme>/<path>#<descriptor>`. If input is `"rust/auth#desc#extra"`, should this return `InvalidFormat` or `SlashInDescriptor` (since the second `#` is in the descriptor portion after split)? The answer determines parse's split strategy (split once vs. split on first `#`).

4. **SymbolNode with same relationship target twice**: The contract says no deduplication invariant. Tests should not assert uniqueness of `relationships` Vec entries. Confirmed by [PRE-N4] and contract decision.

5. **`#` in descriptor for `new()`** (m9): The contract PRE-S5 says descriptor "Must not contain `/`" but does not explicitly forbid `#`. However, INV-1 requires exactly one `#` in the canonical string. The test plan includes a scenario (section 3.4) that verifies `#` in descriptor is rejected. The exact error variant is implementation-dependent — the test asserts rejection, not a specific variant.
