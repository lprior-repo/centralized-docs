---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#7-standard
chunk_level: standard
chunk_type: table
heading: Language
token_count: 465
summary: ### Use code style for inline code, commands For inline code in an HTML document, use the `&lt;code&gt;` tag. In a Markdown document, use the backtick (```). However, API kinds such as StatefulSet or...
---

### Use code style for inline code, commands
For inline code in an HTML document, use the `&lt;code&gt;` tag. In a Markdown
document, use the backtick (```). However, API kinds such as StatefulSet
or ConfigMap are written verbatim (no backticks); this allows using possessive
apostrophes.
Do and Don't - Use code style for inline code, commands, and API objects|Do|Don't|
|The `kubectl run` command creates a Pod.|The "kubectl run" command creates a Pod.|
|The kubelet on each node acquires a Lease…|The kubelet on each node acquires a `Lease`…|
|A PersistentVolume represents durable storage…|A `PersistentVolume` represents durable storage…|
|The CustomResourceDefinition's `.spec.group` field…|The `CustomResourceDefinition.spec.group` field…|
|For declarative management, use `kubectl apply`.|For declarative management, use "kubectl apply".|
|Enclose code samples with triple backticks. (```)|Enclose code samples with any other syntax.|
|Use single backticks to enclose inline code. For example, `var example = true`.|Use two asterisks (`\*\*`) or an underscore (`\_`) to enclose inline code. For example, **var example = true**.|
|Use triple backticks before and after a multi-line block of code for fenced code blocks.|Use multi-line blocks of code to create diagrams, flowcharts, or other illustrations.|
|Use meaningful variable names that have a context.|Use variable names such as 'foo','bar', and 'baz' that are not meaningful and lack context.|
|Remove trailing spaces in the code.|Add trailing spaces in the code, where these are important, because the screen reader will read out the spaces as well.|
#### Note:
The website supports syntax highlighting for code samples, but specifying a language
is optional. Syntax highlighting in the code block should conform to the
[contrast guidelines.](https://www.w3.org/WAI/WCAG21/quickref/?versions=2.0&amp;showtechniques=141,143#contrast-minimum)