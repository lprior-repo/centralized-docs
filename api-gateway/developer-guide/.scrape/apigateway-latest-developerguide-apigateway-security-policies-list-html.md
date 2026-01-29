---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-security-policies-list.html
title: Supported security policies
word_count: 831
filtered: true
elements_removed: 0
density_score: 0.95
---

Supported security policies - Amazon API Gateway
Supported security policies - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-security-policies-list)
[Default security policies](#apigateway-security-policies-default)[Supported security policies for Regional and private
APIs and custom domain names](#apigateway-security-policies-non-edge)[Supported security policies for edge-optimized
APIs and custom domain names](#apigateway-security-policies-edge-optimized)[OpenSSL and
RFC cipher names](#apigateway-secure-connections-openssl-rfc-cipher-names)
# Supported security policies
The following tables describe the [security policies](./apigateway-security-policies.html) that
can be specified for each REST API endpoint type and custom domain name type. These policies allow you to control
incoming connections. API Gateway only supports TLS 1.2 on egress. You can update the security policy for your API
or custom domain name at any time.
Policies that contain `FIPS` in the title are compatible with the Federal Information Processing Standard
(FIPS), which is a US and Canadian government standard that specifies the security requirements for cryptographic modules
that protect sensitive information. To learn more, see [Federal Information Processing Standard (FIPS) 140](https://aws.amazon.com/compliance/fips/)
on the *AWS Cloud Security Compliance* page.
All FIPS policies leverage the AWS-LC FIPS validated cryptographic module. To learn more,
see the [
AWS-LC Cryptographic Module](https://csrc.nist.gov/projects/cryptographic-module-validation-program/certificate/4631) page on the *NIST Cryptographic Module Validation Program* site.
Policies that contain `PQ` in the title use
[Post-Quantum Cryptography (PQC)](https://aws.amazon.com/security/post-quantum-cryptography/) to implement hybrid key
exchange algorithms for TLS to ensure traffic confidentiality against future quantum computing threats.
Policies that contain `PFS` in the title use
[Perfect Forward Secrecy (PFS)](https://en.wikipedia.org/wiki/Forward_secrecy) to make sure session keys
aren't compromised.
Policies that contain both `FIPS` and `PQ` in their title support both of these
features.
## Default security policies
When you create a new REST API or custom domain, the resource is assigned a default security policy. The
following table shows the default security policy for these resources.
|
**Resource**
|
**Default security policy name**
|
|Regional APIs|TLS\_1\_0|
|Edge-optimized APIs|TLS\_1\_0|
|Private APIs|TLS\_1\_2|
|Regional domain|TLS\_1\_2|
|Edge-optimized domain|TLS\_1\_2|
|Private domain|TLS\_1\_2|
## Supported security policies for Regional and private
APIs and custom domain names
The following table describes the security policies that can be specified for Regional and private
APIs and custom domain names:
|
**Security policy**
|
**Supported TLS versions**
|
**Supported ciphers**
|
|SecurityPolicy\_TLS13\_1\_3\_2025\_09|TLS1.3|
###### TLS1.3
* TLS\_AES\_128\_GCM\_SHA256
* TLS\_AES\_256\_GCM\_SHA384
* TLS\_CHACHA20\_POLY1305\_SHA256
|
|SecurityPolicy\_TLS13\_1\_3\_FIPS\_2025\_09|TLS1.3|
###### TLS1.3
* TLS\_AES\_128\_GCM\_SHA256
* TLS\_AES\_256\_GCM\_SHA384
|
|SecurityPolicy\_TLS13\_1\_2\_PFS\_PQ\_2025\_09|
TLS1.3
TLS1.2
|
###### TLS1.2
* TLS\_AES\_128\_GCM\_SHA256
* TLS\_AES\_256\_GCM\_SHA384
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-RSA-AES256-GCM-SHA384
|
|SecurityPolicy\_TLS13\_1\_2\_PQ\_2025\_09|
TLS1.3
TLS1.2
|
###### TLS1.2
* TLS\_AES\_128\_GCM\_SHA256
* TLS\_AES\_256\_GCM\_SHA384
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES128-SHA256
* ECDHE-RSA-AES128-SHA256
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-RSA-AES256-GCM-SHA384
* ECDHE-ECDSA-AES256-SHA384
* ECDHE-RSA-AES256-SHA384
* AES128-GCM-SHA256
* AES128-SHA256
* AES256-GCM-SHA384
* AES256-SHA256
|
|TLS\_1\_2|
TLS1.3
TLS1.2
|
###### TLS1.2
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES128-SHA256
* ECDHE-RSA-AES128-SHA256
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-RSA-AES256-GCM-SHA384
* ECDHE-ECDSA-AES256-SHA384
* ECDHE-RSA-AES256-SHA384
* AES128-GCM-SHA256
* AES128-SHA256
* AES256-GCM-SHA384
* AES256-SHA256
|
|TLS\_1\_0|
TLS1.3
TLS1.2
TLS1.1
TLS1.0
|
###### TLS1.0-TLS1.2
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES128-SHA256
* ECDHE-RSA-AES128-SHA256
* ECDHE-ECDSA-AES128-SHA
* ECDHE-RSA-AES128-SHA
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-RSA-AES256-GCM-SHA384
* ECDHE-ECDSA-AES256-SHA384
* ECDHE-RSA-AES256-SHA384
* ECDHE-ECDSA-AES256-SHA
* ECDHE-RSA-AES256-SHA
* AES128-GCM-SHA256
* AES128-SHA256
* AES128-SHA
* AES256-GCM-SHA384
* AES256-SHA256
* AES256-SHA
|
## Supported security policies for edge-optimized
APIs and custom domain names
The following table describes the security policies that can be specified for edge-optimized
APIs and edge-optimized custom domain names:
|
**Security policy name**
|
**Supported TLS versions**
|
**Supported ciphers**
|
|SecurityPolicy\_TLS13\_2025\_EDGE|TLS1.3|
###### TLS1.3
* TLS\_AES\_128\_GCM\_SHA256
* TLS\_AES\_256\_GCM\_SHA384
* TLS\_CHACHA20\_POLY1305\_SHA256
|
|SecurityPolicy\_TLS12\_PFS\_2025\_EDGE|
TLS1.3
TLS1.2
|
###### TLS1.2
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-ECDSA-CHACHA20-POLY1305
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-RSA-AES256-GCM-SHA384
* ECDHE-RSA-CHACHA20-POLY1305
|
|SecurityPolicy\_TLS12\_2018\_EDGE|
TLS1.3
TLS1.2
|
###### TLS1.2
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES128-SHA256
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-ECDSA-CHACHA20-POLY1305
* ECDHE-ECDSA-AES256-SHA384
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-RSA-AES128-SHA256
* ECDHE-RSA-AES256-GCM-SHA384
* ECDHE-RSA-CHACHA20-POLY1305
* ECDHE-RSA-AES256-SHA384
|
|TLS\_1\_0|
TLS1.3
TLS1.2
TLS1.1
TLS1.0
|
###### TLS1.0-TLS1.2
* ECDHE-ECDSA-AES128-GCM-SHA256
* ECDHE-RSA-AES128-GCM-SHA256
* ECDHE-ECDSA-AES128-SHA256
* ECDHE-RSA-AES128-SHA256
* ECDHE-ECDSA-AES128-SHA
* ECDHE-RSA-AES128-SHA
* ECDHE-ECDSA-AES256-GCM-SHA384
* ECDHE-RSA-AES256-GCM-SHA384
* ECDHE-ECDSA-AES256-SHA384
* ECDHE-RSA-AES256-SHA384
* ECDHE-ECDSA-AES256-SHA
* ECDHE-RSA-AES256-SHA
* AES256-SHA256
* AES256-SHA
|
## OpenSSL and
RFC cipher names
OpenSSL and IETF RFC 5246 use different names
for the same ciphers. The following table maps the OpenSSL name to the RFC name for
each cipher. For more information, see
[ciphers](https://docs.openssl.org/1.1.1/man1/ciphers/) in the OpenSSL Documentation.
|
**OpenSSL cipher name**
|
**RFC cipher name**
|
|
TLS\_AES\_128\_GCM\_SHA256
|
TLS\_AES\_128\_GCM\_SHA256
|
|
TLS\_AES\_256\_GCM\_SHA384
|
TLS\_AES\_256\_GCM\_SHA384
|
|
TLS\_CHACHA20\_POLY1305\_SHA256
|
TLS\_CHACHA20\_POLY1305\_SHA256
|
|
ECDHE-RSA-AES128-GCM-SHA256
|
TLS\_ECDHE\_RSA\_WITH\_AES\_128\_GCM\_SHA256
|
|
ECDHE-RSA-AES128-SHA256
|
TLS\_ECDHE\_RSA\_WITH\_AES\_128\_CBC\_SHA256
|
|
ECDHE-RSA-AES128-SHA
|
TLS\_ECDHE\_RSA\_WITH\_AES\_128\_CBC\_SHA
|
|
ECDHE-RSA-AES256-GCM-SHA384
|
TLS\_ECDHE\_RSA\_WITH\_AES\_256\_GCM\_SHA384
|
|
ECDHE-RSA-AES256-SHA384
|
TLS\_ECDHE\_RSA\_WITH\_AES\_256\_CBC\_SHA384
|
|
ECDHE-RSA-AES256-SHA
|
TLS\_ECDHE\_RSA\_WITH\_AES\_256\_CBC\_SHA
|
|
AES128-GCM-SHA256
|
TLS\_RSA\_WITH\_AES\_128\_GCM\_SHA256
|
|
AES256-GCM-SHA384
|
TLS\_RSA\_WITH\_AES\_256\_GCM\_SHA384
|
|
AES128-SHA256
|
TLS\_RSA\_WITH\_AES\_128\_CBC\_SHA256
|
|
AES256-SHA
|
TLS\_RSA\_WITH\_AES\_256\_CBC\_SHA
|
|
AES128-SHA
|
TLS\_RSA\_WITH\_AES\_128\_CBC\_SHA
|
|
DES-CBC3-SHA
|
TLS\_RSA\_WITH\_3DES\_EDE\_CBC\_SHA
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Security policies
How to change a security policy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.