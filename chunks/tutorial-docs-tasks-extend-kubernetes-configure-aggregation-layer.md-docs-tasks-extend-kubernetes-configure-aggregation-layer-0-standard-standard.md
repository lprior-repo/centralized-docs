---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 323
summary: ## Table of Contents  - [Configure the Aggregation Layer](#configure-the-aggregation-layer)   - [Before you begin](#before-you-begin)       - [Note:](#note)       - [Caution:](#caution)   -...
---

## Table of Contents

- [Configure the Aggregation Layer](#configure-the-aggregation-layer)
  - [Before you begin](#before-you-begin)
      - [Note:](#note)
      - [Caution:](#caution)
  - [Authentication Flow](#authentication-flow)
    - [Kubernetes Apiserver Authentication and Authorization](#kubernetes-apiserver-authentication-and-authorization)
    - [Kubernetes Apiserver Proxies the Request](#kubernetes-apiserver-proxies-the-request)
      - [Kubernetes Apiserver Client Authentication](#kubernetes-apiserver-client-authentication)
      - [Note:](#note)
      - [Original Request Username and Group](#original-request-username-and-group)
    - [Extension Apiserver Authenticates the Request](#extension-apiserver-authenticates-the-request)
    - [Extension Apiserver Authorizes the Request](#extension-apiserver-authorizes-the-request)
    - [Extension Apiserver Executes](#extension-apiserver-executes)
  - [Enable Kubernetes Apiserver flags](#enable-kubernetes-apiserver-flags)
    - [CA Reusage and Conflicts](#ca-reusage-and-conflicts)
      - [Warning:](#warning)
    - [Register APIService objects](#register-apiservice-objects)
      - [Contacting the extension apiserver](#contacting-the-extension-apiserver)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---