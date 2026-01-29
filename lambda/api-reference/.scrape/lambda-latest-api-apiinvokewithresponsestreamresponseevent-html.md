---
url: https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStreamResponseEvent.html
title: InvokeWithResponseStreamResponseEvent
word_count: 75
filtered: true
elements_removed: 0
density_score: 0.93
---

InvokeWithResponseStreamResponseEvent - AWS Lambda
InvokeWithResponseStreamResponseEvent - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_InvokeWithResponseStreamResponseEvent)
[Contents](#API_InvokeWithResponseStreamResponseEvent_Contents)[See Also](#API_InvokeWithResponseStreamResponseEvent_SeeAlso)
# InvokeWithResponseStreamResponseEvent
An object that includes a chunk of the response payload. When the stream has ended, Lambda includes a `InvokeComplete` object.
## Contents
**
InvokeComplete
**
An object that's returned when the stream has ended and all the payload chunks have been
returned.
Type: [InvokeWithResponseStreamCompleteEvent](./API_InvokeWithResponseStreamCompleteEvent.html) object
Required: No
**
PayloadChunk
**
A chunk of the streamed response payload.
Type: [InvokeResponseStreamUpdate](./API_InvokeResponseStreamUpdate.html) object
Required: No