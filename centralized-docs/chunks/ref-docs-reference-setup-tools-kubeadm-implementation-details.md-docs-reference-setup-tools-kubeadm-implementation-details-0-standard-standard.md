---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 503
summary: ## Table of Contents  - [Implementation details](#implementation-details)   - [Core design principles](#core-design-principles)   - [kubeadm init workflow internal...
---

## Table of Contents

- [Implementation details](#implementation-details)
  - [Core design principles](#core-design-principles)
  - [kubeadm init workflow internal design](#kubeadm-init-workflow-internal-design)
    - [Preflight checks](#preflight-checks)
      - [Note:](#note)
    - [Generate the necessary certificates](#generate-the-necessary-certificates)
      - [Caution:](#caution)
    - [Generate static Pod manifest for local etcd](#generate-static-pod-manifest-for-local-etcd)
    - [Save the kubeadm ClusterConfiguration in a ConfigMap for later reference](#save-the-kubeadm-clusterconfiguration-in-a-configmap-for-later-reference)
    - [Configure TLS-Bootstrapping for node joining](#configure-tls-bootstrapping-for-node-joining)
      - [Note:](#note)
      - [Create a bootstrap token](#create-a-bootstrap-token)
      - [Set up auto approval for new bootstrap tokens](#set-up-auto-approval-for-new-bootstrap-tokens)
      - [Set up nodes certificate rotation with auto approval](#set-up-nodes-certificate-rotation-with-auto-approval)
      - [Create the public cluster-info ConfigMap](#create-the-public-cluster-info-configmap)
      - [Note:](#note)
    - [Install addons](#install-addons)
      - [Note:](#note)
      - [proxy](#proxy)
  - [kubeadm join phases internal design](#kubeadm-join-phases-internal-design)
    - [Preflight checks](#preflight-checks)
      - [Shared token discovery](#shared-token-discovery)
      - [Note:](#note)
      - [File/https discovery](#filehttps-discovery)
  - [TLS Bootstrap](#tls-bootstrap)
      - [Note:](#note)
  - [kubeadm upgrade workflow internal design](#kubeadm-upgrade-workflow-internal-design)
    - [kubeadm upgrade plan](#kubeadm-upgrade-plan)
    - [kubeadm upgrade diff](#kubeadm-upgrade-diff)
    - [kubeadm upgrade apply](#kubeadm-upgrade-apply)
  - [Feedback](#feedback)

---