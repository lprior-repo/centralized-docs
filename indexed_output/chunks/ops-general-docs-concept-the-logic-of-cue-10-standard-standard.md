---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#10-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 518
summary: Most of the inspiration for the underlying work. presented here comes from the Lingo and LKB project
---

Most of the inspiration for the underlying work
presented here comes from the Lingo and LKB project.
One can read more about this in Ann Copestake’s
“Implementing Typed Feature Structure Grammars.”
(2002, CSLI Publications, ISBN 1-57586-261-1).

FOOTNOTES

----------------------------------------

 1. Although CUE could be used to verify those properties in such data-only configurations. ↩︎

 2. TFSs typically don’t have default values, it is the structure itself that
    is boilerplate removing, as the structure itself is what is the useful value.
    But that is a different topic. It doesn’t work quite as well if one needs
    numeric values. This is why CUE adds defaults. ↩︎

 3. Detection of structural cycles (an occurs check) is not yet implemented,
    and thus printing infinite structures will still result in a loop. ↩︎

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/the-logic-of-cue/&text=This%20page%20explains%20the%20core%20concept%20on%20which%20pretty%20much%20everything%20that%20is%20CUE%20depends.%20It%20helps%20to%20get%20a%20top-down%20understanding%20and%20frame%20of%20reference,%20but%20it%20is%20not%20necessary%20for%20learning%20the%20language.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/the-logic-of-cue/&summary=This%20page%20explains%20the%20core%20concept%20on%20which%20pretty%20much%20everything%20that%20is%20CUE%20depends.%20It%20helps%20to%20get%20a%20top-down%20understanding%20and%20frame%20of%20reference,%20but%20it%20is%20not%20necessary%20for%20learning%20the%20language.%0a]


Popular guides
[/docs/concept/popular-guides/]Alias and reference scopes
[/docs/concept/alias-and-reference-scopes/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
