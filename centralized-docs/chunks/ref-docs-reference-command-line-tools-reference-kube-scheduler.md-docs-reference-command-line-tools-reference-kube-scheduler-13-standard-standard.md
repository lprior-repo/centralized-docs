---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#13-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 512
summary: | |--requestheader-extra-headers-prefix stringsDefault: \"x-remote-extra-\"| || List of request header prefixes to inspect. X-Remote-Extra- is suggested. | |--requestheader-group-headers...
---

|
|--requestheader-extra-headers-prefix stringsDefault: "x-remote-extra-"|
||
List of request header prefixes to inspect. X-Remote-Extra- is suggested.
|
|--requestheader-group-headers stringsDefault: "x-remote-group"|
||
List of request headers to inspect for groups. X-Remote-Group is suggested.
|
|--requestheader-uid-headers strings|
||
List of request headers to inspect for UIDs. X-Remote-Uid is suggested. Requires the RemoteRequestHeaderUID feature to be enabled.
|
|--requestheader-username-headers stringsDefault: "x-remote-user"|
||
List of request headers to inspect for usernames. X-Remote-User is common.
|
|--secure-port intDefault: 10259|
||
The port on which to serve HTTPS with authentication and authorization. If 0, don't serve HTTPS at all.
|
|--show-hidden-metrics-for-version string|
||
The previous version for which you want to show hidden metrics. Only the previous minor version is meaningful, other values will not be allowed. The format is &lt;major&gt;.&lt;minor&gt;, e.g.: '1.16'. The purpose of this format is make sure you have the opportunity to notice if the next release hides additional metrics, rather than being surprised when they are permanently removed in the release after that.
|
|--tls-cert-file string|
||
File containing the default x509 Certificate for HTTPS. (CA cert, if any, concatenated after server cert). If HTTPS serving is enabled, and --tls-cert-file and --tls-private-key-file are not provided, a self-signed certificate and key are generated for the public address and saved to the directory specified by --cert-dir.
|
|--tls-cipher-suites strings|
||
Comma-separated list of cipher suites for the server. If omitted, the default Go cipher suites will be used.
Preferred values: TLS\_AES\_128\_GCM\_SHA256, TLS\_AES\_256\_GCM\_SHA384, TLS\_CHACHA20\_POLY1305\_SHA256, TLS\_ECDHE\_ECDSA\_WITH\_AES\_128\_CBC\_SHA, TLS\_ECDHE\_ECDSA\_WITH\_AES\_128\_GCM\_SHA256, TLS\_ECDHE\_ECDSA\_WITH\_AES\_256\_CBC\_SHA, TLS\_ECDHE\_ECDSA\_WITH\_AES\_256\_GCM\_SHA384, TLS\