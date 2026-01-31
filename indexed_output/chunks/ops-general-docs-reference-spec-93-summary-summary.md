---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#93-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: #Foo: {      // visible outside mypackage.     a:  1    // visible outside mypackage
---


#Foo: {      // visible outside mypackage
    a:  1    // visible outside mypackage
    _b: 2    // not visible outside mypackage

    #C: {    // visible outside mypackage
        d: 4 // visible outside mypackage
    }
    _#E: foo // not visible outside mypackage
}

UNIQUENESS OF IDENTIFIERS

Given a set of identifiers, an identifier is called unique if it is different
from every other in the set, after applying normalization following
Unicode Annex #31 [https://unicode.org/reports/tr31/].
Two identifiers are different if they are spelled differently
