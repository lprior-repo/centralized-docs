---
doc_id: ref/docs-contribute-participate.md/docs-contribute-participate
chunk_id: ref/docs-contribute-participate.md/docs-contribute-participate#14-summary
chunk_level: summary
chunk_type: prose
heading: How merging works
token_count: 102
summary: * When a pull request has both the `lgtm` and `approve` labels, has no `hold` labels, and all tests are passing, the pull request merges automatically. * Kubernetes organization members and SIG Docs...
---

* When a pull request has both the `lgtm` and `approve` labels, has no `hold`
labels, and all tests are passing, the pull request merges automatically.
* Kubernetes organization members and SIG Docs approvers can add comments to
prevent automatic merging of a given pull request (by adding a `/hold` comment
or withholding a `/lgtm` comment).
* Any Kubernetes member can add the `lgtm` label by adding a `/lgtm` comment.