---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 452
summary: ## Table of Contents  - [Creating a cluster with kubeadm](#creating-a-cluster-with-kubeadm)   - [Before you begin](#before-you-begin)       - [Note:](#note)   - [Objectives](#objectives)       -...
---

## Table of Contents

- [Creating a cluster with kubeadm](#creating-a-cluster-with-kubeadm)
  - [Before you begin](#before-you-begin)
      - [Note:](#note)
  - [Objectives](#objectives)
      - [Component installation](#component-installation)
      - [Note:](#note)
      - [Network setup](#network-setup)
      - [Note:](#note)
      - [Note:](#note)
      - [Warning:](#warning)
    - [Preparing the required container images](#preparing-the-required-container-images)
    - [Initializing your control-plane node](#initializing-your-control-plane-node)
    - [Considerations about apiserver-advertise-address and ControlPlaneEndpoint](#considerations-about-apiserver-advertise-address-and-controlplaneendpoint)
    - [More information](#more-information)
      - [Warning:](#warning)
      - [Caution:](#caution)
      - [Note:](#note)
      - [Note:](#note)
    - [Managed node labels](#managed-node-labels)
    - [Control plane node isolation](#control-plane-node-isolation)
    - [Adding more control plane nodes](#adding-more-control-plane-nodes)
    - [Adding worker nodes](#adding-worker-nodes)
      - [Note:](#note)
    - [(Optional) Proxying API Server to localhost](#optional-proxying-api-server-to-localhost)
  - [Clean up](#clean-up)
    - [Remove the node](#remove-the-node)
    - [Clean up the control plane](#clean-up-the-control-plane)
  - [Version skew policy](#version-skew-policy)
    - [kubeadm's skew against the Kubernetes version](#kubeadms-skew-against-the-kubernetes-version)
    - [Cluster resilience](#cluster-resilience)
    - [Platform compatibility](#platform-compatibility)
  - [Troubleshooting](#troubleshooting)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---