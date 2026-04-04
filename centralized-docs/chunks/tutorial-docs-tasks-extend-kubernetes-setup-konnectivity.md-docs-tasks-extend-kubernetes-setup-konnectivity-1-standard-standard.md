---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 492
summary: - [Set up Konnectivity service](#set-up-konnectivity-service)   - [Before you begin](#before-you-begin)   - [Configure the Konnectivity service](#configure-the-konnectivity-service) - [Since we want...
---

- [Set up Konnectivity service](#set-up-konnectivity-service)
  - [Before you begin](#before-you-begin)
  - [Configure the Konnectivity service](#configure-the-konnectivity-service)
- [Since we want to control the egress traffic to the cluster, we use the](#since-we-want-to-control-the-egress-traffic-to-the-cluster-we-use-the)
- ["cluster" as the name. Other supported values are "etcd", and "controlplane".](#cluster-as-the-name-other-supported-values-are-etcd-and-controlplane)
- [server. Supported values are "GRPC" and "HTTPConnect". There is no](#server-supported-values-are-grpc-and-httpconnect-there-is-no)
- [end user visible difference between the two modes. You need to set the](#end-user-visible-difference-between-the-two-modes-you-need-to-set-the)
- [Konnectivity server to work in the same mode.](#konnectivity-server-to-work-in-the-same-mode)
- [This controls what transport the API Server uses to communicate with the](#this-controls-what-transport-the-api-server-uses-to-communicate-with-the)
- [Konnectivity server. UDS is recommended if the Konnectivity server](#konnectivity-server-uds-is-recommended-if-the-konnectivity-server)
- [locates on the same machine as the API Server. You need to configure the](#locates-on-the-same-machine-as-the-api-server-you-need-to-configure-the)
- [Konnectivity server to listen on the same UDS socket.](#konnectivity-server-to-listen-on-the-same-uds-socket)
- [The other supported transport is "tcp". You will need to set up TLS](#the-other-supported-transport-is-tcp-you-will-need-to-set-up-tls)
- [config to secure the TCP transport.](#config-to-secure-the-tcp-transport)
- [This needs to be consistent with the value set in egressSelectorConfiguration.](#this-needs-to-be-consistent-with-the-value-set-in-egressselectorconfiguration)
- [The following two lines assume the Konnectivity server is](#the-following-two-lines-assume-the-konnectivity-server-is)