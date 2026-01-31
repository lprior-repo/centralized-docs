---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1050
summary: all of these fall in the realm of possibilities of CUE’s model. The title of this section refers to Bob Carpenter’s
---

all of these fall in the realm of possibilities of CUE’s model.

REFERENCES

The title of this section refers to Bob Carpenter’s
“The Logic of Typed Feature Structures”
(1992, Cambridge University Press, ISBN:0-521-41932-8).
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
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
   * Popular guides [/docs/concept/popular-guides/]
   * The Logic of CUE [/docs/concept/the-logic-of-cue/]
      1. Types are values
      2. The Value Lattice
      3. Reasoning and Inference
      4. References
      5. Footnotes
   
   * Modules [/docs/concept/modules/]
   * Frequently Asked Questions [/docs/concept/faq/]
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fthe-logic-of-cue%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
