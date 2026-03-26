---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#10-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of Secret
token_count: 972
summary: ### Using Secrets as files from a Pod If you want to access data from a Secret in a Pod, one way to do that is to have Kubernetes make the value of that Secret be available as a file inside the...
---

### Using Secrets as files from a Pod
If you want to access data from a Secret in a Pod, one way to do that is to
have Kubernetes make the value of that Secret be available as a file inside
the filesystem of one or more of the Pod's containers.
For instructions, refer to
[Create a Pod that has access to the secret data through a Volume](/docs/tasks/inject-data-application/distribute-credentials-secure/#create-a-pod-that-has-access-to-the-secret-data-through-a-volume).
When a volume contains data from a Secret, and that Secret is updated, Kubernetes tracks
this and updates the data in the volume, using an eventually-consistent approach.
#### Note:
A container using a Secret as a
[subPath](/docs/concepts/storage/volumes/#using-subpath) volume mount does not receive
automated Secret updates.
The kubelet keeps a cache of the current keys and values for the Secrets that are used in
volumes for pods on that node.
You can configure the way that the kubelet detects changes from the cached values. The
`configMapAndSecretChangeDetectionStrategy` field in the
[kubelet configuration](/docs/reference/config-api/kubelet-config.v1beta1/) controls
which strategy the kubelet uses. The default strategy is `Watch`.
Updates to Secrets can be either propagated by an API watch mechanism (the default), based on
a cache with a defined time-to-live, or polled from the cluster API server on each kubelet
synchronisation loop.
As a result, the total delay from the moment when the Secret is updated to the moment
when new keys are projected to the Pod can be as long as the kubelet sync period + cache
propagation delay, where the cache propagation delay depends on the chosen cache type
(following the same order listed in the previous paragraph, these are:
watch propagation delay, the configured cache TTL, or zero for direct polling).
### Using Secrets as environment variables
To use a Secret in an [environment variable](/docs/concepts/containers/container-environment/)
in a Pod:
1. For each container in your Pod specification, add an environment variable
for each Secret key that you want to use to the
`env[].valueFrom.secretKeyRef` field.
2. Modify your image and/or command line so that the program looks for values
in the specified environment variables.
For instructions, refer to
[Define container environment variables using Secret data](/docs/tasks/inject-data-application/distribute-credentials-secure/#define-container-environment-variables-using-secret-data).
It's important to note that the range of characters allowed for environment variable
names in pods is [restricted](/docs/tasks/inject-data-application/define-environment-variable-container/#using-environment-variables-inside-of-your-config).
If any keys do not meet the rules, those keys are not made available to your container, though
the Pod is allowed to start.
### Container image pull Secrets
If you want to fetch container images from a private repository, you need a way for
the kubelet on each node to authenticate to that repository. You can configure
*image pull Secrets* to make this possible. These Secrets are configured at the Pod
level.
#### Using imagePullSecrets
The `imagePullSecrets` field is a list of references to Secrets in the same namespace.
You can use an `imagePullSecrets` to pass a Secret that contains a Docker (or other) image registry
password to the kubelet. The kubelet uses this information to pull a private image on behalf of your Pod.
See the [PodSpec API](/docs/reference/generated/kubernetes-api/v1.35/#podspec-v1-core)
for more information about the `imagePullSecrets` field.
##### Manually specifying an imagePullSecret
You can learn how to specify `imagePullSecrets` from the
[container images](/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod)
documentation.
##### Arranging for imagePullSecrets to be automatically attached
You can manually create `imagePullSecrets`, and reference these from a ServiceAccount. Any Pods
created with that ServiceAccount or created with that ServiceAccount by default, will get their
`imagePullSecrets` field set to that of the service account.
See [Add ImagePullSecrets to a service account](/docs/tasks/configure-pod-container/configure-service-account/#add-imagepullsecrets-to-a-service-account)
for a detailed explanation of that process.
### Using Secrets with static Pods
You cannot use ConfigMaps or Secrets with [static Pods](/docs/tasks/configure-pod-container/static-pod/).