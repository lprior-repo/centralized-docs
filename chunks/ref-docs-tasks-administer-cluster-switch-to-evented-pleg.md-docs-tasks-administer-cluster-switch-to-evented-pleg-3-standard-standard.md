---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#3-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 439
summary: ``` `crio config | grep enable\_pod\_events ` ``` If it is enabled, the output should be similar to the following: ``` `enable\_pod\_events = true ` ``` To enable it, start the CRI-O daemon with the...
---

```
`crio config | grep enable\_pod\_events
`
```
If it is enabled, the output should be similar to the following:
```
`enable\_pod\_events = true
`
```
To enable it, start the CRI-O daemon with the flag `--enable-pod-events=true` or
use a dropin config with the following lines:
```
`[crio.runtime]
enable\_pod\_events: true
`
```
Your Kubernetes server must be at or later than version 1.26.
To check the version, enter `kubectl version`.
4. Verify that the kubelet is using event-based container stage change monitoring.
To check, look for the term `EventedPLEG` in the kubelet logs.
The output should be similar to this:
```
`I0314 11:10:13.909915 1105457 feature\_gate.go:249] feature gates: &amp;{map[EventedPLEG:true]}
`
```
If you have set `--v` to 4 and above, you might see more entries that indicate
that the kubelet is using event-based container state monitoring.
```
`I0314 11:12:42.009542 1110177 evented.go:238] "Evented PLEG: Generated pod status from the received event" podUID=3b2c6172-b112-447a-ba96-94e7022912dc
I0314 11:12:44.623326 1110177 evented.go:238] "Evented PLEG: Generated pod status from the received event" podUID=b3fba5ea-a8c5-4b76-8f43-481e17e8ec40
I0314 11:12:44.714564 1110177 evented.go:238] "Evented PLEG: Generated pod status from the received event" podUID=b3fba5ea-a8c5-4b76-8f43-481e17e8ec40
`
```