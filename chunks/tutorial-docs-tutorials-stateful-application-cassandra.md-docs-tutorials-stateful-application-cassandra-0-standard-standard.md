---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 438
summary: ## Table of Contents  - [Example: Deploying Cassandra with a StatefulSet](#example-deploying-cassandra-with-a-statefulset)       - [Note:](#note)   - [Objectives](#objectives)       -...
---

## Table of Contents

- [Example: Deploying Cassandra with a StatefulSet](#example-deploying-cassandra-with-a-statefulset)
      - [Note:](#note)
  - [Objectives](#objectives)
      - [Caution:](#caution)
  - [Creating a headless Service for Cassandra](#creating-a-headless-service-for-cassandra)
    - [Validating (optional)](#validating-optional)
  - [Using a StatefulSet to create a Cassandra ring](#using-a-statefulset-to-create-a-cassandra-ring)
      - [Note:](#note)
- [These volume mounts are persistent. They are like inline claims,](#these-volume-mounts-are-persistent-they-are-like-inline-claims)
- [but not exactly because the names need to match exactly one of](#but-not-exactly-because-the-names-need-to-match-exactly-one-of)
- [the stateful pod volumes.](#the-stateful-pod-volumes)
- [These are converted to volume claims by the controller](#these-are-converted-to-volume-claims-by-the-controller)
- [do not use these in production until ssd GCEPersistentDisk or other ssd pd](#do-not-use-these-in-production-until-ssd-gcepersistentdisk-or-other-ssd-pd)
  - [Validating the Cassandra StatefulSet](#validating-the-cassandra-statefulset)
  - [Modifying the Cassandra StatefulSet](#modifying-the-cassandra-statefulset)
- [and an empty file will abort the edit. If an error occurs while saving this file will be](#and-an-empty-file-will-abort-the-edit-if-an-error-occurs-while-saving-this-file-will-be)
  - [Cleaning up](#cleaning-up)
      - [Warning:](#warning)
  - [Cassandra container environment variables](#cassandra-container-environment-variables)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---