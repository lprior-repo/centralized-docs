---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#17-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 117
summary: * Undefined/nil * `AUDIT\_WRITE` * `CHOWN` * `DAC\_OVERRIDE` * `FOWNER` * `FSETID` * `KILL` * `MKNOD` * `NET\_BIND\_SERVICE` * `SETFCAP` * `SETGID` * `SETPCAP` * `SETUID` * `SYS\_CHROOT`| |HostPath...
---

* Undefined/nil
* `AUDIT\_WRITE`
* `CHOWN`
* `DAC\_OVERRIDE`
* `FOWNER`
* `FSETID`
* `KILL`
* `MKNOD`
* `NET\_BIND\_SERVICE`
* `SETFCAP`
* `SETGID`
* `SETPCAP`
* `SETUID`
* `SYS\_CHROOT`|
|HostPath Volumes|
HostPath volumes must be forbidden.
**Restricted Fields**
* `spec.volumes[\*].hostPath`
**Allowed Values**