---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 344
summary: ## Table of Contents  - [kubectl create ingress](#kubectl-create-ingress)   - [Synopsis](#synopsis)   - [Examples](#examples) - [svc1:8080 with a TLS secret...
---

## Table of Contents

- [kubectl create ingress](#kubectl-create-ingress)
  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [svc1:8080 with a TLS secret "my-cert"](#svc18080-with-a-tls-secret-my-cert)
- [Create a catch all ingress of "/path" pointing to service svc:port and Ingress Class as "otheringress"](#create-a-catch-all-ingress-of-path-pointing-to-service-svcport-and-ingress-class-as-otheringress)
- [Create an ingress with two annotations: ingress.annotation1 and ingress.annotations2](#create-an-ingress-with-two-annotations-ingressannotation1-and-ingressannotations2)
- [Create an ingress with the same host and multiple paths](#create-an-ingress-with-the-same-host-and-multiple-paths)
- [Create an ingress with multiple hosts and the pathType as Prefix](#create-an-ingress-with-multiple-hosts-and-the-pathtype-as-prefix)
- [Create an ingress with TLS enabled using the default ingress certificate and different path types](#create-an-ingress-with-tls-enabled-using-the-default-ingress-certificate-and-different-path-types)
- [Create an ingress with TLS enabled using a specific secret and pathType as Prefix](#create-an-ingress-with-tls-enabled-using-a-specific-secret-and-pathtype-as-prefix)
- [Create an ingress with a default backend](#create-an-ingress-with-a-default-backend)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---