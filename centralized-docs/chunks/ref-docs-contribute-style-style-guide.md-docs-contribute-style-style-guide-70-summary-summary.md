---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#70-summary
chunk_level: summary
chunk_type: prose
heading: Shortcodes
token_count: 101
summary: #### Note: Grease the pan for best results. 3. Bake for 20-25 minutes or until set. ### Include Statements Shortcodes inside include statements will break the build. You must insert them in the...
---

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