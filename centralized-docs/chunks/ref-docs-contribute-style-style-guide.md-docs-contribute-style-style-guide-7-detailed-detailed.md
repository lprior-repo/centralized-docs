---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#7-detailed
chunk_level: detailed
chunk_type: code
heading: Shortcodes
token_count: 854
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
### Caution
Use `{{&lt; caution &gt;}}` to call attention to an important piece of information to avoid pitfalls.
For example:
```
`{{&lt; caution &gt;}}
The callout style only applies to the line directly above the tag.
{{&lt; /caution &gt;}}
`
```
The output is:
#### Caution:
The callout style only applies to the line directly above the tag.
### Warning
Use `{{&lt; warning &gt;}}` to indicate danger or a piece of information that is crucial to follow.
For example:
```
`{{&lt; warning &gt;}}
Beware.
{{&lt; /warning &gt;}}
`
```
The output is:
### Ordered Lists
Shortcodes will interrupt numbered lists unless you indent four spaces before the notice and the tag.
For example:
```
`1. Preheat oven to 350˚F
1. Prepare the batter, and pour into springform pan.
{{&lt; note &gt;}}Grease the pan for best results.{{&lt; /note &gt;}}
1. Bake for 20-25 minutes or until set.
`
```
The output is:
1. Preheat oven to 350˚F
2. Prepare the batter, and pour into springform pan.
#### Note:
Grease the pan for best results.
3. Bake for 20-25 minutes or until set.
### Include Statements
Shortcodes inside include statements will break the build. You must insert them
in the parent document, before and after you call the include. For example:
```
`{{&lt; note &gt;}}
{{&lt; include "task-tutorial-prereqs.md" &gt;}}
{{&lt; /note &gt;}}
`
```