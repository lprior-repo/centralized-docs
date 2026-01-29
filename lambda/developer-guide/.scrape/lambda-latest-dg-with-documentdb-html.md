---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-documentdb.html
title: Process Amazon DocumentDB events with Lambda
word_count: 2970
filtered: true
elements_removed: 0
density_score: 0.82
---

Process Amazon DocumentDB events with Lambda - AWS Lambda
Process Amazon DocumentDB events with Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-documentdb)
[Example Amazon DocumentDB event](#docdb-sample-event)[Prerequisites and permissions](#docdb-prereqs)[Configure network security](#docdb-network)[Creating an Amazon DocumentDB event source mapping (console)](#docdb-configuration)[Creating an Amazon DocumentDB event source mapping (SDK or CLI)](#docdb-api)[Polling and stream starting positions](#docdb-stream-polling)[Monitoring your Amazon DocumentDB event source](#docdb-monitoring)
# Process Amazon DocumentDB events with Lambda
You can use a Lambda function to process events in an [Amazon DocumentDB (with MongoDB compatibility) change stream](https://docs.aws.amazon.com/documentdb/latest/developerguide/change_streams.html) by configuring an
Amazon DocumentDB cluster as an event source. Then, you can automate event-driven workloads by invoking your Lambda function
each time that data changes with your Amazon DocumentDB cluster.
###### Note
Lambda supports version 4.0 and 5.0 of Amazon DocumentDB only. Lambda doesn't support version 3.6.
Also, for event source mappings, Lambda supports instance-based clusters and
regional clusters only. Lambda doesn't support
[
elastic clusters](https://docs.aws.amazon.com/documentdb/latest/developerguide/docdb-using-elastic-clusters.html) or
[
global clusters](https://docs.aws.amazon.com/documentdb/latest/developerguide/global-clusters.html). This limitation doesn't apply when using Lambda as a client to connect to Amazon DocumentDB. Lambda can connect to
all cluster types to perform CRUD operations.
Lambda processes events from Amazon DocumentDB change streams sequentially in the order in which they arrive.
Because of this, your function can handle only one concurrent invocation from Amazon DocumentDB at a time.
To monitor your function, you can track its [concurrency metrics](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-concurrency.html).
###### Warning
Lambda event source mappings process each event at least once, and duplicate processing of records can occur. To avoid potential issues
related to duplicate events, we strongly recommend that you make your function code idempotent. To learn more, see [How do I make my Lambda function idempotent](https://repost.aws/knowledge-center/lambda-function-idempotent)
in the AWS Knowledge Center.
###### Topics
* [Example Amazon DocumentDB event](#docdb-sample-event)
* [Prerequisites and permissions](#docdb-prereqs)
* [Configure network security](#docdb-network)
* [Creating an Amazon DocumentDB event source mapping (console)](#docdb-configuration)
* [Creating an Amazon DocumentDB event source mapping (SDK or CLI)](#docdb-api)
* [Polling and stream starting positions](#docdb-stream-polling)
* [Monitoring your Amazon DocumentDB event source](#docdb-monitoring)
* [Tutorial: Using AWS Lambda with Amazon DocumentDB Streams](./with-documentdb-tutorial.html)
## Example Amazon DocumentDB event
```
`{
"eventSourceArn": "arn:aws:rds:us-east-1:123456789012:cluster:canaryclusterb2a659a2-qo5tcmqkcl03",
"events": [
{
"event": {
"\_id": {
"\_data": "0163eeb6e7000000090100000009000041e1"
},
"clusterTime": {
"$timestamp": {
"t": 1676588775,
"i": 9
}
},
"documentKey": {
"\_id": {
"$oid": "63eeb6e7d418cd98afb1c1d7"
}
},
"fullDocument": {
"\_id": {
"$oid": "63eeb6e7d418cd98afb1c1d7"
},
"anyField": "sampleValue"
},
"ns": {
"db": "test\_database",
"coll": "test\_collection"
},
"operationType": "insert"
}
}
],
"eventSource": "aws:docdb"
}`
```
For more information about the events in this example and their shapes, see [Change Events](https://www.mongodb.com/docs/manual/reference/change-events/) on the MongoDB
Documentation website.
## Prerequisites and permissions
Before you can use Amazon DocumentDB as an event source for your Lambda function, note the following prerequisites. You
must:
* **Have an existing Amazon DocumentDB cluster in the same AWS account and AWS Region as your
function.** If you don't have an existing cluster, you can create one by following the steps in
[Get Started with
Amazon DocumentDB](https://docs.aws.amazon.com/documentdb/latest/developerguide/get-started-guide.html) in the *Amazon DocumentDB Developer Guide*. Alternatively, the first set of steps in
[Tutorial: Using AWS Lambda with Amazon DocumentDB Streams](./with-documentdb-tutorial.html) guide you
through creating an Amazon DocumentDB cluster with all the necessary prerequisites.
* **Allow Lambda to access the Amazon Virtual Private Cloud (Amazon VPC) resources associated with your Amazon DocumentDB
cluster.** For more information, see [Configure network security](#docdb-network).
* **Enable TLS on your Amazon DocumentDB cluster.** This is the default setting. If you
disable TLS, then Lambda cannot communicate with your cluster.
* **Activate change streams on your Amazon DocumentDB cluster.** For more information,
see [Using Change
Streams with Amazon DocumentDB](https://docs.aws.amazon.com/documentdb/latest/developerguide/change_streams.html) in the *Amazon DocumentDB Developer Guide*.
* **Provide Lambda with credentials to access your Amazon DocumentDB cluster.** When
setting up the event source, provide the [AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html) key that contains the authentication
details (username and password) required to access your cluster. To provide this key during setup, do either
of the following:
* If you're using the Lambda console for setup, then provide the key in the **Secrets manager
key** field.
* If you're using the AWS Command Line Interface (AWS CLI) for setup, then provide this key in the
`source-access-configurations` option. You can include this option with either the [`create-event-source-mapping`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-event-source-mapping.html) command or the [`update-event-source-mapping`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-event-source-mapping.html) command. For example:
```
`aws lambda create-event-source-mapping \\
...
--source-access-configurations '[{"Type":"BASIC\_AUTH","URI":"arn:aws:secretsmanager:us-west-2:123456789012:secret:DocDBSecret-AbC4E6"}]' \\
...`
```
* **Grant Lambda permissions to manage resources related to your Amazon DocumentDB
stream.** Manually add the following permissions to your function's [execution role](./lambda-intro-execution-role.html):
* [rds:DescribeDBClusters](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBClusters.html)
* [rds:DescribeDBClusterParameters](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBClusterParameters.html)
* [rds:DescribeDBSubnetGroups](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBSubnetGroups.html)
* [ec2:CreateNetworkInterface](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html)
* [ec2:DescribeNetworkInterfaces](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeNetworkInterfaces.html)
* [ec2:DescribeVpcs](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpcs.html)
* [ec2:DeleteNetworkInterface](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DeleteNetworkInterface.html)
* [ec2:DescribeSubnets](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSubnets.html)
* [ec2:DescribeSecurityGroups](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html)
* [kms:Decrypt](https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html)
* [secretsmanager:GetSecretValue](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html)
* **Keep the size of Amazon DocumentDB change stream events that you send to Lambda under 6
MB.** Lambda supports payload sizes of up to 6 MB. If your change stream tries to send Lambda an
event larger than 6 MB, then Lambda drops the message and emits the `OversizedRecordCount` metric.
Lambda emits all metrics on a best-effort basis.
###### Note
While Lambda functions typically have a maximum timeout limit of 15 minutes,
event source mappings for Amazon MSK, self-managed Apache Kafka, Amazon DocumentDB, and Amazon MQ for ActiveMQ and RabbitMQ only support functions with
maximum timeout limits of 14 minutes. This constraint ensures that the event source mapping can properly
handle function errors and retries.
## Configure network security
To give Lambda full access to Amazon DocumentDB through your event source mapping, either your cluster must use a public endpoint
(public IP address), or you must provide access to the Amazon VPC you created the cluster in.
When you use Amazon DocumentDB with Lambda, create [AWS PrivateLink VPC endpoints](https://docs.aws.amazon.com/vpc/latest/privatelink/create-interface-endpoint.html) that provide your function
access to the resources in your Amazon VPC.
###### Note
AWS PrivateLink VPC endpoints are required for functions with event source mappings that use the default (on-demand) mode
for event pollers. If your event source mapping uses [
provisioned mode](./invocation-eventsourcemapping.html#invocation-eventsourcemapping-provisioned-mode), you don't need to configure AWS PrivateLink VPC endpoints.
Create an endpoint to provide access to the following resources:
*
Lambda — Create an endpoint for the Lambda service principal.
*
AWS STS — Create an endpoint for the AWS STS in order for a service principal to assume a role on your behalf.
*
Secrets Manager — If your cluster uses Secrets Manager to store credentials, create an endpoint for Secrets Manager.
Alternatively, configure a NAT gateway on each public subnet in the Amazon VPC. For more information,
see [Enable internet access for VPC-connected Lambda functions](./configuration-vpc-internet.html).
When you create an event source mapping for Amazon DocumentDB, Lambda checks whether Elastic Network Interfaces (ENIs)
are already present for the subnets and security groups configured for your Amazon VPC. If Lambda finds existing ENIs, it
attempts to re-use them. Otherwise, Lambda creates new ENIs to connect to the event source and invoke your function.
###### Note
Lambda functions always run inside VPCs owned by the Lambda service. Your function's VPC configuration
does not affect the event source mapping. Only the networking configuration of the event source's determines
how Lambda connects to your event source.
Configure the security groups for the Amazon VPC containing your cluster. By default,
Amazon DocumentDB uses the following ports: `27017`.
* Inbound rules – Allow all traffic on the default broker port for the security group associated with your event source.
Alternatively, you can use a self-referencing security group rule to allow access from instances within the same security group.
* Outbound rules – Allow all traffic on port `443` for external destinations if your function needs to communicate with AWS services.
Alternatively, you can also use a self-referencing security group rule to limit access to the broker if you don't need to communicate with other AWS services.
* Amazon VPC endpoint inbound rules — If you are using an Amazon VPC endpoint, the security group associated with your Amazon VPC endpoint must allow inbound traffic
on port `443` from the cluster security group.
If your cluster uses authentication, you can also restrict the endpoint policy for the Secrets Manager endpoint.
To call the Secrets Manager API, Lambda uses your function role, not the Lambda service principal.
###### Example VPC endpoint policy — Secrets Manager endpoint
```
`{
"Statement": [
{
"Action": "secretsmanager:GetSecretValue",
"Effect": "Allow",
"Principal": {
"AWS": [
"arn:aws::iam::123456789012:role/`my-role`"
]
},
"Resource": "arn:aws::secretsmanager:`us-west-2`:123456789012:secret:`my-secret`"
}
]
}`
```
When you use Amazon VPC endpoints, AWS routes your API calls to invoke your function using the endpoint's Elastic Network Interface (ENI).
The Lambda service principal needs to call `lambda:InvokeFunction` on any roles and functions that use those ENIs.
By default, Amazon VPC endpoints have open IAM policies that allow broad access to resources. Best practice is to restrict these
policies to perform the needed actions using that endpoint. To ensure that your event source mapping is able to invoke your Lambda
function, the VPC endpoint policy must allow the Lambda service principal to call `sts:AssumeRole` and
`lambda:InvokeFunction`. Restricting your VPC endpoint policies to allow only API calls originating within your organization
prevents the event source mapping from functioning properly, so `"Resource": "\*"` is required in these policies.
The following example VPC endpoint policies show how to grant the required access to the Lambda service principal for the
AWS STS and Lambda endpoints.
###### Example VPC Endpoint policy — AWS STS endpoint
```
`{
"Statement": [
{
"Action": "sts:AssumeRole",
"Effect": "Allow",
"Principal": {
"Service": [
"lambda.amazonaws.com"
]
},
"Resource": "\*"
}
]
}`
```
###### Example VPC Endpoint policy — Lambda endpoint
```
`{
"Statement": [
{
"Action": "lambda:InvokeFunction",
"Effect": "Allow",
"Principal": {
"Service": [
"lambda.amazonaws.com"
]
},
"Resource": "\*"
}
]
}`
```
## Creating an Amazon DocumentDB event source mapping (console)
For a Lambda function to read from an Amazon DocumentDB cluster's change stream, create an [event source mapping](./invocation-eventsourcemapping.html). This section describes how to do this from
the Lambda console. For AWS SDK and AWS CLI instructions, see [Creating an Amazon DocumentDB event source mapping (SDK or CLI)](#docdb-api).
###### To create an Amazon DocumentDB event source mapping (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the name of a function.
3. Under **Function overview**, choose **Add
trigger**.
4. Under **Trigger configuration**, in the dropdown list, choose
**DocumentDB**.
5. Configure the required options, and then choose **Add**.
Lambda supports the following options for Amazon DocumentDB event sources:
* **DocumentDB cluster** – Select an Amazon DocumentDB cluster.
* **Activate trigger** – Choose whether you want to activate the trigger
immediately. If you select this check box, then your function immediately starts receiving traffic from the
specified Amazon DocumentDB change stream upon creation of the event source mapping. We recommend that you clear the
check box to create the event source mapping in a deactivated state for testing. After creation, you can
activate the event source mapping at any time.
* **Database name** – Enter the name of a database within the cluster to
consume.
* (Optional) **Collection name** – Enter the name of a collection within the
database to consume. If you don't specify a collection, then Lambda listens to all events from each collection
in the database.
* **Batch size** – Set the maximum number of messages to retrieve in a single batch,
up to 10,000. The default batch size is 100.
* **Starting position** – Choose the position in the stream to start reading records
from.
* **Latest** – Process only new records that are added to the stream. Your
function starts processing records only after Lambda finishes creating your event source. This means that
some records may be dropped until your event source is created successfully.
* **Trim horizon** – Process all records in the stream. Lambda uses the log
retention duration of your cluster to determine where to start reading events from. Specifically, Lambda
starts reading from `current\_time - log\_retention\_duration`. Your change stream must already be
active before this timestamp for Lambda to read all events properly.
* **At timestamp** – Process records starting from a specific time. Your change
stream must already be active before the specified timestamp for Lambda to read all events properly.
* **Authentication** – Choose the authentication method for accessing the brokers in
your cluster.
* **BASIC\_AUTH** – With basic authentication, you must provide the Secrets Manager key
that contains the credentials to access your cluster.
* **Secrets Manager key** – Choose the Secrets Manager key that contains the authentication details
(username and password) required to access your Amazon DocumentDB cluster.
* (Optional) **Batch window** – Set the maximum amount of time in seconds to gather
records before invoking your function, up to 300.
* (Optional) **Full document configuration** – For document update operations,
choose what you want to send to the stream. The default value is `Default`, which means that for
each change stream event, Amazon DocumentDB sends only a delta describing the changes made. For more information about
this field, see [FullDocument](https://mongodb.github.io/mongo-java-driver/3.9/javadoc/com/mongodb/client/model/changestream/FullDocument.html#DEFAULT) in the MongoDB Javadoc API documentation.
* **Default** – Lambda sends only a partial document describing the changes
made.
* **UpdateLookup** – Lambda sends a delta describing the changes, along with a
copy of the entire document.
## Creating an Amazon DocumentDB event source mapping (SDK or CLI)
To create or manage an Amazon DocumentDB event source mapping with an [AWS SDK](https://aws.amazon.com/developer/tools/), you can use the following API
operations:
* [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html)
* [ListEventSourceMappings](https://docs.aws.amazon.com/lambda/latest/api/API_ListEventSourceMappings.html)
* [GetEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_GetEventSourceMapping.html)
* [UpdateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html)
* [DeleteEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteEventSourceMapping.html)
To create the event source mapping with the AWS CLI, use the [`create-event-source-mapping`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-event-source-mapping.html) command. The following example uses this command to map a
function named `my-function` to an Amazon DocumentDB change stream. The event source is specified by an Amazon
Resource Name (ARN), with a batch size of 500, starting from the timestamp in Unix time. The command also
specifies the Secrets Manager key that Lambda uses to connect to Amazon DocumentDB. Additionally, it includes
`document-db-event-source-config` parameters that specify the database and the collection to read
from.
```
`aws lambda create-event-source-mapping --function-name my-function \\
--event-source-arn arn:aws:rds:us-west-2:123456789012:cluster:privatecluster7de2-epzcyvu4pjoy
--batch-size 500 \\
--starting-position AT\_TIMESTAMP \\
--starting-position-timestamp 1541139109 \\
--source-access-configurations '[{"Type":"BASIC\_AUTH","URI":"arn:aws:secretsmanager:us-east-1:123456789012:secret:DocDBSecret-BAtjxi"}]' \\
--document-db-event-source-config '{"DatabaseName":"test\_database", "CollectionName": "test\_collection"}' \\`
```
You should see output that looks like this:
```
`{
"UUID": "2b733gdc-8ac3-cdf5-af3a-1827b3b11284",
"BatchSize": 500,
"DocumentDBEventSourceConfig": {
"CollectionName": "test\_collection",
"DatabaseName": "test\_database",
"FullDocument": "Default"
},
"MaximumBatchingWindowInSeconds": 0,
"EventSourceArn": "arn:aws:rds:us-west-2:123456789012:cluster:privatecluster7de2-epzcyvu4pjoy",
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"LastModified": 1541348195.412,
"LastProcessingResult": "No records processed",
"State": "Creating",
"StateTransitionReason": "User action"
}`
```
After creation, you can use the [`update-event-source-mapping`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-event-source-mapping.html) command to update the settings for your Amazon DocumentDB event
source. The following example updates the batch size to 1,000 and the batch window to 10 seconds. For this
command, you need the UUID of your event source mapping, which you can retrieve using the
`list-event-source-mapping` command or the Lambda console.
```
`aws lambda update-event-source-mapping --function-name my-function \\
--uuid f89f8514-cdd9-4602-9e1f-01a5b77d449b \\
--batch-size 1000 \\
--batch-window 10`
```
You should see this output that looks like this:
```
`{
"UUID": "2b733gdc-8ac3-cdf5-af3a-1827b3b11284",
"BatchSize": 500,
"DocumentDBEventSourceConfig": {
"CollectionName": "test\_collection",
"DatabaseName": "test\_database",
"FullDocument": "Default"
},
"MaximumBatchingWindowInSeconds": 0,
"EventSourceArn": "arn:aws:rds:us-west-2:123456789012:cluster:privatecluster7de2-epzcyvu4pjoy",
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"LastModified": 1541359182.919,
"LastProcessingResult": "OK",
"State": "Updating",
"StateTransitionReason": "User action"
}`
```
Lambda updates settings asynchronously, so you may not see these changes in the output until the process
completes. To view the current settings of your event source mapping, use the [`get-event-source-mapping`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-event-source-mapping.html) command.
```
`aws lambda get-event-source-mapping --uuid f89f8514-cdd9-4602-9e1f-01a5b77d449b`
```
You should see this output that looks like this:
```
`{
"UUID": "2b733gdc-8ac3-cdf5-af3a-1827b3b11284",
"DocumentDBEventSourceConfig": {
"CollectionName": "test\_collection",
"DatabaseName": "test\_database",
"FullDocument": "Default"
},
"BatchSize": 1000,
"MaximumBatchingWindowInSeconds": 10,
"EventSourceArn": "arn:aws:rds:us-west-2:123456789012:cluster:privatecluster7de2-epzcyvu4pjoy",
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"LastModified": 1541359182.919,
"LastProcessingResult": "OK",
"State": "Enabled",
"StateTransitionReason": "User action"
}`
```
To delete your Amazon DocumentDB event source mapping, use the [`delete-event-source-mapping`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/delete-event-source-mapping.html) command.
```
`aws lambda delete-event-source-mapping \\
--uuid 2b733gdc-8ac3-cdf5-af3a-1827b3b11284`
```
## Polling and stream starting positions
Be aware that stream polling during event source mapping creation and updates is eventually consistent.
* During event source mapping creation, it may take several minutes to start polling events from the stream.
* During event source mapping updates, it may take several minutes to stop and restart polling events from the stream.
This behavior means that if you specify `LATEST` as the starting position for the stream, the event source mapping could
miss events during creation or updates. To ensure that no events are missed, specify the stream starting position as `TRIM\_HORIZON`
or `AT\_TIMESTAMP`.
## Monitoring your Amazon DocumentDB event source
To help you monitor your Amazon DocumentDB event source, Lambda emits the `IteratorAge` metric when your
function finishes processing a batch of records. *Iterator age* is the difference between the
timestamp of the most recent event and the current timestamp. Essentially, the `IteratorAge` metric
indicates how old the last processed record in the batch is. If your function is currently processing new events,
then you can use the iterator age to estimate the latency between when a record is added and when your function
processes it. An increasing trend in `IteratorAge` can indicate issues with your function.
For more information, see [Using CloudWatch metrics with Lambda](./monitoring-metrics.html).
Amazon DocumentDB change streams aren't optimized to handle large time gaps between events. If your Amazon DocumentDB event source doesn't
receive any events for an extended period of time, Lambda may disable the event source mapping. The length of
this time period can vary from a few weeks to a few months depending on cluster size and other workloads.
Lambda supports payloads of up to 6 MB. However, Amazon DocumentDB change stream events can be up to 16 MB in size. If
your change stream tries to send Lambda a change stream event larger than 6 MB, then Lambda drops the message and
emits the `OversizedRecordCount` metric. Lambda emits all metrics on a best-effort basis.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
CloudFormation
Tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.