---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html
title: Using Lambda with Amazon MSK
word_count: 514
filtered: true
elements_removed: 0
density_score: 0.77
---

Using Lambda with Amazon MSK - AWS Lambda
Using Lambda with Amazon MSK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-msk)
[ Example event](#msk-sample-event)
# Using Lambda with Amazon MSK
[Amazon Managed Streaming for Apache Kafka (Amazon MSK)](https://docs.aws.amazon.com/msk/latest/developerguide/what-is-msk.html) is a
fully-managed service that you can use to build and run applications that use Apache Kafka to process streaming data.
Amazon MSK simplifies the setup, scaling, and management of Kafka clusters. Amazon MSK also makes it easier to
configure your application for multiple Availability Zones and for security with AWS Identity and Access Management (IAM).
This chapter explains how to use an Amazon MSK cluster as an event source for your Lambda function. The general
process for integrating Amazon MSK with Lambda involves the following steps:
1. **[Cluster and network setup](./with-msk-cluster-network.html)**
– First, set up your [Amazon MSK
cluster](https://docs.aws.amazon.com/msk/latest/developerguide/what-is-msk.html). This includes the correct networking configuration to allow Lambda to access your cluster.
2. **[Event source mapping setup](./with-msk-configure.html)**
– Then, create the [event source mapping](./invocation-eventsourcemapping.html)
resource that Lambda needs to securely connect your Amazon MSK cluster to your function.
3. **[Function and permissions setup](./with-msk-permissions.html)**
– Finally, ensure that your function is correctly set up, and has the necessary permissions in its
[execution role](./lambda-intro-execution-role.html).
###### Note
You can now create and manage your Amazon MSK event source mappings directly from either the Lambda or the Amazon MSK console. Both consoles offer the option to automatically handle the setup of the necessary Lambda execution role permissions for a more streamlined configuration process.
For examples on how to set up a Lambda integration with an Amazon MSK cluster, see [Tutorial: Using an Amazon MSK event source mapping to invoke a Lambda function](./services-msk-tutorial.html),
[Using Amazon MSK as an event source for
AWS Lambda](https://aws.amazon.com/blogs/compute/using-amazon-msk-as-an-event-source-for-aws-lambda/) on the AWS Compute Blog, and [
Amazon MSK Lambda Integration](https://amazonmsk-labs.workshop.aws/en/msklambda.html) in the Amazon MSK Labs.
###### Topics
* [ Example event](#msk-sample-event)
* [Configuring your Amazon MSK cluster and Amazon VPC network for Lambda](./with-msk-cluster-network.html)
* [Configuring Lambda permissions for Amazon MSK event source mappings](./with-msk-permissions.html)
* [Configuring Amazon MSK event sources for Lambda](./with-msk-configure.html)
* [Tutorial: Using an Amazon MSK event source mapping to invoke a Lambda function](./services-msk-tutorial.html)
## Example event
Lambda sends the batch of messages in the event parameter when it invokes your function. The event payload
contains an array of messages. Each array item contains details of the Amazon MSK topic and partition identifier,
together with a timestamp and a base64-encoded message.
```
`{
"eventSource":"aws:kafka",
"eventSourceArn":"arn:aws:kafka:us-east-1:123456789012:cluster/vpc-2priv-2pub/751d2973-a626-431c-9d4e-d7975eb44dd7-2",
"bootstrapServers":"b-2.demo-cluster-1.a1bcde.c1.kafka.us-east-1.amazonaws.com:9092,b-1.demo-cluster-1.a1bcde.c1.kafka.us-east-1.amazonaws.com:9092",
"records":{
"mytopic-0":[
{
"topic":"mytopic",
"partition":0,
"offset":15,
"timestamp":1545084650987,
"timestampType":"CREATE\_TIME",
"key":"abcDEFghiJKLmnoPQRstuVWXyz1234==",
"value":"SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==",
"headers":[
{
"headerKey":[
104,
101,
97,
100,
101,
114,
86,
97,
108,
117,
101
]
}
]
}
]
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Apache Kafka
Cluster and network setup
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.