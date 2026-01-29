---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionCode.html
title: FunctionCode
word_count: 230
filtered: true
elements_removed: 0
density_score: 0.81
---

FunctionCode - AWS Lambda
FunctionCode - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionCode)
[Contents](#API_FunctionCode_Contents)[See Also](#API_FunctionCode_SeeAlso)
# FunctionCode
The code for the Lambda function. You can either specify an object in Amazon S3, upload a
.zip file archive deployment package directly, or specify the URI of a container image.
## Contents
**
ImageUri
**
URI of a [container image](https://docs.aws.amazon.com/lambda/latest/dg/lambda-images.html) in the
Amazon ECR registry.
Type: String
Required: No
**
S3Bucket
**
An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.
Type: String
Length Constraints: Minimum length of 3. Maximum length of 63.
Pattern: `[0-9A-Za-z\\.\\-\_]\*(?&lt;&lt;!\\.)`
Required: No
**
S3Key
**
The Amazon S3 key of the deployment package.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
S3ObjectVersion
**
For versioned objects, the version of the deployment package object to use.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
SourceKMSKeyArn
**
The ARN of the AWS Key Management Service (AWS KMS) customer managed key that's used to encrypt your function's
.zip deployment package. If you don't provide a customer managed key, Lambda uses an [AWS owned key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk).
Type: String
Pattern: `(arn:(aws[a-zA-Z-]\*)?:[a-z0-9-.]+:.\*)|()`
Required: No
**
ZipFile
**
The base64-encoded contents of the deployment package. AWS SDK and AWS CLI clients handle the encoding for
you.
Type: Base64-encoded binary data object
Required: No