---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#18-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 292
summary:  * Discussion #2939 [/issue/2939]: Modules and package management proposal. Last modified December 30, 2025 [https://github
---

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
