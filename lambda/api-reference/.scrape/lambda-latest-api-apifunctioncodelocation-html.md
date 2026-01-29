---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionCodeLocation.html
title: API FunctionCodeLocation.html
word_count: 118
filtered: true
elements_removed: 0
density_score: 0.93
---

FunctionCodeLocation - AWS Lambda
FunctionCodeLocation - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionCodeLocation)
[Contents](#API_FunctionCodeLocation_Contents)[See Also](#API_FunctionCodeLocation_SeeAlso)
## Contents
**
ImageUri
**
URI of a container image in the Amazon ECR registry.
Type: String
Required: No
**
Location
**
A presigned URL that you can use to download the deployment package.
Type: String
Required: No
**
RepositoryType
**
The service that's hosting the file.
Type: String
Required: No
**
ResolvedImageUri
**
The resolved URI for the image.
Type: String
Required: No
**
SourceKMSKeyArn
**
The ARN of the AWS Key Management Service (AWS KMS) customer managed key that's used to encrypt your function's
.zip deployment package. If you don't provide a customer managed key, Lambda uses an [AWS owned key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk).
Type: String
Required: No