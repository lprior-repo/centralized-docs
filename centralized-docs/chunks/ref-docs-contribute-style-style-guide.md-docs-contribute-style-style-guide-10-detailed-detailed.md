---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#10-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 789
summary: ### Avoid jargon and idioms Some readers speak English as a second language. Avoid jargon and idioms to help them understand better. Do and Don't - Avoid jargon and idioms|Do|Don't| |Internally,...
---

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
### Avoid words that assume a specific level of understanding
Avoid words such as "just", "simply", "easy", "easily", or "simple". These words do not add value.
Do and Don't - Avoid insensitive words|Do|Don't|
|Include one command in ...|Include just one command in ...|
|Run the container ...|Simply run the container ...|
|You can remove ...|You can easily remove ...|
|These steps ...|These simple steps ...|
### EditorConfig file
The Kubernetes project maintains an EditorConfig file that sets common style preferences in text editors
such as VS Code. You can use this file if you want to ensure that your contributions are consistent with
the rest of the project. To view the file, refer to
[`.editorconfig`](https://github.com/kubernetes/website/blob/main/.editorconfig) in the repository root.
## What's next
* Learn about [writing a new topic](/docs/contribute/style/write-new-topic/).
* Learn about [using page templates](/docs/contribute/style/page-content-types/).
* Learn about [custom hugo shortcodes](/docs/contribute/style/hugo-shortcodes/).
* Learn about [creating a pull request](/docs/contribute/new-content/open-a-pr/).
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified January 29, 2025 at 4:14 PM PST: [Add support for KaTeX formule (2108c450f6)](https://github.com/kubernetes/website/commit/2108c450f63bfb42efa99061b66a0996925adf9f)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Advanced contributing](docs-contribute-advanced.md)
- [Tutorials](docs-tutorials.md)
- [Securing a Cluster](docs-tasks-administer-cluster-securing-a-cluster.md)