---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#14-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 100
summary: ### Check the local DNS configuration first Take a look inside the resolv.conf file. (See [Customizing DNS Service](/docs/tasks/administer-cluster/dns-custom-nameservers/) and [Known...
---

### Check the local DNS configuration first
Take a look inside the resolv.conf file.
(See [Customizing DNS Service](/docs/tasks/administer-cluster/dns-custom-nameservers/) and
[Known issues](#known-issues) below for more information)
```
`kubectl exec -ti dnsutils -- cat /etc/resolv.conf
`
```
Verify that the search path and name server are set up like the following
(note that search path may vary for different cloud providers):