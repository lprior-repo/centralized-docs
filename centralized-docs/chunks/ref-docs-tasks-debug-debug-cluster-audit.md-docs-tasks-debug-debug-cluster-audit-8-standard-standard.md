---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#8-standard
chunk_level: standard
chunk_type: prose
heading: Audit backends
token_count: 509
summary: ### Log backend The log backend writes audit events to a file in [JSONlines](https://jsonlines.org/) format. You can configure the log audit backend using the following `kube-apiserver` flags: *...
---

### Log backend
The log backend writes audit events to a file in [JSONlines](https://jsonlines.org/) format.
You can configure the log audit backend using the following `kube-apiserver` flags:
* `--audit-log-path` specifies the log file path that log backend uses to write
audit events. Not specifying this flag disables log backend. `-` means standard out
* `--audit-log-maxage` defined the maximum number of days to retain old audit log files
* `--audit-log-maxbackup` defines the maximum number of audit log files to retain
* `--audit-log-maxsize` defines the maximum size in megabytes of the audit log file before it gets rotated
If your cluster's control plane runs the kube-apiserver as a Pod, remember to mount the `hostPath`
to the location of the policy file and log file, so that audit records are persisted. For example:
```
` - --audit-policy-file=/etc/kubernetes/audit-policy.yaml
- --audit-log-path=/var/log/kubernetes/audit/audit.log
`
```
then mount the volumes:
```
`...
volumeMounts:
- mountPath: /etc/kubernetes/audit-policy.yaml
name: audit
readOnly: true
- mountPath: /var/log/kubernetes/audit/
name: audit-log
readOnly: false
`
```
and finally configure the `hostPath`:
```
`...
volumes:
- name: audit
hostPath:
path: /etc/kubernetes/audit-policy.yaml
type: File
- name: audit-log
hostPath:
path: /var/log/kubernetes/audit/
type: DirectoryOrCreate
`
```
### Webhook backend
The webhook audit backend sends audit events to a remote web API, which is assumed to
be a form of the Kubernetes API, including means of authentication. You can configure
a webhook audit backend using the following kube-apiserver flags:
* `--audit-webhook-config-file` specifies the path to a file with a webhook
configuration. The webhook configuration is effectively a specialized
[kubeconfig](/docs/tasks/access-application-cluster/configure-access-multiple-clusters/).
* `--audit-webhook-initial-backoff` specifies the amount of time to wait after the first failed
request before retrying. Subsequent requests are retried with exponential backoff.
The webhook config file uses the kubeconfig format to specify the remote address of
the service and credentials used to connect to it.