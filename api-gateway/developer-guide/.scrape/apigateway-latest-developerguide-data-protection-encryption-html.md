---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/data-protection-encryption.html
title: Data encryption in Amazon API Gateway
word_count: 833
filtered: true
elements_removed: 0
density_score: 0.85
---

Data encryption in Amazon API Gateway - Amazon API Gateway
Data encryption in Amazon API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#data-protection-encryption)
[Encryption at rest](#data-protection-at-rest)[Encryption in transit](#data-protection-in-transit)
# Data encryption in Amazon API Gateway
Data protection refers to protecting data while in transit (as it travels to and from API Gateway) and at rest (while it is stored in AWS).
## Data encryption at rest in Amazon API Gateway
If you choose to enable caching for a REST API, you can enable cache encryption. To learn more, see
[Cache settings for REST APIs in API Gateway](./api-gateway-caching.html).
For more information about data protection, see the [AWS Shared Responsibility Model and GDPR](https://aws.amazon.com/blogs/security/the-aws-shared-responsibility-model-and-gdpr/) blog post on the *AWS Security
Blog*.
### Encryption and decryption of your certificate’s private key
When you create a custom domain name for private APIs, your ACM certificate and private key are encrypted using an AWS managed KMS key
that has the alias **aws/acm**. You can view the key ID with
this alias in the AWS KMS console under **AWS managed keys**.
API Gateway does not directly access your ACM resources. It uses AWS TLS Connection
Manager to secure and access the private keys for your certificate. When you use your
ACM certificate to create an API Gateway custom domain name for private APIs, API Gateway associates your certificate with
AWS TLS Connection Manager. This is done by creating a grant in AWS KMS against your AWS
Managed Key with the prefix **aws/acm**. A grant is a policy
instrument that allows TLS Connection Manager to use KMS keys in cryptographic operations.
The grant allows the grantee principal (TLS Connection Manager) to call the specified
grant operations on the KMS key to decrypt your certificate's private key. TLS Connection
Manager then uses the certificate and the decrypted (plaintext) private key to establish a
secure connection (SSL/TLS session) with clients of API Gateway services. When the
certificate is disassociated from an API Gateway custom domain name for private APIs, the grant is retired.
If you want to remove access to the KMS key, we recommend that you replace or delete
the certificate from the service using the AWS Management Console or the `update-service`
command in the AWS CLI.
### Encryption context for API Gateway
An [encryption context](https://docs.aws.amazon.com/kms/latest/developerguide/encrypt_context.html)
is an optional set of key-value pairs that contain contextual
information about what your private key might be used for. AWS KMS binds the encryption
context to the encrypted data and uses it as additional authenticated data to support
authenticated encryption.
When your TLS keys are used with API Gateway and TLS Connection manager, the name of your
API Gateway service is included in the encryption context used to encrypt your key at rest.
You can verify which API Gateway custom domain name your certificate and private key are being used for
by viewing the encryption context in your CloudTrail logs as shown in the next section, or by
looking at the **Associated Resources** tab in the ACM console.
To decrypt data, the same encryption context is included in the request. API Gateway uses
the same encryption context in all AWS KMS cryptographic operations, where the key is
`aws:apigateway:arn` and the value is the Amazon Resource Name (ARN) of the
API Gateway `PrivateDomainName` resource.
The following example shows the encryption context in the output of an operation such
as `CreateGrant`.
```
`"constraints": {
"encryptionContextEquals": {
"aws:acm:arn": "arn:aws:acm:us-west-2:859412291086:certificate/9177097a-f0ae-4be1-93b1-19f911ea4f88",
"aws:apigateway:arn": "arn:aws:apigateway:us-west-2:859412291086:/domainnames/denytest-part1.pdx.sahig.people.aws.dev+cbaeumzjhg"
}
},
"operations": [
"Decrypt"
],
"granteePrincipal": "tlsconnectionmanager.amazonaws.com"`
```
## Data encryption in transit in Amazon API Gateway
The APIs created with Amazon API Gateway expose HTTPS endpoints only. API Gateway doesn't
support unencrypted (HTTP) endpoints.
API Gateway manages the certificates for default `execute-api` endpoints. If you configure a
custom domain name, [you specify the certificate for the domain name](./how-to-custom-domains.html#custom-domain-names-certificates). As a best practice, don't [pin certificates](https://docs.aws.amazon.com/acm/latest/userguide/troubleshooting-pinning.html).
For greater security, you can choose a minimum Transport Layer Security (TLS) protocol version to be
enforced for your API Gateway custom domain. WebSocket APIs and HTTP APIs support only TLS 1.2. To learn more, see
[Choose a security policy for
your custom domain in API Gateway](./apigateway-custom-domain-tls-version.html).
You can also set up a Amazon CloudFront distribution with a custom SSL certificate in your
account and use it with Regional APIs. You can then configure the security policy for the
CloudFront distribution with TLS 1.1 or higher based on your security and compliance requirements.
For more information about data protection, see [Protect your REST APIs in API Gateway](./rest-api-protect.html) and the [AWS Shared Responsibility Model and GDPR](https://aws.amazon.com/blogs/security/the-aws-shared-responsibility-model-and-gdpr/) blog post on the *AWS Security
Blog*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data protection
Internetwork traffic privacy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.