---
url: https://docs.aws.amazon.com/lambda/latest/dg/msk-cross-account.html
title: Creating cross-account event source mappings in Lambda
word_count: 476
filtered: true
elements_removed: 0
density_score: 0.89
---

Creating cross-account event source mappings in Lambda - AWS Lambda
Creating cross-account event source mappings in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#msk-cross-account)
# Creating cross-account event source mappings in Lambda
You can use [multi-VPC private
connectivity](https://docs.aws.amazon.com/msk/latest/developerguide/aws-access-mult-vpc.html) to connect a Lambda function to a provisioned MSK cluster in a different AWS account.
Multi-VPC connectivity uses AWS PrivateLink, which keeps all traffic within the AWS network.
###### Note
You can't create cross-account event source mappings for serverless MSK clusters.
To create a cross-account event source mapping, you must first [configure multi-VPC
connectivity for the MSK cluster](https://docs.aws.amazon.com/msk/latest/developerguide/aws-access-mult-vpc.html#mvpc-cluster-owner-action-turn-on). When you create the event source mapping, use the managed VPC connection
ARN instead of the cluster ARN, as shown in the following examples. The [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) operation
also differs depending on which authentication type the MSK cluster uses.
###### Example — Create cross-account event source mapping for cluster that uses IAM authentication
When the cluster uses [IAM role-based authentication](./msk-cluster-auth.html#msk-iam-auth),
you don't need a [SourceAccessConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html) object. Example:
```
`aws lambda create-event-source-mapping \\
--event-source-arn arn:aws:kafka:`us-east-1:111122223333`:vpc-connection/`444455556666/my-cluster-name/51jn98b4-0a61-46cc-b0a6-61g9a3d797d5-7` \\
--topics AWSKafkaTopic \\
--starting-position LATEST \\
--function-name my-kafka-function`
```
###### Example — Create cross-account event source mapping for cluster that uses SASL/SCRAM authentication
If the cluster uses [SASL/SCRAM authentication](./msk-cluster-auth.html#msk-sasl-scram),
you must include a [SourceAccessConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html) object that specifies `SASL\_SCRAM\_512\_AUTH`
and a Secrets Manager secret ARN.
There are two ways to use secrets for cross-account Amazon MSK event source mappings with SASL/SCRAM authentication:
* Create a secret in the Lambda function account and sync it with the cluster secret.
[Create a rotation](https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets.html)
to keep the two secrets in sync. This option allows you to control the secret from the function account.
* Use the secret that's associated with the MSK cluster. This secret must allow cross-account
access to the Lambda function account. For more information, see [Permissions to AWS Secrets Manager
secrets for users in a different account](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples_cross.html).
```
``aws lambda create-event-source-mapping \\
--event-source-arn arn:aws:kafka:`us-east-1:111122223333`:vpc-connection/`444455556666/my-cluster-name/51jn98b4-0a61-46cc-b0a6-61g9a3d797d5-7` \\
--topics AWSKafkaTopic \\
--starting-position LATEST \\
--function-name my-kafka-function \\
--source-access-configurations `'[{"Type": "SASL\_SCRAM\_512\_AUTH","URI": "arn:aws:secretsmanager:us-east-1:444455556666:secret:my-secret"}]'```
```
###### Example — Create cross-account event source mapping for cluster that uses mTLS authentication
If the cluster uses [mTLS authentication](./msk-cluster-auth.html#msk-mtls),
you must include a [SourceAccessConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_SourceAccessConfiguration.html) object that specifies `CLIENT\_CERTIFICATE\_TLS\_AUTH`
and a Secrets Manager secret ARN. The secret can be stored in the cluster account or the Lambda function account.
```
``aws lambda create-event-source-mapping \\
--event-source-arn arn:aws:kafka:`us-east-1:111122223333`:vpc-connection/`444455556666/my-cluster-name/51jn98b4-0a61-46cc-b0a6-61g9a3d797d5-7` \\
--topics AWSKafkaTopic \\
--starting-position LATEST \\
--function-name my-kafka-function \\
--source-access-configurations `'[{"Type": "CLIENT\_CERTIFICATE\_TLS\_AUTH","URI": "arn:aws:secretsmanager:us-east-1:444455556666:secret:my-secret"}]'```
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Event source mapping
Configuration parameters
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.