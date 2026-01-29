---
url: https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html
title: SourceAccessConfiguration
word_count: 347
filtered: true
elements_removed: 0
density_score: 0.87
---

SourceAccessConfiguration - AWS Lambda
SourceAccessConfiguration - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_SourceAccessConfiguration)
[Contents](#API_SourceAccessConfiguration_Contents)[See Also](#API_SourceAccessConfiguration_SeeAlso)
# SourceAccessConfiguration
To secure and define access to your event source, you can specify the authentication protocol, VPC components, or virtual host.
## Contents
**
Type
**
The type of authentication protocol, VPC components, or virtual host for your event source. For example: `"Type":"SASL\_SCRAM\_512\_AUTH"`.
* `BASIC\_AUTH` – (Amazon MQ) The AWS Secrets Manager secret that stores your broker credentials.
* `BASIC\_AUTH` – (Self-managed Apache Kafka) The Secrets Manager ARN of your secret key used for SASL/PLAIN authentication of your Apache Kafka brokers.
* `VPC\_SUBNET` – (Self-managed Apache Kafka) The subnets associated with your VPC. Lambda connects to these subnets to fetch data from your self-managed Apache Kafka cluster.
* `VPC\_SECURITY\_GROUP` – (Self-managed Apache Kafka) The VPC security group used to manage access to your self-managed Apache Kafka brokers.
* `SASL\_SCRAM\_256\_AUTH` – (Self-managed Apache Kafka) The Secrets Manager ARN of your secret key used for SASL SCRAM-256 authentication of your self-managed Apache Kafka brokers.
* `SASL\_SCRAM\_512\_AUTH` – (Amazon MSK, Self-managed Apache Kafka) The Secrets Manager ARN of your secret key used for SASL SCRAM-512 authentication of your self-managed Apache Kafka brokers.
* `VIRTUAL\_HOST` –- (RabbitMQ) The name of the virtual host in your RabbitMQ broker. Lambda uses this RabbitMQ host as the event source.
This property cannot be specified in an UpdateEventSourceMapping API call.
* `CLIENT\_CERTIFICATE\_TLS\_AUTH` – (Amazon MSK, self-managed Apache Kafka) The Secrets Manager ARN of your secret key containing the certificate chain (X.509 PEM),
private key (PKCS#8 PEM), and private key password (optional) used for mutual TLS authentication of your MSK/Apache Kafka brokers.
* `SERVER\_ROOT\_CA\_CERTIFICATE` – (Self-managed Apache Kafka) The Secrets Manager ARN of your secret key containing the root CA certificate (X.509 PEM) used for TLS encryption of your Apache Kafka brokers.
Type: String
Valid Values: `BASIC\_AUTH | VPC\_SUBNET | VPC\_SECURITY\_GROUP | SASL\_SCRAM\_512\_AUTH | SASL\_SCRAM\_256\_AUTH | VIRTUAL\_HOST | CLIENT\_CERTIFICATE\_TLS\_AUTH | SERVER\_ROOT\_CA\_CERTIFICATE`
Required: No
**
URI
**
The value for your chosen configuration in `Type`. For example: `"URI": "arn:aws:secretsmanager:us-east-1:01234567890:secret:MyBrokerSecretName"`.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 200.
Pattern: `[a-zA-Z0-9-\\/\*:\_+=.@-]\*`
Required: No