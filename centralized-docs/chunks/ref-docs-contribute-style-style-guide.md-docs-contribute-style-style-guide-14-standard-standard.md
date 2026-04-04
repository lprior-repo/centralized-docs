---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#14-standard
chunk_level: standard
chunk_type: prose
heading: Shortcodes
token_count: 469
summary: ## Shortcodes Hugo [Shortcodes](https://gohugo.io/content-management/shortcodes) help create different rhetorical appeal levels. Our documentation supports three different shortcodes in this...
---

## Shortcodes
Hugo [Shortcodes](https://gohugo.io/content-management/shortcodes) help create
different rhetorical appeal levels. Our documentation supports three different
shortcodes in this category: **Note** `{{&lt; note &gt;}}`,
**Caution** `{{&lt; caution &gt;}}`, and **Warning** `{{&lt; warning &gt;}}`.
1. Surround the text with an opening and closing shortcode.
2. Use the following syntax to apply a style:
```
`{{&lt; note &gt;}}
No need to include a prefix; the shortcode automatically provides one. (Note:, Caution:, etc.)
{{&lt; /note &gt;}}
`
```
The output is:
#### Note:
The prefix you choose is the same text for the tag.
### Note
Use `{{&lt; note &gt;}}` to highlight a tip or a piece of information that may be helpful to know.
For example:
```
`{{&lt; note &gt;}}
You can \_still\_ use Markdown inside these callouts.
{{&lt; /note &gt;}}
`
```
The output is:
#### Note:
You can *still* use Markdown inside these callouts.
You can use a `{{&lt; note &gt;}}` in a list:
```
`1. Use the note shortcode in a list
1. A second item with an embedded note
{{&lt; note &gt;}}
Warning, Caution, and Note shortcodes, embedded in lists, need to be indented four spaces. See [Common Shortcode Issues](#common-shortcode-issues).
{{&lt; /note &gt;}}
1. A third item in a list
1. A fourth item in a list
`
```
The output is:
1. Use the note shortcode in a list
2. A second item with an embedded note
#### Note:
```
`Warning, Caution, and Note shortcodes, embedded in lists, need to be indented four spaces. See [Common Shortcode Issues](#common-shortcode-issues).
`
```
3. A third item in a list
4. A fourth item in a list