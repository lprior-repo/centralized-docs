---
id: ops/general/docs-reference-glossary
title: Docs Reference Glossary
category: ops
tags: ["glossary", "ops", "terms"]
---

# Glossary of terms | CUE

> **Context**: **Source:** https://cuelang.org/docs/reference/glossary/


**Source:** https://cuelang.org/docs/reference/glossary/

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

 1. References [https://cuelang.org/docs/reference/]


 2. GLOSSARY OF TERMS

A

AND()

🔗 Language Spec [/docs/reference/spec/#and]
| Howto Guide [/docs/howto/use-the-built-in-function-and/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that accepts a
   list [/docs/reference/glossary/#list] and returns the
   unification [/docs/reference/glossary/#unification] of all elements in the list

B

BOOL

🔗 Language Spec [/docs/reference/spec/#boolean-values]

 * A primitive type [/docs/reference/glossary/#type] representing the set of Boolean
   truth values denoted by the keywords true and false

BUILT-IN FUNCTIONS

🔗 Language Spec [/docs/reference/spec/#built-in-functions]

 * Predeclared functions provided by the CUE runtime that are available without
   being imported
 * see also:
   and() [/docs/reference/glossary/#and-built-in-function]
   | close() [/docs/reference/glossary/#close-built-in-function]
   | div() [/docs/reference/glossary/#div-built-in-function]
   | mod() [/docs/reference/glossary/#mod-built-in-function]
   | len() [/docs/reference/glossary/#len-built-in-function]
   | or() [/docs/reference/glossary/#or-built-in-function]
   | quo() [/docs/reference/glossary/#quo-built-in-function]
   | rem() [/docs/reference/glossary/#rem-built-in-function]

BYTES

🔗 Language Spec [/docs/reference/spec/#bytes]
| Tour [/docs/tour/types/bytes/]

 * A primitive type [/docs/reference/glossary/#type] representing a possibly empty
   sequence of arbitrary bytes

C

CLOSE()

🔗 Language Spec [/docs/reference/spec/#close]
| Howto Guide [/docs/howto/use-the-built-in-function-close/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that converts a
   partially defined (“open”) struct [/docs/reference/glossary/#struct] to a fully
   defined (“closed”) struct

D

DEFAULT VALUE

🔗 Tour [/docs/tour/types/defaults/]
| Howto Guide [/docs/howto/specify-a-default-value-for-a-field/]

 * The single element of a disjunction [/docs/reference/glossary/#disjunction] that CUE
   assigns to a field [/docs/reference/glossary/#field] if and only if
   unification [/docs/reference/glossary/#unification] fails to resolve a concrete value
   for the field
 * An element of a disjunction prefixed with an asterisk (*)

DISJUNCTION

🔗 Tour #1 [/docs/tour/types/disjunctions/]
| Tour #2 [/docs/tour/types/sumstruct/]

DIV()

🔗 Language Spec [/docs/reference/spec/#div-mod-quo-and-rem]
| Howto Guide [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that performs
   Euclidean division with its parameters and returns the integer quotient

E

F

FIELD

🔗 Tour [/docs/tour/types/structs/]

 * A key-value pair inside a map, associating a value with a given set of keys

FIELD CONSTRAINT

🔗 Language Spec [/docs/reference/spec/#field-constraints]
| Tour [/docs/tour/types/structs/]

 * A field constraint restricts a field [/docs/reference/glossary/#field]’s value
   without actually defining the field, with the field only forming part of the
   output if it is successfully unified [/docs/reference/glossary/#unification] with a
   concrete value
 * see also:
   Required field constraint [/docs/reference/glossary/#required-field-constraint]
   | Optional field constraint [/docs/reference/glossary/#optional-field-constraint]

FLOAT

🔗 Language Spec [/docs/reference/spec/#numeric-values]
| Tour [/docs/tour/types/numbers/]

 * A primitive type [/docs/reference/glossary/#type] representing the set of all
   decimal floating-point numbers

G

H

I

INT

🔗 Language Spec [/docs/reference/spec/#numeric-values]
| Tour [/docs/tour/types/numbers/]

 * A primitive type [/docs/reference/glossary/#type] representing the set of all
   integer numbers

J

JSON

🔗 json.org [https://www.json.org/]

 * “A lightweight data-interchange format … easy for humans to read and write
   [and] easy for machines to parse and generate.” – json.org
 * A data format understood by the cue CLI, which can both parse and emit JSON
   as input and output

K

L

LEN()

🔗 Language Spec [/docs/reference/spec/#len]
| Howto Guide [/docs/howto/use-the-built-in-function-len/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that returns the
   lengths of various types as an integer

LIST

🔗 Language Spec [/docs/reference/spec/#lists]
| Tour [/docs/tour/types/lists/]

 * An arbitrary sequence of CUE values, enclosed in square brackets ([ ])
   with values separated by commas (,)

M

MOD()

🔗 Language Spec [/docs/reference/spec/#div-mod-quo-and-rem]
| Howto Guide [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that performs
   Euclidean division with its parameters and returns the integer remainder

N

NULL

🔗 Language Spec [/docs/reference/spec/#null]

 * A primitive type [/docs/reference/glossary/#type] whose only value, the null
   value, is represented with the keyword null. Comparable with itself and
   any other type, the comparison with a null value always being equal and the
   comparison with any other type always being unequal

NUMBER

🔗 Language Spec [/docs/reference/spec/#numeric-values]
| Tour [/docs/tour/types/numbers/]

 * A generic primitive type [/docs/reference/glossary/#type] representing the set of
   all members of both int [/docs/reference/glossary/#int-type] and
   float [/docs/reference/glossary/#float-type] types

O

OPTIONAL FIELD CONSTRAINT

🔗 Tour [/docs/tour/types/structs/]
| Howto Guide [/docs/howto/mark-a-field-as-optional/]

 * A field constraint [/docs/reference/glossary/#field-constraint] that restricts the
   field [/docs/reference/glossary/#field]’s value if the field is present, whilst also
   permitting the field’s absence

OR()

🔗 Language Spec [/docs/reference/spec/#or]
| Howto Guide [/docs/howto/use-the-built-in-function-or/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that accepts a
   list [/docs/reference/glossary/#list] and produces a
   disjunction [/docs/reference/glossary/#disjunction]

P

Q

QUO()

🔗 Language Spec [/docs/reference/spec/#div-mod-quo-and-rem]
| Howto Guide [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that performs
   truncated division with its parameters and returns the integer quotient

R

“RAW” STRINGS

🔗 Language Spec [/docs/reference/spec/#string-and-byte-sequence-literals]
| Tour [/docs/tour/types/stringraw/]

 * A string [/docs/reference/glossary/#string-type] literal enclosed with an equal
   number of hashes on both sides, allowing escape sequences to appear inside
   the text verbatim, without their usual escaping taking effect

REM()

🔗 Language Spec [/docs/reference/spec/#div-mod-quo-and-rem]
| Howto Guide [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that performs
   truncated division with its parameters and returns the integer remainder

REQUIRED FIELD CONSTRAINT

🔗 Tour [/docs/tour/types/structs/]
| Howto Guide [/docs/howto/mark-a-field-as-required/]

 * A field constraint [/docs/reference/glossary/#field-constraint] that restricts the
   field [/docs/reference/glossary/#field]’s value whilst also requiring the field to
   be present

S

STRING

🔗 Language Spec [/docs/reference/spec/#strings]
| Tour [/docs/tour/types/stringlit/]

 * A primitive type [/docs/reference/glossary/#type] representing the set of UTF-8
   strings
 * see also: Raw strings [/docs/reference/glossary/#raw-strings]

STRUCT

🔗 Language Spec [/docs/reference/spec/#structs]
| Tour [/docs/tour/types/structs/]

 * A composite type representing a set of elements (called
   fields [/docs/reference/glossary/#field]) each of which has a name (called a label)
   and a value

T

TYPE

🔗 Tour [/docs/tour/types/types/]

U

UNIFICATION

🔗 Language Spec [/docs/reference/spec/#unification]

V

W

X

Y

YAML

🔗 yaml.org [https://yaml.org/]

 * “YAML is a human-friendly data serialization language for all programming
   languages” – yaml.org
 * A data format understood by the cue CLI, which can both parse and emit YAML
   as input and output

Z

#


Last modified March 2, 2024 [https://github.com/cue-lang/cuelang.org/commit/7bc69d008ea3a1cef9e2df0927306f09d21396c1]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/reference/glossary/&text=A%20and%28%29%20%f0%9f%94%97%20Language%20Spec%20%7c%20Howto%20Guide%0aA%20built-in%20function%20that%20accepts%20a%20list%20and%20returns%20the%20unification%20of%20all%20elements%20in%20the%20list%20B%20bool%20%f0%9f%94%97%20Language%20Spec%0aA%20primitive%20type%20representing%20the%20set%20of%20Boolean%20truth%20values%20denoted%20by%20the%20keywords%20true%20and%20false%20Built-in%20functions%20%f0%9f%94%97%20Language%20Spec%0aPredeclared%20functions%20provided%20by%20the%20CUE%20runtime%20that%20are%20available%20without%20being%20imported%20see%20also:%20and%28%29%20%7c%20close%28%29%20%7c%20div%28%29%20%7c%20mod%28%29%20%7c%20len%28%29%20%7c%20or%28%29%20%7c%20quo%28%29%20%7c%20rem%28%29%20bytes%20%f0%9f%94%97%20Language%20Spec%20%7c%20Tour%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/reference/glossary/&summary=A%20and%28%29%20%f0%9f%94%97%20Language%20Spec%20%7c%20Howto%20Guide%0aA%20built-in%20function%20that%20accepts%20a%20list%20and%20returns%20the%20unification%20of%20all%20elements%20in%20the%20list%20B%20bool%20%f0%9f%94%97%20Language%20Spec%0aA%20primitive%20type%20representing%20the%20set%20of%20Boolean%20truth%20values%20denoted%20by%20the%20keywords%20true%20and%20false%20Built-in%20functions%20%f0%9f%94%97%20Language%20Spec%0aPredeclared%20functions%20provided%20by%20the%20CUE%20runtime%20that%20are%20available%20without%20being%20imported%20see%20also:%20and%28%29%20%7c%20close%28%29%20%7c%20div%28%29%20%7c%20mod%28%29%20%7c%20len%28%29%20%7c%20or%28%29%20%7c%20quo%28%29%20%7c%20rem%28%29%20bytes%20%f0%9f%94%97%20Language%20Spec%20%7c%20Tour%0a]


The CUE Language Specification
[/docs/reference/spec/]Code of Conduct
[/docs/reference/code-of-conduct/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]
   * The CUE Language Specification [/docs/reference/spec/]
   * Glossary of terms [/docs/reference/glossary/]
     
   * The cue command [/docs/reference/command/]
   * Code of Conduct [/docs/reference/code-of-conduct/]

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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Freference%2Fglossary%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
