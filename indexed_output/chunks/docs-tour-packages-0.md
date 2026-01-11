---
doc_id: docs-tour-packages
chunk_id: docs-tour-packages#0
heading: Introduction
token_count: 1464
summary: # Packages | CUE. **Source:** https://cuelang
---

# Packages | CUE

**Source:** https://cuelang.org/docs/tour/packages/

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
 2. Packages and Imports [https://cuelang.org/docs/tour/packages/]


 3. PACKAGES

By default, each CUE file is a standalone file.

A configuration can be split across multiple files by adding a package clause
to each file.

The configuration for each package is defined by the concatenation of all its files,
after stripping their package clauses and merging their import statements.
Multiple definitions of the same field across files and within the same file
are treated similarly, because
order does not matter [/docs/tour/basics/order-irrelevance/].

Copied!
policy.cue

Copy code
Copied!

package config

foo:  bar/2 - 1
bar!: int

Copied!
data.cue

Copy code
Copied!

package config

bar: 200

TERMINAL

Copy code
Copied!

$ cue export # No filenames mentioned
{
    "foo": 99,
    "bar": 200
}

The cue tool processes lists of CUE files and package paths.
Because working with a single package split across multiple files in the
current directory is such a common situation,
cue processes that single package if it isn’t told to look at anything else.

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tour/packages/packages/&text=By%20default,%20each%20CUE%20file%20is%20a%20standalone%20file.%0aA%20configuration%20can%20be%20split%20across%20multiple%20files%20by%20adding%20a%20package%20clause%20to%20each%20file.%0aThe%20configuration%20for%20each%20package%20is%20defined%20by%20the%20concatenation%20of%20all%20its%20files,%20after%20stripping%20their%20package%20clauses%20and%20merging%20their%20import%20statements.%20Multiple%20definitions%20of%20the%20same%20field%20across%20files%20and%20within%20the%20same%20file%20are%20treated%20similarly,%20because%20order%20does%20not%20matter.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tour/packages/packages/&summary=By%20default,%20each%20CUE%20file%20is%20a%20standalone%20file.%0aA%20configuration%20can%20be%20split%20across%20multiple%20files%20by%20adding%20a%20package%20clause%20to%20each%20file.%0aThe%20configuration%20for%20each%20package%20is%20defined%20by%20the%20concatenation%20of%20all%20its%20files,%20after%20stripping%20their%20package%20clauses%20and%20merging%20their%20import%20statements.%20Multiple%20definitions%20of%20the%20same%20field%20across%20files%20and%20within%20the%20same%20file%20are%20treated%20similarly,%20because%20order%20does%20not%20matter.%0a]


Previous
Imports
[/docs/tour/packages/imports/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
   * The Basics of CUE [/docs/tour/basics/]
   * Types and Values [/docs/tour/types/]
   * References and Visibility [/docs/tour/references/]
   * Expressions [/docs/tour/expressions/]
   * Packages and Imports [/docs/tour/packages/]
     * Packages [/docs/tour/packages/packages/]
       
     * Imports [/docs/tour/packages/imports/]
     * Standard Library [/docs/tour/packages/standard-library/]
     * Next: Explore CUE! [/docs/tour/packages/next/]
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Ftour%2Fpackages%2Fpackages%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
