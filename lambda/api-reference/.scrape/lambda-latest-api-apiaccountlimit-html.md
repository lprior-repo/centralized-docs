---
url: https://docs.aws.amazon.com/lambda/latest/api/API_AccountLimit.html
title: AccountLimit
word_count: 144
filtered: true
elements_removed: 0
density_score: 0.92
---

AccountLimit - AWS Lambda
AccountLimit - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_AccountLimit)
[Contents](#API_AccountLimit_Contents)[See Also](#API_AccountLimit_SeeAlso)
# AccountLimit
Limits that are related to concurrency and storage. All file and storage sizes are in bytes.
## Contents
**
CodeSizeUnzipped
**
The maximum size of a function's deployment package and layers when they're extracted.
Type: Long
Required: No
**
CodeSizeZipped
**
The maximum size of a deployment package when it's uploaded directly to Lambda. Use Amazon S3 for larger
files.
Type: Long
Required: No
**
ConcurrentExecutions
**
The maximum number of simultaneous function executions.
Type: Integer
Required: No
**
TotalCodeSize
**
The amount of storage space that you can use for all deployment packages and layer archives.
Type: Long
Required: No
**
UnreservedConcurrentExecutions
**
The maximum number of simultaneous function executions, minus the capacity that's reserved for individual
functions with [PutFunctionConcurrency](./API_PutFunctionConcurrency.html).
Type: Integer
Valid Range: Minimum value of 0.
Required: No