---
url: https://docs.aws.amazon.com/lambda/latest/api/API_DestinationConfig.html
title: DestinationConfig
word_count: 70
filtered: true
elements_removed: 0
density_score: 0.93
---

DestinationConfig - AWS Lambda
DestinationConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_DestinationConfig)
[Contents](#API_DestinationConfig_Contents)[See Also](#API_DestinationConfig_SeeAlso)
# DestinationConfig
A configuration object that specifies the destination of an event after Lambda processes it. For more information, see [Adding a destination](https://docs.aws.amazon.com/lambda/latest/dg/invocation-async-retain-records.html#invocation-async-destinations).
## Contents
**
OnFailure
**
The destination configuration for failed invocations.
Type: [OnFailure](./API_OnFailure.html) object
Required: No
**
OnSuccess
**
The destination configuration for successful invocations. Not supported in `CreateEventSourceMapping` or `UpdateEventSourceMapping`.
Type: [OnSuccess](./API_OnSuccess.html) object
Required: No