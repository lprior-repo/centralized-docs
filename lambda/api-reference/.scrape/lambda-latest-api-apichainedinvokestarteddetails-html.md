---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ChainedInvokeStartedDetails.html
title: ChainedInvokeStartedDetails
word_count: 161
filtered: true
elements_removed: 0
density_score: 0.85
---

ChainedInvokeStartedDetails - AWS Lambda
ChainedInvokeStartedDetails - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ChainedInvokeStartedDetails)
[Contents](#API_ChainedInvokeStartedDetails_Contents)[See Also](#API_ChainedInvokeStartedDetails_SeeAlso)
# ChainedInvokeStartedDetails
Contains details about a chained function invocation that has started execution, including start time and execution context.
## Contents
**
FunctionName
**
The name or ARN of the Lambda function being invoked.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `(arn:(aws[a-zA-Z-]\*)?:lambda:)?((eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:)?(\\d{12}:)?(function:)?([a-zA-Z0-9-\_\\.]+)(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: Yes
**
DurableExecutionArn
**
The Amazon Resource Name (ARN) that identifies the durable execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Pattern: `arn:([a-zA-Z0-9-]+):lambda:([a-zA-Z0-9-]+):(\\d{12}):function:([a-zA-Z0-9\_-]+):(\\$LATEST(?:\\.PUBLISHED)?|[0-9]+)/durable-execution/([a-zA-Z0-9\_-]+)/([a-zA-Z0-9\_-]+)`
Required: No
**
ExecutedVersion
**
The version of the function that was executed.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Pattern: `(\\$LATEST(\\.PUBLISHED)?|[0-9]+)`
Required: No
**
Input
**
The JSON input payload provided to the chained invocation.
Type: [EventInput](./API_EventInput.html) object
Required: No
**
TenantId
**
The tenant identifier for the chained invocation.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `[a-zA-Z0-9\\.\_:\\/=+\\-@ ]+`
Required: No