---
url: https://docs.aws.amazon.com/lambda/latest/api/API_LayerVersionContentInput.html
title: LayerVersionContentInput
word_count: 150
filtered: true
elements_removed: 0
density_score: 0.83
---

LayerVersionContentInput - AWS Lambda
LayerVersionContentInput - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_LayerVersionContentInput)
[Contents](#API_LayerVersionContentInput_Contents)[See Also](#API_LayerVersionContentInput_SeeAlso)
# LayerVersionContentInput
A ZIP archive that contains the contents of an [AWS Lambda
layer](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html). You can specify either an Amazon S3 location,
or upload a layer archive directly.
## Contents
**
S3Bucket
**
The Amazon S3 bucket of the layer archive.
Type: String
Length Constraints: Minimum length of 3. Maximum length of 63.
Pattern: `[0-9A-Za-z\\.\\-\_]\*(?&lt;&lt;!\\.)`
Required: No
**
S3Key
**
The Amazon S3 key of the layer archive.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
S3ObjectVersion
**
For versioned objects, the version of the layer archive object to use.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
ZipFile
**
The base64-encoded contents of the layer archive. AWS SDK and AWS CLI clients handle the encoding for
you.
Type: Base64-encoded binary data object
Required: No