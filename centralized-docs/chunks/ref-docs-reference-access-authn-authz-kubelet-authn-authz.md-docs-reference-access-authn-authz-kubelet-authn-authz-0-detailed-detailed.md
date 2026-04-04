---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Overview
token_count: 100
summary: ## Table of Contents    - [Overview](#overview)   - [Kubelet authentication](#kubelet-authentication)       - [Warning:](#warning)   - [Feedback](#feedback)  ---  ## Overview A kubelet's HTTPS...
---

## Table of Contents

  - [Overview](#overview)
  - [Kubelet authentication](#kubelet-authentication)
      - [Warning:](#warning)
  - [Feedback](#feedback)

---

## Overview
A kubelet's HTTPS endpoint exposes APIs which give access to data of varying sensitivity,
and allow you to perform operations with varying levels of power on the node and within containers.
This document describes how to authenticate and authorize access to the kubelet's HTTPS endpoint.