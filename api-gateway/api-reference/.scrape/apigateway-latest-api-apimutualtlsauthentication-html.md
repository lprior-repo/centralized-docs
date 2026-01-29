---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_MutualTlsAuthentication.html
title: MutualTlsAuthentication
word_count: 209
filtered: true
elements_removed: 0
density_score: 0.82
---

MutualTlsAuthentication - Amazon API Gateway
MutualTlsAuthentication - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_MutualTlsAuthentication)
[Contents](#API_MutualTlsAuthentication_Contents)[See Also](#API_MutualTlsAuthentication_SeeAlso)
# MutualTlsAuthentication
The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway
performs two-way authentication between the client and the server. Clients must present a
trusted certificate to access your API.
## Contents
**
truststoreUri
**
An Amazon S3 URL that specifies the truststore for mutual TLS authentication, for example
`s3://bucket-name/key-name`. The truststore can contain certificates from public or private
certificate authorities. To update the truststore, upload a new version to S3, and then update
your custom domain name to use the new version. To update the truststore, you must have
permissions to access the S3 object.
Type: String
Required: No
**
truststoreVersion
**
The version of the S3 object that contains your truststore. To specify a version, you must have versioning enabled for the S3 bucket.
Type: String
Required: No
**
truststoreWarnings
**
A list of warnings that API Gateway returns while processing your truststore. Invalid
certificates produce warnings. Mutual TLS is still enabled, but some clients might not be able
to access your API. To resolve warnings, upload a new truststore to S3, and then update you
domain name to use the new version.
Type: Array of strings
Required: No