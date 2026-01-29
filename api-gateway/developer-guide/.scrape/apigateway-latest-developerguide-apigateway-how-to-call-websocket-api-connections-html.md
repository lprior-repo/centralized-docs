---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-connections.html
title: Use `@connections` commands in your
word_count: 480
filtered: true
elements_removed: 0
density_score: 0.90
---

Use @connections commands in your backend service - Amazon API Gateway
Use @connections commands in your backend service - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-how-to-call-websocket-api-connections)
# Use `@connections` commands in your
backend service
Your backend service can use the following WebSocket connection HTTP requests to send a callback message to a
connected client, get connection information, or disconnect the client.
###### Important
These requests use [IAM authorization](./apigateway-websocket-control-access-iam.html), so you
must sign them with [Signature Version 4 (SigV4)](https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html). To do
this, you can use the API Gateway Management API. For more information, see [ApiGatewayManagementApi](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigatewaymanagementapi.html).
In the following command, you need to replace ``{api-id}`` with the actual
API ID, which is displayed in the API Gateway console or returned by the AWS CLI [create-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-api.html) command.
You must establish the connection before using this command.
To send a callback message to the client, use:
```
`POST https://`{api-id}`.execute-api.`us-east-1`.amazonaws.com/`{stage}`/@connections/`{connection\_id}``
```
You can test this request by using `[Postman](https://www.postman.com/)` or by
calling `[awscurl](https://github.com/okigan/awscurl)` as in the following
example:
```
`awscurl --service execute-api -X POST -d "hello world" https://`{prefix}`.execute-api.`us-east-1`.amazonaws.com/`{stage}`/@connections/`{connection\_id}``
```
You need to URL-encode the command as in the following example:
```
`awscurl --service execute-api -X POST -d "hello world" https://`aabbccddee`.execute-api.`us-east-1`.amazonaws.com/`prod`/%40connections/`R0oXAdfD0kwCH6w%3D``
```
To get the latest connection status of the client, use:
```
`GET https://`{api-id}`.execute-api.`us-east-1`.amazonaws.com/`{stage}`/@connections/`{connection\_id}``
```
To disconnect the client, use:
```
`DELETE https://`{api-id`}.execute-api.`us-east-1`.amazonaws.com/`{stage}`/@connections/`{connection\_id`}`
```
You can dynamically build a callback URL by using the `$context` variables in your integration. For
example, if you use Lambda proxy integration with a `Node.js` Lambda function, you can build the URL and
send a message to a connected client as follows:
```
`import {
ApiGatewayManagementApiClient,
PostToConnectionCommand,
} from "@aws-sdk/client-apigatewaymanagementapi";
export const handler = async (event) =&gt; {
const domain = event.requestContext.domainName;
const stage = event.requestContext.stage;
const connectionId = event.requestContext.connectionId;
const callbackUrl = `https://${domain}/${stage}`;
const client = new ApiGatewayManagementApiClient({ endpoint: callbackUrl });
const requestParams = {
ConnectionId: connectionId,
Data: "Hello!",
};
const command = new PostToConnectionCommand(requestParams);
try {
await client.send(command);
} catch (error) {
console.log(error);
}
return {
statusCode: 200,
};
};`
```
If you use a custom domain name for your WebSocket API, remove the `stage` variable from your
function code.
When sending a callback message, your Lambda function must have permission to call the API Gateway Management API.
You might receive an error that contains `GoneException` if you post a message before the connection is established,
or after the client has disconnected.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use wscat to
connect to a WebSocket API and send messages to it
Publish
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.