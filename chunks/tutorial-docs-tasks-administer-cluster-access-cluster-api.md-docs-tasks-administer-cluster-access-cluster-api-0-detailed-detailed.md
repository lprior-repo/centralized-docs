---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 390
summary: ## Table of Contents  - [Access Clusters Using the Kubernetes API](#access-clusters-using-the-kubernetes-api)   - [Before you begin](#before-you-begin)     - [Accessing for the first time with...
---

## Table of Contents

- [Access Clusters Using the Kubernetes API](#access-clusters-using-the-kubernetes-api)
  - [Before you begin](#before-you-begin)
    - [Accessing for the first time with kubectl](#accessing-for-the-first-time-with-kubectl)
    - [Directly accessing the REST API](#directly-accessing-the-rest-api)
      - [Using kubectl proxy](#using-kubectl-proxy)
      - [Without kubectl proxy](#without-kubectl-proxy)
- [Select name of cluster you want to interact with from above output:](#select-name-of-cluster-you-want-to-interact-with-from-above-output)
- [Point to the API server referring the cluster name](#point-to-the-api-server-referring-the-cluster-name)
- [Create a secret to hold a token for the default service account](#create-a-secret-to-hold-a-token-for-the-default-service-account)
- [Wait for the token controller to populate the secret with a token:](#wait-for-the-token-controller-to-populate-the-secret-with-a-token)
- [Get the token value](#get-the-token-value)
- [Explore the API with TOKEN](#explore-the-api-with-token)
    - [Programmatic access to the API](#programmatic-access-to-the-api)
      - [Go client](#go-client)
      - [Note:](#note)
      - [Python client](#python-client)
      - [Java client](#java-client)
- [Installing project artifacts, POM etc:](#installing-project-artifacts-pom-etc)
      - [dotnet client](#dotnet-client)
      - [JavaScript client](#javascript-client)
      - [Haskell client](#haskell-client)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---