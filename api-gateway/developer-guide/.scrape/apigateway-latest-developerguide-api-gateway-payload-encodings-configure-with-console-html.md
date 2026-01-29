---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-payload-encodings-configure-with-console.html
title: Enabling binary
word_count: 605
filtered: true
elements_removed: 0
density_score: 0.73
---

Enabling binary support using the API Gateway console - Amazon API Gateway
Enabling binary support using the API Gateway console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-payload-encodings-configure-with-console)
# Enabling binary
support using the API Gateway console
The section explains how to enable binary support using the API Gateway console. As an
example, we use an API that is integrated with Amazon S3. We focus on the tasks to set the
supported media types and to specify how the payload should be handled. For detailed
information on how to create an API integrated with Amazon S3, see [Tutorial: Create a REST API as an Amazon S3 proxy](./integrating-api-with-aws-services-s3.html).
###### To enable binary support by using the API Gateway console
1. Set binary media types for the API:
1. Create a new API or choose an existing API. For this example, we name
the API `FileMan`.
2. Under the selected API in the primary navigation panel, choose
**API settings**.
3. In the **API settings** pane, choose **Manage media types** in the **Binary Media Types**
section.
4. Choose **Add binary media type**.
5. Enter a required media type, for example, `image/png`, in
the input text field. If needed, repeat this step to add more media
types. To support all binary media types, specify `\*/\*`.
6. Choose **Save changes**.
7. Set how message payloads are handled for the API method:
1. Create a new or choose an existing resource in the API. For this
example, we use the `/{folder}/{item}`
resource.
2. Create a new or choose an existing method on the resource. As an
example, we use the `GET /{folder}/{item}`
method integrated with the `Object GET` action in Amazon S3.
3. For **Content handling**, choose an option.
![Set up the GET method in the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/binary-support-content-handling-on-method-new-console.png)
Choose **Passthrough** if you don't want to convert
the body when the client and backend accepts the same binary format.
Choose **Convert to text** to convert the
binary body to a base64-encoded string when, for example, the backend
requires that a binary request payload is passed in as a JSON property.
And choose **Convert to binary** when the
client submits a base64-encoded string and the backend requires the
original binary format, or when the endpoint returns a base64-encoded
string and the client accepts only the binary output.
4. For **Request body passthrough**, choose **When there are no templates defined (recommended)** to enable the passthrough behavior on the request body.
You could also choose **Never**. This means that the API will reject data with content-types that do not have a mapping template.
5. Preserve the incoming request's `Accept`
header in the integration request. You should do this if you've set
`contentHandling` to `passthrough` and want to override that
setting at runtime.
![Keep the Accept header in the integration request.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/binary-support-preserve-incoming-accept-header-new-console.png)
6. For conversion to text, define a mapping template to put the
base64-encoded binary data into the required format.
An example of a mapping template to convert to text is the following:
```
`{
"operation": "thumbnail",
"base64Image": "$input.body"
}`
```
The format of this mapping template depends on the endpoint
requirements of the input.
7. Choose **Save**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Content type conversions in
API Gateway
Enabling binary support using the API Gateway REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.