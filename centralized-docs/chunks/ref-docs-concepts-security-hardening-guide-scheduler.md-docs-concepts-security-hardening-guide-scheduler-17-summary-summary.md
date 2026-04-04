---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#17-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 125
summary: `apiVersion: kubescheduler.config.k8s.io/v1 kind: KubeSchedulerConfiguration profiles: - schedulerName: my-scheduler plugins: # You can disable all plugins for an extension point using \"\*\"...
---

`apiVersion: kubescheduler.config.k8s.io/v1
kind: KubeSchedulerConfiguration
profiles:
- schedulerName: my-scheduler
plugins:
# You can disable all plugins for an extension point using "\*"
queueSort:
disabled:
- name: "\*" # Disable all queueSort plugins
# - name: "PrioritySort" # Disable specific queueSort plugin
filter:
disabled:
- name: "\*" # Disable all filter plugins
# - name: "NodeResourcesFit" # Disable specific filter plugin
permit:
disabled:
- name: "\*" # Disables all permit plugins