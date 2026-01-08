---
doc_id: docs-tour-references
chunk_id: docs-tour-references#0
heading: Introduction
token_count: 1398
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

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tour/references/scopes/&summary=A%20reference%20refers%20to%20the%20value%20of%20the%20field%20defined%20within%20the%20nearest%20enclosing%20scope.%0aIf%20a%20reference%20doesn&rsquo;t%20match%20a%20field%20within%20the%20same%20file,%20then%20it%20may%20match%20a%20top-level%20field%20defined%20in%20any%20other%20file%20making%20up%20the%20same%20CUE%20package.%0aIf%20there%20is%20still%20no%20match%20then%20it%20may%20match%20a%20predefined%20value,%20such%20as%20a%20predefined%20bound.%0a]


Previous
Accessing Fields
[/docs/tour/references/selectors/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
   * The Basics of CUE [/docs/tour/basics/]
   * Types and Values [/docs/tour/types/]
   * References and Visibility [/docs/tour/references/]
     * References and Scopes [/docs/tour/references/scopes/]
       
     * Accessing Fields [/docs/tour/references/selectors/]
     * Aliases [/docs/tour/references/aliases/]
     * Emitting Values [/docs/tour/references/emit/]
     * Reference Cycles [/docs/tour/references/cycle/]
     * Cycles in Fields [/docs/tour/references/cycleref/]
     * Hidden Fields [/docs/tour/references/hidden/]
     * Next: Expressions [/docs/tour/references/next/]
   * Expressions [/docs/tour/expressions/]
   * Packages and Imports [/docs/tour/packages/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Ftour%2Freferences%2Fscopes%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

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
