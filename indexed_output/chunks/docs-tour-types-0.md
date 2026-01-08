---
doc_id: docs-tour-types
chunk_id: docs-tour-types#0
heading: Introduction
token_count: 2098
summary: # Type Hierarchy | CUE. **Source:** https://cuelang
---

# Type Hierarchy | CUE

**Source:** https://cuelang.org/docs/tour/types/

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
 2. Types and Values [https://cuelang.org/docs/tour/types/]


 3. TYPE HIERARCHY

CUE defines the following type hierarchy:

_
_|_
{...}
[...]
null
bool
string
bytes
number
int
float
CUE's predefined type hierarchy

CUE defines the value top (or any),
written “_”, such that all types are an instance of top,
and the value bottom (or error),
written “_|_”,
which is an instance of all types.

We can mix the terms types and values interchangeably because
CUE doesn’t distinguish between
types and values [/docs/tour/basics/types-are-values/].
The term “type” merely refers to the kind of a value,
which may or may not be a concrete instance.

In the following hypothetical example, point defines an arbitrary point,
while xaxis and yaxis define any point on their respective axes.
We say that point, xaxis, and yaxis are incomplete,
as they don’t contain values that specify a specific point.
Incomplete values cannot be included when exporting to formats that can’t
represent them.
JSON and YAML, for example, have no way to represent the type number.

Copied!
file.cue

Copy code
Copied!

point: {
	x: number
	y: number
}

xaxis: point
xaxis: y: 0

yaxis: point
yaxis: x: 0

origin: xaxis & yaxis

TERMINAL

Copy code
Copied!

$ cue eval file.cue
point: {
    x: number
    y: number
}
xaxis: {
    x: number
    y: 0
}
yaxis: {
    x: 0
    y: number
}
origin: {
    x: 0
    y: 0
}

By contrast, origin is not incomplete, as it contains only concrete values.
However, notice that we didn’t need to specify its values explicitly.
CUE is able to infer from the constraints applied, placing origin on
both the x-axis and y-axis, that its coordinates must be (x = 0, y = 0).

The output in this example is produced by
cue eval [/docs/reference/command/cue-help-eval/].
This command validates a configuration but, unlike cue export, doesn’t
require it to be completely concrete.
cue eval produces CUE, not JSON or YAML.

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tour/types/types/&text=CUE%20defines%20the%20following%20type%20hierarchy:%0aflowchart%20TD%20top[%22_%22]%20bottom[%22_%7c_%22]%20struct[%22%7b...%7d%22]%20list[%22[...]%22]%20top%20--%3e%20null%20--%3e%20bottom%20top%20--%3e%20bool%20--%3e%20bottom%20top%20--%3e%20string%20--%3e%20bottom%20top%20--%3e%20bytes%20--%3e%20bottom%20top%20--%3e%20number%20--%3e%20int%20&%20float%20--%3e%20bottom%20top%20--%3e%20struct%20--%3e%20bottom%20top%20--%3e%20list%20--%3e%20bottom%20CUE&#39;s%20predefined%20type%20hierarchy%20CUE%20defines%20the%20value%20top%20%28or%20any%29,%20written%20&ldquo;_&rdquo;,%20such%20that%20all%20types%20are%20an%20instance%20of%20top,%20and%20the%20value%20bottom%20%28or%20error%29,%20written%20&ldquo;_%7c_&rdquo;,%20which%20is%20an%20instance%20of%20all%20types.%0aWe%20can%20mix%20the%20terms%20types%20and%20values%20interchangeably%20because%20CUE%20doesn&rsquo;t%20distinguish%20between%20types%20and%20values.%0aThe%20term%20&ldquo;type&rdquo;%20merely%20refers%20to%20the%20kind%20of%20a%20value,%20which%20may%20or%20may%20not%20be%20a%20concrete%20instance.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tour/types/types/&summary=CUE%20defines%20the%20following%20type%20hierarchy:%0aflowchart%20TD%20top[%22_%22]%20bottom[%22_%7c_%22]%20struct[%22%7b...%7d%22]%20list[%22[...]%22]%20top%20--%3e%20null%20--%3e%20bottom%20top%20--%3e%20bool%20--%3e%20bottom%20top%20--%3e%20string%20--%3e%20bottom%20top%20--%3e%20bytes%20--%3e%20bottom%20top%20--%3e%20number%20--%3e%20int%20&%20float%20--%3e%20bottom%20top%20--%3e%20struct%20--%3e%20bottom%20top%20--%3e%20list%20--%3e%20bottom%20CUE&#39;s%20predefined%20type%20hierarchy%20CUE%20defines%20the%20value%20top%20%28or%20any%29,%20written%20&ldquo;_&rdquo;,%20such%20that%20all%20types%20are%20an%20instance%20of%20top,%20and%20the%20value%20bottom%20%28or%20error%29,%20written%20&ldquo;_%7c_&rdquo;,%20which%20is%20an%20instance%20of%20all%20types.%0aWe%20can%20mix%20the%20terms%20types%20and%20values%20interchangeably%20because%20CUE%20doesn&rsquo;t%20distinguish%20between%20types%20and%20values.%0aThe%20term%20&ldquo;type&rdquo;%20merely%20refers%20to%20the%20kind%20of%20a%20value,%20which%20may%20or%20may%20not%20be%20a%20concrete%20instance.%0a]


Previous
Bottom / Error
[/docs/tour/types/bottom/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
   * The Basics of CUE [/docs/tour/basics/]
   * Types and Values [/docs/tour/types/]
     * Type Hierarchy [/docs/tour/types/types/]
       
     * Bottom / Error [/docs/tour/types/bottom/]
     * Top / Any [/docs/tour/types/top/]
     * Numbers [/docs/tour/types/numbers/]
     * Strings [/docs/tour/types/stringlit/]
     * "Raw" Strings [/docs/tour/types/stringraw/]
     * Bytes [/docs/tour/types/bytes/]
     * Structs [/docs/tour/types/structs/]
     * Closed structs [/docs/tour/types/closed/]
     * Definitions [/docs/tour/types/definitions/]
     * Disjunctions [/docs/tour/types/disjunctions/]
     * Default Values [/docs/tour/types/defaults/]
     * Disjunctions of Structs [/docs/tour/types/sumstruct/]
     * Bounds [/docs/tour/types/bounds/]
     * Predefined Bounds [/docs/tour/types/bounddef/]
     * Lists [/docs/tour/types/lists/]
     * Templates [/docs/tour/types/templates/]
     * Next: References and Visibility [/docs/tour/types/next/]
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Ftour%2Ftypes%2Ftypes%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
