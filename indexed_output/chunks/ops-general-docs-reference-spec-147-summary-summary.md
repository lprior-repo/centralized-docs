---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#147-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: This table illustrates how Sin is accessed in files. that import the package after the various types of import declaration
---

This table illustrates how Sin is accessed in files
that import the package after the various types of import declaration.


Copy code
Copied!

Import declaration          Local name of Sin

import   "lib/math"         math.Sin
import   "lib/math:math"    math.Sin
import m "lib/math"         m.Sin

An import declaration declares a dependency relation between the importing and
imported package. It is illegal for a package to import itself, directly or
indirectly, or to directly import a package without referring to any of its
