---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 420
summary: ## Table of Contents  - [kubectl run](#kubectl-run)   - [Synopsis](#synopsis)   - [Examples](#examples) - [Start a hazelcast pod and let the container expose port...
---

## Table of Contents

- [kubectl run](#kubectl-run)
  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Start a hazelcast pod and let the container expose port 5701](#start-a-hazelcast-pod-and-let-the-container-expose-port-5701)
- [Start a hazelcast pod and set environment variables "DNS\_DOMAIN=cluster" and "POD\_NAMESPACE=default" in the container](#start-a-hazelcast-pod-and-set-environment-variables-dnsdomaincluster-and-podnamespacedefault-in-the-container)
- [Start a hazelcast pod and set labels "app=hazelcast" and "env=prod" in the container](#start-a-hazelcast-pod-and-set-labels-apphazelcast-and-envprod-in-the-container)
- [Dry run; print the corresponding API objects without creating them](#dry-run-print-the-corresponding-api-objects-without-creating-them)
- [Start a nginx pod, but overload the spec with a partial set of values parsed from JSON](#start-a-nginx-pod-but-overload-the-spec-with-a-partial-set-of-values-parsed-from-json)
- [Start a busybox pod and keep it in the foreground, don't restart it if it exits](#start-a-busybox-pod-and-keep-it-in-the-foreground-dont-restart-it-if-it-exits)
- [Start the nginx pod using the default command, but use custom arguments (arg1 .. argN) for that command](#start-the-nginx-pod-using-the-default-command-but-use-custom-arguments-arg1--argn-for-that-command)
- [Start the nginx pod using a different command and custom arguments](#start-the-nginx-pod-using-a-different-command-and-custom-arguments)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---