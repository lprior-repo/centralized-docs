---
url: https://docs.aws.amazon.com/lambda/latest/dg/kafka-cluster-auth.html
title: Configuring cluster authentication methods in Lambda
word_count: 782
filtered: true
elements_removed: 0
density_score: 0.86
---

Configuring cluster authentication methods in Lambda - AWS Lambda
Configuring cluster authentication methods in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#kafka-cluster-auth)
[SASL/SCRAM authentication](#smaa-auth-sasl)[Mutual TLS authentication](#smaa-auth-mtls)[Configuring the client certificate secret](#smaa-auth-secret)[Configuring the server root CA certificate secret](#smaa-auth-ca-cert)
# Configuring cluster authentication methods in Lambda
Lambda supports several methods to authenticate with your self-managed Apache Kafka cluster. Make sure that you configure the
Kafka cluster to use one of these supported authentication methods. For more information about Kafka security, see
the [Security](http://kafka.apache.org/documentation.html#security) section of the Kafka
documentation.
## SASL/SCRAM authentication
Lambda supports Simple Authentication and Security Layer/Salted Challenge Response Authentication Mechanism
(SASL/SCRAM) authentication with Transport Layer Security (TLS) encryption (`SASL\_SSL`). Lambda sends the encrypted credentials to authenticate with
the cluster. Lambda doesn't support SASL/SCRAM with plaintext (`SASL\_PLAINTEXT`). For more information about SASL/SCRAM authentication, see [RFC 5802](https://tools.ietf.org/html/rfc5802).
Lambda also supports SASL/PLAIN authentication. Because this mechanism uses clear text credentials, the connection to the
server must use TLS encryption to ensure that the credentials are protected.
For SASL authentication, you store the sign-in credentials as a secret in AWS Secrets Manager. For more information
about using Secrets Manager, see [Create an AWS Secrets Manager secret](https://docs.aws.amazon.com/secretsmanager/latest/userguide/create_secret.html) in the *AWS Secrets Manager User Guide*.
###### Important
To use Secrets Manager for authentication, secrets must be stored in the same AWS region as your Lambda function.
## Mutual TLS authentication
Mutual TLS (mTLS) provides two-way authentication between the client and server. The client sends a
certificate to the server for the server to verify the client, and the server sends a certificate to the client
for the client to verify the server.
In self-managed Apache Kafka, Lambda acts as the client. You configure a client certificate (as a secret in Secrets Manager) to
authenticate Lambda with your Kafka brokers. The client certificate must be signed by a CA in the server's trust
store.
The Kafka cluster sends a server certificate to Lambda to authenticate the Kafka brokers with Lambda. The
server certificate can be a public CA certificate or a private CA/self-signed certificate. The public CA
certificate must be signed by a certificate authority (CA) that's in the Lambda trust store. For a private
CA/self-signed certificate, you configure the server root CA certificate (as a secret in Secrets Manager). Lambda uses
the root certificate to verify the Kafka brokers.
For more information about mTLS, see [
Introducing mutual TLS authentication for Amazon MSK as an event source](https://aws.amazon.com/blogs/compute/introducing-mutual-tls-authentication-for-amazon-msk-as-an-event-source).
## Configuring the client certificate secret
The CLIENT\_CERTIFICATE\_TLS\_AUTH secret requires a certificate field and a private key field. For an
encrypted private key, the secret requires a private key password. Both the certificate and private key must be
in PEM format.
###### Note
Lambda supports the [PBES1](https://datatracker.ietf.org/doc/html/rfc2898/#section-6.1) (but not
PBES2) private key encryption algorithms.
The certificate field must contain a list of certificates, beginning with the client certificate, followed
by any intermediate certificates, and ending with the root certificate. Each certificate must start on a new
line with the following structure:
```
`-----BEGIN CERTIFICATE-----
&lt;certificate contents&gt;
-----END CERTIFICATE----- `
```
Secrets Manager supports secrets up to 65,536 bytes, which is enough space for long certificate chains.
The private key must be in [PKCS #8](https://datatracker.ietf.org/doc/html/rfc5208)
format, with the following structure:
```
`-----BEGIN PRIVATE KEY-----
&lt;private key contents&gt;
-----END PRIVATE KEY----- `
```
For an encrypted private key, use the following structure:
```
`-----BEGIN ENCRYPTED PRIVATE KEY-----
&lt;private key contents&gt;
-----END ENCRYPTED PRIVATE KEY----- `
```
The following example shows the contents of a secret for mTLS authentication using an encrypted private key.
For an encrypted private key, include the private key password in the secret.
```
{"privateKeyPassword":"testpassword",
"certificate":"-----BEGIN CERTIFICATE-----
MIIE5DCCAsygAwIBAgIRAPJdwaFaNRrytHBto0j5BA0wDQYJKoZIhvcNAQELBQAw
...
j0Lh4/+1HfgyE2KlmII36dg4IMzNjAFEBZiCRoPimO40s1cRqtFHXoal0QQbIlxk
cmUuiAii9R0=
-----END CERTIFICATE-----
-----BEGIN CERTIFICATE-----
MIIFgjCCA2qgAwIBAgIQdjNZd6uFf9hbNC5RdfmHrzANBgkqhkiG9w0BAQsFADBb
...
rQoiowbbk5wXCheYSANQIfTZ6weQTgiCHCCbuuMKNVS95FkXm0vqVD/YpXKwA/no
c8PH3PSoAaRwMMgOSA2ALJvbRz8mpg==
-----END CERTIFICATE-----",
"privateKey":"-----BEGIN ENCRYPTED PRIVATE KEY-----
MIIFKzBVBgkqhkiG9w0BBQ0wSDAnBgkqhkiG9w0BBQwwGgQUiAFcK5hT/X7Kjmgp
...
QrSekqF+kWzmB6nAfSzgO9IaoAaytLvNgGTckWeUkWn/V0Ck+LdGUXzAC4RxZnoQ
zp2mwJn2NYB7AZ7+imp0azDZb+8YG2aUCiyqb6PnnA==
-----END ENCRYPTED PRIVATE KEY-----"
}
```
## Configuring the server root CA certificate secret
You create this secret if your Kafka brokers use TLS encryption with certificates signed by a private CA.
You can use TLS encryption for VPC, SASL/SCRAM, SASL/PLAIN, or mTLS authentication.
The server root CA certificate secret requires a field that contains the Kafka broker's root CA certificate
in PEM format. The following example shows the structure of the secret.
```
{"certificate":"-----BEGIN CERTIFICATE-----
MIID7zCCAtegAwIBAgIBADANBgkqhkiG9w0BAQsFADCBmDELMAkGA1UEBhMCVVMx
EDAOBgNVBAgTB0FyaXpvbmExEzARBgNVBAcTClNjb3R0c2RhbGUxJTAjBgNVBAoT
HFN0YXJmaWVsZCBUZWNobm9sb2dpZXMsIEluYy4xOzA5BgNVBAMTMlN0YXJmaWVs
ZCBTZXJ2aWNlcyBSb290IENlcnRpZmljYXRlIEF1dG...
-----END CERTIFICATE-----"
}
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure event source
Event source mapping
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.