---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#4-standard
chunk_level: standard
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 512
summary: #### Note: If your storage plugin supports [Dynamic provisioning](/docs/concepts/storage/dynamic-provisioning/), the \"cat-picutres-pv\" will be created for you, but its name may differ. To get your...
---

#### Note:
If your storage plugin supports
[Dynamic provisioning](/docs/concepts/storage/dynamic-provisioning/),
the "cat-picutres-pv" will be created for you, but its name may differ. To get
your PersistentVolume's name run:
```
`kubectl get pvc cat-pictures-pvc -o jsonpath='{.spec.volumeName}'
`
```
And you can view the PVC before you make changes. Either view the manifest
locally, or run `kubectl get pvc &lt;name-of-pvc&gt; -o yaml`. The output is similar
to:
```
`# cat-pictures-pvc.yaml
kind: PersistentVolumeClaim
apiVersion: v1
metadata:
name: cat-pictures-pvc
spec:
accessModes:
- ReadWriteOnce
resources:
requests:
storage: 1Gi
`
```
Here's an example Deployment that relies on that PersistentVolumeClaim:
```
`# cat-pictures-writer-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
name: cat-pictures-writer
spec:
replicas: 3
selector:
matchLabels:
app: cat-pictures-writer
template:
metadata:
labels:
app: cat-pictures-writer
spec:
containers:
- name: nginx
image: nginx:1.14.2
ports:
- containerPort: 80
volumeMounts:
- name: cat-pictures
mountPath: /mnt
volumes:
- name: cat-pictures
persistentVolumeClaim:
claimName: cat-pictures-pvc
readOnly: false
`
```
As a first step, you need to edit your PersistentVolume's
`spec.persistentVolumeReclaimPolicy` and set it to `Retain`. This ensures your
PersistentVolume will not be deleted when you delete the corresponding
PersistentVolumeClaim:
```
`kubectl patch pv cat-pictures-pv -p '{"spec":{"persistentVolumeReclaimPolicy":"Retain"}}'
`
```
Next you need to stop any workloads that are using the PersistentVolumeClaim
bound to the PersistentVolume you want to migrate, and then delete the
PersistentVolumeClaim. Avoid making any other changes to the
PersistentVolumeClaim, such as volume resizes, until after the migration is
complete.
Once that is done, you need to clear your PersistentVolume's `spec.claimRef.uid`
to ensure PersistentVolumeClaims can bind to it upon recreation: