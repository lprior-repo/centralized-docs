---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#8-summary
chunk_level: summary
chunk_type: prose
heading: Adding additional entries with hostAliases
token_count: 111
summary: In addition to the default boilerplate, you can add additional entries to the `hosts` file. For example: to resolve `foo.local`, `bar.local` to `127.0.0.1` and `foo.remote`, `bar.remote` to...
---

In addition to the default boilerplate, you can add additional entries to the
`hosts` file.
For example: to resolve `foo.local`, `bar.local` to `127.0.0.1` and `foo.remote`,
`bar.remote` to `10.1.2.3`, you can configure HostAliases for a Pod under
`.spec.hostAliases`:
[`service/networking/hostaliases-pod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/hostaliases-pod.yaml)