---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/lambda-proxy-binary-media.html
title: Return binary media from a Lambda proxy
word_count: 508
filtered: true
elements_removed: 0
density_score: 0.78
---

Return binary media from a Lambda proxy integration in API Gateway - Amazon API Gateway
Return binary media from a Lambda proxy integration in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#lambda-proxy-binary-media)
# Return binary media from a Lambda proxy
integration in API Gateway
To return binary media from an [AWS Lambda proxy
integration](./set-up-lambda-proxy-integrations.html), base64 encode the response from your Lambda function. You must also [configure your API's binary media types](./api-gateway-payload-encodings-configure-with-console.html).
When you configure your API's binary media types, your API treats that content type as binary data. The payload
size limit is 10 MB.
###### Note
To use a web browser to invoke an API with this example integration, set your
API's binary media types to `\*/\*`. API Gateway uses the first
`Accept` header from clients to determine if a response should
return binary media. To return binary media when you can't control the order of
`Accept` header values, such as requests from a browser, set your
API's binary media types to `\*/\*` (for all content types).
The following example Lambda function can return a binary image from Amazon S3
or text to clients. The function's response includes a `Content-Type`
header to indicate to the client the type of data that it returns. The function
conditionally sets the `isBase64Encoded` property in its response,
depending on the type of data that it returns.
Node.js
```
`import { S3Client, GetObjectCommand } from "@aws-sdk/client-s3"
const client = new S3Client({region: '`us-east-2`'});
export const handler = async (event) =&gt; {
var randomint = function(max) {
return Math.floor(Math.random() \* max);
}
var number = randomint(2);
if (number == 1){
const input = {
"Bucket" : "`bucket-name`",
"Key" : "`image.png`"
}
try {
const command = new GetObjectCommand(input)
const response = await client.send(command);
var str = await response.Body.transformToByteArray();
} catch (err) {
console.error(err);
}
const base64body = Buffer.from(str).toString('base64');
return {
'headers': { "Content-Type": "image/png" },
'statusCode': 200,
'body': base64body,
'isBase64Encoded': true
}
} else {
return {
'headers': { "Content-Type": "text/html" },
'statusCode': 200,
'body': "&lt;h1&gt;This is text&lt;/h1&gt;",
}
}
}`
```
Python
```
`import base64
import boto3
import json
import random
s3 = boto3.client('s3')
def lambda\_handler(event, context):
number = random.randint(0,1)
if number == 1:
response = s3.get\_object(
Bucket='`bucket-name`',
Key='`image.png'`,
)
image = response['Body'].read()
return {
'headers': { "Content-Type": "image/png" },
'statusCode': 200,
'body': base64.b64encode(image).decode('utf-8'),
'isBase64Encoded': True
}
else:
return {
'headers': { "Content-type": "text/html" },
'statusCode': 200,
'body': "&lt;h1&gt;This is text&lt;/h1&gt;",
}`
```
To learn more about binary media types, see [Binary media types for
REST APIs in API Gateway](./api-gateway-payload-encodings.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Import and export
content encodings for API Gateway
Access binary
files in Amazon S3 through an API Gateway API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.