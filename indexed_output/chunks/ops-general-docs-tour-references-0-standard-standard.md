---
doc_id: ops/general/docs-tour-references
chunk_id: ops/general/docs-tour-references#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 526
summary: # References and Scopes | CUE. **Source:** https://cuelang
---

# References and Scopes | CUE

**Source:** https://cuelang.org/docs/tour/references/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. Tour [https://cuelang.org/docs/tour/]
 2. References and Visibility [https://cuelang.org/docs/tour/references/]


 3. REFERENCES AND SCOPES

A reference refers to the value of the field defined within the nearest
enclosing scope.

If a reference doesn’t match a field within the same file,
then it may match a top-level field defined in any other file making up the
same CUE package.

If there is still no match then it may match a predefined value, such as a
predefined bound [/docs/tour/types/bounddef/].

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
