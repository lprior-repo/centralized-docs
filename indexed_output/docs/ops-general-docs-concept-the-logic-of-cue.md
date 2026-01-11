---
id: ops/general/docs-concept-the-logic-of-cue
title: Docs Concept The Logic Of Cue
category: ops
tags: ["logic", "ops"]
---

# Docs Concept The Logic Of Cue

> **Context**: **Source:** https://cuelang.org/docs/concept/the-logic-of-cue/

# The Logic of CUE | CUE

**Source:** https://cuelang.org/docs/concept/the-logic-of-cue/

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

 1. Concept Guides [https://cuelang.org/docs/concept/]


 2. THE LOGIC OF CUE

This page explains the core concept on which pretty much everything that is CUE
depends.
It helps to get a top-down understanding and frame of reference,
but it is not necessary for learning the language.

TYPES ARE VALUES

There are two core aspects of CUE that make it different from
the usual programming or configuration languages:

 * Types are values
 * Values (and thus types) are ordered into a lattice

These properties are relevant almost to everything that makes CUE what it is.
They simplify the language, as many concepts that are distinct in other
languages fold together.
The resulting order independence
simplifies reasoning about values for both humans and machines.

It also forces formal rigor on the language, such as defining
exactly what it means to be optional, a default value, or null.
Making sure all values fit in a value lattice leaves no wiggle room.

Finally, the combination of all this allows for many unique features,
for instance:

 * a single language for specifying data, schema, validation
   and policy constraints,
 * meta reasoning, such as determining whether
   a new schema version is backwards compatible,
 * automated rewriting, such as is done by cue trim,
 * creating multi-source constraint pipelines, retaining documentation
   across normalization,

and so on.

THE VALUE LATTICE

Every value in CUE, including what would in most programming languages
be considered types, is partially ordered in a single hierarchy
(a lattice, to be precise).
Even entire configurations and schemas are placed in this hierarchy.

WHAT IS A LATTICE?

This section is useful to understand what a lattice is,
but is not strictly needed to grasp the following sections,
nor the specifics of CUE itself. Skip at will.

A lattice is a partially ordered set, in which every two elements
have a unique least upper bound (join) and greatest lower bound (meet).
By definition this means there is always a single root (top) and a single
leaf (bottom).
Let’s consider what this means by looking at an example.
This diagrams below show a lattice of all values of respectively a
2- and 3- element set, ordered by the subset relation.

{x, y}
{x}
{y}
{}
{x, y, z}
{x, y}
{x, z}
{y, z}
{x}
{y}
{z}
{}
Squint harder if you can't recognize the cube.

If an element B is a subset of element A, there is a path from A to B.
In more general terms, we then say that A _subsumes_ B, or that
B is an _instance of_ A.
In our examples, `{x}` is an instance of `{x, y}`,
because we defined our lattice to use the subset relation.
But we can use any relation we want as long as the properties of a lattice
are upheld.

An important aspect of a lattice is that for every two elements,
there is a unique instance of both elements that subsumes all other
elements that are an instance of both elements.
This is called the greatest lower bound, or meet.
Now let’s imagine we could define a lattice for, say,
all configurations, schemas and data.
In that case, we could always unambiguously merge two such configurations
independently of order.
This is exactly what CUE does!

CUE’S HIERARCHY

In this section we will introduce CUE’s value hierarchy.
The goal here is to get the big picture, and will only present the details
when it helps for this purpose.


BOOLEANS

Let’s start simple, with booleans.

bool
true
false
⊥ (bottom)

This diagram shows that CUE interprets both true and false as
an instance of bool.
No surprises there.
What is less ordinary is that, to CUE, bool is just as much a value
as true and false.
For instance, when we say that a value is both a bool and true,
or in lattice terms,
if we find the greatest lower bound of these values,
the answer is true.
Again maybe no surprise, except that in CUE this is actually
an operation, denoted bool & true.

This also explains the odd fourth element in the graph labeled bottom.
Bottom, in this example, is the result of computing true & false.
A value cannot be both true and false, so this an error.
Bottom is analogous to an error in many other languages.
Bottom is an instance of every value and type, in fact.
More on errors later.

One more detail:
besides the meet operator (&), CUE also has a join operator (|),
which computes the least upper bound.
The result of true | false is indeed bool in CUE.


NUMBERS

With numbers things get a bit more interesting.
CUE has gone through various iterations of the number type system
to find the mix of being practical and strict, while still being simple.
CUE recognizes number, and the instances int and float as classes
of numbers.
For now it suffices to only consider number and int, the latter being
an instance of the former.

Let’s consider a lattice with some example numeric values.
We cannot show a complete lattice, of course, as the number of elements is
infinite (it actually is, CUE has arbitrary precision arithmetic).

number
int
>=0.5
<10
0
1
1.1
20.0
⊥ (bottom)

Here we see what is traditionally a type class (number and int)
and some concrete instances, that is, specific numbers.
They are ordered as expected: 0 and 1 are
integral numbers, whereas 20.0 (by definition) and 1.1 are numbers,
but not integers.
But we also see “constraints”, a category of values that falls between
the traditional concepts of value and type.

CUE defines the constraints we see here in terms of its binary operators
>= and <.
It allows all binary operators that result in a boolean, except ==,
to be used as a constraint by leaving off the left value,
where op B defines the set of all values A for which A op B is true.
The constraint <10 means all numbers less than 10.
Note that we say all numbers, even though 10 is an integer.
This is because CUE allows implicit conversion between
number types in comparisons.


CUE TYPES

Let’s look at all types CUE supports.

⊤ (top)
bool
null
bytes
number
int
struct
list
string
⊥ (bottom)

There are actually values between top and the basic types.
The | operator in CUE allows one to define “sum types” like int | string.
The same operator can also be used to describe what are called “enums”
in other languages, for instance, 1 | 2 | 3.
To CUE these two things—disjunctions of types and disjunctions of values—are the same thing.
You can also mix types and values in a disjunction, as in *1 | intto define defaults (marked by *),
and you can use expressions as well, like *pet.species | "cat".
The latter evaluates to the value of pet.species, or "cat" if
pet.species is null; this is called null coalescing in some languages.

These various uses of | are not the result of operator overloading: they are
all the same operation in CUE.


STRUCTS

Ordering of scalar types, like numbers and strings, is fairly straightforward
and will feel familiar to anyone that has worked with a typed programming
language.
But ordering structs might seem a bit unusual.

Below are two examples of an ordering defined on structs.

municipality
name: string
population: int
big city
name: string
population: >1M
London
name: 'London'
population: 8M
London is a big city, which is a municipality
⊤
a: int
b: int
a: 1
a: int
b: int
a: 1
b: 1
⊥
b: 1

Loosely speaking, a struct is an instance of another if it has at least
all the fields defined by the parent and if its constraints on these fields
are at least as strict as those defined by its parent.

The instance relation for structs has an analogy in
software engineering: backwards compatibility.
For a newer version of an API to be backwards compatible with the previous
version it must subsume it.
In other words, the old version must be an instance of the new one.
Or yet another way to say it: a new version may not forbid what was allowed
in the older version.

With optional fields it gets a bit more subtle, but basically,
an instance may change an optional field to required, but not remove it.
The backwards compatibility metaphor applies here as well.

a?: int
a: int
a?: int & <10
a: int & <10
Required is more specific than optional
a?: 0
a?: 1
a?: ⊥
a: 0
a: 1
⊥
Conflicting values for optional fields result in disallowing that field, conflicting required fields result in a faulty struct

An important thing to note is that, unlike for required fields,
conflicting values for an optional field do not cause a struct to be faulty.
This definition was a result from fitting the notion of closed structs into
the value lattice.
But it can also be explained with some logic.
A common practice in interpretations of logic is to allow
infering ¬P from P→⊥.
If for an optional field we find the value ⊥, we can infer
"not that field", or, drop it.
If we derive ⊥ for a required field, we have a problem,
as a required field cannot be omitted.

THE ANSWER TO LIFE, THE UNIVERSE AND EVERYTHING

CUE has its own equivalent of 42, the answer to life, the universe and
everything, albeit more than 2 characters.
Graph unification of typed feature structures,
CUE’s theoretical foundation, can be described at many levels of abstraction.
CUE’s language specification, and most literature,
take a less abstract and more comprehensible approach,
but in its most abstract form, it can loosely be defined as follows:

Subsumption: given a set F of all TFSs (graphs, CUE values, basically),
and F and F′ in F,
F subsumes F′, denoted F⊑F′, if and only if:
π≡Fπ′ implies π≡F′π′PF(π)=t implies PF′(π)=t′ and t′⊑t
where π≡Fπ′ means that
F∈F contains a path equivalence or reentrancy between
the paths π and π′
(two references starting from the root of a config end up at the same node)
and PF(π)=t means the type
at path π is t (itself a graph in F).

Unification F⊓F′ of two TFSs F and F′ is then the greatest lower
bound of F and F′ in F ordered by subsumption.

This highly abstract definition determines almost everything about CUE.
For instance, lazy binding was not a design decision,
but a direct consequence of following this definition.
It determines the possible evaluation strategies and
what cycles mean, if allowed.
Optional fields, definitions and default values were added to the language
by choice,
but what they can mean strictly follows from this definition.


NULL

We conveniently left out the discussion of null before.
Not only does it make an uninspiring example to describe a lattice,
it is also actually surprisingly complicated to pin down what it means.
This is partly due to lack of guidance from the JSON
standard regarding its
meaning and the different interpretations it gets in practice.

TypeScript creates some order in the chaos by introducing the concepts
undefined and void in addition to null.
It is a necessary evil to give null some meaning
that is compatible with common practices,
within the context of its type system.

CUE got lucky.
CUE’s interpretation of null, optionality, and related concepts
is actually inspired by TypeScript.
But because types are values in CUE, TypeScript’s concepts of
undefined, void and null and optional fields, roughly collapse onto CUE’s
null, bottom (_|_), and optional fields,
resulting in a somewhat simpler model.

DEFAULT VALUES

Default values are CUE’s equivalent of inheritance,
specifically the kind that allows instances to override any value of its parent.
Without it, very little boilerplate removal would be possible.
That is fine if CUE is used just for validation,
but as it aims to be useful across the entire configuration continuum,
it seemed too restrictive to not have such a construct.


RELATION TO INHERITANCE

In CUE, if one sees a concrete value for a field,
it is guaranteed that this will be the final result.
If a value is not concrete (like string), it is clear the search
for a concrete value is not over.
In other words, an instance may never violate the constraints of its parent.
This property makes it very hard to inadvertently make false conclusions in CUE.
Default values do not change this property; they syntactically appear as
non-concrete values.
CUE also bails out and requires explicit values if two conflicting defaults
are specified for the same field, again limiting the search space.

With approaches that allow overrides, whether it be the complex inheritance
used in languages like GCL and Jsonnet
or the much simpler file-based approaches as used in HCL and Kustomize,
finding a declaration for a concrete field value does not guarantee
a final answer,
because another concrete value that occurs elsewhere can override it.
When one needs to change a value of such a field,
it can be time-consuming and,
especially when under pressure,
very tempting to skip following complicated inheritance chains,
double-check a configuration file specifying overlay order,
or look for a file that is lexically sorted after the one under consideration.

So there is a clear benefit to having fully expanded configurations
over such override methods.
CUE simulates that benefit by guaranteeing that any observed field value
holds for the final result.

If the user makes the false assumption that no concrete value is specified to discard the default value,
CUE will catch an erroneous change to that value and report the conflicting
locations.

But there is more.
In CUE one can apply a constraint to a group of values at once,
even across files.
Once set, there is no need to look at the individual values and files to
know these constraints apply.
Such information is not readily available for
fully expanded configurations.1
But also with inheritance-based solutions
that allow arbitrary overrides, templates give little information.

The ability to enforce constraints top down is crucial for any
large-scale configuration setup.
GCL and Jsonnet address this with assertions.
Assertions, however, are typically decoupled from their fields,
making them both hard to discover and hard to reason about.
Where CUE simplifies constraints
(>=3 & <=10 and >=5 & <=20 become >=5 & <=10, >=1 & <=1 becomes 1),
GCL and Jsonnet do not (it would be quite complex),
causing an ever-growing pile of assertions.


SEMANTICS

CUE defaults, which are values marked with a * in disjunctions,
preserve the beneficial properties of the lattice.
In order to do so,
CUE must ensure that the order of picking defaults does not influence the outcome.
Suppose we define two fields, each with the same default value.
We also define that these fields are equal to each other.


Copy code
Copied!

a: int | *1
b: int | *1
a: b
b: a

This is fine.
The obvious answer is a: 1, b: 1.

But now suppose we change one of the default values:


Copy code
Copied!

a: int | *1
b: int | *2
a: b
b: a

What should the answer be?
Picking either 1 or 2 as the default would result in a resolution of the
constraints, but would also be highly undesirable, as the result would depend
on the mood of the implementation.
This also starts to smell like an NP-complete constraint solving problem.
(Basic graph unification itself is pseudo linear.)
CUE wants no part of these shenanigans.
So the answer in this case is that there are no concrete values
as the defaults cannot be used.

The model for this is actually quite simple.
Conceptually, CUE keeps two parallel values, one for all possible values
and one for the default, which must be an instance of the former.
Roughly speaking, for the example with the conflict,
it simultaneously evaluates:


Copy code
Copied!

// All allowed values
a: int
b: int
a: b
b: a


Copy code
Copied!

// Default
a: 1
b: 2
a: b
b: a

Equating a and b clearly results in a conflict (1 != 2) and each will
result in _|_, leaving the left values as the only viable answer.

Now consider the two values corresponding to the original example:


Copy code
Copied!

// All allowed values
a: int
b: int
a: b
b: a


Copy code
Copied!

// Default
a: 1
b: 1
a: b
b: a

Here the defaults are not in conflict and can safely be returned.
Note that it is not an all-or-nothing game.
The parallel values are determined on a field-by-field basis.
So defaults can be selected, or not, independently for fields
that do not depend on each other.

REASONING AND INFERENCE

The values lattice brings CUE another advantage: the ability to reason about
values, schemas, and constraints.

We already discussed how limiting inheritance,
whether language-based or file-based,
makes it easier for people to reason about values.
But it also makes it easier for machines.

BOILERPLATE REMOVAL

CUE’s severe restrictions on inheritance limit its
ability to define hierarchies of templates to remove boilerplate.
But CUE provides some new mechanisms for removing boilerplate.

Suppose a node must inherit from multiple templates, or mixins.
Because order is irrelevant in CUE,
there is no need to specify these in a particular order or even in one location.
One can even say on a single line that a collection of
fields must mix in a template.
For instance,


Copy code
Copied!

jobs: [string]: acmeMonitoring

tells CUE that all jobs in jobs must mix in acmeMonitoring.
There is no need to repeat this at every node.

In CUE, though, we typically refer to acmeMonitoring as a constraint.
After all, applying it will guarantee
that a job implements monitoring in a certain way.
If such a constraint also contains sensible defaults, however,
it simultaneously validates and reduces boilerplate.2

This ability to simultaneously
enforce constraints and remove boilerplate
was a key factor in the success of
the typed feature structure systems that inspired the creation of CUE.

This property is also useful in automation.
The cue trim tool can automatically remove boilerplate from configurations
using the same logic.

CYCLES

An astute reader may have noticed that there were cyclic references
between fields in some of the examples,
something that is not allowed in your typical programming or
configuration language.
CUE’s underlying model allows reasoning over cycles.
Consider a CUE program defining two fields;


Copy code
Copied!

a: b
b: a

This can only be interpreted to mean that a and b must be equal.
If there is no concrete value assigned to a or b,
they remain unspecified in the same way as if each had been declared as string.

This particular case comes in handy in Kubernetes, for instance,
if one wants to equate a set
of labels with a set of selectors
(regardless of whether that is good practice).

But it goes further. Consider


Copy code
Copied!

a: b + 1
b: a - 1
b: 1

When evaluating a, CUE will attempt to resolve b and will find
(a-1) & 1 after unifying the two declarations for b.
It cannot recursively resolve a, as this would result in an
evaluation cycle.
However, the expression (a-1) & 1 is an error
unless (a-1) is 1.
So if this configuration is ever to be a valid, we can safely assume
the answer is 1 and verify that a-1 == 1 after resolving a.

So CUE happily resolves this to


Copy code
Copied!

a: 2
b: 1

without resorting to any fancy algebraic constraint satisfaction solvers,
just plain ol’ logic.
Most cycles that do not result in infinite structures can be handled by CUE.
In fact, it could handle most infinite structures in bounded time
as well, but it puts limits on such cycles for
practical reasons.3

FILE ORGANIZATION

What applies at the language level also applies at the file level.
Within a package, or project, there is no need for files to mutually
import each other.

Files can be split across organizational lines each with a different set
of policies, all implemented with the same familiar constraints.

THE SKY IS THE LIMIT

Many other things are possible.
Take for instance querying.
Whereas validating data is the problem of finding data that is inconsistent with
some constraints,
querying is the problem of finding data that matches some given constraints.
Clearly these two concepts are related.

Computing backwards compatibility (instance of),
computing the most general schema mutually compatible with a set of others
(greatest lower bound),
inferring optimal templates from concrete instances (least upper bound):
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

