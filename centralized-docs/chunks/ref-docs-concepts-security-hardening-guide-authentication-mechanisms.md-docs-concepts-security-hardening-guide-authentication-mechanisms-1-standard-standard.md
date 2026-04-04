---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 217
summary: # Hardening Guide - Authentication Mechanisms Information on authentication options in Kubernetes and their security properties. Selecting the appropriate authentication mechanism(s) is a crucial...
---

# Hardening Guide - Authentication Mechanisms
Information on authentication options in Kubernetes and their security properties.
Selecting the appropriate authentication mechanism(s) is a crucial aspect of securing your cluster.
Kubernetes provides several built-in mechanisms, each with its own strengths and weaknesses that
should be carefully considered when choosing the best authentication mechanism for your cluster.
In general, it is recommended to enable as few authentication mechanisms as possible to simplify
user management and prevent cases where users retain access to a cluster that is no longer required.
It is important to note that Kubernetes does not have an in-built user database within the cluster.
Instead, it takes user information from the configured authentication system and uses that to make
authorization decisions. Therefore, to audit user access, you need to review credentials from every
configured authentication source.
For production clusters with multiple users directly accessing the Kubernetes API, it is
recommended to use external authentication sources such as OIDC. The internal authentication
mechanisms, such as client certificates and service account tokens, described below, are not
suitable for this use case.