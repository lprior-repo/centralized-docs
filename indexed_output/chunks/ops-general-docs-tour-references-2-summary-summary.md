---
doc_id: ops/general/docs-tour-references
chunk_id: ops/general/docs-tour-references#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 260
summary: 	B:   val // Matches the inner val. 	C: val // Matches the top-level val
---


Copied!
file.cue

Copy code
Copied!

val: 1

A: {
	val: 2
	B:   val // Matches the inner val
}

A: {
	C: val // Matches the top-level val
}

B: val // Matches the top-level val

TERMINAL

Copy code
Copied!

$ cue eval file.cue
val: 1
A: {
    val: 2
    B:   2
    C:   1
}
B: 1

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tour/references/scopes/&text=A%20reference%20refers%20to%20the%20value%20of%20the%20field%20defined%20within%20the%20nearest%20enclosing%20scope.%0aIf%20a%20reference%20doesn&rsquo;t%20match%20a%20field%20within%20the%20same%20file,%20then%20it%20may%20match%20a%20top-level%20field%20defined%20in%20any%20other%20file%20making%20up%20the%20same%20CUE%20package.%0aIf%20there%20is%20still%20no%20match%20then%20it%20may%20match%20a%20predefined%20value,%20such%20as%20a%20predefined%20bound.%0a]
