---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 202
summary: ## Table of Contents  - [Hardening Guide - Scheduler Configuration](#hardening-guide---scheduler-configuration)     - [Scheduler authentication &amp; authorization command line...
---

## Table of Contents

- [Hardening Guide - Scheduler Configuration](#hardening-guide---scheduler-configuration)
    - [Scheduler authentication &amp; authorization command line options](#scheduler-authentication-amp-authorization-command-line-options)
    - [Key considerations](#key-considerations)
- [You can disable all plugins for an extension point using "\*"](#you-can-disable-all-plugins-for-an-extension-point-using-)
- [- name: "PrioritySort" # Disable specific queueSort plugin](#--name-prioritysort--disable-specific-queuesort-plugin)
- [- name: "NodeResourcesFit" # Disable specific filter plugin](#--name-noderesourcesfit--disable-specific-filter-plugin)
- [- name: "TaintToleration" # Disable specific permit plugin](#--name-tainttoleration--disable-specific-permit-plugin)
  - [Disallow labeling nodes](#disallow-labeling-nodes)
  - [Feedback](#feedback)

---