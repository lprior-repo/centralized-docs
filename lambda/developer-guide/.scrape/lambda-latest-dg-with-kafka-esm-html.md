---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-kafka-esm.html
title: Using Lambda with Apache Kafka
word_count: 404
filtered: true
elements_removed: 0
density_score: 0.85
---

Using Lambda with Apache Kafka - AWS Lambda
Using Lambda with Apache Kafka - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-kafka-esm)
# Using Lambda with Apache Kafka
Lambda supports [Apache Kafka](https://kafka.apache.org/) as an
[event source](./invocation-eventsourcemapping.html). Apache Kafka is an open-source event
streaming platform designed to handle high-throughput, real-time data pipelines and streaming applications.
There are two main ways to use Lambda with Apache Kafka:
* [Using Lambda with Amazon MSK](./with-msk.html) – Amazon Managed Streaming for Apache Kafka (Amazon MSK) is a fully-managed service by AWS.
Amazon MSK helps automate management of your Kafka infrastructure, including provisioning, patching,
and scaling.
* [Using Lambda with self-managed Apache Kafka](./with-kafka.html) – In AWS terminology, a self-managed cluster includes
non-AWS hosted Kafka clusters. For example, you can still use Lambda with a Kafka cluster hosted
with a non-AWS cloud provider such as [
Confluent Cloud](https://www.confluent.io/confluent-cloud/) or [Redpanda](https://www.redpanda.com/).
When deciding between Amazon MSK and self-managed Apache Kafka, consider your operational needs and control requirements. Amazon MSK
is a better choice if you want AWS to quickly help you manage a scalable, production-ready Kafka setup
with minimal operational overhead. It simplifies security, monitoring, and high availability, helping you
focus on application development rather than infrastructure management. On the other hand, self-managed Apache Kafka is
better suited for use cases running on non-AWS hosted environments, including on-premises clusters.
###### Topics
* [Using Lambda with Amazon MSK](./with-msk.html)
* [Using Lambda with self-managed Apache Kafka](./with-kafka.html)
* [Apache Kafka event poller scaling modes in Lambda](./kafka-scaling-modes.html)
* [Apache Kafka polling and stream starting positions in Lambda](./kafka-starting-positions.html)
* [Customizable consumer group ID in Lambda](./kafka-consumer-group-id.html)
* [Filtering events from Amazon MSK and self-managed Apache Kafka event sources](./kafka-filtering.html)
* [Using schema registries with Kafka event sources in Lambda](./services-consume-kafka-events.html)
* [Low latency processing for Kafka event
sources](./with-kafka-low-latency.html)
* [Configuring error handling controls for Kafka event sources](./kafka-retry-configurations.html)
* [Capturing discarded batches for Amazon MSK and self-managed Apache Kafka event sources](./kafka-on-failure.html)
* [Using a Kafka topic as an on-failure destination](./kafka-on-failure-destination.html)
* [Troubleshooting Kafka event source mapping errors](./with-kafka-troubleshoot.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrating other services
MSK
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.