---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-wscat.html
title: Use `wscat` to
word_count: 316
filtered: true
elements_removed: 0
density_score: 0.93
---

Use wscat to connect to a WebSocket API and send messages to it - Amazon API Gateway
Use wscat to connect to a WebSocket API and send messages to it - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-how-to-call-websocket-api-wscat)
# Use `wscat` to
connect to a WebSocket API and send messages to it
The `[wscat](https://www.npmjs.com/package/wscat)`
utility is a convenient tool for testing a WebSocket API that you have created and
deployed in API Gateway. You can install and use `wscat` as follows:
1. Download `wscat` from [https://www.npmjs.com/package/wscat](https://www.npmjs.com/package/wscat).
2. Install `wscat` by running the following command:
```
`npm install -g wscat`
```
3. To connect to your API, run the `wscat` command as shown in the
following example. Note that this example assumes that the
`Authorization` setting is `NONE`.
```
`wscat -c wss://`aabbccddee`.execute-api.`us-east-1`.amazonaws.com/test/`
```
You need to replace ``aabbccddee`` with
the actual API ID, which is displayed in the API Gateway console or returned by the
AWS CLI [`create-api`](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-api.html) command.
In addition, if your API is in a Region other than `us-east-1`, you
need to substitute the correct Region.
4. To test your API, enter a message such as the following while
connected:
```
`{"`{jsonpath-expression}`":"`{route-key}`"}`
```
where `{jsonpath-expression}` is a JSONPath
expression and `{route-key}` is a route key for the
API. For example:
```
`{"action":"action1"}
{"message":"test response body"}`
```
For more information about JSONPath, see [JSONPath](https://goessner.net/articles/JsonPath/) or [JSONPath for Java](https://github.com/json-path/JsonPath).
5. To disconnect from your API, enter `ctrl-C`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke
Use @connections commands in your
backend service
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.