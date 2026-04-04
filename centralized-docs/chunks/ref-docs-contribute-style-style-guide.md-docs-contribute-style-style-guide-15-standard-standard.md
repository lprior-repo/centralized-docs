---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#15-standard
chunk_level: standard
chunk_type: prose
heading: Shortcodes
token_count: 384
summary: ### Caution Use `{{&lt; caution &gt;}}` to call attention to an important piece of information to avoid pitfalls. For example: ``` `{{&lt; caution &gt;}} The callout style only applies to the line...
---

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