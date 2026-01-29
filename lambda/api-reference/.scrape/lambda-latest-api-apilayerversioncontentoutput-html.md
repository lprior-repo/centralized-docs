---
url: https://docs.aws.amazon.com/lambda/latest/api/API_LayerVersionContentOutput.html
title: LayerVersionContentOutput
word_count: 108
filtered: true
elements_removed: 0
density_score: 0.93
---

LayerVersionContentOutput - AWS Lambda
LayerVersionContentOutput - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_LayerVersionContentOutput)
[Contents](#API_LayerVersionContentOutput_Contents)[See Also](#API_LayerVersionContentOutput_SeeAlso)
# LayerVersionContentOutput
Details about a version of an [AWS Lambda
layer](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
## Contents
**
CodeSha256
**
The SHA-256 hash of the layer archive.
Type: String
Required: No
**
CodeSize
**
The size of the layer archive in bytes.
Type: Long
Required: No
**
Location
**
A link to the layer archive in Amazon S3 that is valid for 10 minutes.
Type: String
Required: No
**
SigningJobArn
**
The Amazon Resource Name (ARN) of a signing job.
Type: String
Required: No
**
SigningProfileVersionArn
**
The Amazon Resource Name (ARN) for a signing profile version.
Type: String
Required: No