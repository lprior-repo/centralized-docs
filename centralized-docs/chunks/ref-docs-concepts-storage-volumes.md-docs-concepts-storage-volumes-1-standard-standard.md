---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 441
summary: - [Volumes](#volumes)   - [Why volumes are important](#why-volumes-are-important)   - [How volumes work](#how-volumes-work)   - [Types of volumes](#types-of-volumes)     - [configMap](#configmap)    ...
---

- [Volumes](#volumes)
  - [Why volumes are important](#why-volumes-are-important)
  - [How volumes work](#how-volumes-work)
  - [Types of volumes](#types-of-volumes)
    - [configMap](#configmap)
      - [Note:](#note)
    - [downwardAPI](#downwardapi)
      - [Note:](#note)
    - [emptyDir](#emptydir)
      - [Note:](#note)
      - [Caution:](#caution)
      - [emptyDir configuration example](#emptydir-configuration-example)
      - [emptyDir memory configuration example](#emptydir-memory-configuration-example)
    - [fc (fibre channel)](#fc-fibre-channel)
      - [Note:](#note)
    - [gcePersistentDisk (deprecated)](#gcepersistentdisk-deprecated)
      - [Warning:](#warning)
    - [hostPath](#hostpath)
      - [Warning:](#warning)
      - [Caution:](#caution)
- [This manifest mounts /data/foo on the host as /foo inside the](#this-manifest-mounts-datafoo-on-the-host-as-foo-inside-the)
- [The mount into the container is read-only.](#the-mount-into-the-container-is-read-only)
- [mount /data/foo, but only if that directory already exists](#mount-datafoo-but-only-if-that-directory-already-exists)
- [This manifest mounts C:\\Data\\foo on the host as C:\\foo, inside the](#this-manifest-mounts-cdatafoo-on-the-host-as-cfoo-inside-the)
- [The mount into the container is read-only.](#the-mount-into-the-container-is-read-only)
- [mount C:\\Data\\foo from the host, but only if that directory already exists](#mount-cdatafoo-from-the-host-but-only-if-that-directory-already-exists)
      - [hostPath FileOrCreate configuration example](#hostpath-fileorcreate-configuration-example)