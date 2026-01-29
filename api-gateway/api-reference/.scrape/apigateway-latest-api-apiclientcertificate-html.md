---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_ClientCertificate.html
title: ClientCertificate
word_count: 141
filtered: true
elements_removed: 0
density_score: 0.92
---

ClientCertificate - Amazon API Gateway
ClientCertificate - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_ClientCertificate)
[Contents](#API_ClientCertificate_Contents)[See Also](#API_ClientCertificate_SeeAlso)
# ClientCertificate
Represents a client certificate used to configure client-side SSL authentication while sending requests to the integration endpoint.
## Contents
**
clientCertificateId
**
The identifier of the client certificate.
Type: String
Required: No
**
createdDate
**
The timestamp when the client certificate was created.
Type: Timestamp
Required: No
**
description
**
The description of the client certificate.
Type: String
Required: No
**
expirationDate
**
The timestamp when the client certificate will expire.
Type: Timestamp
Required: No
**
pemEncodedCertificate
**
The PEM-encoded public key of the client certificate, which can be used to configure certificate authentication in the integration endpoint .
Type: String
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No