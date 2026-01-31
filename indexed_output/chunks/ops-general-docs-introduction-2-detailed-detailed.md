---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1180
summary: soft constraints. SEPARATE CONFIGURATION FROM COMPUTATION
---

soft constraints.

SEPARATE CONFIGURATION FROM COMPUTATION

There comes a time that one (seemingly) will need do complex
computations to generate some configuration data.
But simplicity of a configuration language can be paramount when one quickly
needs to make changes.
These are obviously conflicting interests.

CUE takes the stance that computation and configuration should
be separated.
And CUE actually makes this easy.
The data that needs to be computed can be generated outside of CUE
and put in a file that is to be mixed in.
The data can even be generated in CUE’s scripting layer and automatically
injected in a configuration pipeline.
Both approaches rely on CUE’s property that the order in which this data gets
added is irrelevant.

BE USEFUL AT ALL SCALES

The usefulness of a language may depend on the scale of the project.
Having too many different languages can put a cognitive strain on
developers, though, and migrating from one language to another as
scaling requirements change can be very costly.
CUE aims to minimize these costs
by covering a myriad of data- and configuration-related tasks at all scales.

Small scale
At small scales, reducing boilerplate in configurations is not necessarily
the best thing to do.
Even at a small scale, however, repetition can be error prone.
For such cases, CUE can define schema to validate otherwise
typeless data files.

Medium scale
As soon the desire arises to reduce boilerplate, the cue tool can
help to automatically rewrite configurations.
See the Quick and Dirty section of the
Kubernetes tutorial [https://github.com/cue-labs/cue-by-example/blob/main/003_kubernetes_tutorial/README.md]
for an example using the import and trim tool.
Thousands of lines can be obliterated automatically using this approach.

Large scale
CUE’s underlying formalism was developed for large-scale configuration.
Its import model incorporates best practices for large-scale engineering
and it is optimized for automation.
A key to this is advanced tooling.
The mathematical model underlying CUE’s operations allows for
automation that is intractable for most other approaches.
CUE’s trim command is an example of this.

TOOLING

Automation is key.
Nowadays, a good chunk of code gets generated, analyzed, reformatted,
and so on by machines.
The CUE language, APIs, and tooling have been designed to allow for
machine manipulation.
Aspects of this are:

 * make the language easy to scan and parse,
 * restrictions on imports,
 * allow any piece of data to be split across files and generated
   from different sources,
 * define packages at the directory level,
 * and of course its value and type model.

The order independence also plays a key role in this.
It allows combining constraints from various sources without having
to define any order in which they are to be applied to get
predictable results.

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/introduction/&text=%20Welcome!%20CUE%20is%20an%20open-source%20data%20validation%20language%20and%20inference%20engine%20with%20its%20roots%20in%20logic%20programming.%20Although%20the%20language%20is%20not%20a%20general-purpose%20programming%20language,%20it%20has%20many%20applications,%20such%20as%20data%20validation,%20data%20templating,%20configuration,%20querying,%20code%20generation%20and%20even%20scripting.%20The%20inference%20engine%20can%20be%20used%20to%20validate%20data%20in%20code%20or%20to%20include%20it%20as%20part%20of%20a%20code%20generation%20pipeline.%0aA%20key%20thing%20that%20sets%20CUE%20apart%20from%20its%20peer%20languages%20is%20that%20it%20merges%20types%20and%20values%20into%20a%20single%20concept.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/introduction/&summary=%20Welcome!%20CUE%20is%20an%20open-source%20data%20validation%20language%20and%20inference%20engine%20with%20its%20roots%20in%20logic%20programming.%20Although%20the%20language%20is%20not%20a%20general-purpose%20programming%20language,%20it%20has%20many%20applications,%20such%20as%20data%20validation,%20data%20templating,%20configuration,%20querying,%20code%20generation%20and%20even%20scripting.%20The%20inference%20engine%20can%20be%20used%20to%20validate%20data%20in%20code%20or%20to%20include%20it%20as%20part%20of%20a%20code%20generation%20pipeline.%0aA%20key%20thing%20that%20sets%20CUE%20apart%20from%20its%20peer%20languages%20is%20that%20it%20merges%20types%20and%20values%20into%20a%20single%20concept.%0a]
