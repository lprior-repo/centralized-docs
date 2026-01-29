---
url: https://docs.aws.amazon.com/lambda/latest/api/API_EnvironmentResponse.html
title: EnvironmentResponse
word_count: 85
filtered: true
elements_removed: 0
density_score: 0.87
---

EnvironmentResponse - AWS Lambda
EnvironmentResponse - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_EnvironmentResponse)
[Contents](#API_EnvironmentResponse_Contents)[See Also](#API_EnvironmentResponse_SeeAlso)
# EnvironmentResponse
The results of an operation to update or read environment variables. If the operation succeeds, the response
contains the environment variables. If it fails, the response contains details about the error.
## Contents
**
Error
**
Error messages for environment variables that couldn't be applied.
Type: [EnvironmentError](./API_EnvironmentError.html) object
Required: No
**
Variables
**
Environment variable key-value pairs. Omitted from AWS CloudTrail logs.
Type: String to string map
Key Pattern: `[a-zA-Z]([a-zA-Z0-9\_])+`
Required: No