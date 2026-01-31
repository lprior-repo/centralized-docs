---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#88-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary:     declared at top level (outside any struct literal) is the package block.  The scope of an identifier denoting an alias
---

    declared at top level (outside any struct literal) is the package block.
 3. The scope of an identifier denoting an alias
    declared at top level (outside any struct literal) is the file block.
 4. The scope of a let identifier
    declared at top level (outside any struct literal) is the file block.
 5. The scope of the package name of an imported package is the file block of the
    file containing the import declaration.
 6. The scope of a field, alias or let identifier declared inside a struct
    literal is the innermost containing block.
