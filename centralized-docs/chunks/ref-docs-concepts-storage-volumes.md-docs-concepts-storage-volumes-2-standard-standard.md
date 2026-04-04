---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 305
summary: - [The mount into the container is read-only.](#the-mount-into-the-container-is-read-only) - [mount C:\\Data\\foo from the host, but only if that directory already...
---

- [The mount into the container is read-only.](#the-mount-into-the-container-is-read-only)
- [mount C:\\Data\\foo from the host, but only if that directory already exists](#mount-cdatafoo-from-the-host-but-only-if-that-directory-already-exists)
      - [hostPath FileOrCreate configuration example](#hostpath-fileorcreate-configuration-example)
- [Ensure the file directory is created.](#ensure-the-file-directory-is-created)
    - [image](#image)
    - [iscsi](#iscsi)
      - [Note:](#note)
    - [local](#local)
      - [Note:](#note)
    - [nfs](#nfs)
      - [Note:](#note)
    - [persistentVolumeClaim](#persistentvolumeclaim)
    - [portworxVolume (deprecated)](#portworxvolume-deprecated)
- [This Portworx volume must already exist.](#this-portworx-volume-must-already-exist)
      - [Note:](#note)
      - [Portworx CSI migration](#portworx-csi-migration)
    - [projected](#projected)
    - [secret](#secret)
      - [Note:](#note)
  - [Using subPath](#using-subpath)
    - [Using subPath with expanded environment variables](#using-subpath-with-expanded-environment-variables)