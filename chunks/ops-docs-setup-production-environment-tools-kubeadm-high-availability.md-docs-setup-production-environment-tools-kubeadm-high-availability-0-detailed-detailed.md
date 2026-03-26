---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 365
summary: ## Table of Contents  - [Creating Highly Available Clusters with kubeadm](#creating-highly-available-clusters-with-kubeadm)       - [Caution:](#caution)   - [Before you begin](#before-you-begin)    ...
---

## Table of Contents

- [Creating Highly Available Clusters with kubeadm](#creating-highly-available-clusters-with-kubeadm)
      - [Caution:](#caution)
  - [Before you begin](#before-you-begin)
    - [Container images](#container-images)
    - [Command line interface](#command-line-interface)
      - [Note:](#note)
    - [Steps for the first control plane node](#steps-for-the-first-control-plane-node)
      - [Note:](#note)
      - [Note:](#note)
      - [Note:](#note)
      - [Caution:](#caution)
      - [Note:](#note)
    - [Steps for the rest of the control plane nodes](#steps-for-the-rest-of-the-control-plane-nodes)
      - [Note:](#note)
  - [External etcd nodes](#external-etcd-nodes)
    - [Set up the etcd cluster](#set-up-the-etcd-cluster)
      - [Note:](#note)
      - [Note:](#note)
    - [Steps for the rest of the control plane nodes](#steps-for-the-rest-of-the-control-plane-nodes)
    - [Install workers](#install-workers)
  - [Manual certificate distribution](#manual-certificate-distribution)
- [Skip the next line if you are using external etcd](#skip-the-next-line-if-you-are-using-external-etcd)
      - [Caution:](#caution)
- [Skip the next line if you are using external etcd](#skip-the-next-line-if-you-are-using-external-etcd)
  - [Feedback](#feedback)

---