---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_TlsConfig.html
title: API TlsConfig.html
word_count: 121
filtered: true
elements_removed: 0
density_score: 0.87
---

TlsConfig - Amazon API Gateway
TlsConfig - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_TlsConfig)
[Contents](#API_TlsConfig_Contents)[See Also](#API_TlsConfig_SeeAlso)
## Contents
**
insecureSkipVerification
**
Specifies whether or not API Gateway skips verification that the certificate for an integration endpoint is
issued by a supported certificate authority. This isn’t recommended, but it enables you to
use certificates that are signed by private certificate authorities, or certificates
that are self-signed. If enabled, API Gateway still performs basic certificate
validation, which includes checking the certificate's expiration date, hostname, and
presence of a root certificate authority. Supported only for `HTTP` and
`HTTP\_PROXY` integrations.
###### Important
Enabling `insecureSkipVerification` isn't recommended, especially for integrations with public
HTTPS endpoints. If you enable `insecureSkipVerification`, you increase the risk of man-in-the-middle attacks.
Type: Boolean
Required: No