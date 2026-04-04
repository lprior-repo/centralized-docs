---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#20-standard
chunk_level: standard
chunk_type: table
heading: Shortcodes
token_count: 423
summary: ### Avoid Latin phrases Prefer English terms over Latin abbreviations. Do and Don't - Avoid Latin phrases|Do|Don't| |For example, ...|e.g., ...| |That is, ...|i.e., ...| Exception: Use \"etc.\" for et...
---

### Avoid Latin phrases
Prefer English terms over Latin abbreviations.
Do and Don't - Avoid Latin phrases|Do|Don't|
|For example, ...|e.g., ...|
|That is, ...|i.e., ...|
Exception: Use "etc." for et cetera.
### Avoid using "we"
Using "we" in a sentence can be confusing, because the reader might not know
whether they're part of the "we" you're describing.
Do and Don't - Patterns to avoid|Do|Don't|
|Version 1.4 includes ...|In version 1.4, we have added ...|
|Kubernetes provides a new feature for ...|We provide a new feature ...|
|This page teaches you how to use pods.|In this page, we are going to learn about pods.|
### Avoid jargon and idioms
Some readers speak English as a second language. Avoid jargon and idioms to help them understand better.
Do and Don't - Avoid jargon and idioms|Do|Don't|
|Internally, ...|Under the hood, ...|
|Create a new cluster.|Turn up a new cluster.|
### Avoid statements about the future
Avoid making promises or giving hints about the future. If you need to talk about
an alpha feature, put the text under a heading that identifies it as alpha
information.
An exception to this rule is documentation about announced deprecations
targeting removal in future versions. One example of documentation like this
is the [Deprecated API migration guide](/docs/reference/using-api/deprecation-guide/).
### Avoid statements that will soon be out of date
Avoid words like "currently" and "new." A feature that is new today might not be
considered new in a few months.
Do and Don't - Avoid statements that will soon be out of date|Do|Don't|
|In version 1.4, ...|In the current version, ...|
|The Federation feature provides ...|The new Federation feature provides ...|