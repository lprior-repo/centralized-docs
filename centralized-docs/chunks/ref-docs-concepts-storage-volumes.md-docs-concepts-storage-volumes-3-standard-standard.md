---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 321
summary: - [The variable expansion uses round brackets (not curly brackets).](#the-variable-expansion-uses-round-brackets-not-curly-brackets)   - [Resources](#resources)   - [Out-of-tree volume...
---

- [The variable expansion uses round brackets (not curly brackets).](#the-variable-expansion-uses-round-brackets-not-curly-brackets)
  - [Resources](#resources)
  - [Out-of-tree volume plugins](#out-of-tree-volume-plugins)
    - [csi](#csi)
      - [Note:](#note)
      - [Note:](#note)
      - [CSI ephemeral volumes](#csi-ephemeral-volumes)
      - [Windows CSI proxy](#windows-csi-proxy)
      - [Migrating to CSI drivers from in-tree plugins](#migrating-to-csi-drivers-from-in-tree-plugins)
      - [Note:](#note)
    - [flexVolume (deprecated)](#flexvolume-deprecated)
      - [Note:](#note)
      - [Caution:](#caution)
      - [Warning:](#warning)
  - [Read-only mounts](#read-only-mounts)
    - [Recursive read-only mounts](#recursive-read-only-mounts)
- [tmpfs is mounted on /mnt/tmpfs](#tmpfs-is-mounted-on-mnttmpfs)
- [/mnt-rro/tmpfs is not writable](#mnt-rrotmpfs-is-not-writable)
- [/mnt-ro/tmpfs is writable](#mnt-rotmpfs-is-writable)
- [/mnt-rw/tmpfs is writable](#mnt-rwtmpfs-is-writable)
      - [Implementations](#implementations)
  - [Feedback](#feedback)

---