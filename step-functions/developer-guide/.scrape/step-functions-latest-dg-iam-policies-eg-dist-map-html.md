---
url: https://docs.aws.amazon.com/step-functions/latest/dg/iam-policies-eg-dist-map.html
title: IAM policies for using Distributed Map states
word_count: 1086
filtered: true
elements_removed: 0
density_score: 0.86
---

IAM policies for using Distributed Map states - AWS Step Functions
IAM policies for using Distributed Map states - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#iam-policies-eg-dist-map)
[Example of IAM policy for running a Distributed Map state](#iam-policy-run-dist-map)[Example of IAM policy for redriving a Distributed Map](#iam-policy-redrive-dist-map)[Examples of IAM policies for reading data from Amazon S3 datasets](#iam-policy-eg-item-reader)[Example of IAM policy for writing data to an Amazon S3 bucket](#iam-policy-eg-result-writer)
# IAM policies for using Distributed Map states
When you create workflows with the Step Functions console, Step Functions can automatically generate IAM policies based on the resources in your workflow definition. Generated policies include the least privileges necessary to allow the state machine role to invoke the `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API action for the *Distributed Map state* and access AWS resources, such as Amazon S3 buckets and objects, and Lambda functions.
We recommend including only the necessary permissiosn in your IAM policies. For example, if your workflow includes a `Map` state in Distributed mode, scope your policies down to the specific Amazon S3 bucket and folder that contains your data.
###### Important
If you specify an Amazon S3 bucket and object, or prefix, with a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) to an existing key-value pair in your *Distributed Map state* input, make sure that you update the IAM policies for your workflow. Scope the policies down to the bucket and object names the path resolves to at runtime.
## Example of IAM policy for running a Distributed Map state
When you include a *Distributed Map state* in your workflows, Step Functions needs appropriate permissions to allow the state machine role to invoke the `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API action for the *Distributed Map state*.
The following IAM policy example grants the least privileges required to your state machine role for running the *Distributed Map state*.
###### Note
Make sure that you replace ``stateMachineName`` with the name of the state machine in which you're using the *Distributed Map state*. For example, `arn:aws:states:`region`:`account-id`:stateMachine:`mystateMachine``.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:StartExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:stateMachine:`myStateMachineName`"
]
},
{
"Effect": "Allow",
"Action": [
"states:DescribeExecution"
],
"Resource": "arn:aws:states:`us-east-1`:`123456789012`:execution:`myStateMachineName`:\*"
}
]
}`
`
```
## Example of IAM policy for redriving a Distributed Map
You can restart unsuccessful child workflow executions in a Map Run by [redriving](./redrive-executions.html) your [parent workflow](./state-map-distributed.html#dist-map-orchestrate-parallel-workloads-key-terms). A redriven
parent workflow redrives all the unsuccessful states, including Distributed Map. Make sure that your execution role has the least privileges necessary to allow it to invoke the `[RedriveExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_RedriveExecution.html)` API action on the parent workflow.
The following IAM policy example grants the least privileges required to your state machine role for redriving a *Distributed Map state*.
###### Note
Make sure that you replace ``stateMachineName`` with the name of the state machine in which you're using the *Distributed Map state*. For example, `arn:aws:states:`region`:`account-id`:stateMachine:`mystateMachine``.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:RedriveExecution"
],
"Resource": "arn:aws:states:us-east-2:`123456789012`:execution:`myStateMachineName`/`myMapRunLabel`:\*"
}
]
}`
`
```
## Examples of IAM policies for reading data from Amazon S3 datasets
The following examples show techniques for granting the least privileges required to access your
Amazon S3 datasets using the [ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html) and [GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html) API
actions.
###### Example condition using an Amazon S3 object as a dataset
The following condition grants the least privileges to access
objects in a ``processImages`` folder of an
Amazon S3 bucket.
```
`"Resource": [ "arn:aws:s3:::`amzn-s3-demo-bucket`" ],
"Condition": {
"StringLike": {
"s3:prefix": [ "`processImages`" ]
}
}`
```
###### Example using a CSV file as a dataset
The following example shows the actions required to access a CSV file named ``ratings.csv``.
```
`"Action": [ "s3:GetObject" ],
"Resource": [
"arn:aws:s3:::`amzn-s3-demo-bucket`/`csvDataset`/`ratings.csv`"
]`
```
###### Example using an Amazon S3 inventory as a dataset
The following shows example resources for an Amazon S3 inventory manifest and data files.
```
`"Resource": [
"arn:aws:s3:::myPrefix/`amzn-s3-demo-bucket`/myConfig-id/`YYYY-MM-DDTHH-MMZ`/manifest.json",
"arn:aws:s3:::myPrefix/`amzn-s3-demo-bucket`/myConfig-id/data/\*"
]`
```
###### Example using ListObjectsV2 to restrict to a folder prefix
When using [ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html), two policies
will be generated. One is needed to allow **listing** the
contents of the bucket (`ListBucket`) and another policy will allow **retrieving objects** in the bucket (`GetObject`).
The following show example actions, resources, and a condition:
```
`"Action": [ "s3:ListBucket" ],
"Resource": [ "arn:aws:s3:::`amzn-s3-demo-bucket`" ],
"Condition": {
"StringLike": {
"s3:prefix": [ "/path/to/your/json/" ]
}
}`
```
```
`"Action": [ "s3:GetObject" ],
"Resource": [ "arn:aws:s3:::`amzn-s3-demo-bucket`/path/to/your/json/\*" ]`
```
Note that `GetObject` will not be scoped and you will use a wildcard
(`\*`) for the object.
## Example of IAM policy for writing data to an Amazon S3 bucket
The following IAM policy example grants the least privileges required to write your
child workflow execution results to a folder named `csvJobs` in an
Amazon S3 bucket using the `[PutObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)` API
action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"s3:PutObject",
"s3:GetObject",
"s3:ListMultipartUploadParts",
"s3:AbortMultipartUpload"
],
"Resource": [
"arn:aws:s3:::`amzn-s3-demo-destination-bucket`/`csvJobs`/\*"
]
}
]
}`
`
```
### IAM permissions for AWS KMS key encrypted Amazon S3 bucket
*Distributed Map state* uses multipart uploads to write the child workflow execution results to an Amazon S3 bucket. If the bucket is encrypted using an AWS Key Management Service (AWS KMS) key, you must also include permissions in your IAM policy to perform the `kms:Decrypt`, `kms:Encrypt`, and `kms:GenerateDataKey` actions on the key. These permissions are required because Amazon S3 must decrypt and read data from the encrypted file parts before it completes the multipart upload.
The following IAM policy example grants permission to the `kms:Decrypt`, `kms:Encrypt`, and `kms:GenerateDataKey` actions on the key used to encrypt your Amazon S3 bucket.
****
```
``{
"Version":"2012-10-17",
"Statement": {
"Effect": "Allow",
"Action": [
"kms:Decrypt",
"kms:Encrypt",
"kms:GenerateDataKey"
],
"Resource": [
"arn:aws:kms:`us-east-1`:`123456789012`:key/`111aa2bb-333c-4d44-5555-a111bb2c33dd`"
]
}
}`
`
```
For more information, see [Uploading
a large file to Amazon S3 with encryption using an AWS KMS key](https://aws.amazon.com/premiumsupport/knowledge-center/s3-large-file-encryption-kms-key/) in the *AWS Knowledge Center*.
If your IAM user or role is in the same AWS account as the KMS key, then you must have these permissions on the key policy. If your IAM user or role belongs to a different account than the KMS key, then you must have the permissions on both the key policy and your IAM user or role.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IAM Policies for integrated
services
Creating tag-based policies
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.