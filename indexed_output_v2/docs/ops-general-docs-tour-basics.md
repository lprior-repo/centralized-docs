---
id: ops/general/docs-tour-basics
title: Docs Tour Basics
category: ops
tags: ["ops", "superset"]
---

# JSON Superset | CUE

> **Context**: **Source:** https://cuelang.org/docs/tour/basics/


**Source:** https://cuelang.org/docs/tour/basics/

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
 2. The Basics of CUE [https://cuelang.org/docs/tour/basics/]


 3. JSON SUPERSET

In its simplest form, CUE looks a lot like JSON.
This is because CUE is a superset of JSON.
Or, put differently: all valid JSON is CUE (but not vice versa).

CUE significantly reduces the pain of dealing with JSON
by introducing several conveniences, including:

 * C-style comments are allowed
 * field names without special characters don’t need to be quoted
 * commas after a field are optional (and are usually omitted)
 * commas after the final element of a list are allowed
 * the outermost curly braces in a CUE file are optional

JSON objects are called structs or maps in CUE.
JSON arrays are called lists
Object members are called fields, which link their name, or label, to a value.

Throughout this tour there are examples that show
some CUE,
a command that processes it,
and then the command’s output
- with each pane featuring a mouseover button that copies the related text to
your clipboard.
Here’s an example that uses cue export to turn file.cue into JSON.


Copied!
file.cue

Copy code
Copied!

import "math"

// Simple labels don't need to be quoted.
one:       1
two:       2
piPlusOne: math.Pi + 1

// Field names must be quoted if they contain
// special characters, such as hyphen and space.
"quoted field names": {
	"two-and-a-half":    2.5
	"three point three": 3.3
	"four^four":         math.Pow(4, 4)
}

aList: [
	1,
	2,
	3,
]

TERMINAL

Copy code
Copied!

$ cue export file.cue --out json
{
    "one": 1,
    "two": 2,
    "piPlusOne": 4.141592653589793238462643383279503,
    "quoted field names": {
        "two-and-a-half": 2.5,
        "three point three": 3.3,
        "four^four": 256
    },
    "aList": [
        1,
        2,
        3
    ]
}

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tour/basics/json-superset/&text=In%20its%20simplest%20form,%20CUE%20looks%20a%20lot%20like%20JSON.%20This%20is%20because%20CUE%20is%20a%20superset%20of%20JSON.%0aOr,%20put%20differently:%20all%20valid%20JSON%20is%20CUE%20%28but%20not%20vice%20versa%29.%0aCUE%20significantly%20reduces%20the%20pain%20of%20dealing%20with%20JSON%20by%20introducing%20several%20conveniences,%20including:%0aC-style%20comments%20are%20allowed%20field%20names%20without%20special%20characters%20don&rsquo;t%20need%20to%20be%20quoted%20commas%20after%20a%20field%20are%20optional%20%28and%20are%20usually%20omitted%29%20commas%20after%20the%20final%20element%20of%20a%20list%20are%20allowed%20the%20outermost%20curly%20braces%20in%20a%20CUE%20file%20are%20optional%20JSON%20objects%20are%20called%20structs%20or%20maps%20in%20CUE.%20JSON%20arrays%20are%20called%20lists%0aObject%20members%20are%20called%20fields,%20which%20link%20their%20name,%20or%20label,%20to%20a%20value.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tour/basics/json-superset/&summary=In%20its%20simplest%20form,%20CUE%20looks%20a%20lot%20like%20JSON.%20This%20is%20because%20CUE%20is%20a%20superset%20of%20JSON.%0aOr,%20put%20differently:%20all%20valid%20JSON%20is%20CUE%20%28but%20not%20vice%20versa%29.%0aCUE%20significantly%20reduces%20the%20pain%20of%20dealing%20with%20JSON%20by%20introducing%20several%20conveniences,%20including:%0aC-style%20comments%20are%20allowed%20field%20names%20without%20special%20characters%20don&rsquo;t%20need%20to%20be%20quoted%20commas%20after%20a%20field%20are%20optional%20%28and%20are%20usually%20omitted%29%20commas%20after%20the%20final%20element%20of%20a%20list%20are%20allowed%20the%20outermost%20curly%20braces%20in%20a%20CUE%20file%20are%20optional%20JSON%20objects%20are%20called%20structs%20or%20maps%20in%20CUE.%20JSON%20arrays%20are%20called%20lists%0aObject%20members%20are%20called%20fields,%20which%20link%20their%20name,%20or%20label,%20to%20a%20value.%0a]


Previous
Types are Values
[/docs/tour/basics/types-are-values/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
   * The Basics of CUE [/docs/tour/basics/]
     * JSON Superset [/docs/tour/basics/json-superset/]
       
     * Types are Values [/docs/tour/basics/types-are-values/]
     * Unification [/docs/tour/basics/unification/]
     * Constraints [/docs/tour/basics/constraints/]
     * Definitions [/docs/tour/basics/definitions/]
     * Validation [/docs/tour/basics/validation/]
     * Order is Irrelevant [/docs/tour/basics/order-irrelevance/]
     * Concise Specifications [/docs/tour/basics/folding-structs/]
     * Next: Types and Values [/docs/tour/basics/next/]
   * Types and Values [/docs/tour/types/]
   * References and Visibility [/docs/tour/references/]
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Ftour%2Fbasics%2Fjson-superset%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
## See Also

- [Documentation Index](./COMPASS.md)
