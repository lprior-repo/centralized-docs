---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 113
summary: Information on authentication options in Kubernetes and their security properties. Selecting the appropriate authentication mechanism(s) is a crucial aspect of securing your cluster. Kubernetes...
---

Information on authentication options in Kubernetes and their security properties.
Selecting the appropriate authentication mechanism(s) is a crucial aspect of securing your cluster.
Kubernetes provides several built-in mechanisms, each with its own strengths and weaknesses that
should be carefully considered when choosing the best authentication mechanism for your cluster.
In general, it is recommended to enable as few authentication mechanisms as possible to simplify
user management and prevent cases where users retain access to a cluster that is no longer required.
It is important to note that Kubernetes does not have an in-built user database within the cluster.