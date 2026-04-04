---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#134-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 128
summary: Pods interact with FlexVolume drivers through the `flexVolume` in-tree volume plugin. The following FlexVolume...
---

Pods interact with FlexVolume drivers through the `flexVolume` in-tree volume plugin.
The following FlexVolume [plugins](https://github.com/Microsoft/K8s-Storage-Plugins/tree/master/flexvolume/windows),
deployed as PowerShell scripts on the host, support Windows nodes:
* [SMB](https://github.com/microsoft/K8s-Storage-Plugins/tree/master/flexvolume/windows/plugins/microsoft.com~smb.cmd)
* [iSCSI](https://github.com/microsoft/K8s-Storage-Plugins/tree/master/flexvolume/windows/plugins/microsoft.com~iscsi.cmd)
#### Note: