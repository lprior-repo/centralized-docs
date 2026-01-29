---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_LambdaFunctionSucceededEventDetails.html
title: LambdaFunctionSucceededEventDetails
word_count: 83
filtered: true
elements_removed: 0
density_score: 0.93
---

LambdaFunctionSucceededEventDetails - AWS Step Functions
LambdaFunctionSucceededEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_LambdaFunctionSucceededEventDetails)
[Contents](#API_LambdaFunctionSucceededEventDetails_Contents)[See Also](#API_LambdaFunctionSucceededEventDetails_SeeAlso)
# LambdaFunctionSucceededEventDetails
Contains details about a Lambda function that successfully terminated during an
execution.
## Contents
**
output
**
The JSON data output by the Lambda function. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
outputDetails
**
Contains details about the output of an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No