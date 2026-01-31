---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1031
summary: “Structural OpenAPI” form required by CRDs targeting Kubernetes version 1. FUTURE PLANS
---

“Structural OpenAPI” form required by CRDs targeting Kubernetes version 1.15
and later.

FUTURE PLANS

One of CUE’s goals is to act as an interlingua: a bidirectional bridge
between all the formats that CUE speaks, linking constraints with data sources
of truth, no matter where they exist.

For now, only OpenAPI’s data schemas, in the components.schemas namespace,
are handled by CUE. More complete support is tracked in issue #3133 [/issue/3133].

RELATED CONTENT

 * Reference: cue help def [/docs/reference/command/cue-help-def/]
 * Reference: cue help import [/docs/reference/command/cue-help-import/]
 * Reference: cue help vet [/docs/reference/command/cue-help-vet/]
 * The encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi] package
 * Issue #3133 [/issue/3133] tracks the support of other namespaces defined by
   the OpenAPI standard [https://github.com/OAI/OpenAPI-Specification/tree/3.0.0]

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-openapi/&text=CUE%20has%20first%20class%20support%20for%20OpenAPI%20data%20schemas:%20the%20cue%20command%20automatically%20recognises%20OpenAPI%20by%20its%20signature%20fields,%20and%20the%20Go%20API%20has%20packages%20dedicated%20to%20the%20format.%20Specifically,%20CUE%20supports%20the%20OpenAPI%203.0.0%20standard%20through%20its%20components.schemas%20namespace%20for%20data%20schemas.%0aConstraints%20stored%20as%20OpenAPI%20data%20schemas%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20these%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE%e2%80%99s%20significantly%20more%20concise%20form.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-openapi/&summary=CUE%20has%20first%20class%20support%20for%20OpenAPI%20data%20schemas:%20the%20cue%20command%20automatically%20recognises%20OpenAPI%20by%20its%20signature%20fields,%20and%20the%20Go%20API%20has%20packages%20dedicated%20to%20the%20format.%20Specifically,%20CUE%20supports%20the%20OpenAPI%203.0.0%20standard%20through%20its%20components.schemas%20namespace%20for%20data%20schemas.%0aConstraints%20stored%20as%20OpenAPI%20data%20schemas%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20these%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE%e2%80%99s%20significantly%20more%20concise%20form.%0a]


How CUE works with JSON Schema
[/docs/concept/how-cue-works-with-json-schema/]How CUE works with Protocol Buffers
[/docs/concept/how-cue-works-with-protocol-buffers/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
   * Popular guides [/docs/concept/popular-guides/]
   * The Logic of CUE [/docs/concept/the-logic-of-cue/]
   * Modules [/docs/concept/modules/]
   * Frequently Asked Questions [/docs/concept/faq/]
   * How CUE works with OpenAPI [/docs/concept/how-cue-works-with-openapi/]
      1. Using OpenAPI with the cue command
      2. Using OpenAPI with the Go API
      3. Future plans
      4. Related content
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
