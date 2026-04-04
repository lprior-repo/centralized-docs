---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#14-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 124
summary: 1. Make sure that [Service Account Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection) feature enabled in your cluster. It...
---

1. Make sure that
[Service Account Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection)
feature enabled in your cluster. It is enabled by default since Kubernetes v1.20.
2. Create an egress configuration file such as `admin/konnectivity/egress-selector-configuration.yaml`.
3. Set the `--egress-selector-config-file` flag of the API Server to the path of
your API Server egress configuration file.
4. If you use UDS connection, add volumes config to the kube-apiserver: