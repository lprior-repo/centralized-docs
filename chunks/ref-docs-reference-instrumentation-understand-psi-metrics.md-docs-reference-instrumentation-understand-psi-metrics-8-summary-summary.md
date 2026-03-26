---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 103
summary: * The Linux kernel must be version **4.20 or newer**. * The kernel must be compiled with the `CONFIG\_PSI=y` option. Most modern distributions enable this by default. You can check your kernel's...
---

* The Linux kernel must be version **4.20 or newer**.
* The kernel must be compiled with the `CONFIG\_PSI=y` option. Most modern distributions enable this by default. You can check your kernel's configuration by running `zgrep CONFIG\_PSI /proc/config.gz`.
* Some Linux distributions may compile PSI into the kernel but disable it by default. If so, you need to enable it at boot time by adding the `psi=1` parameter to the kernel command line.