---
url: https://docs.aws.amazon.com/lambda/latest/dg/msk-esm-create.html
title: Creating a Lambda event source mapping for an Amazon MSK event source
word_count: 841
filtered: true
elements_removed: 0
density_score: 0.85
---

Creating a Lambda event source mapping for an Amazon MSK event source - AWS Lambda
Creating a Lambda event source mapping for an Amazon MSK event source - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#msk-esm-create)
# Creating a Lambda event source mapping for an Amazon MSK event source
To create an event source mapping, you can use the Lambda console, the [AWS Command Line Interface (CLI)](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html), or an
[AWS SDK](https://aws.amazon.com/getting-started/tools-sdks/).
###### Note
When you create the event source mapping, Lambda creates a [
hyperplane ENI](./configuration-vpc.html#configuration-vpc-enis) in the private subnet that contains your MSK cluster, allowing Lambda to
establish a secure connection. This hyperplane ENI allows uses the subnet and security group
configuration of your MSK cluster, not your Lambda function.
The following console steps add an Amazon MSK cluster as a trigger for your Lambda function. Under the hood,
this creates an event source mapping resource.
###### To add an Amazon MSK trigger to your Lambda function (console)
1. Open the [Function page](https://console.aws.amazon.com/lambda/home#/functions)
of the Lambda console.
2. Choose the name of the Lambda function you want to add an Amazon MSK trigger to.
3. Under **Function overview**, choose **Add trigger**.
4. Under **Trigger configuration**, choose **MSK**.
5. To specify your Kafka cluster details, do the following:
1. For **MSK cluster**, select your cluster.
2. For **Topic name**, enter the name of the Kafka topic to consume
messages from.
3. For **Consumer group ID**, enter the ID of a Kafka consumer group
to join, if applicable. For more information, see [Customizable consumer group ID in Lambda](./kafka-consumer-group-id.html).
4. For **Cluster authentication**, make the necessary configurations. For more
information about cluster authentication, see [Configuring Amazon MSK cluster authentication methods in Lambda](./msk-cluster-auth.html).
* Toggle on **Use authentication** if you want Lambda to perform
authentication with your MSK cluster when establishing a connection. Authentication
is recommended.
* If you use authentication, for **Authentication method**, choose
the authentication method to use.
* If you use authentication, for **Secrets Manager key**, choose the Secrets Manager key
that contains the authentication credentials needed to access your cluster.
* Under **Event poller configuration**, make the necessary configurations.
* Choose **Activate trigger** to enable the trigger immediately after creation.
* Choose whether you want to **Configure provisioned mode** for your event source
mapping. For more information, see [Apache Kafka event poller scaling modes in Lambda](./kafka-scaling-modes.html).
* If you configure provisioned mode, enter a value for **Minimum event pollers**,
a value for **Maximum event pollers**, and an optional value for PollerGroupName to specify grouping of multiple ESMs within the same event source VPC.
* For **Starting position**, choose how you want Lambda to start reading from your stream.
For more information, see [Apache Kafka polling and stream starting positions in Lambda](./kafka-starting-positions.html).
* Under **Batching**, make the necessary configurations. For more information about
batching, see [Batching behavior](./invocation-eventsourcemapping.html#invocation-eventsourcemapping-batching).
1. For **Batch size**, enter the maximum number of messages to receive in
a single batch.
2. For **Batch window**, enter the maximum number of seconds that Lambda spends
gathering records before invoking the function.
3. Under **Filtering**, make the necessary configurations. For more information about
filtering, see [Filtering events from Amazon MSK and self-managed Apache Kafka event sources](./kafka-filtering.html).
* For **Filter criteria**, add filter criteria definitions to determine whether
or not to process an event.
* Under **Failure handling**, make the necessary configurations. For more information
about failure handling, see [Capturing discarded batches for Amazon MSK and self-managed Apache Kafka event sources](./kafka-on-failure.html).
* For **On-failure destination**, specify the ARN of your on-failure destination.
* For **Tags**, enter the tags to associate with this event source mapping.
* To create the trigger, choose **Add**.
You can also create the event source mapping using the AWS CLI with the [
create-event-source-mapping](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-event-source-mapping.html) command. The following example creates an event source mapping to map the Lambda
function `my-msk-function` to the `AWSKafkaTopic` topic, starting from the `LATEST`
message. This command also uses the [SourceAccessConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html) object to instruct
Lambda to use [SASL/SCRAM](./msk-cluster-auth.html#msk-sasl-scram) authentication when connecting to the cluster.
```
`aws lambda create-event-source-mapping \\
--event-source-arn arn:aws:kafka:us-east-1:111122223333:cluster/my-cluster/fc2f5bdf-fd1b-45ad-85dd-15b4a5a6247e-2 \\
--topics AWSKafkaTopic \\
--starting-position LATEST \\
--function-name my-kafka-function
--source-access-configurations '[{"Type": "SASL\_SCRAM\_512\_AUTH","URI": "arn:aws:secretsmanager:us-east-1:111122223333:secret:my-secret"}]'`
```
If the cluster uses [mTLS authentication](./msk-cluster-auth.html#msk-mtls), include a [SourceAccessConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html) object that specifies
`CLIENT\_CERTIFICATE\_TLS\_AUTH` and a Secrets Manager key ARN. This is shown in the following command:
```
`aws lambda create-event-source-mapping \\
--event-source-arn arn:aws:kafka:us-east-1:111122223333:cluster/my-cluster/fc2f5bdf-fd1b-45ad-85dd-15b4a5a6247e-2 \\
--topics AWSKafkaTopic \\
--starting-position LATEST \\
--function-name my-kafka-function
--source-access-configurations '[{"Type": "CLIENT\_CERTIFICATE\_TLS\_AUTH","URI": "arn:aws:secretsmanager:us-east-1:111122223333:secret:my-secret"}]'`
```
When the cluster uses [IAM authentication](./msk-cluster-auth.html#msk-iam-auth), you don’t need a
[
SourceAccessConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html) object. This is shown in the following command:
```
`aws lambda create-event-source-mapping \\
--event-source-arn arn:aws:kafka:us-east-1:111122223333:cluster/my-cluster/fc2f5bdf-fd1b-45ad-85dd-15b4a5a6247e-2 \\
--topics AWSKafkaTopic \\
--starting-position LATEST \\
--function-name my-kafka-function`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Cluster authentication
Cross-account event source mappings
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.