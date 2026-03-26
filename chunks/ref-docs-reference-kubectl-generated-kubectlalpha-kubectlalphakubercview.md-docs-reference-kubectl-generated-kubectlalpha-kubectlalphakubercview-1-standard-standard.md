---
doc_id: ref/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview.md/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview
chunk_id: ref/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview.md/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview#1-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 356
summary: # kubectl alpha kuberc view Display the current kuberc configuration ## Synopsis Display the contents of the kuberc file in the specified output format. ``` `kubectl alpha kuberc view ` ``` ##...
---

# kubectl alpha kuberc view
Display the current kuberc configuration
## Synopsis
Display the contents of the kuberc file in the specified output format.
```
`kubectl alpha kuberc view
`
```
## Examples
```
` # View kuberc configuration in YAML format (default)
kubectl alpha kuberc view
# View kuberc configuration in JSON format
kubectl alpha kuberc view --output json
# View a specific kuberc file
kubectl alpha kuberc view --kuberc /path/to/kuberc
`
```
## Options
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|-h, --help|
||
help for view
|
|--kuberc string|
||
Path to the kuberc file to use for preferences. This can be disabled by exporting KUBECTL\_KUBERC=false feature gate or turning off the feature KUBERC=off.
|
|-o, --output stringDefault: "yaml"|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|