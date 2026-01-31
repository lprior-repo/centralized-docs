---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#4-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 621
summary: exit status 1. FUTURE PLANS
---

    bad.json:2:13
    schema.json:13:13
exit status 1

FUTURE PLANS

One of CUE’s goals is to act as an interlingua: a bidirectional bridge
between all the formats that CUE speaks, linking constraints and data sources
of truth, no matter where they exist.

To meet this goal, CUE will gain the ability to export native CUE constraints
as JSON Schema, enabling their use by tools that aren’t aware of CUE. This is
tracked in issue #929 [/issue/929].

RELATED CONTENT

 * Reference: cue help import [/docs/reference/command/cue-help-import/]
 * The encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema] Go API
 * Reference: cue help vet [/docs/reference/command/cue-help-vet/]
 * Reference: cue help filetypes [/docs/reference/command/cue-help-filetypes/]
 * Issue #929 [/issue/929] tracks the conversion of CUE to JSON Schema

Last modified September 4, 2025 [https://github.com/cue-lang/cuelang.org/commit/c675de963f4124145b48e2681dab7b4aacab71e2]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]
 * go api [/search?q=tag:%22go%20api%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-json-schema/&text=CUE%20has%20first%20class%20support%20for%20JSON%20Schema:%20both%20the%20cue%20command%20and%20the%20Go%20API%20understand%20the%20format.%0aConstraints%20stored%20as%20JSON%20Schema%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20JSON%20Schema%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE&rsquo;s%20more%20succinct%20and%20expressive%20form.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-json-schema/&summary=CUE%20has%20first%20class%20support%20for%20JSON%20Schema:%20both%20the%20cue%20command%20and%20the%20Go%20API%20understand%20the%20format.%0aConstraints%20stored%20as%20JSON%20Schema%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20JSON%20Schema%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE&rsquo;s%20more%20succinct%20and%20expressive%20form.%0a]
