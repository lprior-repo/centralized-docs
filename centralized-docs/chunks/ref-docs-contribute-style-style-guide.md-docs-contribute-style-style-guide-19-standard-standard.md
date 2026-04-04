---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#19-standard
chunk_level: standard
chunk_type: table
heading: Shortcodes
token_count: 464
summary: ### Tables The semantic purpose of a data table is to present tabular data. Sighted users can quickly scan the table but a screen reader goes through line by line. A table caption is used to create a...
---

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