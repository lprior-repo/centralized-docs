---
url: https://docs.aws.amazon.com/lambda/latest/dg/config-rs-write-functions.html
title: Writing response streaming-enabled Lambda functions
word_count: 549
filtered: true
elements_removed: 0
density_score: 0.86
---

Writing response streaming-enabled Lambda functions - AWS Lambda
Writing response streaming-enabled Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#config-rs-write-functions)
[Configuring a handler function to stream responses](#config-rs-write-functions-handler)[Ending the stream](#config-rs-write-functions-end)
# Writing response streaming-enabled Lambda functions
Writing the handler for response streaming functions is different than typical handler
patterns. When writing streaming functions, be sure to do the following:
* Wrap your function with the `awslambda.streamifyResponse()` decorator. The `awslambda` global object is provided by Lambda's Node.js runtime environment.
* End the stream gracefully to ensure that all data processing is complete.
## Configuring a handler function to stream responses
To indicate to the runtime that Lambda should stream your function's responses, you must
wrap your function with the `streamifyResponse()` decorator. This tells the
runtime to use the proper logic path for streaming responses and enables the function to
stream responses.
The `streamifyResponse()` decorator accepts a function that accepts the following parameters:
* `event` – Provides information about the function URL's invocation event,
such as the HTTP method, query parameters, and the request body.
* `responseStream` – Provides a writable stream.
* `context` – Provides methods and properties with information about the
invocation, function, and execution environment.
The `responseStream` object is a [Node.js `writableStream`](https://nodesource.com/blog/understanding-streams-in-nodejs/).
As with any such stream, you should use the `pipeline()` method.
###### Note
The `awslambda` global object is automatically provided by Lambda's Node.js runtime and no import is required.
###### Example response streaming-enabled handler
```
`import { pipeline } from 'node:stream/promises';
import { Readable } from 'node:stream';
export const echo = awslambda.streamifyResponse(async (event, responseStream, \_context) =&gt;&gt; {
// As an example, convert event to a readable stream.
const requestStream = Readable.from(Buffer.from(JSON.stringify(event)));
await pipeline(requestStream, responseStream);
});`
```
While `responseStream` offers the `write()` method to write to the
stream, we recommend that you use [`pipeline()`](https://nodejs.org/api/stream.html#streampipelinesource-transforms-destination-callback) wherever possible. Using `pipeline()`
ensures that the writable stream is not overwhelmed by a faster readable stream.
## Ending the stream
Make sure that you properly end the stream before the handler returns. The
`pipeline()` method handles this automatically.
For other use cases, call the `responseStream.end()` method to properly end a
stream. This method signals that no more data should be written to the stream. This method
isn't required if you write to the stream with `pipeline()` or
`pipe()`.
Starting with Node.js 24, Lambda no longer waits for unresolved promises to complete after your handler returns or the response stream ends. If your function depends on additional asynchronous operations, such as timers or fetches, you should `await` them in your handler.
###### Example ending a stream with pipeline()
```
`import { pipeline } from 'node:stream/promises';
export const handler = awslambda.streamifyResponse(async (event, responseStream, \_context) =&gt;&gt; {
await pipeline(requestStream, responseStream);
});`
```
###### Example ending a stream without pipeline()
```
`export const handler = awslambda.streamifyResponse(async (event, responseStream, \_context) =&gt;&gt; {
responseStream.write("Hello ");
responseStream.write("world ");
responseStream.write("from ");
responseStream.write("Lambda!");
responseStream.end();
});`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Response streaming
Invoking functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.