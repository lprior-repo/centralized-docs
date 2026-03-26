---
doc_id: ref/docs-contribute-participate.md/docs-contribute-participate
chunk_id: ref/docs-contribute-participate.md/docs-contribute-participate#7-summary
chunk_level: summary
chunk_type: prose
heading: SIG Docs teams and automation
token_count: 124
summary: ### GitHub teams There are two categories of SIG Docs [teams](https://github.com/orgs/kubernetes/teams?query=sig-docs) on GitHub: * `@sig-docs-{language}-owners` are approvers and leads *...
---

### GitHub teams
There are two categories of SIG Docs [teams](https://github.com/orgs/kubernetes/teams?query=sig-docs) on GitHub:
* `@sig-docs-{language}-owners` are approvers and leads
* `@sig-docs-{language}-reviews` are reviewers
Each can be referenced with their `@name` in GitHub comments to communicate with
everyone in that group.
Sometimes Prow and GitHub teams overlap without matching exactly. For
assignment of issues, pull requests, and to support PR approvals, the
automation uses information from `OWNERS` files.