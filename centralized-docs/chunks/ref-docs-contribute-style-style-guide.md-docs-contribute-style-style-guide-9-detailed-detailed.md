---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#9-detailed
chunk_level: detailed
chunk_type: table
heading: Shortcodes
token_count: 1019
summary: ### Lists Group items in a list that are related to each other and need to appear in a specific order or to indicate a correlation between multiple items. When a screen reader comes across a...
---

### Lists
Group items in a list that are related to each other and need to appear in a specific
order or to indicate a correlation between multiple items. When a screen reader comes
across a list—whether it is an ordered or unordered list—it will be announced to the
user that there is a group of list items. The user can then use the arrow keys to move
up and down between the various items in the list. Website navigation links can also be
marked up as list items; after all they are nothing but a group of related links.
* End each item in a list with a period if one or more items in the list are complete
sentences. For the sake of consistency, normally either all items or none should be complete sentences.
#### Note:
Ordered lists that are part of an incomplete introductory sentence can be in lowercase
and punctuated as if each item was a part of the introductory sentence.
* Use the number one (`1.`) for ordered lists.
* Use (`+`), (`\*`), or (`-`) for unordered lists.
* Leave a blank line after each list.
* Indent nested lists with four spaces (for example, ⋅⋅⋅⋅).
* List items may consist of multiple paragraphs. Each subsequent paragraph in a list
item must be indented by either four spaces or one tab.
### Tables
The semantic purpose of a data table is to present tabular data. Sighted users can
quickly scan the table but a screen reader goes through line by line. A table caption
is used to create a descriptive title for a data table. Assistive technologies (AT)
use the HTML table caption element to identify the table contents to the user within the page structure.
* Add table captions using [Hugo shortcodes](/docs/contribute/style/hugo-shortcodes/#table-captions) for tables.## Content best practices
This section contains suggested best practices for clear, concise, and consistent content.
### Use present tense
Do and Don't - Use present tense|Do|Don't|
|This command starts a proxy.|This command will start a proxy.|
Exception: Use future or past tense if it is required to convey the correct
meaning.
### Use active voice
Do and Don't - Use active voice|Do|Don't|
|You can explore the API using a browser.|The API can be explored using a browser.|
|The YAML file specifies the replica count.|The replica count is specified in the YAML file.|
Exception: Use passive voice if active voice leads to an awkward construction.
### Use simple and direct language
Use simple and direct language. Avoid using unnecessary phrases, such as saying "please."
Do and Don't - Use simple and direct language|Do|Don't|
|To create a ReplicaSet, ...|In order to create a ReplicaSet, ...|
|See the configuration file.|Please see the configuration file.|
|View the pods.|With this next command, we'll view the pods.|
### Address the reader as "you"
Do and Don't - Addressing the reader|Do|Don't|
|You can create a Deployment by ...|We'll create a Deployment by ...|
|In the preceding output, you can see...|In the preceding output, we can see ...|
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