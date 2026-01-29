---
url: https://docs.aws.amazon.com/lambda/latest/api/API_OnSuccess.html
title: OnSuccess
word_count: 140
filtered: true
elements_removed: 0
density_score: 0.90
---

OnSuccess - AWS Lambda
OnSuccess - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_OnSuccess)
[Contents](#API_OnSuccess_Contents)[See Also](#API_OnSuccess_SeeAlso)
# OnSuccess
A destination for events that were processed successfully.
To retain records of successful [asynchronous invocations](https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-async-destinations),
you can configure an Amazon SNS topic, Amazon SQS queue, Lambda function,
or Amazon EventBridge event bus as the destination.
###### Note
`OnSuccess` is not supported in `CreateEventSourceMapping` or `UpdateEventSourceMapping` requests.
## Contents
**
Destination
**
The Amazon Resource Name (ARN) of the destination resource.
###### Note
Amazon SNS destinations have a message size limit of 256 KB. If the combined size of the function request and response payload exceeds the limit, Lambda will drop the payload when sending `OnFailure` event to the destination. For details on this behavior, refer to [Retaining records of asynchronous invocations](https://docs.aws.amazon.com/lambda/latest/dg/invocation-async-retain-records.html).
Type: String
Length Constraints: Minimum length of 0. Maximum length of 350.
Pattern: `$|kafka://([^.]([a-zA-Z0-9\\-\_.]{0,248}))|arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No