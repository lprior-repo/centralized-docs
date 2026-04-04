---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#18-standard
chunk_level: standard
chunk_type: table
heading: Shortcodes
token_count: 505
summary: ### Links Do and Don't - Links|Do|Don't| |Write hyperlinks that give you context for the content they link to. For example: Certain ports are open on your machines. See [Check required...
---

### Links
Do and Don't - Links|Do|Don't|
|Write hyperlinks that give you context for the content they link to. For example: Certain ports are open on your machines. See [Check required ports](#check-required-ports) for more details.|Use ambiguous terms such as "click here". For example: Certain ports are open on your machines. See [here](#check-required-ports) for more details.|
|Write Markdown-style links: `[link text](URL)`. For example: `[Hugo shortcodes](/docs/contribute/style/hugo-shortcodes/#table-captions)` and the output is [Hugo shortcodes](/docs/contribute/style/hugo-shortcodes/#table-captions).|Write HTML-style links: `&lt;a href="/media/examples/link-element-example.css" target="\_blank"&gt;Visit our tutorial!&lt;/a&gt;`, or create links that open in new tabs or windows. For example: `[example website](https://example.com){target="\_blank"}`|
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