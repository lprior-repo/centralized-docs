---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#4-standard
chunk_level: standard
chunk_type: prose
heading: Uses for Secrets
token_count: 466
summary: ## Uses for Secrets You can use Secrets for purposes such as the following: * [Set environment variables for a...
---

## Uses for Secrets
You can use Secrets for purposes such as the following:
* [Set environment variables for a container](/docs/tasks/inject-data-application/distribute-credentials-secure/#define-container-environment-variables-using-secret-data).
* [Provide credentials such as SSH keys or passwords to Pods](/docs/tasks/inject-data-application/distribute-credentials-secure/#provide-prod-test-creds).
* [Allow the kubelet to pull container images from private registries](/docs/tasks/configure-pod-container/pull-image-private-registry/).
The Kubernetes control plane also uses Secrets; for example,
[bootstrap token Secrets](#bootstrap-token-secrets) are a mechanism to
help automate node registration.
### Use case: dotfiles in a secret volume
You can make your data "hidden" by defining a key that begins with a dot.
This key represents a dotfile or "hidden" file. For example, when the following Secret
is mounted into a volume, `secret-volume`, the volume will contain a single file,
called `.secret-file`, and the `dotfile-test-container` will have this file
present at the path `/etc/secret-volume/.secret-file`.
#### Note:
Files beginning with dot characters are hidden from the output of `ls -l`;
you must use `ls -la` to see them when listing directory contents.
[`secret/dotfile-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/dotfile-secret.yaml)![](/images/copycode.svg "Copy secret/dotfile-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: dotfile-secret
data:
.secret-file: dmFsdWUtMg0KDQo=
---
apiVersion: v1
kind: Pod
metadata:
name: secret-dotfiles-pod
spec:
volumes:
- name: secret-volume
secret:
secretName: dotfile-secret
containers:
- name: dotfile-test-container
image: registry.k8s.io/busybox
command:
- ls
- "-l"
- "/etc/secret-volume"
volumeMounts:
- name: secret-volume
readOnly: true
mountPath: "/etc/secret-volume"`
```