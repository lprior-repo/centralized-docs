---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#4-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
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
