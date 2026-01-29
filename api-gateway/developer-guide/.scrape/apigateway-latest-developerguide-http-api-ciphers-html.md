---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-ciphers.html
title: Security policy for HTTP APIs in API Gateway
word_count: 305
filtered: true
elements_removed: 0
density_score: 0.93
---

Security policy for HTTP APIs in API Gateway - Amazon API Gateway
Security policy for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-ciphers)
[Supported TLS protocols and ciphers for HTTP APIs](#http-api-ciphers-list)[OpenSSL and
RFC cipher names](#apigateway-secure-connections-openssl-rfc-cipher-names-http)[Information about REST APIs and WebSocket APIs](#apigateway-http-additional-apis)
# Security policy for HTTP APIs in API Gateway
API Gateway enforces a security policy of `TLS\_1\_2` for all HTTP API endpoints.
A *security policy* is a predefined combination of minimum TLS version and cipher suites
offered by Amazon API Gateway. The TLS protocol addresses network security problems such as tampering and eavesdropping
between a client and server. When your clients establish a TLS handshake to your API through the custom domain, the
security policy enforces the TLS version and cipher suite options your clients can choose to use. This security
policy accepts TLS 1.2 and TLS 1.3 traffic and rejects TLS 1.0 traffic.
## OpenSSL and
RFC cipher names
OpenSSL and IETF RFC 5246 use different names for the same ciphers. For a list of the cipher names, see
[OpenSSL and
RFC cipher names](./apigateway-security-policies-list.html#apigateway-secure-connections-openssl-rfc-cipher-names).
## Information about REST APIs and WebSocket APIs
For more information about REST APIs and WebSocket APIs, see [Choose a security policy for
your custom domain in API Gateway](./apigateway-custom-domain-tls-version.html) and
[Security policy for WebSocket APIs in API Gateway](./websocket-api-ciphers.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway stage
variables reference for HTTP APIs in API Gateway
Custom domain names
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.