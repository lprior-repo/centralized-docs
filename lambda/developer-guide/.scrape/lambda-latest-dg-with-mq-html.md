---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-mq.html
title: with mq.html
word_count: 1383
filtered: true
elements_removed: 0
density_score: 0.82
---

Using Lambda with Amazon MQ - AWS Lambda
Using Lambda with Amazon MQ - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-mq)
[Understanding the Lambda consumer group for Amazon MQ](#services-mq-configure)
###### Note
If you want to send data to a target other than a Lambda function or enrich the data before sending it, see [
Amazon EventBridge Pipes](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).
Amazon MQ is a managed message broker service for [Apache ActiveMQ](https://activemq.apache.org/)
and [RabbitMQ](https://www.rabbitmq.com). A *message broker* enables software
applications and components to communicate using various programming languages, operating systems, and formal
messaging protocols through either topic or queue event destinations.
Amazon MQ can also manage Amazon Elastic Compute Cloud (Amazon EC2) instances on your behalf by installing ActiveMQ or RabbitMQ brokers and by providing
different network topologies and other infrastructure needs.
You can use a Lambda function to process records from your Amazon MQ message broker. Lambda invokes your function
through an [event source mapping](./invocation-eventsourcemapping.html), a Lambda resource that reads
messages from your broker and invokes the function [synchronously](./invocation-sync.html).
###### Warning
Lambda event source mappings process each event at least once, and duplicate processing of records can occur. To avoid potential issues
related to duplicate events, we strongly recommend that you make your function code idempotent. To learn more, see [How do I make my Lambda function idempotent](https://repost.aws/knowledge-center/lambda-function-idempotent)
in the AWS Knowledge Center.
The Amazon MQ event source mapping has the following configuration restrictions:
* Concurrency – Lambda functions that use an Amazon MQ event source mapping have a default maximum [concurrency](./lambda-concurrency.html)
setting. For ActiveMQ, the Lambda service limits the number of concurrent execution environments to five per Amazon MQ
event source mapping. For RabbitMQ, the number of concurrent execution environments is limited to 1 per Amazon MQ
event source mapping. Even if you change your function's reserved or provisioned concurrency settings, the Lambda
service won't make more execution environments available. To request an increase in the default maximum concurrency
for a single Amazon MQ event source mapping, contact Support with the event source mapping UUID, as well as the region.
Because increases are applied at the specific event source mapping level, not the account or region level,
you need to manually request a scaling increase for each event source mapping.
* Cross account – Lambda does not support cross-account processing. You cannot use Lambda to process records
from an Amazon MQ message broker that is in a different AWS account.
* Authentication – For ActiveMQ, only the ActiveMQ [SimpleAuthenticationPlugin](https://activemq.apache.org/security#simple-authentication-plugin) is
supported. For RabbitMQ, only the [PLAIN](https://www.rabbitmq.com/access-control.html#mechanisms) authentication mechanism is supported. Users must use AWS Secrets Manager to manage their credentials.
For more information about ActiveMQ authentication, see [Integrating ActiveMQ brokers
with LDAP](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/security-authentication-authorization.html) in the *Amazon MQ Developer Guide*.
* Connection quota – Brokers have a maximum number of allowed connections per wire-level protocol. This
quota is based on the broker instance type. For more information, see the [Brokers](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-limits.html#broker-limits)
section of **Quotas in Amazon MQ** in the
*Amazon MQ Developer Guide*.
* Connectivity – You can create brokers in a public or private virtual private cloud (VPC). For private VPCs, your Lambda function needs access to the VPC to receive messages. For more information, see [Configure network security](./process-mq-messages-with-lambda.html#process-mq-messages-with-lambda-networkconfiguration) later in this
section.
* Event destinations – Only queue destinations are supported. However, you can use a virtual topic,
which behaves as a topic internally while interacting with Lambda as a queue. For more information, see [Virtual Destinations](https://activemq.apache.org/virtual-destinations)
on the Apache ActiveMQ website, and [Virtual Hosts](https://www.rabbitmq.com/vhosts.html)
on the RabbitMQ website.
* Network topology – For ActiveMQ, only one single-instance or standby broker is supported per event source mapping.
For RabbitMQ, only one single-instance broker or cluster deployment is supported per event source mapping.
Single-instance brokers require a failover endpoint. For more information about these broker deployment modes, see
[Active MQ Broker Architecture](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-broker-architecture.html) and
[Rabbit MQ Broker Architecture](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/rabbitmq-broker-architecture.html)in the *Amazon MQ Developer Guide*.
* Protocols – Supported protocols depend on the type of Amazon MQ integration.
* For ActiveMQ integrations, Lambda consumes messages using the OpenWire/Java Message Service (JMS) protocol. No other protocols are supported for consuming messages. Within the JMS protocol, only [`TextMessage`](https://activemq.apache.org/components/cms/api_docs/activemqcpp-3.6.0/html/classactivemq_1_1commands_1_1_active_m_q_text_message.html) and [`BytesMessage`](https://activemq.apache.org/components/cms/api_docs/activemqcpp-3.9.0/html/classactivemq_1_1commands_1_1_active_m_q_bytes_message.html) are supported. Lambda also supports JMS custom properties. For more information about the OpenWire protocol, see
[OpenWire](https://activemq.apache.org/openwire.html) on the Apache ActiveMQ website.
* For RabbitMQ integrations, Lambda consumes messages using the AMQP 0-9-1 protocol. No other protocols are supported
for consuming messages. For more information about RabbitMQ's implementation of the AMQP 0-9-1 protocol, see
[AMQP 0-9-1 Complete Reference
Guide](https://www.rabbitmq.com/amqp-0-9-1-reference.html) on the RabbitMQ website.
Lambda automatically supports the latest versions of ActiveMQ and RabbitMQ that Amazon MQ supports. For the latest
supported versions, see [Amazon MQ release notes](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-release-notes.html) in the
*Amazon MQ Developer Guide*.
###### Note
By default, Amazon MQ has a weekly maintenance window for brokers. During that
window of time, brokers are unavailable. For brokers without standby, Lambda cannot process any messages during that window.
###### Topics
* [Understanding the Lambda consumer group for Amazon MQ](#services-mq-configure)
* [Configuring Amazon MQ event source for Lambda](./process-mq-messages-with-lambda.html)
* [Event source mapping parameters](./services-mq-params.html)
* [Filter events from an Amazon MQ event source](./with-mq-filtering.html)
* [Troubleshoot Amazon MQ event source mapping errors](./services-mq-errors.html)
## Understanding the Lambda consumer group for Amazon MQ
To interact with Amazon MQ, Lambda creates a consumer group which can read from your Amazon MQ brokers. The consumer
group is created with the same ID as the event source mapping UUID.
For Amazon MQ event sources, Lambda batches records together and sends them to your function in a single payload.
To control behavior, you can configure the batching window and batch size. Lambda pulls messages until it processes
the payload size maximum of 6 MB, the batching window expires, or the number of records reaches the full batch
size. For more information, see [Batching behavior](./invocation-eventsourcemapping.html#invocation-eventsourcemapping-batching).
The consumer group retrieves the messages as a BLOB of bytes, base64-encodes them into a single JSON payload, and then invokes your function. If your function returns an error for any of the messages in a batch, Lambda retries the
whole batch of messages until processing succeeds or the messages expire.
###### Note
While Lambda functions typically have a maximum timeout limit of 15 minutes,
event source mappings for Amazon MSK, self-managed Apache Kafka, Amazon DocumentDB, and Amazon MQ for ActiveMQ and RabbitMQ only support functions with
maximum timeout limits of 14 minutes. This constraint ensures that the event source mapping can properly
handle function errors and retries.
You can monitor a given function's concurrency usage using the `ConcurrentExecutions` metric in
Amazon CloudWatch. For more information about concurrency, see [Configuring reserved concurrency for a function](./configuration-concurrency.html).
###### Example Amazon MQ record events
ActiveMQ
```
`{
"eventSource": "aws:mq",
"eventSourceArn": "arn:aws:mq:us-east-2:111122223333:broker:test:b-9bcfa592-423a-4942-879d-eb284b418fc8",
"messages": [
{
"messageID": "ID:b-9bcfa592-423a-4942-879d-eb284b418fc8-1.mq.us-east-2.amazonaws.com-37557-1234520418293-4:1:1:1:1",
"messageType": "jms/text-message",
"deliveryMode": 1,
"replyTo": null,
"type": null,
"expiration": "60000",
"priority": 1,
"correlationId": "myJMSCoID",
"redelivered": false,
"destination": {
"physicalName": "testQueue"
},
"data":"QUJDOkFBQUE=",
"timestamp": 1598827811958,
"brokerInTime": 1598827811958,
"brokerOutTime": 1598827811959,
"properties": {
"index": "1",
"doAlarm": "false",
"myCustomProperty": "value"
}
},
{
"messageID": "ID:b-9bcfa592-423a-4942-879d-eb284b418fc8-1.mq.us-east-2.amazonaws.com-37557-1234520418293-4:1:1:1:1",
"messageType": "jms/bytes-message",
"deliveryMode": 1,
"replyTo": null,
"type": null,
"expiration": "60000",
"priority": 2,
"correlationId": "myJMSCoID1",
"redelivered": false,
"destination": {
"physicalName": "testQueue"
},
"data":"LQaGQ82S48k=",
"timestamp": 1598827811958,
"brokerInTime": 1598827811958,
"brokerOutTime": 1598827811959,
"properties": {
"index": "1",
"doAlarm": "false",
"myCustomProperty": "value"
}
}
]
}`
```
RabbitMQ
```
`
{
"eventSource": "aws:rmq",
"eventSourceArn": "arn:aws:mq:us-east-2:111122223333:broker:pizzaBroker:b-9bcfa592-423a-4942-879d-eb284b418fc8",
"rmqMessagesByQueue": {
"pizzaQueue::/": [
{
"basicProperties": {
"contentType": "text/plain",
"contentEncoding": null,
"headers": {
"header1": {
"bytes": [
118,
97,
108,
117,
101,
49
]
},
"header2": {
"bytes": [
118,
97,
108,
117,
101,
50
]
},
"numberInHeader": 10
},
"deliveryMode": 1,
"priority": 34,
"correlationId": null,
"replyTo": null,
"expiration": "60000",
"messageId": null,
"timestamp": "Jan 1, 1970, 12:33:41 AM",
"type": null,
"userId": "AIDACKCEVSQ6C2EXAMPLE",
"appId": null,
"clusterId": null,
"bodySize": 80
},
"redelivered": false,
"data": "eyJ0aW1lb3V0IjowLCJkYXRhIjoiQ1pybWYwR3c4T3Y0YnFMUXhENEUifQ=="
}
]
}
}`
```
###### Note
In the RabbitMQ example, `pizzaQueue` is the name of the RabbitMQ queue, and `/` is the
name of the virtual host. When receiving messages, the event source lists messages under
`pizzaQueue::/`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Kubernetes
Configure event source
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.