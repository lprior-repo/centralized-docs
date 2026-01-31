---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: rule [https://research. com/vgo-import]:
---

rule [https://research.swtch.com/vgo-import]:

> If an old package and a new package have the same import path,
> the new package must be backwards compatible with the old package.

By definition, packages in a new major version of a module are not backwards
compatible with the corresponding packages in the previous major version.
Consequently each new major version of a package needs a new import path.
This is accomplished by adding a major version suffix to the module path.
The import path for a package also includes the major version suffix,
