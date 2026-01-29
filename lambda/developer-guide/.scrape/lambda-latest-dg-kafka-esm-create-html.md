---
url: https://docs.aws.amazon.com/lambda/latest/dg/kafka-esm-create.html
title: Creating a Lambda event source mapping for a self-managed Apache Kafka event source
word_count: 860
filtered: true
elements_removed: 0
density_score: 0.85
---

Creating a Lambda event source mapping for a self-managed Apache Kafka event source - AWS Lambda
Creating a Lambda event source mapping for a self-managed Apache Kafka event source - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#kafka-esm-create)
[Prerequisites](#kafka-esm-prereqs)[Using the Lambda console](#kafka-esm-console)[Using the AWS CLI](#kafka-esm-cli)
# Creating a Lambda event source mapping for a self-managed Apache Kafka event source
To create an event source mapping, you can use the Lambda console, the [AWS Command Line Interface (CLI)](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html), or an
[AWS SDK](https://aws.amazon.com/getting-started/tools-sdks/).
The following console steps add a self-managed Apache Kafka cluster as a trigger for your Lambda function. Under the hood,
this creates an event source mapping resource.
## Prerequisites
* A self-managed Apache Kafka cluster. Lambda supports Apache Kafka version 0.10.1.0 and later.
* An [execution role](./lambda-intro-execution-role.html) with permission to access the AWS resources that your self-managed Kafka
cluster uses.
## Adding a self-managed Kafka cluster (console)
Follow these steps to add your self-managed Apache Kafka cluster and a Kafka topic as a trigger for your Lambda function.
###### To add an Apache Kafka trigger to your Lambda function (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda
console.
2. Choose the name of your Lambda function.
3. Under **Function overview**, choose **Add trigger**.
4. Under **Trigger configuration**, do the following:
1. Choose the **Apache Kafka** trigger type.
2. For **Bootstrap servers**, enter the host and port pair address of a Kafka broker
in your cluster, and then choose **Add**. Repeat for each Kafka broker in the
cluster.
3. For **Topic name**, enter the name of the Kafka topic used to store records in the
cluster.
4. If you configure provisioned mode, enter a value for **Minimum event pollers**,
a value for **Maximum event pollers**, and an optional value for PollerGroupName to specify grouping of multiple ESMs within the same event source VPC.
5. (Optional) For **Batch size**, enter the maximum number of records to receive in a
single batch.
6. For **Batch window**, enter the maximum amount of seconds that Lambda spends
gathering records before invoking the function.
7. (Optional) For **Consumer group ID**, enter the ID of a Kafka consumer group to join.
8. (Optional) For **Starting position**, choose **Latest** to start
reading the stream from the latest record, **Trim horizon** to start at the
earliest available record, or **At timestamp** to specify a timestamp to start
reading from.
9. (Optional) For **VPC**, choose the Amazon VPC for your Kafka cluster. Then, choose the
**VPC subnets** and **VPC security groups**.
This setting is required if only users within your VPC access your brokers.
10. (Optional) For **Authentication**, choose **Add**, and then do the
following:
1. Choose the access or authentication protocol of the Kafka brokers in your cluster.
* If your Kafka broker uses SASL/PLAIN authentication, choose
**BASIC\_AUTH**.
* If your broker uses SASL/SCRAM authentication, choose one of the
**SASL\_SCRAM** protocols.
* If you're configuring mTLS authentication, choose the
**CLIENT\_CERTIFICATE\_TLS\_AUTH** protocol.
* For SASL/SCRAM or mTLS authentication, choose the Secrets Manager secret key that contains the
credentials for your Kafka cluster.
* (Optional) For **Encryption**, choose the Secrets Manager secret containing the root CA
certificate that your Kafka brokers use for TLS encryption, if your Kafka brokers use certificates
signed by a private CA.
This setting applies to TLS encryption for SASL/SCRAM or SASL/PLAIN, and to mTLS
authentication.
* To create the trigger in a disabled state for testing (recommended), clear **Enable
trigger**. Or, to enable the trigger immediately, select **Enable
trigger**.
* To create the trigger, choose **Add**.
## Adding a self-managed Kafka cluster (AWS CLI)
Use the following example AWS CLI commands to create and view a self-managed Apache Kafka trigger for your Lambda function.
### Using SASL/SCRAM
If Kafka users access your Kafka brokers over the internet, specify the Secrets Manager secret that you created
for SASL/SCRAM authentication. The following example uses the [create-event-source-mapping](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-event-source-mapping.html) AWS CLI command to map a Lambda function named `my-kafka-function` to a Kafka topic named `AWSKafkaTopic`.
```
``aws lambda create-event-source-mapping \\
--topics `AWSKafkaTopic` \\
--source-access-configuration Type=SASL\_SCRAM\_512\_AUTH,URI=arn:aws:secretsmanager:us-east-1:`111122223333`:secret:`MyBrokerSecretName` \\
--function-name arn:aws:lambda:us-east-1:`111122223333`:function:`my-kafka-function` \\
--self-managed-event-source '{"Endpoints":{"KAFKA\_BOOTSTRAP\_SERVERS":["`abc3.xyz.com:9092`", "`abc2.xyz.com:9092`"]}}'``
```
### Using a VPC
If only Kafka users within your VPC access your Kafka brokers, you must specify your VPC, subnets, and VPC
security group. The following example uses the [create-event-source-mapping](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-event-source-mapping.html) AWS CLI command to map a Lambda function named `my-kafka-function` to a Kafka topic named `AWSKafkaTopic`.
```
``aws lambda create-event-source-mapping \\
--topics `AWSKafkaTopic` \\
--source-access-configuration '[{"Type": "VPC\_SUBNET", "URI": "subnet:subnet-0011001100"}, {"Type": "VPC\_SUBNET", "URI": "subnet:subnet-0022002200"}, {"Type": "VPC\_SECURITY\_GROUP", "URI": "security\_group:sg-0123456789"}]' \\
--function-name arn:aws:lambda:us-east-1:`111122223333`:function:`my-kafka-function` \\
--self-managed-event-source '{"Endpoints":{"KAFKA\_BOOTSTRAP\_SERVERS":["`abc3.xyz.com:9092`", "`abc2.xyz.com:9092`"]}}'``
```
### Viewing the status using the AWS CLI
The following example uses the [get-event-source-mapping](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-event-source-mapping.html) AWS CLI command to describe the status of the event source mapping that you created.
```
``aws lambda get-event-source-mapping
--uuid `dh38738e-992b-343a-1077-3478934hjkfd7```
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Cluster authentication
Configuration parameters
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.