---
doc_id: docs-howto-about-commented-cue-guides
chunk_id: docs-howto-about-commented-cue-guides#0
heading: Introduction
token_count: 1650
summary: # Commented CUE Guides | CUE. **Source:** https://cuelang
---

# Commented CUE Guides | CUE

**Source:** https://cuelang.org/docs/howto/about-commented-cue-guides/

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

 1. How-to Guides [https://cuelang.org/docs/howto/]


 2. COMMENTED CUE GUIDES

Some how-to guides use CUE’s useful style of
“literate [https://en.wikipedia.org/wiki/Literate_programming] configuration”.
We refer to them as commented cue
[/search?q=tag:%22commented%20cue%22]guides.

Commented CUE guides tend to be short, presenting commented CUE code alongside
some input data, with a cue command showing the concrete result of the
technique they’re demonstrating. Often that result is an error message, showing
that cue has caught the deliberate mistakes in our example data.

Here’s an example:

Copied!
example.cue

Copy code
Copied!

package example

// CUE comments start with "//"
// and run to the end of the line

// f1 is a regular field which must be a string
f1: string

// f2 is a required field which must be an
// integer over 10
f2!: int & >10

Copied!
data.yml

Copy code
Copied!

# f1 is actually an integer
f1: 123

# f2 is actually a string
f2: "some string value"

TERMINAL

Copy code
Copied!

$ cue vet -c .:example data.yml
f1: conflicting values 123 and string (mismatched types int and string):
    ./data.yml:2:5
    ./example.cue:7:5
f2: conflicting values "some string value" and int (mismatched types string and int):
    ./data.yml:5:5
    ./example.cue:11:6

Commented CUE guides require you, the reader, to modify their examples
before you use them. Don’t use the CUE exactly as it’s presented, but first
adapt the variable names and data structures to suit your configuration.

Unlike some of our longer, step-by-step guides, the features showcased in
Commented CUE guides are self-contained and – we believe! – easy to adapt to
any situation. Each guide has a “Related content” section containing links to
in-depth explanation and discussion of the techniques and language features
used.

If you find any guides confusing, incomplete, or not detailed enough, then we
would really appreciate you telling us about your experience - just use the
“Report an Issue” link at the very bottom of the guide’s page!

Last modified July 9, 2025 [https://github.com/cue-lang/cuelang.org/commit/f899c33690510f00d401b24e59f60369f5d550d1]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/howto/about-commented-cue-guides/&text=Some%20how-to%20guides%20use%20CUE&rsquo;s%20useful%20style%20of%20&ldquo;literate%20configuration&rdquo;.%20We%20refer%20to%20them%20as%20commented%20cue%20guides.%0aCommented%20CUE%20guides%20tend%20to%20be%20short,%20presenting%20commented%20CUE%20code%20alongside%20some%20input%20data,%20with%20a%20cue%20command%20showing%20the%20concrete%20result%20of%20the%20technique%20they&rsquo;re%20demonstrating.%20Often%20that%20result%20is%20an%20error%20message,%20showing%20that%20cue%20has%20caught%20the%20deliberate%20mistakes%20in%20our%20example%20data.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/howto/about-commented-cue-guides/&summary=Some%20how-to%20guides%20use%20CUE&rsquo;s%20useful%20style%20of%20&ldquo;literate%20configuration&rdquo;.%20We%20refer%20to%20them%20as%20commented%20cue%20guides.%0aCommented%20CUE%20guides%20tend%20to%20be%20short,%20presenting%20commented%20CUE%20code%20alongside%20some%20input%20data,%20with%20a%20cue%20command%20showing%20the%20concrete%20result%20of%20the%20technique%20they&rsquo;re%20demonstrating.%20Often%20that%20result%20is%20an%20error%20message,%20showing%20that%20cue%20has%20caught%20the%20deliberate%20mistakes%20in%20our%20example%20data.%0a]


Popular guides
[/docs/howto/popular-guides/]Encoding JSON or YAML with CUE
[/docs/howto/encode-json-yaml-with-cue/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
   * Popular guides [/docs/howto/popular-guides/]
   * Commented CUE Guides [/docs/howto/about-commented-cue-guides/]
     
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fhowto%2Fabout-commented-cue-guides%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
