---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#92-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: 797693134862315708145274237317043567981e+308. EXPORTED IDENTIFIERS
---

          <=1.797693134862315708145274237317043567981e+308

EXPORTED IDENTIFIERS

An identifier of a package may be exported to permit access to it
from another package.
All identifiers not starting with _ (so all regular fields and definitions
starting with #) are exported.
Any identifier starting with _ is not visible outside the package and resides
in a separate namespace than namesake identifiers of other packages.


Copy code
Copied!

package mypackage

foo:   string  // visible outside mypackage
"bar": string  // visible outside mypackage
