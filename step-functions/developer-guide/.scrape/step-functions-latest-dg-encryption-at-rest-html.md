---
url: https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html
title: encryption at rest.html
word_count: 3572
filtered: true
elements_removed: 0
density_score: 0.83
---

Data at rest encryption in Step Functions - AWS Step Functions
Data at rest encryption in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#encryption-at-rest)
[Encrypting with a customer managed key](#enable-custom-encryption)[Create a state machine with AWS KMS key](#create-state-machine-with-cmk)[Create an activity with AWS KMS key](#create-activity-with-cmk)[Scope down permissions](#scope-down-kms-permission-policies-with-conditions)[Required permissions for API](#using-api-with-encryption-required-permissions)[CloudFormation resources](#cfn-resources-for-encryption-configuration)[Monitoring your encryption keys](#monitoring-encryption-keys)[FAQs](#faqs)[Learn more](#learn-more-data-at-rest-encryption)
###### Read the blog
Read about customer managed keys in [Strengthening data security with a customer-managed AWS KMS key](https://aws.amazon.com/blogs/compute/strengthening-data-security-in-aws-step-functions-with-a-customer-managed-aws-kms-key/)
AWS Step Functions always encrypts your data at rest using transparent server-side encryption.
Encryption of data at rest by default reduces the operational overhead and complexity
involved in protecting sensitive data. You can build security-sensitive applications
that meet strict encryption compliance and regulatory requirements.
Although you can't disable this layer of encryption or select an alternate encryption
type, you can add a second layer of encryption over the existing AWS owned encryption
keys by choosing a customer managed key when you create your state machine and activity resources:
* **Customer managed keys** — Step Functions supports
the use of a symmetric customer managed key that you create, own, and manage to add a
second layer of encryption over the existing AWS owned encryption. Because you
have full control of this layer of encryption, you can perform such tasks as:
* Establishing and maintaining key policies
* Establishing and maintaining IAM policies and grants
* Enabling and disabling key policies
* Rotating key cryptographic material
* Adding tags
* Creating key aliases
* Scheduling keys for deletion
For information, see [customer managed key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk)
in the *AWS Key Management Service Developer Guide*.
You can encrypt your data using a **customer-managed
key** for AWS Step Functions state machines and activities. You can configure a
symmetric AWS KMS key and data key reuse period when creating or updating a **State Machine**, and when creating an **Activity**. The execution history and state machine definition will be
encrypted with the key applied to the State Machine. Activity inputs will be encrypted with
the key applied to the Activity.
With customer managed AWS KMS keys, you can secure customer data that includes **protected health information (PHI)** from unauthorized access. Step Functions is integrated with CloudTrail, so you can view and audit the most recent events in the CloudTrail console in the event history.
For information on AWS KMS, see [What is AWS Key Management Service?](https://docs.aws.amazon.com/kms/latest/developerguide/overview.html)
###### Note
Step Functions automatically enables encryption at rest using AWS owned keys at no charge. However, AWS KMS charges apply when using a customer managed key. For information about pricing, see [AWS Key Management Service pricing](https://aws.amazon.com/kms/pricing/).
## Encrypting with a customer managed key
Step Functions decrypts payload data with your customer managed AWS KMS key before passing it to another service to perform a task. The data is encrypted in transit using Transport Layer Security (TLS).
When data is returned from an integrated service, Step Functions encrypts the data with your customer managed AWS KMS key. You can use the same key to consistently apply encryption across many AWS services.
You can use a customer managed key with the following resources:
* **State Machine** - both Standard and
Express workflow types
* **Activity**
You can specify the data key by entering a **KMS key
ID**, which Step Functions uses to encrypt your data.
* **KMS key ID** —
[key
identifier](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id) for an AWS KMS customer managed key in the form of a key ID, key ARN,
alias name, or alias ARN.
## Create a State Machine with a customer managed key
**Prerequisite:** Before you can create a state machine with customer managed AWS KMS keys, your user or role must have AWS KMS permissions to `DescribeKey` and `GenerateDataKey`.
You can perform the following steps in the AWS console, through the API, or by provisioning infrastructure through CloudFormation resources. (CloudFormation examples are presented later in this guide.)
### Step 1: Create AWS KMS key
You can create a symmetric customer managed key with the AWS KMS console or AWS KMS APIs.
**To create a symmetric customer managed key**
Follow the steps for [Creating
symmetric customer managed key](https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#create-symmetric-cmk) in the *AWS Key Management Service Developer
Guide*.
###### Note
*Optional*: When creating a key, you may choose **Key administrators**.
The selected users or roles will be granted access manage the key, such as enabling or disabling the key
through the API. You may also choose **Key users**. These users or roles will be granted the ability to use the AWS KMS key in cryptographic operations.
### Step 2: Set AWS KMS key policy
Key policies control access to your customer managed key. Every customer managed key must have exactly
one key policy, which contains statements that determine who can use the key and how
they can use it. When you create your customer managed key, you can specify a key policy. For
information, see [Managing access to customer managed keys](https://docs.aws.amazon.com/kms/latest/developerguide/control-access-overview.html#managing-access) in the *AWS Key Management Service
Developer Guide*.
The following is an example AWS KMS key policy from console, without **Key administrators** or **Key users**:
****
```
``{
"Version":"2012-10-17",
"Id": "key-consolepolicy-1",
"Statement": [
{
"Sid": "Enable IAM User Permissions for the key",
"Effect": "Allow",
"Principal": {
"AWS": "arn:aws:iam::`123456789012`:root"
},
"Action": "kms:\*",
"Resource": "\*"
}
]
}`
`
```
See the *AWS Key Management Service Developer Guide* for information about [specifying permissions in a policy](https://docs.aws.amazon.com/kms/latest/developerguide/control-access-overview.html#overview-policy-elements) and [troubleshooting key access](https://docs.aws.amazon.com/kms/latest/developerguide/policy-evaluation.html#example-no-iam).
### Step 3: Add key policy to encrypt CloudWatch logs
Step Functions is integrated with CloudWatch for logging and monitoring. When you enable server-side encryption for your state machine using your own KMS key and enable CloudWatch Log integration,
you must allow `**delivery.logs.amazonaws.com**` to do `kms:Decrypt`
action from your AWS KMS key policy:
```
`{
"Sid": "Enable log service delivery for integrations",
"Effect": "Allow",
"Principal": {
"Service": "delivery.logs.amazonaws.com"
},
"Action": "kms:Decrypt",
"Resource": "\*"
}`
```
If you enable state machine encryption with a AWS KMS key, and your state machine has CloudWatch Logs integration enabled, the state machine's execution role needs the following policy:
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "AllowKMSPermissionForCloudWatchLogGroup",
"Effect": "Allow",
"Action": "kms:GenerateDataKey",
"Resource": "arn:aws:kms:`us-east-1`:`123456789012`:key/`keyId`",
"Condition": {
"StringEquals": {
"kms:EncryptionContext:SourceArn": "arn:aws:logs:`us-east-1`:`123456789012`:\*"
}
}
}
]
}`
`
```
### Step 4: Encrypt CloudWatch Log Group *(Optional)*
You can enable encryption of the logs in a CloudWatch Log Group using your own AWS KMS key. To do that, you must also add the following policy to that AWS KMS key.
###### Note
You can choose the same or different AWS KMS keys to encrypt your logs and your state machine definitions.
****
```
``{
"Id": "key-consolepolicy-logging",
"Version":"2012-10-17",
"Statement": [
{
"Sid": "Enable log service for a single log group",
"Effect": "Allow",
"Principal": {
"Service": "logs.`us-east-1`.amazonaws.com"
},
"Action": [
"kms:Encrypt\*",
"kms:Decrypt\*",
"kms:ReEncrypt\*",
"kms:GenerateDataKey\*",
"kms:Describe\*"
],
"Resource": "\*",
"Condition": {
"ArnEquals": {
"kms:EncryptionContext:aws:logs:arn": "arn:aws:logs:`us-east-1`:`123456789012`:log-group:`LOG\_GROUP\_NAME`"
}
}
}
]
}`
`
```
###### Note
The `Condition` section restricts the AWS KMS key to a single log group ARN.
###### Note
See [CloudWatch logs documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/encrypt-log-data-kms.html#cmk-permissions) to learn more about setting permissions on the AWS KMS key for your log group.
### Step 5: Create state machine
After you have created a key and set up the policy, you can use the key to create a new state machine.
When creating the state machine, choose **Additional configuration**, then choose to
encrypt with customer managed key. You can then select your key and set the data key reuse period from 1 min to 15 minutes.
Optionally, you can enable logging by setting a log level and choosing to encrypt the log group with your AWS KMS key.
###### Note
You can only enable encryption on a **new log group** in the
Step Functions console. To learn how to associate a AWS KMS key with an existing log group, see [Associate a AWS KMS key with a log group](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/encrypt-log-data-kms.html#associate-cmk).
To successfully start an execution for Standard workflows and Asynchronous Express workflows with customer managed keys, your execution role requires `kms:Decrypt` and
`kms:GenerateDataKey` permissions. The execution role for Synchronous Express execution requires `kms:Decrypt`. When you create a state machine in the console and choose **Create a new
role**, these permissions are automatically included for you.
The following is an example execution role policy:
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "AllowKMSPermissionsForStepFunctionsWorkflowExecutions",
"Effect": "Allow",
"Action": [
"kms:Decrypt",
"kms:GenerateDataKey"
],
"Resource": [
"arn:aws:kms:`us-east-1`:`123456789012`:key/keyId"
],
"Condition": {
"StringEquals": {
"kms:EncryptionContext:aws:states:stateMachineArn": [
"arn:aws:states:`us-east-1`:`123456789012`:stateMachine:stateMachineName"
]
}
}
}
]
}`
`
```
### Step 6: Invoke state machine encrypted with your AWS KMS key
You can invoke your encrypted state machine as you normally would, and your data will be encrypted with your customer managed key.
## Create an Activity with a customer managed key
Creating an Step Functions Activity with a customer managed key is similar to creating a state machine with a customer managed key. Before you can create a activity with customer managed AWS KMS keys, your user or role only needs AWS KMS permissions to `DescribeKey`. During creation of the Activity, you choose the key and set the encryption configuration parameters.
Note that Step Functions Activity resources remain **immutable**. You cannot update the `encryptionConfiguration` for an activity ARN of an existing activity; you must create a new Activity resource. Callers to the Activity API endpoints must have `kms:DescribeKey` permissions to successfully create an activity with a AWS KMS key.
When customer managed key encryption is enabled on an Activity Task, the state machine execution role will require `kms:GenerateDataKey` and `kms:Decrypt` permission for the activity key. If you are creating this state machine from the Step Functions console, the auto role creation feature will add these permissions.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "AllowKMSPermissionsForStepFunctionsActivities",
"Effect": "Allow",
"Action": [
"kms:Decrypt",
"kms:GenerateDataKey"
],
"Resource": [
"arn:aws:kms:`us-east-1`:`123456789012`:key/keyId"
],
"Condition": {
"StringEquals": {
"kms:EncryptionContext:aws:states:activityArn": [
"arn:aws:states:`us-east-1`:`123456789012`:activity:activityName"
]
}
}
}
]
}`
`
```
## Scope down AWS KMS permission policies with conditions
You can use the *encryption context* in key policies and IAM policies as `conditions` to control access to your symmetric customer managed key. To limit the use of a AWS KMS key to requests from Step Functions on behalf of a specific role, you can use the `kms:ViaService` condition.
### Scoping with encryption context
An [encryption context](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context) is an optional set of key-value pairs that contain additional
contextual information about the data.
AWS KMS uses the encryption context as additional authenticated data to support authenticated encryption. When you include an encryption context in a request to encrypt data, AWS KMS binds the encryption context to the encrypted data.
To decrypt data, you include the same encryption context in the request.
Step Functions provides an encryption context in AWS KMS cryptographic
operations, where the key is `aws:states:stateMachineArn` for State Machines or `aws:states:activityArn` for Activities, and the value is the
resource [Amazon Resource
Name](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) (ARN).
```
`"encryptionContext": {"aws:states:stateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:stateMachineName"}`
```
```
`"encryptionContext": {"aws:states:activityArn": "arn:aws:states:`region`:`account-id`:activity:activityName"}`
```
The following example shows how to limit the use of a AWS KMS key for execution roles to specific state machines with `kms:EncryptionContext` and the `aws:states:stateMachineArn` context key:
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "AllowKeyManagement",
"Effect": "Allow",
"Action": [
"kms:Decrypt",
"kms:GenerateDataKey"
],
"Resource": [
"arn:aws:kms:`us-east-1`:`123456789012`:key/keyId"
],
"Condition": {
"StringEquals": {
"kms:EncryptionContext:aws:states:stateMachineArn": "arn:aws:states:`us-east-1`:`123456789012`:stateMachine:stateMachineName"
}
}
}
]
}`
`
```
### Scoping with kms:ViaService
The `kms:ViaService` condition key limits use of an AWS Key Management Service key to requests from specified AWS services.
The following example policy uses the `kms:ViaService` condition to allow the AWS KMS key to be used for specific actions only when the request originates from Step Functions in the `us-east-1` region, acting on behalf of the ExampleRole:
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "Allow access for Key Administrators in a region",
"Effect": "Allow",
"Principal": {
"AWS": "arn:aws:iam::`123456789012`:role/ExampleRole"
},
"Action": [
"kms:Decrypt",
"kms:GenerateDataKey"
],
"Resource": "\*",
"Condition": {
"StringEquals": {
"kms:ViaService": "states.us-east-1.amazonaws.com"
}
}
}
]
}`
`
```
###### Note
The `kms:ViaService` condition is only applicable when AWS KMS permissions are required by the API caller (for example, `CreateStateMachine`, `CreateActivity`, `GetActivityTask`, etc.). Adding `kms:ViaService` condition to an **execution role** can prevent a new execution from starting or cause a running execution to fail.
## Required permissions for API callers
To call Step Functions API actions that return encrypted data, callers need AWS KMS
permissions. Alternatively, some API actions have an option (`METADATA\_ONLY`) to return only metadata, removing the requirement for AWS KMS permissions. Refer to the Step Functions API for information.
For an execution to successfully complete when using customer managed key encryption, the
execution role needs to be granted `kms:GenerateDataKey` and `kms:Decrypt` permissions for AWS KMS keys used by the state machine.
The following table shows the AWS KMS permissions you need to provide to Step Functions API callers for the APIs using a **State Machine's AWS KMS key**. You can provide the permissions to
the key policy or IAM policy for the role.
|**APIs using State Machine's AWS KMS
key**|**Required by Caller**|
|**CreateStateMachine**| kms:DescribeKey, kms:GenerateDataKey |
|**UpdateStateMachine**| kms:DescribeKey, kms:GenerateDataKey |
|**DescribeStateMachine**| kms:Decrypt |
|**DescribeStateMachineForExecution**| kms:Decrypt |
|**StartExecution**| -- |
|**StartSyncExecution**| kms:Decrypt |
|**SendTaskSuccess**| -- |
|**SendTaskFailure**| -- |
|**StopExecution**| -- |
|**RedriveExecution**| -- |
|**DescribeExecution**| kms:Decrypt |
|**GetExecutionHistory**| kms:Decrypt |
The following table shows the AWS KMS permissions you need to provide to Step Functions API callers for the APIs using an **Activity's AWS KMS key**. You can provide the
permissions in the key policy or IAM policy for the role.
|**APIs using Activity's AWS KMS
key**|**Required by Caller**|
|**CreateActivity**| kms:DescribeKey |
|**GetActivityTask**| kms:Decrypt |
###### When do I grant permissions to the Caller or the Execution role?
When an IAM role or user calls the Step Functions API, the Step Functions service calls AWS KMS on behalf of the API caller. In this case, you must grant AWS KMS permission to the API caller. When an execution role calls AWS KMS directly, you must grant AWS KMS permissions on the execution role.
## CloudFormation resources for encryption configuration
CloudFormation resource types for Step Functions can provision state machine and activity resources with encryption configurations.
By default, Step Functions provides transparent server-side encryption. Both [`AWS::StepFunctions::Activity`](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-activity.html) and [`AWS::StepFunctions::StateMachine`](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html) accept an optional `EncryptionConfiguration` property which can configure a customer managed AWS KMS key for server-side encryption.
**Prerequisite:** Before you can create a state machine with customer managed AWS KMS keys, your user or role must have AWS KMS permissions to `DescribeKey` and `GenerateDataKey`.
Updates to StateMachine requires [No interruption](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html#update-no-interrupt). Updates to Activity resources requires: [Replacement](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html#update-replacement).
To declare an **`EncryptionConfiguration`** property in your CloudFormation template, use the following syntax:
**JSON**
```
`{
"KmsKeyId" : String,
"KmsDataKeyReusePeriodSeconds" : Integer,
"Type" : String
}`
```
**YAML**
```
`KmsKeyId: String
KmsDataKeyReusePeriodSeconds: Integer
Type: String`
```
**Properties**
* **Type** - Encryption option for the state machine or activity. *Allowed values*: `CUSTOMER\_MANAGED\_KMS\_KEY` | `AWS\_OWNED\_KEY`
* **KmsKeyId** - Alias, alias ARN, key ID, or key ARN of the symmetric encryption AWS KMS key that encrypts the data key. To specify a AWS KMS key in a different AWS account, the customer must use the key ARN or alias ARN. For information regarding kmsKeyId, see [KeyId](https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters) in AWS KMS docs.
* **KmsDataKeyReusePeriodSeconds** - Maximum duration for which SFN will reuse data keys. When the period expires, Step Functions will call `GenerateDataKey`. This setting can only be set when **Type** is `CUSTOMER\_MANAGED\_KMS\_KEY`. The value can range from 60-900 seconds. Default is 300 seconds.
#### Example: StateMachine with customer managed key
```
`AWSTemplateFormatVersion: '2010-09-09'
Description: An example template for a Step Functions State Machine.
Resources:
MyStateMachine:
Type: AWS::StepFunctions::StateMachine
Properties:
StateMachineName: HelloWorld-StateMachine
Definition:
StartAt: PassState
States:
PassState:
Type: Pass
End: true
RoleArn: !Sub "arn:${AWS::Partition}:iam::${AWS::AccountId}:role/example"
EncryptionConfiguration:
KmsKeyId: !Ref MyKmsKey
KmsDataKeyReusePeriodSeconds: 100
Type: CUSTOMER\_MANAGED\_KMS\_KEY
MyKmsKey:
Type: AWS::KMS::Key
Properties:
Description: Symmetric KMS key used for encryption/decryption`
```
#### Example: Activity with customer managed key
```
`AWSTemplateFormatVersion: '2010-09-09'
Description: An example template for a Step Functions Activity.
Resources:
Activity:
Type: AWS::StepFunctions::Activity
Properties:
Name: ActivityWithKmsEncryption
EncryptionConfiguration:
KmsKeyId: !Ref MyKmsKey
KmsDataKeyReusePeriodSeconds: 100
Type: CUSTOMER\_MANAGED\_KMS\_KEY
MyKmsKey:
Type: AWS::KMS::Key
Properties:
Description: Symmetric KMS key used for encryption/decryption
`
```
#### Updating encryption for an Activity requires creating a new resource
Activity configuration is immutable, and resource names must be unique. To set customer managed keys for encryption, you must create a **new Activity**. If you attempt to change the configuration in your CFN template for an existing activity, you will receive an `ActivityAlreadyExists` exception.
To update your activity to include customer managed keys, set a new activity name within your CFN template. The following shows an example that creates a new activity with a customer managed key configuration:
**Existing activity definition**
```
`AWSTemplateFormatVersion: '2010-09-09'
Description: An example template for a new Step Functions Activity.
Resources:
Activity:
Type: AWS::StepFunctions::Activity
Properties:
Name: ActivityName
EncryptionConfiguration:
Type: AWS\_OWNED\_KEY`
```
**New activity definition **
```
`AWSTemplateFormatVersion: '2010-09-09'
Description: An example template for a Step Functions Activity.
Resources:
Activity:
Type: AWS::StepFunctions::Activity
Properties:
Name: ActivityWithKmsEncryption
EncryptionConfiguration:
KmsKeyId: !Ref MyKmsKey
KmsDataKeyReusePeriodSeconds: 100
Type: CUSTOMER\_MANAGED\_KMS\_KEY
MyKmsKey:
Type: AWS::KMS::Key
Properties:
Description: Symmetric KMS key used for encryption/decryption`
```
## Monitoring your encryption key usage
When you use an AWS KMS customer managed key to encrypt your Step Functions resources, you can use CloudTrail
to track requests that Step Functions sends to AWS KMS.
You can also use the encryption context in audit records and logs to identify how
the customer managed key is being used. The encryption context also appears in logs generated by [AWS CloudTrail](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-user-guide.html).
The following examples are CloudTrail events for `Decrypt`, `DescribeKey`, and
`GenerateDataKey` to monitor AWS KMS operations called by Step Functions to access
data encrypted by your customer managed key:
Decrypt
When you access an encrypted state machine or activity, Step Functions
calls the `Decrypt` operation to use the stored encrypted
data key to access the encrypted data.
The following example event records the `Decrypt` operation:
```
`{
"eventVersion": "1.09",
"userIdentity": {
"type": "AssumedRole",
"principalId": "111122223333:Sampleuser01",
"arn": "arn:aws:sts::111122223333:assumed-role/Admin/Sampleuser01",
"accountId": "111122223333",
"accessKeyId": "ASIAIOSFODNN7EXAMPLE",
"sessionContext": {
"sessionIssuer": {
"type": "Role",
"principalId": "111122223333:Sampleuser01",
"arn": "arn:aws:sts::111122223333:assumed-role/Admin/Sampleuser01",
"accountId": "111122223333",
"userName": "Admin"
},
"attributes": {
"creationDate": "2024-07-05T21:06:27Z",
"mfaAuthenticated": "false"
}
},
"invokedBy": "states.amazonaws.com"
},
"eventTime": "2024-07-05T21:12:21Z",
"eventSource": "kms.amazonaws.com",
"eventName": "Decrypt",
"awsRegion": "aa-example-1",
"sourceIPAddress": "states.amazonaws.com",
"userAgent": "states.amazonaws.com",
"requestParameters": {
"encryptionAlgorithm": "SYMMETRIC\_DEFAULT",
"keyId": "arn:aws:kms:aa-example-1:111122223333:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111",
"encryptionContext": {
"aws:states:stateMachineArn": "arn:aws:states:aa-example-1:111122223333:stateMachine:example1"
}
},
"responseElements": null,
"requestID": "ff000af-00eb-00ce-0e00-ea000fb0fba0SAMPLE",
"eventID": "ff000af-00eb-00ce-0e00-ea000fb0fba0SAMPLE",
"readOnly": true,
"resources": [
{
"accountId": "111122223333",
"type": "AWS::KMS::Key",
"ARN": "arn:aws:kms:aa-example-1:111122223333:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
}
],
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "111122223333",
"eventCategory": "Management"
}`
```
DescribeKey
Step Functions uses the `DescribeKey` operation to verify if the
AWS KMS customer managed key associated with your State Machine or Activity
exists in the account and region.
The following example event records the `DescribeKey`operation:
```
`{
"eventVersion": "1.09",
"userIdentity": {
"type": "AssumedRole",
"principalId": "111122223333:Sampleuser01",
"arn": "arn:aws:sts::111122223333:assumed-role/Admin/Sampleuser01",
"accountId": "111122223333",
"accessKeyId": "ASIAIOSFODNN7EXAMPLE",
"sessionContext": {
"sessionIssuer": {
"type": "Role",
"principalId": "111122223333:Sampleuser01",
"arn": "arn:aws:sts::111122223333:assumed-role/Admin/Sampleuser01",
"accountId": "111122223333",
"userName": "Admin"
},
"attributes": {
"creationDate": "2024-07-05T21:06:27Z",
"mfaAuthenticated": "false"
}
},
"invokedBy": "states.amazonaws.com"
},
"eventTime": "2024-07-05T21:12:21Z",
"eventSource": "kms.amazonaws.com",
"eventName": "DescribeKey",
"awsRegion": "aa-example-1",
"sourceIPAddress": "states.amazonaws.com",
"userAgent": "states.amazonaws.com",
"requestParameters": {
"keyId": "arn:aws:kms:aa-example-1:111122223333:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
},
"responseElements": null,
"requestID": "ff000af-00eb-00ce-0e00-ea000fb0fba0SAMPLE",
"eventID": "ff000af-00eb-00ce-0e00-ea000fb0fba0SAMPLE",
"readOnly": true,
"resources": [
{
"accountId": "111122223333",
"type": "AWS::KMS::Key",
"ARN": "arn:aws:kms:aa-example-1:111122223333:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
}
],
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "111122223333",
"eventCategory": "Management",
"sessionCredentialFromConsole": "true"
}`
```
GenerateDataKey
When you enable an AWS KMS customer managed key for your State Machine or Activity, Step Functions sends a `GenerateDataKey` request to get a data key to the encrypt state machine definition or execution data.
The following example event records the `GenerateDataKey`operation:
```
`{
"eventVersion": "1.09",
"userIdentity": {
"type": "AssumedRole",
"principalId": "111122223333:Sampleuser01",
"arn": "arn:aws:sts::111122223333:assumed-role/Admin/Sampleuser01",
"accountId": "111122223333",
"accessKeyId": "ASIAIOSFODNN7EXAMPLE",
"sessionContext": {
"sessionIssuer": {
"type": "Role",
"principalId": "111122223333:Sampleuser01",
"arn": "arn:aws:iam::111122223333:role/Admin",
"accountId": "111122223333",
"userName": "Admin"
},
"attributes": {
"creationDate": "2024-07-05T21:06:27Z",
"mfaAuthenticated": "false"
}
},
"invokedBy": "states.amazonaws.com"
},
"eventTime": "2024-07-05T21:12:21Z",
"eventSource": "kms.amazonaws.com",
"eventName": "GenerateDataKey",
"awsRegion": "aa-example-1",
"sourceIPAddress": "states.amazonaws.com",
"userAgent": "states.amazonaws.com",
"requestParameters": {
"keySpec": "AES\_256",
"encryptionContext": {
"aws:states:stateMachineArn": "arn:aws:states:aa-example-1:111122223333:stateMachine:example1"
},
"keyId": "arn:aws:kms:aa-example-1:111122223333:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
},
"responseElements": null,
"requestID": "ff000af-00eb-00ce-0e00-ea000fb0fba0SAMPLE",
"eventID": "ff000af-00eb-00ce-0e00-ea000fb0fba0SAMPLE",
"readOnly": true,
"resources": [
{
"accountId": "111122223333",
"type": "AWS::KMS::Key",
"ARN": "arn:aws:kms:aa-example-1:111122223333:key/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111"
}
],
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "111122223333",
"eventCategory": "Management"
}`
```
### What happens if my key is marked for deletion or deleted in AWS KMS?
If the key is deleted or marked for deletion in AWS KMS, any related running executions will fail. New executions cannot be started until you remove or change the key associated with the workflow. After a AWS KMS key is deleted, all encrypted data associated with the workflow execution will remain encrypted and can no longer be decrypted, making the data ***unrecoverable***.
### What happens if a AWS KMS key is disabled in AWS KMS?
If a AWS KMS key is disabled in AWS KMS, any related running executions will fail. New executions cannot be started. You can no longer decrypt the data encrypted under that disabled AWS KMS key until it is re-enabled.
### What happens to Execution Status change events sent to EventBridge?
Execution Input, Output, Error, and Cause will not be included for execution status change events for workflows that are encrypted using your customer managed AWS KMS key.
## Learn more
For information about data encryption at rest, see [AWS Key Management Service concepts](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html) and [security best practices for AWS Key Management Service](https://docs.aws.amazon.com/kms/latest/developerguide/best-practices.html) in the *AWS Key Management Service Developer Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data protection
Data in transit encryption
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.