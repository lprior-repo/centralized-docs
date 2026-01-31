---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1029
summary: # Introduction | CUE. **Source:** https://cuelang
---

# Introduction | CUE

**Source:** https://cuelang.org/docs/introduction/

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

 1. INTRODUCTION

WELCOME!

CUE is an open-source data validation language and inference engine
with its roots in logic programming.
Although the language is not a general-purpose programming language,
it has many applications, such as
data validation, data templating, configuration, querying,
code generation and even scripting.
The inference engine can be used to validate
data in code or to include it as part of a code generation pipeline.

A key thing that sets CUE apart from its peer languages
is that it merges types and values into a single concept.

Whereas in most languages types and values are strictly distinct,
CUE orders them in a single hierarchy (a lattice, to be precise).
This is a very powerful concept that allows CUE to do
many fancy things.
It also simplifies matters.
For instance, there is no need for generics, and enums, sum types
and null coalescing are all the same thing.

APPLICATIONS

CUE’s design ensures that combining CUE values in any
order always gives the same result
(it is associative, commutative and idempotent).
This makes CUE particularly well-suited for cases where CUE
constraints are combined from different sources:

 * Data validation: different departments or groups can each
   define their own constraints to apply to the same set of data.

 * Code extraction and generation: extract CUE definitions from
   multiple sources (Go code, Protobuf), combine them into a single
   definition, and use that to generate definitions in another
   format (e.g. OpenAPI).

 * Configuration: values can be combined from different sources
   without one having to import the other.

The ordering of values also allows set containment analysis of entire
configurations.
Where most validation systems are limited to checking whether a concrete
value matches a schema, CUE can validate whether any instance of
one schema is also an instance of another (is it backwards compatible?),
or compute a new schema that represents all instances that match
two other schema.

HISTORY

Although it is a very different language, the roots of CUE lie in GCL,
the dominant configuration language in use at Google as of this writing.
It was originally designed to configure Borg, the predecessor of Kubernetes.
In fact, the original idea was to use graph unification as used in CUE for GCL.
One of the authors of GCL had extensive experience with such systems and
experienced the benefit of being able to compute and reason with types for the
creation of powerful tooling.

The graph unification model CUE is based on
was in common use in computational linguistics at that time and was
successfully used to manage grammars and lexicons of over 100k lines of
declarative definitions.
These were effectively very large
configurations of something as irregular and complex as a human language.
A property of these systems were that the types, or constraints, one
defines validate the data while simultaneously reducing boilerplate.
Overall, this approach seemed to be extremely well-suited
for cloud configuration.

However, the early design of GCL went for something simpler that coincidentally
was also incompatible with the notion of graph unification.
This simpler approach proved insufficient, but it was already too late to
move to the earlier foreseen approach.
Instead, an inheritance-based override model was adopted.
Its complexity made the earlier foreseen tooling intractable
and they never materialized.
The same holds for the GCL offsprings that copied its model.

CUE goes back to the original idea of using a constraint-based approach and
