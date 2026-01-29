---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-custom-domain-tls-version.html
title: Choose a security policy for
word_count: 931
filtered: true
elements_removed: 0
density_score: 0.86
---

Choose a security policy for your custom domain in API Gateway - Amazon API Gateway
Choose a security policy for your custom domain in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-custom-domain-tls-version)
[Considerations](#apigateway-custom-domain-tls-version-considerations)[How API Gateway applies security policies](#apigateway-custom-domain-tls-version-understanding)[Change your custom domain name's
security policy](#apigateway-security-policies-update-custom-domain-name)[Information about HTTP APIs and WebSocket APIs](#apigateway-rest-additional-apis)
# Choose a security policy for
your custom domain in API Gateway
A *security policy* is a predefined combination of minimum TLS version and cipher suites
offered by API Gateway. When your clients establish a TLS handshake to your API or custom domain name, the security
policy enforces the TLS version and cipher suite accepted by API Gateway. Security policies protect your APIs and custom
domain names from network security problems such as tampering and eavesdropping between a client and server.
API Gateway supports legacy security policies and enhanced security policies. `TLS\_1\_0` and
`TLS\_1\_2` are legacy security policies. Use these security policies for generalized workloads, or to
get started creating an API. Any policy that starts with `SecurityPolicy\_` is an enhanced security
policy. Use these policies for regulated workloads, advanced governance, or to use post-quantum cryptography. When
you use an enhanced security policy, you must also set the endpoint access mode for additional governance. For more
information, see [Endpoint access mode](./apigateway-security-policies.html#apigateway-security-policies-endpoint-access-mode).
## Considerations
The following are considerations for security policies for custom domain names for REST APIs in API Gateway:
* You can't enable mutual TLS on a domain name that uses an enhanced security policy.
* You can't map an HTTP API to a domain name that uses an enhanced security policy.
* If you enable multi-level base path mapping to a REST API that uses an enhanced security policy, you
can’t create a base path mapping to an HTTP API for the same domain name.
* Your API can be mapped to a custom domain name with a different security policy than your API. When you
invoke that custom domain name, API Gateway uses the security policy of the API to negotiate the TLS handshake. If
you disable your default API endpoint, this might affect how callers can invoke your API.
* API Gateway supports security policies on all APIs. However, you can only choose a security policy for REST
APIs. API Gateway only supports the `TLS\_1\_2` security policy for HTTP or WebSocket APIs.
* API Gateway doesn't support updating a security policy for a domain name with multiple endpoint types. If you
have multiple endpoint types for a domain name, delete one of them to update the security policy.
## How API Gateway applies security policies
The following example shows how API Gateway applies security policies using the
`SecurityPolicy\_TLS13\_1\_3\_2025\_09` security policy as an example.
The `SecurityPolicy\_TLS13\_1\_3\_2025\_09` security policy accepts TLS 1.3 traffic and rejects TLS 1.2
and TLS 1.0 traffic. For TLS 1.3 traffic, the security policy accepts the following cipher suites:
* `TLS\_AES\_128\_GCM\_SHA256`
* `TLS\_AES\_256\_GCM\_SHA384`
* `TLS\_CHACHA20\_POLY1305\_SHA256`
API Gateway does not accept any other cipher suites. For instance, the security
policy would reject any TLS 1.3 traffic that uses the `AES128-SHA` cipher suite.
To monitor which TLS protocol and ciphers clients used to access your API Gateway, you can use the `$context.tlsVersion` and
`$context.cipherSuite` context variables in your access logs. For more information, see [Monitor REST APIs in API Gateway](./rest-api-monitor.html).
To see the default security policies for all REST APIs and custom domain names, see [Default security policies](./apigateway-security-policies-list.html#apigateway-security-policies-default). To see the supported security policies for all REST APIs and
custom domain names, see [Supported security policies](./apigateway-security-policies-list.html).
## Change your custom domain name's
security policy
If you change your security policy, it takes about 15 minutes for the update to complete. You can monitor the
`lastUpdateStatus` of your custom domain name. As your custom domain name updates, the
`lastUpdateStatus` is `PENDING` and when it completes, it will be `AVAILABLE`.
When you use a security policy that starts with `SecurityPolicy\_`, you must also turn on
endpoint access mode. For more information, see
[Endpoint access mode](./apigateway-security-policies.html#apigateway-security-policies-endpoint-access-mode).
AWS Management Console
###### To change the security policy of a custom domain name
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a custom domain name that sends traffic to REST APIs.
Make sure there is only one endpoint type associated with your custom domain name.
3. Choose **Custom domain name settings**, and then choose
**Edit**.
4. For **Security policy**, select a new policy.
5. For **Endpoint access mode**, choose **Strict**.
6. Choose **Save changes**.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command
updates a domain name to use the `SecurityPolicy\_TLS13\_1\_3\_2025\_09` security policy:
```
`aws apigateway update-domain-name \\
--domain-name example.com \\
--patch-operations '[
{
"op": "replace",
"path": "/securityPolicy",
"value": "SecurityPolicy\_TLS13\_1\_3\_2025\_09"
},
{
"op": "replace",
"path": "/endpointAccessMode",
"value": "STRICT"
}
]'`
```
The output will look like the following:
```
`{
"domainName": "example.com",
"endpointConfiguration": {
"types": [ "REGIONAL" ],
"ipAddressType": "dualstack"
},
"regionalCertificateArn": "arn:aws:acm:us-west-2:111122223333:certificate/a1b2c3d4-5678-90ab-cdef",
"securityPolicy": "SecurityPolicy\_TLS13\_1\_3\_2025\_09",
"endpointAccessMode": "STRICT"
}`
```
## Information about HTTP APIs and WebSocket APIs
For more information about HTTP APIs and WebSocket APIs, see [Security policy for HTTP APIs in API Gateway](./http-api-ciphers.html)
and [Security policy for WebSocket APIs in API Gateway](./websocket-api-ciphers.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IP address types for custom domain names in API Gateway
Disable the default endpoint
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.