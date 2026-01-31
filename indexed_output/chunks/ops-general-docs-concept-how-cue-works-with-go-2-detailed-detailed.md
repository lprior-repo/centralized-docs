---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1028
summary: FUTURE PLANS. The CUE project believes that its role can be one of interlingua:
---

FUTURE PLANS

The CUE project believes that its role can be one of interlingua:
a bidirectional bridge between all the formats that CUE speaks,
linking sources of truth with data - wherever they exist.

On the way towards that goal, the project has plans to extend CUE to
directly generate code in Go (and other languages),
beginning with the ability to
declare native types that mirror CUE counterparts.

Looking further forward, the project plans to expand CUE’s generation
capabilities to include
producing native code that implements CUE constraints,
which will enable non-CUE languages to gain highly efficient implementations of
CUE features such as data validation, policy enforcement, and more.

RELATED CONTENT

 * Reference: cue help get go [/docs/reference/command/cue-help-get-go/]
 * Go API:
   cue [https://pkg.go.dev/cuelang.org/go/cue#section-documentation]
   | cue/load [https://pkg.go.dev/cuelang.org/go/cue/load#section-documentation]
   | cue/cuecontext [https://pkg.go.dev/cuelang.org/go/cue/cuecontext#section-documentation]
   | encoding/yaml [https://pkg.go.dev/cuelang.org/go/encoding/yaml#section-documentation]
   | encoding/json [https://pkg.go.dev/cuelang.org/go/encoding/json#section-documentation]
 * CUE By Example:
   Controlling Kubernetes with CUE [https://github.com/cue-labs/cue-by-example/blob/main/003_kubernetes_tutorial/README.md]
 * Discussion #2939 [/issue/2939]: Modules and package management proposal

Last modified December 30, 2025 [https://github.com/cue-lang/cuelang.org/commit/72e9d5a34edd0ed8ba86fe0990b9ef3a945a37fa]

 * encodings [/search?q=tag:encodings]
 * go api [/search?q=tag:%22go%20api%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-go/&text=CUE%20is%20designed%20to%20complement%20and%20work%20with%20the%20Go%20programming%20language.%20It%20offers%20a%20powerful%20API%20that%20enables%20Go%20code%20to%20take%20advantage%20of%20CUE&rsquo;s%20advanced%20capabilites.%20Additionally,%20CUE%20makes%20it%20easy%20to%20use%20Go%20as%20your%20source%20of%20truth%20by%20using%20the%20cue%20command%20to%20convert%20Go%20types%20to%20CUE.%0aIn%20this%20guide%20we&rsquo;ll%20demonstrate%20importing%20some%20Kubernetes%20API%20code%20to%20generate%20CUE%20schemas.%20We&rsquo;ll%20also%20use%20the%20API%20to%20convert%20both%20CUE%20and%20non-CUE%20data%20to%20native%20Go%20values,%20and%20validate%20some%20Go%20data%20natively%20with%20CUE.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-go/&summary=CUE%20is%20designed%20to%20complement%20and%20work%20with%20the%20Go%20programming%20language.%20It%20offers%20a%20powerful%20API%20that%20enables%20Go%20code%20to%20take%20advantage%20of%20CUE&rsquo;s%20advanced%20capabilites.%20Additionally,%20CUE%20makes%20it%20easy%20to%20use%20Go%20as%20your%20source%20of%20truth%20by%20using%20the%20cue%20command%20to%20convert%20Go%20types%20to%20CUE.%0aIn%20this%20guide%20we&rsquo;ll%20demonstrate%20importing%20some%20Kubernetes%20API%20code%20to%20generate%20CUE%20schemas.%20We&rsquo;ll%20also%20use%20the%20API%20to%20convert%20both%20CUE%20and%20non-CUE%20data%20to%20native%20Go%20values,%20and%20validate%20some%20Go%20data%20natively%20with%20CUE.%0a]


How CUE enables data validation
[/docs/concept/how-cue-enables-data-validation/]How CUE works with JSON
[/docs/concept/how-cue-works-with-json/]
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
   * How CUE works with Go [/docs/concept/how-cue-works-with-go/]
      1. Converting Go types to CUE
      2. Using CUE’s Go API
      3. Future plans
      4. Related content
