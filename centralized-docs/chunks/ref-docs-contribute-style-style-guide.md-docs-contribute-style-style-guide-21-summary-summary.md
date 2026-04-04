---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#21-summary
chunk_level: summary
chunk_type: table
heading: Language
token_count: 121
summary: ### Use angle brackets for placeholders Use angle brackets for placeholders. Tell the reader what a placeholder represents, for example: Display information about a pod: ``` `kubectl describe pod...
---

### Use angle brackets for placeholders
Use angle brackets for placeholders. Tell the reader what a placeholder
represents, for example:
Display information about a pod:
```
`kubectl describe pod &lt;pod-name&gt; -n &lt;namespace&gt;
`
```
If the namespace of the pod is `default`, you can omit the '-n' parameter.
### Use bold for user interface elements
Do and Don't - Bold interface elements|Do|Don't|
|Click **Fork**.|Click "Fork".|
|Select **Other**.|Select "Other".|