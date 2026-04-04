---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#15-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 363
summary: \_RSA\_WITH\_AES\_128\_GCM\_SHA256, TLS\_RSA\_WITH\_AES\_256\_CBC\_SHA, TLS\_RSA\_WITH\_AES\_256\_GCM\_SHA384, TLS\_RSA\_WITH\_RC4\_128\_SHA. | |--tls-min-version string| || Minimum TLS version...
---

\_RSA\_WITH\_AES\_128\_GCM\_SHA256, TLS\_RSA\_WITH\_AES\_256\_CBC\_SHA, TLS\_RSA\_WITH\_AES\_256\_GCM\_SHA384, TLS\_RSA\_WITH\_RC4\_128\_SHA.
|
|--tls-min-version string|
||
Minimum TLS version supported. Possible values: VersionTLS10, VersionTLS11, VersionTLS12, VersionTLS13
|
|--tls-private-key-file string|
||
File containing the default x509 private key matching --tls-cert-file.
|
|--tls-sni-cert-key string|
||
A pair of x509 certificate and private key file paths, optionally suffixed with a list of domain patterns which are fully qualified domain names, possibly with prefixed wildcard segments. The domain patterns also allow IP addresses, but IPs should only be used if the apiserver has visibility to the IP address requested by a client. If no domain patterns are provided, the names of the certificate are extracted. Non-wildcard matches trump over wildcard matches, explicit domain patterns trump over extracted names. For multiple key/certificate pairs, use the --tls-sni-cert-key multiple times. Examples: "example.crt,example.key" or "foo.crt,foo.key:\*.foo.com,foo.com".
|
|-v, --v int|
||
number for the log level verbosity
|
|--version version[=true]|
||
--version, --version=raw prints version information and quits; --version=vX.Y.Z... sets the reported version
|
|--vmodule pattern=N,...|
||
comma-separated list of pattern=N settings for file-filtered logging (only works for text log format)
|
|--write-config-to string|
||
If set, write the configuration values to this file and exit.
|