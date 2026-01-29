---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionEventInvokeConfig.html
title: API FunctionEventInvokeConfig.html
word_count: 220
filtered: true
elements_removed: 0
density_score: 0.88
---

FunctionEventInvokeConfig - AWS Lambda
FunctionEventInvokeConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionEventInvokeConfig)
[Contents](#API_FunctionEventInvokeConfig_Contents)[See Also](#API_FunctionEventInvokeConfig_SeeAlso)
## Contents
**
DestinationConfig
**
A destination for events after they have been sent to a function for processing.
###### Destinations
* **Function** - The Amazon Resource Name (ARN) of a Lambda function.
* **Queue** - The ARN of a standard SQS queue.
* **Bucket** - The ARN of an Amazon S3 bucket.
* **Topic** - The ARN of a standard SNS topic.
* **Event Bus** - The ARN of an Amazon EventBridge event bus.
###### Note
S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.
Type: [DestinationConfig](./API_DestinationConfig.html) object
Required: No
**
FunctionArn
**
The Amazon Resource Name (ARN) of the function.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
Required: No
**
LastModified
**
The date and time that the configuration was last updated, in Unix time seconds.
Type: Timestamp
Required: No
**
MaximumEventAgeInSeconds
**
The maximum age of a request that Lambda sends to a function for processing.
Type: Integer
Valid Range: Minimum value of 60. Maximum value of 21600.
Required: No
**
MaximumRetryAttempts
**
The maximum number of times to retry when the function returns an error.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 2.
Required: No