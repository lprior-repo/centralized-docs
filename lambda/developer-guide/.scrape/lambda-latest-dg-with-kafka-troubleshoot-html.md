---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-kafka-troubleshoot.html
title: Troubleshooting Kafka event source mapping errors
word_count: 756
filtered: true
elements_removed: 0
density_score: 0.87
---

Troubleshooting Kafka event source mapping errors - AWS Lambda
Troubleshooting Kafka event source mapping errors - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-kafka-troubleshoot)
[Authentication and authorization errors](#kafka-permissions-errors)[Event source errors](#services-event-errors)
# Troubleshooting Kafka event source mapping errors
The following topics provide troubleshooting advice for errors and issues that you might encounter when using
Amazon MSK or self-managed Apache Kafka with Lambda.
For more help with troubleshooting, visit the [AWS Knowledge Center](https://repost.aws/knowledge-center#AWS_Lambda).
## Authentication and authorization errors
If any of the permissions required to consume data from the Kafka cluster are missing, Lambda displays one of
the following error messages in the event source mapping under **LastProcessingResult**.
###### Error messages
* [Cluster failed to authorize Lambda](#kafka-authorize-errors)
* [SASL authentication failed](#kafka-sasl-errors)
* [Server failed to authenticate Lambda](#kafka-mtls-errors-server)
* [Lambda failed to authenticate server](#kafka-mtls-errors-lambda)
* [Provided certificate or private key is invalid](#kafka-key-errors)
### Cluster failed to authorize Lambda
For SASL/SCRAM or mTLS, this error indicates that the provided user doesn't have all of the following
required Kafka access control list (ACL) permissions:
* DescribeConfigs Cluster
* Describe Group
* Read Group
* Describe Topic
* Read Topic
When you create Kafka ACLs with the required `kafka-cluster` permissions, specify the topic and
group as resources. The topic name must match the topic in the event source mapping. The group name must match
the event source mapping's UUID.
After you add the required permissions to the execution role, it might take several minutes for the changes
to take effect.
### SASL authentication failed
For SASL/SCRAM or SASL/PLAIN, this error indicates that the provided sign-in credentials aren't
valid.
For IAM access control, the execution role is missing the `kafka-cluster:Connect` permission
for the cluster. Add this permission to the role and specify the cluster's Amazon Resource Name (ARN) as a
resource.
You might see this error occurring intermittently. The cluster rejects connections after the number of TCP
connections exceeds the service quota. Lambda backs off and retries until a connection is successful. After Lambda connects to the
cluster and polls for records, the last processing result changes to `OK`.
### Server failed to authenticate Lambda
This error indicates that the Kafka broker failed to authenticate Lambda. This can occur for any of the
following reasons:
* You didn't provide a client certificate for mTLS authentication.
* You provided a client certificate, but the Kafka brokers aren't configured to use mTLS authentication.
* A client certificate isn't trusted by the Kafka brokers.
### Lambda failed to authenticate server
This error indicates that Lambda failed to authenticate the Kafka broker. This can occur for any of the
following reasons:
* For self-managed Apache Kafka: The Kafka brokers use self-signed certificates or a private CA, but didn't provide the server root CA
certificate.
* For self-managed Apache Kafka: The server root CA certificate doesn't match the root CA that signed the broker's certificate.
* Hostname validation failed because the broker's certificate doesn't contain the broker's DNS name or IP address as
a subject alternative name.
### Provided certificate or private key is invalid
This error indicates that the Kafka consumer couldn't use the provided certificate or private key. Make sure
that the certificate and key use PEM format, and that the private key encryption uses a PBES1 algorithm.
## Event source mapping errors
When you add your Apache Kafka cluster as an [event source](./invocation-eventsourcemapping.html) for your Lambda function, if your function encounters an error, your Kafka consumer stops processing records. Consumers of a topic partition are those that subscribe to, read, and process your records. Your other Kafka consumers can continue processing records, provided they don't encounter the same error.
To determine the cause of a stopped consumer, check the `StateTransitionReason` field in the response of `EventSourceMapping`. The following list describes the event source errors that you can receive:
**`ESM\_CONFIG\_NOT\_VALID`**
The event source mapping configuration isn't valid.
**`EVENT\_SOURCE\_AUTHN\_ERROR`**
Lambda couldn't authenticate the event source.
**`EVENT\_SOURCE\_AUTHZ\_ERROR`**
Lambda doesn't have the required permissions to access the event source.
**`FUNCTION\_CONFIG\_NOT\_VALID`**
The function configuration isn't valid.
###### Note
If your Lambda event records exceed the allowed size limit of 6 MB, they can go
unprocessed.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Kafka on-failure destination
API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.