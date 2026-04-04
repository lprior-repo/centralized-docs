---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#3-detailed
chunk_level: detailed
chunk_type: table
heading: Language
token_count: 963
summary: ### Use code style for filenames, directories, and paths Do and Don't - Use code style for filenames, directories, and paths|Do|Don't| |Open the `envars.yaml` file.|Open the envars.yaml file.| |Go to...
---

### Use code style for filenames, directories, and paths
Do and Don't - Use code style for filenames, directories, and paths|Do|Don't|
|Open the `envars.yaml` file.|Open the envars.yaml file.|
|Go to the `/docs/tutorials` directory.|Go to the /docs/tutorials directory.|
|Open the `/\_data/concepts.yaml` file.|Open the /\_data/concepts.yaml file.|
### Use the international standard for punctuation inside quotes
Do and Don't - Use the international standard for punctuation inside quotes|Do|Don't|
|events are recorded with an associated "stage".|events are recorded with an associated "stage."|
|The copy is called a "fork".|The copy is called a "fork."|
### Use code style for inline code, commands
For inline code in an HTML document, use the `&lt;code&gt;` tag. In a Markdown
document, use the backtick (```). However, API kinds such as StatefulSet
or ConfigMap are written verbatim (no backticks); this allows using possessive
apostrophes.
Do and Don't - Use code style for inline code, commands, and API objects|Do|Don't|
|The `kubectl run` command creates a Pod.|The "kubectl run" command creates a Pod.|
|The kubelet on each node acquires a Lease…|The kubelet on each node acquires a `Lease`…|
|A PersistentVolume represents durable storage…|A `PersistentVolume` represents durable storage…|
|The CustomResourceDefinition's `.spec.group` field…|The `CustomResourceDefinition.spec.group` field…|
|For declarative management, use `kubectl apply`.|For declarative management, use "kubectl apply".|
|Enclose code samples with triple backticks. (```)|Enclose code samples with any other syntax.|
|Use single backticks to enclose inline code. For example, `var example = true`.|Use two asterisks (`\*\*`) or an underscore (`\_`) to enclose inline code. For example, **var example = true**.|
|Use triple backticks before and after a multi-line block of code for fenced code blocks.|Use multi-line blocks of code to create diagrams, flowcharts, or other illustrations.|
|Use meaningful variable names that have a context.|Use variable names such as 'foo','bar', and 'baz' that are not meaningful and lack context.|
|Remove trailing spaces in the code.|Add trailing spaces in the code, where these are important, because the screen reader will read out the spaces as well.|
#### Note:
The website supports syntax highlighting for code samples, but specifying a language
is optional. Syntax highlighting in the code block should conform to the
[contrast guidelines.](https://www.w3.org/WAI/WCAG21/quickref/?versions=2.0&amp;showtechniques=141,143#contrast-minimum)
### Use code style for object field names and namespaces
Do and Don't - Use code style for object field names|Do|Don't|
|Set the value of the `replicas` field in the configuration file.|Set the value of the "replicas" field in the configuration file.|
|The value of the `exec` field is an ExecAction object.|The value of the "exec" field is an ExecAction object.|
|Run the process as a DaemonSet in the `kube-system` namespace.|Run the process as a DaemonSet in the kube-system namespace.|
### Use code style for Kubernetes command tool and component names
Do and Don't - Use code style for Kubernetes command tool and component names|Do|Don't|
|The `kubelet` preserves node stability.|The kubelet preserves node stability.|
|The `kubectl` handles locating and authenticating to the API server.|The kubectl handles locating and authenticating to the apiserver.|
|Run the process with the certificate, `kube-apiserver --client-ca-file=FILENAME`.|Run the process with the certificate, kube-apiserver --client-ca-file=FILENAME.|
### Starting a sentence with a component tool or component name
Do and Don't - Starting a sentence with a component tool or component name|Do|Don't|
|The `kubeadm` tool bootstraps and provisions machines in a cluster.|`kubeadm` tool bootstraps and provisions machines in a cluster.|
|The kube-scheduler is the default scheduler for Kubernetes.|kube-scheduler is the default scheduler for Kubernetes.|