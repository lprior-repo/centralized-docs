---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-specify-certificate-for-custom-domain-name.html
title: Get certificates ready in AWS Certificate Manager
word_count: 936
filtered: true
elements_removed: 0
density_score: 0.83
---

Get certificates ready in AWS Certificate Manager - Amazon API Gateway
Get certificates ready in AWS Certificate Manager - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-specify-certificate-for-custom-domain-name)
[Considerations](#how-to-specify-certificate-for-custom-domain-name-considerations)[To create or import an SSL/TLS certificate into ACM](#how-to-specify-certificate-for-custom-domain-name-setup)
# Get certificates ready in AWS Certificate Manager
Before setting up a custom domain name for an API, you must have an SSL/TLS
certificate ready in AWS Certificate Manager. For
more information, see the [AWS Certificate Manager User Guide](https://docs.aws.amazon.com/acm/latest/userguide/).
## Considerations
The following are considerations for your SSL/TLS certificate.
* If you create an edge-optimized custom domain name, API Gateway leverages CloudFront to support certificates for custom
domain names. As such, the requirements and constraints of a custom
domain name SSL/TLS certificate are dictated by [CloudFront](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html). For example, the maximum size of the public key
is 2048 and the private key size can be 1024, 2048, and 4096. The
public key size is determined by the certificate authority you use.
Ask your certificate authority to return keys of a size different
from the default length. For more information, see [Secure access to your
objects](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https.html) and [Create
signed URLs and signed cookies](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-trusted-signers.html).
* If you create a Regional custom domain name, the maximum size of the public key
is 2048.
* To use an ACM certificate with a Regional custom domain name, you must request or import the
certificate in the same Region as your API. The certificate must cover the custom domain name.
* To use an ACM certificate with an edge-optimized custom domain name, you must request or import the
certificate in the US East (N. Virginia) – `us-east-1` Region.
*
You must have a registered domain name, such as ``example.com``. You can use either [Amazon Route53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/) or a third-party accredited domain registrar.
For a list of such registrars, see [Accredited Registrar Directory](https://www.icann.org/en/accredited-registrars) at the ICANN website.
## To create or import an SSL/TLS certificate into ACM
The following procedures show how to create or import an SSL/TLS certificate for a domain name.
To request a certificate provided by ACM for a domain name
1. Sign in to the [AWS Certificate Manager
console](https://console.aws.amazon.com/acm).
2. Choose **Request a certificate**.
3. For **Certificate type**, choose **Request a public
certificate**.
4. Choose **Next**.
5. For **Fully qualified domain name**, enter a custom domain name for your API, for example, `api.example.com`.
6. Optionally, choose **Add another name to this
certificate**.
7. For **Validation method**, choose a method for validating domain ownership.
8. For **Key algorithm**, choose an encryption algorithm.
9. Choose **Request**.
10. For a valid request, a registered owner of the internet domain must consent to the request before
ACM issues the certificate. If you use Route53 to manage your public DNS records, you can update your
records through the ACM console directly.
To import into ACM a
certificate for a domain name
1. Get a PEM-encoded SSL/TLS certificate for your custom domain name from a
certificate authority. For a partial list of such CAs, see the [Mozilla Included CA List](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport).
1. Generate a private key for the certificate and save the output to a
file, using the [OpenSSL](https://www.openssl.org)
toolkit at the OpenSSL website:
```
`openssl genrsa -out `private-key-file` 2048`
```
2. Generate a certificate signing request (CSR) with the previously
generated private key, using OpenSSL:
```
`openssl req -new -sha256 -key `private-key-file` -out `CSR-file``
```
3. Submit the CSR to the certificate authority and save the resulting
certificate.
4. Download the certificate chain from the certificate authority.
###### Note
If you obtain the private key in another way and the key is encrypted,
you can use the following command to decrypt the key before submitting it to
API Gateway for setting up a custom domain name.
```
`openssl pkcs8 -topk8 -inform pem -in `MyEncryptedKey.pem` -outform pem -nocrypt -out `MyDecryptedKey.pem``
```
5. Upload the certificate to AWS Certificate Manager:
1. Sign in to the [AWS Certificate Manager
console](https://console.aws.amazon.com/acm).
2. Choose **Import a certificate**.
3. For **Certificate body**, enter the body of
the PEM-formatted server certificate from your certificate authority.
The following shows an abbreviated example of such a certificate.
```
`-----BEGIN CERTIFICATE-----
EXAMPLECA+KgAwIBAgIQJ1XxJ8Pl++gOfQtj0IBoqDANBgkqhkiG9w0BAQUFADBB
...
az8Cg1aicxLBQ7EaWIhhgEXAMPLE
-----END CERTIFICATE-----`
```
4. For **Certificate private key**, enter your
PEM-formatted certificate's private key. The following shows an
abbreviated example of such a key.
```
`-----BEGIN RSA PRIVATE KEY-----
EXAMPLEBAAKCAQEA2Qb3LDHD7StY7Wj6U2/opV6Xu37qUCCkeDWhwpZMYJ9/nETO
...
1qGvJ3u04vdnzaYN5WoyN5LFckrlA71+CszD1CGSqbVDWEXAMPLE
-----END RSA PRIVATE KEY-----
`
```
5. For **Certificate chain**, enter the
PEM-formatted intermediate certificates and, optionally, the root
certificate, one after the other without any blank lines. If you include
the root certificate, your certificate chain must start with
intermediate certificates and end with the root certificate. Use the
intermediate certificates provided by your certificate authority. Do not
include any intermediaries that are not in the chain of trust path. The
following shows an abbreviated example.
```
`-----BEGIN CERTIFICATE-----
EXAMPLECA4ugAwIBAgIQWrYdrB5NogYUx1U9Pamy3DANBgkqhkiG9w0BAQUFADCB
...
8/ifBlIK3se2e4/hEfcEejX/arxbx1BJCHBvlEPNnsdw8EXAMPLE
-----END CERTIFICATE-----`
```
Here is another example.
```
`-----BEGIN CERTIFICATE-----
`Intermediate certificate 2`
-----END CERTIFICATE-----
-----BEGIN CERTIFICATE-----
`Intermediate certificate 1`
-----END CERTIFICATE-----
-----BEGIN CERTIFICATE-----
`Optional: Root certificate`
-----END CERTIFICATE-----`
```
6. Choose **Next**, and then choose **Next**.
After the certificate is successfully created or imported, make note of the
certificate ARN. You need it when setting up the custom domain name.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Custom domain names
Set up a Regional custom domain name
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.