---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Uses for Secrets
token_count: 995
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
### Use case: Secret visible to one container in a Pod
Consider a program that needs to handle HTTP requests, do some complex business
logic, and then sign some messages with an HMAC. Because it has complex
application logic, there might be an unnoticed remote file reading exploit in
the server, which could expose the private key to an attacker.
This could be divided into two processes in two containers: a frontend container
which handles user interaction and business logic, but which cannot see the
private key; and a signer container that can see the private key, and responds
to simple signing requests from the frontend (for example, over localhost networking).
With this partitioned approach, an attacker now has to trick the application
server into doing something rather arbitrary, which may be harder than getting
it to read a file.
### Alternatives to Secrets
Rather than using a Secret to protect confidential data, you can pick from alternatives.
Here are some of your options:
* If your cloud-native component needs to authenticate to another application that you
know is running within the same Kubernetes cluster, you can use a
[ServiceAccount](/docs/reference/access-authn-authz/authentication/#service-account-tokens)
and its tokens to identify your client.
* There are third-party tools that you can run, either within or outside your cluster,
that manage sensitive data. For example, a service that Pods access over HTTPS,
that reveals a Secret if the client correctly authenticates (for example, with a ServiceAccount
token).
* For authentication, you can implement a custom signer for X.509 certificates, and use
[CertificateSigningRequests](/docs/reference/access-authn-authz/certificate-signing-requests/)
to let that custom signer issue certificates to Pods that need them.
* You can use a [device plugin](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/)
to expose node-local encryption hardware to a specific Pod. For example, you can schedule
trusted Pods onto nodes that provide a Trusted Platform Module, configured out-of-band.
You can also combine two or more of those options, including the option to use Secret objects themselves.
For example: implement (or deploy) an [operator](/docs/concepts/extend-kubernetes/operator/)
that fetches short-lived session tokens from an external service, and then creates Secrets based
on those short-lived session tokens. Pods running in your cluster can make use of the session tokens,
and operator ensures they are valid. This separation means that you can run Pods that are unaware of
the exact mechanisms for issuing and refreshing those session tokens.