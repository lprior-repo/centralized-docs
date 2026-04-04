---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#24-standard
chunk_level: standard
chunk_type: prose
heading: Using subPath
token_count: 508
summary: # This Portworx volume must already exist. portworxVolume: volumeID: \"pxvol\" fsType: \"&lt;fs-type&gt;\" ` ``` #### Note: Make sure you have an existing PortworxVolume with the name `pxvol` before...
---

# This Portworx volume must already exist.
portworxVolume:
volumeID: "pxvol"
fsType: "&lt;fs-type&gt;"
`
```
#### Note:
Make sure you have an existing PortworxVolume with the name `pxvol`
before using it in the Pod.
#### Portworx CSI migration
FEATURE STATE:
`Kubernetes v1.33 [stable]`(enabled by default)
In Kubernetes 1.35, all operations for the in-tree
Portworx volumes are redirected to the `pxd.portworx.com`
Container Storage Interface (CSI) Driver by default.
[Portworx CSI Driver](https://docs.portworx.com/portworx-enterprise/operations/operate-kubernetes/storage-operations/csi)
must be installed on the cluster.
### projected
A projected volume maps several existing volume sources into the same
directory. For more details, see [projected volumes](/docs/concepts/storage/projected-volumes/).
### secret
A `secret` volume is used to pass sensitive information, such as passwords, to
Pods. You can store secrets in the Kubernetes API and mount them as files for
use by Pods without coupling to Kubernetes directly. `secret` volumes are
backed by tmpfs (a RAM-backed filesystem), so they are never written to
non-volatile storage.
#### Note:
* You must create a Secret in the Kubernetes API before you can use it.
* A Secret is always mounted as `readOnly`.
* A container using a Secret as a [`subPath`](#using-subpath) volume mount will not
receive Secret updates.
For more details, see [Configuring Secrets](/docs/concepts/configuration/secret/).
## Using subPath
Sometimes, it is useful to share one volume for multiple uses in a single Pod.
The `volumeMounts[\*].subPath` property specifies a sub-path inside the referenced volume
instead of its root.
The following example shows how to configure a Pod with a LAMP stack (Linux, Apache, MySQL, PHP)
using a single, shared volume. This sample `subPath` configuration is not recommended
for production use.
The PHP application's code and assets map to the volume's `html` folder and
the MySQL database is stored in the volume's `mysql` folder. For example:
```
`apiVersion: v1
kind: Pod
metadata:
name: my-lamp-site
spec:
containers: