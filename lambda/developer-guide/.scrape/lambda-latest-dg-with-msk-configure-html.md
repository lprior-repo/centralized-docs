---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-msk-configure.html
title: Configuring Amazon MSK event sources for Lambda
word_count: 487
filtered: true
elements_removed: 0
density_score: 0.84
---

Configuring Amazon MSK event sources for Lambda - AWS Lambda
Configuring Amazon MSK event sources for Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-msk-configure)
[Amazon MSK cluster as an event source](#msk-esm-overview)
# Configuring Amazon MSK event sources for Lambda
To use an Amazon MSK cluster as an event source for your Lambda function, you create an [event source mapping](./invocation-eventsourcemapping.html) that connects the two resources.
This page describes how to create an event source mapping for Amazon MSK.
This page assumes that you've already properly configured your MSK cluster and the
[Amazon Virtual Private Cloud (VPC)](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html)
it resides in. If you need to set up your cluster or VPC, see [Configuring your Amazon MSK cluster and Amazon VPC network for Lambda](./with-msk-cluster-network.html).
To configure retry behavior for error handling, see [Configuring error handling controls for Kafka event sources](./kafka-retry-configurations.html).
###### Topics
* [Using an Amazon MSK cluster as an event source](#msk-esm-overview)
* [Configuring Amazon MSK cluster authentication methods in Lambda](./msk-cluster-auth.html)
* [Creating a Lambda event source mapping for an Amazon MSK event source](./msk-esm-create.html)
* [Creating cross-account event source mappings in Lambda](./msk-cross-account.html)
* [All Amazon MSK event source configuration parameters in Lambda](./msk-esm-parameters.html)
## Using an Amazon MSK cluster as an event source
When you add your Apache Kafka or Amazon MSK cluster as a trigger for your Lambda function, the cluster is used
as an [event source](./invocation-eventsourcemapping.html).
Lambda reads event data from the Kafka topics that you specify as `Topics` in a
[CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) request, based on the [starting
position](./kafka-starting-positions.html) that you specify. After successful processing, your Kafka topic is committed to your
Kafka cluster.
Lambda reads messages sequentially for each Kafka topic partition. A single Lambda payload can contain
messages from multiple partitions. When more records are available, Lambda continues processing records in
batches, based on the BatchSize value that you specify in a [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) request, until
your function catches up with the topic.
After Lambda processes each batch, it commits the offsets of the messages in that batch. If your function
returns an error for any of the messages in a batch, Lambda retries the whole batch of messages until
processing succeeds or the messages expire. You can send records that fail all retry attempts to an
on-failure destination for later processing.
###### Note
While Lambda functions typically have a maximum timeout limit of 15 minutes, event source mappings
for Amazon MSK, self-managed Apache Kafka, Amazon DocumentDB, and Amazon MQ for ActiveMQ and RabbitMQ only support functions with maximum
timeout limits of 14 minutes.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure permissions
Cluster authentication
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.