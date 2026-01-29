---
url: https://docs.aws.amazon.com/lambda/latest/dg/security-encryption-at-rest.html
title: Data encryption at rest for AWS Lambda
word_count: 716
filtered: true
elements_removed: 0
density_score: 0.79
---

Data encryption at rest for AWS Lambda - AWS Lambda
Data encryption at rest for AWS Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#security-encryption-at-rest)
[Monitoring your encryption keys for Lambda](#encryption-key-monitoring)
# Data encryption at rest for AWS Lambda
Lambda always provides at-rest encryption for the following resources using an [AWS owned key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk) or an [AWS managed key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk):
* Environment variables
* Files that you upload to Lambda, including deployment packages and layer archives
* Event source mapping filter criteria objects
You can optionally configure Lambda to use a customer managed key to encrypt your [environment variables](./configuration-envvars-encryption.html), [.zip deployment packages](./encrypt-zip-package.html), and [filter criteria objects](./invocation-eventfiltering.html#filter-criteria-encryption).
Amazon CloudWatch Logs and AWS X-Ray also encrypt data by default, and can be configured to use a
customer managed key. For details, see [
Encrypt log data in CloudWatch Logs](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/encrypt-log-data-kms.html) and
[Data protection in AWS X-Ray](https://docs.aws.amazon.com/xray/latest/devguide/xray-console-encryption.html).
## Monitoring your encryption keys for Lambda
When you use an AWS KMS customer managed key with Lambda, you can use [AWS CloudTrail](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-user-guide.html). The following
examples are CloudTrail events for `Decrypt`, `DescribeKey`, and
`GenerateDataKey` calls made by Lambda to access data encrypted by your customer
managed key.
Decrypt
If you used a AWS KMS customer managed key to encrypt your
[filter criteria](./invocation-eventfiltering.html#filter-criteria-encryption) object, Lambda
sends a `Decrypt` request on your behalf when you try to access it
in plaintext (for example, from a `ListEventSourceMappings` call).
The following example event records the `Decrypt` operation:
```
`{
"eventVersion": "1.09",
"userIdentity": {
"type": "AssumedRole",
"principalId": "AROA123456789EXAMPLE:example",
"arn": "arn:aws:sts::123456789012:assumed-role/role-name/example",
"accountId": "123456789012",
"accessKeyId": "ASIAIOSFODNN7EXAMPLE",
"sessionContext": {
"sessionIssuer": {
"type": "Role",
"principalId": "AROA123456789EXAMPLE",
"arn": "arn:aws:iam::123456789012:role/role-name",
"accountId": "123456789012",
"userName": "role-name"
},
"attributes": {
"creationDate": "2024-05-30T00:45:23Z",
"mfaAuthenticated": "false"
}
},
"invokedBy": "lambda.amazonaws.com"
},
"eventTime": "2024-05-30T01:05:46Z",
"eventSource": "kms.amazonaws.com",
"eventName": "Decrypt",
"awsRegion": "eu-west-1",
"sourceIPAddress": "lambda.amazonaws.com",
"userAgent": "lambda.amazonaws.com",
"requestParameters": {
"keyId": "arn:aws:kms:eu-west-1:123456789012:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111",
"encryptionContext": {
"aws-crypto-public-key": "ABCD+7876787678+CDEFGHIJKL/888666888999888555444111555222888333111==",
"aws:lambda:EventSourceArn": "arn:aws:sqs:eu-west-1:123456789012:sample-source",
"aws:lambda:FunctionArn": "arn:aws:lambda:eu-west-1:123456789012:function:sample-function"
},
"encryptionAlgorithm": "SYMMETRIC\_DEFAULT"
},
"responseElements": null,
"requestID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEaaaaa",
"eventID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEbbbbb",
"readOnly": true,
"resources": [
{
"accountId": "AWS Internal",
"type": "AWS::KMS::Key",
"ARN": "arn:aws:kms:eu-west-1:123456789012:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
}
],
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "123456789012",
"eventCategory": "Management",
"sessionCredentialFromConsole": "true"
}`
```
DescribeKey
If you used a AWS KMS customer managed key to encrypt your
[filter criteria](./invocation-eventfiltering.html#filter-criteria-encryption) object, Lambda
sends a `DescribeKey` request on your behalf when you try to access it
(for example, from a `GetEventSourceMapping` call).
The following example event records the `DescribeKey` operation:
```
`{
"eventVersion": "1.09",
"userIdentity": {
"type": "AssumedRole",
"principalId": "AROA123456789EXAMPLE:example",
"arn": "arn:aws:sts::123456789012:assumed-role/role-name/example",
"accountId": "123456789012",
"accessKeyId": "ASIAIOSFODNN7EXAMPLE",
"sessionContext": {
"sessionIssuer": {
"type": "Role",
"principalId": "AROA123456789EXAMPLE",
"arn": "arn:aws:iam::123456789012:role/role-name",
"accountId": "123456789012",
"userName": "role-name"
},
"attributes": {
"creationDate": "2024-05-30T00:45:23Z",
"mfaAuthenticated": "false"
}
}
},
"eventTime": "2024-05-30T01:09:40Z",
"eventSource": "kms.amazonaws.com",
"eventName": "DescribeKey",
"awsRegion": "eu-west-1",
"sourceIPAddress": "54.240.197.238",
"userAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10\_15\_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
"requestParameters": {
"keyId": "a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
},
"responseElements": null,
"requestID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEaaaaa",
"eventID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEbbbbb",
"readOnly": true,
"resources": [
{
"accountId": "AWS Internal",
"type": "AWS::KMS::Key",
"ARN": "arn:aws:kms:eu-west-1:123456789012:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
}
],
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "123456789012",
"eventCategory": "Management",
"tlsDetails": {
"tlsVersion": "TLSv1.3",
"cipherSuite": "TLS\_AES\_256\_GCM\_SHA384",
"clientProvidedHostHeader": "kms.eu-west-1.amazonaws.com"
},
"sessionCredentialFromConsole": "true"
}`
```
GenerateDataKey
When you use a AWS KMS customer managed key to encrypt your
[filter criteria](./invocation-eventfiltering.html#filter-criteria-encryption) object in a
`CreateEventSourceMapping` or `UpdateEventSourceMapping`
call, Lambda sends a `GenerateDataKey` request on your behalf to
generate a data key to encrypt the filter criteria ([envelope encryption](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#enveloping)).
The following example event records the `GenerateDataKey` operation:
```
`{
"eventVersion": "1.09",
"userIdentity": {
"type": "AssumedRole",
"principalId": "AROA123456789EXAMPLE:example",
"arn": "arn:aws:sts::123456789012:assumed-role/role-name/example",
"accountId": "123456789012",
"accessKeyId": "ASIAIOSFODNN7EXAMPLE",
"sessionContext": {
"sessionIssuer": {
"type": "Role",
"principalId": "AROA123456789EXAMPLE",
"arn": "arn:aws:iam::123456789012:role/role-name",
"accountId": "123456789012",
"userName": "role-name"
},
"attributes": {
"creationDate": "2024-05-30T00:06:07Z",
"mfaAuthenticated": "false"
}
},
"invokedBy": "lambda.amazonaws.com"
},
"eventTime": "2024-05-30T01:04:18Z",
"eventSource": "kms.amazonaws.com",
"eventName": "GenerateDataKey",
"awsRegion": "eu-west-1",
"sourceIPAddress": "lambda.amazonaws.com",
"userAgent": "lambda.amazonaws.com",
"requestParameters": {
"numberOfBytes": 32,
"keyId": "arn:aws:kms:eu-west-1:123456789012:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111",
"encryptionContext": {
"aws-crypto-public-key": "ABCD+7876787678+CDEFGHIJKL/888666888999888555444111555222888333111==",
"aws:lambda:EventSourceArn": "arn:aws:sqs:eu-west-1:123456789012:sample-source",
"aws:lambda:FunctionArn": "arn:aws:lambda:eu-west-1:123456789012:function:sample-function"
},
},
"responseElements": null,
"requestID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEaaaaa",
"eventID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEbbbbb",
"readOnly": true,
"resources": [
{
"accountId": "AWS Internal",
"type": "AWS::KMS::Key",
"ARN": "arn:aws:kms:eu-west-1:123456789012:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
}
],
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "123456789012",
"eventCategory": "Management"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data protection
Using service-linked roles
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.