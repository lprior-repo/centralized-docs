---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/integrating-api-with-aws-services-kinesis.html
title: Tutorial: Create a REST API as an
word_count: 4489
filtered: true
elements_removed: 0
density_score: 0.81
---

Tutorial: Create a REST API as an Amazon Kinesis proxy - Amazon API Gateway
Tutorial: Create a REST API as an Amazon Kinesis proxy - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#integrating-api-with-aws-services-kinesis)
[Create an IAM role
and policy for the API to access Kinesis](#integrate-with-kinesis-create-iam-role-and-policy)[Create an API as a
Kinesis proxy](#api-gateway-create-api-as-kinesis-proxy)[List streams in Kinesis](#api-gateway-list-kinesis-streams)[Create, describe, and delete
a stream in Kinesis](#api-gateway-create-describe-delete-stream)[Get records from and add
records to a stream in Kinesis](#api-gateway-get-and-add-records-to-stream)
# Tutorial: Create a REST API as an
Amazon Kinesis proxy
This page describes how to create and configure a REST API with an integration of
the `AWS` type to access Kinesis.
###### Note
To integrate your API Gateway API with Kinesis, you must choose a region where both the API Gateway
and Kinesis services are available. For region availability, see [Service Endpoints and Quotas](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
For the purpose of illustration, we create an example API to enable a client to do the
following:
1. List the user's available streams in Kinesis
2. Create, describe, or delete a specified stream
3. Read data records from or write data records into the specified stream
To accomplish the preceding tasks, the API exposes methods on various resources to invoke
the following, respectively:
1. The `ListStreams` action in Kinesis
2. The `CreateStream`, `DescribeStream`, or
`DeleteStream` action
3. The `GetRecords` or `PutRecords` (including
`PutRecord`) action in Kinesis
Specifically, we build the API as follows:
* Expose an HTTP GET method on the API's `/streams` resource and
integrate the method with the [ListStreams](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_ListStreams.html) action in Kinesis to list the streams in the caller's account.
* Expose an HTTP POST method on the API's `/streams/{stream-name}`
resource and integrate the method with the [CreateStream](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_CreateStream.html) action in Kinesis to create a named stream in the caller's
account.
* Expose an HTTP GET method on the API's `/streams/{stream-name}`
resource and integrate the method with the [DescribeStream](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_DescribeStream.html) action in Kinesis to describe a named stream in the
caller's account.
* Expose an HTTP DELETE method on the API's `/streams/{stream-name}`
resource and integrate the method with the [DeleteStream](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_DeleteStream.html) action in Kinesis to delete a stream in the caller's account.
* Expose an HTTP PUT method on the API's `/streams/{stream-name}/record`
resource and integrate the method with the [PutRecord](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecord.html) action in Kinesis. This enables the client to add a single data
record to the named stream.
* Expose an HTTP PUT method on the API's
`/streams/{stream-name}/records` resource and integrate the method
with the [PutRecords](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecords.html) action in Kinesis. This enables the client to add a list of
data records to the named stream.
* Expose an HTTP GET method on the API's
`/streams/{stream-name}/records` resource and integrate the method
with the [GetRecords](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetRecords.html) action in Kinesis. This enables the client to list data records
in the named stream, with a specified shard iterator. A shard iterator specifies the
shard position from which to start reading data records sequentially.
* Expose an HTTP GET method on the API's
`/streams/{stream-name}/sharditerator` resource and integrate the
method with the [GetShardIterator](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetShardIterator.html) action in Kinesis. This helper method must be supplied to
the `ListStreams` action in Kinesis.
You can apply the instructions presented here to other Kinesis actions. For the complete
list of the Kinesis actions, see [Amazon Kinesis API
Reference](https://docs.aws.amazon.com/kinesis/latest/APIReference/Welcome.html).
Instead of using the API Gateway console to create the sample API, you can import the sample
API into API Gateway using the API Gateway [Import
API](https://docs.aws.amazon.com/apigateway/latest/api/API_ImportRestApi.html). For information on how to use the Import API, see [Develop REST APIs using
OpenAPI in API Gateway](./api-gateway-import-api.html).
## Create an IAM role
and policy for the API to access Kinesis
To allow the API to invoke Kinesis actions, you must have the appropriate IAM policies attached to an IAM role. In this step, you create a new IAM role.
###### To create the AWS service proxy execution role
1. Sign in to the AWS Management Console and open the IAM console at [https://console.aws.amazon.com/iam/](https://console.aws.amazon.com/iam/).
2. Choose **Roles**.
3. Choose **Create role**.
4. Choose **AWS service** under **Select type of
trusted entity**, and then select
**API Gateway** and select **Allows API Gateway to push logs to CloudWatch Logs**.
5. Choose **Next**, and then choose **Next**.
6. For **Role name**, enter `APIGatewayKinesisProxyPolicy`, and then choose **Create
role**.
7. In the **Roles** list, choose the role you just created. You
may need to scroll or use the search bar to find the role.
8. For the selected role, select the **Add permissions** tab.
9. Choose **Attach policies** from the dropdown list.
10. In the search bar, enter `AmazonKinesisFullAccess` and choose **Add permissions**.
###### Note
This tutorial uses a managed policy for simplicity. As a best practice, you should create your own IAM policy to grant the minimum permissions required.
11. Note the newly created **Role ARN**, you will use it later.
## Create an API as a
Kinesis proxy
Use the following steps to create the API in the API Gateway console.
###### To create an API as an AWS service proxy for Kinesis
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. If this is your first time using API Gateway, you see a page that introduces you to
the features of the service. Under **REST API**, choose **Build**. When the
**Create Example API** popup appears, choose
**OK**.
If this is not your first time using API Gateway, choose **Create
API**. Under **REST API**, choose **Build**.
3. Choose **New API**.
4. In **API name**, enter `KinesisProxy`.
Keep the default values for all other fields.
5. (Optional) For **Description**, enter a description.
6. For **IP address type**, select **IPv4**.
7. Choose **Create API**.
After the API is created, the API Gateway console displays the
**Resources** page, which contains only the API's root
(`/`) resource.
## List streams in Kinesis
Kinesis supports the `ListStreams` action with the following REST API call:
```
`POST /?Action=ListStreams HTTP/1.1
Host: kinesis.&lt;region&gt;.&lt;domain&gt;
Content-Length: &lt;PayloadSizeBytes&gt;
User-Agent: &lt;UserAgentString&gt;
Content-Type: application/x-amz-json-1.1
Authorization: &lt;AuthParams&gt;
X-Amz-Date: &lt;Date&gt;
{
...
}`
```
In the above REST API request, the action is specified in the `Action`
query parameter. Alternatively, you can specify the action in a
`X-Amz-Target` header, instead:
```
`POST / HTTP/1.1
Host: kinesis.&lt;&lt;region&gt;&gt;.&lt;&lt;domain&gt;&gt;
Content-Length: &lt;&lt;PayloadSizeBytes&gt;&gt;
User-Agent: &lt;&lt;UserAgentString&gt;&gt;
Content-Type: application/x-amz-json-1.1
Authorization: &lt;&lt;AuthParams&gt;&gt;
X-Amz-Date: &lt;&lt;Date&gt;&gt;
X-Amz-Target: Kinesis\_20131202.ListStreams
{
...
}`
```
In this tutorial, we use the query parameter to specify action.
To expose a Kinesis action in the API, add a `/streams` resource to the API's
root. Then set a `GET` method on the resource and integrate the method with
the `ListStreams` action of Kinesis.
The following procedure describes how to list Kinesis streams by using the API Gateway console.
###### To list Kinesis streams by using the API Gateway console
1. Select the `/` resource, and then choose **Create resource**.
2. For **Resource name**, enter `streams`.
3. Keep **CORS (Cross Origin Resource Sharing)** turned off.
4. Choose **Create resource**.
5. Choose the `/streams` resource, and then choose **Create method**, and then do the following:
1. For **Method type**, select **GET**.
###### Note
The HTTP verb for a method invoked by a client may differ from the HTTP
verb for an integration required by the backend. We select `GET`
here, because listing streams is intuitively a READ operation.
2. For **Integration type**, select **AWS service**.
3. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
4. For **AWS service**, select **Kinesis**.
5. Keep **AWS subdomain** blank.
6. For **HTTP method**, choose
**POST**.
###### Note
We chose `POST` here because Kinesis requires that the
`ListStreams` action be invoked with it.
7. For **Action type**, choose **Use action
name**.
8. For **Action name**, enter
`ListStreams`.
9. For **Execution role**, enter the ARN for your
execution role.
10. Keep the default of **Passthrough** for
**Content Handling**.
11. Choose **Create method**.
12. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
13. For **Request body passthrough**, select **When there are no templates defined (recommended)**.
14. Choose **URL request headers parameters**, and then do the following:
1. Choose **Add request headers parameter**.
2. For **Name**, enter
`Content-Type`.
3. For **Mapped from**, enter
`'application/x-amz-json-1.1'`.
We use a request parameter mapping to set the `Content-Type`
header to the static value of `'application/x-amz-json-1.1'` to
inform Kinesis that the input is of a specific version of JSON.
4. Choose **Mapping templates**, and then choose **Add mapping template**, and do the following:
1. For **Content-Type**, enter
`application/json`.
2. For **Template body**, enter `{}`.
3. Choose **Save**.
The [ListStreams](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_ListStreams.html#API_ListStreams_RequestSyntax) request takes a payload of the following JSON format:
```
`
{
"ExclusiveStartStreamName": "string",
"Limit": number
}
`
```
However, the properties are optional. To use the default values, we opted for
an empty JSON payload here.
4. Test the GET method on the **/streams** resource to invoke the
`ListStreams` action in Kinesis:
Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
Choose **Test** to test your method.
If you already created two streams named
"myStream"
and "yourStream" in Kinesis, the successful test returns a 200
OK response containing the following payload:
```
`
{
"HasMoreStreams": false,
"StreamNames": [
"myStream",
"yourStream"
]
}
`
```
## Create, describe, and delete
a stream in Kinesis
Creating, describing, and deleting a stream in Kinesis involves making the following
Kinesis REST API requests, respectively:
```
`
POST /?Action=CreateStream HTTP/1.1
Host: kinesis.`region`.`domain`
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"ShardCount": number,
"StreamName": "string"
}
`
```
```
`
POST /?Action=DescribeStream HTTP/1.1
Host: kinesis.`region`.`domain`
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"StreamName": "string"
}
`
```
```
`
POST /?Action=DeleteStream HTTP/1.1
Host: kinesis.`region`.`domain`
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"StreamName":"string"
}
`
```
We can build the API to accept the required input as a JSON payload of the method
request and pass the payload through to the integration request. However, to provide
more examples of data mapping between method and integration requests, and method and
integration responses, we create our API somewhat differently.
We expose the `GET`, `POST`, and `Delete` HTTP
methods on a to-be-named `Stream` resource. We use the
`{stream-name}` path variable as the placeholder of the stream resource
and integrate these API methods with the Kinesis' `DescribeStream`,
`CreateStream`, and `DeleteStream` actions, respectively. We
require that the client pass other input data as headers, query parameters, or the
payload of a method request. We provide mapping templates to transform the data to the
required integration request payload.
###### To create the {stream-name} resource
1. Choose the **/streams** resource, and then choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. For **Resource path**, select `/streams`.
4. For **Resource name**, enter `{stream-name}`.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
###### To configure and test the GET method on a stream resource
1. Choose the **/{stream-name}** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`DescribeStream`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **Create method**.
13. In the **Integration request** section, add the following **URL request headers parameters**:
```
`Content-Type: 'x-amz-json-1.1'`
```
The task follows the same procedure to set up the request parameter mapping
for the `GET /streams` method.
14. Add the following body mapping template to map data from the `GET
/streams/{stream-name}` method request to the `POST
/?Action=DescribeStream` integration request:
```
`{
"StreamName": "$input.params('stream-name')"
}`
```
This mapping template generates the required integration request payload for
the `DescribeStream` action of Kinesis from the method request's
`stream-name` path parameter value.
15. To test the `GET /stream/{stream-name}` method to invoke the
`DescribeStream` action in Kinesis, choose the **Test** tab.
16. For **Path**, under **stream-name**, enter the name of an existing Kinesis stream.
17. Choose **Test**. If the test is successful, a 200 OK response is
returned with a payload similar to the following:
```
`{
"StreamDescription": {
"HasMoreShards": false,
"RetentionPeriodHours": 24,
"Shards": [
{
"HashKeyRange": {
"EndingHashKey": "68056473384187692692674921486353642290",
"StartingHashKey": "0"
},
"SequenceNumberRange": {
"StartingSequenceNumber": "49559266461454070523309915164834022007924120923395850242"
},
"ShardId": "shardId-000000000000"
},
...
{
"HashKeyRange": {
"EndingHashKey": "340282366920938463463374607431768211455",
"StartingHashKey": "272225893536750770770699685945414569164"
},
"SequenceNumberRange": {
"StartingSequenceNumber": "49559266461543273504104037657400164881014714369419771970"
},
"ShardId": "shardId-000000000004"
}
],
"StreamARN": "arn:aws:kinesis:us-east-1:12345678901:stream/myStream",
"StreamName": "myStream",
"StreamStatus": "ACTIVE"
}
}`
```
After you deploy the API, you can make a REST request against this API
method:
```
`
GET https://`your-api-id`.execute-api.`region`.amazonaws.com/`stage`/streams/`myStream` HTTP/1.1
Host: `your-api-id`.execute-api.`region`.amazonaws.com
Content-Type: application/json
Authorization: ...
X-Amz-Date: 20160323T194451Z
`
```
###### To configure and test the POST method on a stream resource
1. Choose the **/{stream-name}** resource, and then choose **Create method**.
2. For **Method type**, select **POST**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`CreateStream`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **Create method**.
13. In the **Integration request** section, add the following **URL request headers parameters**:
```
`Content-Type: 'x-amz-json-1.1'`
```
The task follows the same procedure to set up the request parameter mapping
for the `GET /streams` method.
14. Add the following body mapping template to map data from the `POST
/streams/{stream-name}` method request to the `POST
/?Action=CreateStream` integration request:
```
`{
"ShardCount": #if($input.path('$.ShardCount') == '') 5 #else $input.path('$.ShardCount') #end,
"StreamName": "$input.params('stream-name')"
}`
```
In the preceding mapping template, we set `ShardCount` to a fixed
value of 5 if the client does not specify a value in the method request payload.
15. To test the `POST /stream/{stream-name}` method to invoke the
`CreateStream` action in Kinesis, choose the **Test** tab.
16. For **Path**, under **stream-name**, enter the name of a new Kinesis stream.
17. Choose **Test**. If the test is successful, a 200 OK response is
returned with no data.
After you deploy the API, you can also make a REST API request against the
POST method on a Stream resource to invoke the `CreateStream` action
in Kinesis:
```
`
POST https://`your-api-id`.execute-api.`region`.amazonaws.com/`stage`/streams/`yourStream` HTTP/1.1
Host: `your-api-id`.execute-api.`region`.amazonaws.com
Content-Type: application/json
Authorization: ...
X-Amz-Date: 20160323T194451Z
{
"ShardCount": 5
}
`
```
###### Configure and test the DELETE method on a stream resource
1. Choose the **/{stream-name}** resource, and then choose **Create method**.
2. For **Method type**, select **DELETE**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`DeleteStream`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **Create method**.
13. In the **Integration request** section, add the following **URL request headers parameters**:
```
`Content-Type: 'x-amz-json-1.1'`
```
The task follows the same procedure to set up the request parameter mapping
for the `GET /streams` method.
14. Add the following body mapping template to map data from the `DELETE
/streams/{stream-name}` method request to the corresponding
integration request of `POST /?Action=DeleteStream` :
```
`{
"StreamName": "$input.params('stream-name')"
}`
```
This mapping template generates the required input for the `DELETE
/streams/{stream-name}` action from the client-supplied URL path name
of `stream-name`.
15. To test the `DELETE /stream/{stream-name}` method to invoke the
`DeleteStream` action in Kinesis, choose the **Test** tab.
16. For **Path**, under **stream-name**, enter the name of an existing Kinesis stream.
17. Choose
**Test**. If the test is successful, a 200 OK response is
returned with no data.
After you deploy the API, you can also make the following REST API request
against the DELETE method on the Stream resource to call the
`DeleteStream` action in Kinesis:
```
`
DELETE https://`your-api-id`.execute-api.`region`.amazonaws.com/`stage`/streams/`yourStream` HTTP/1.1
Host: `your-api-id`.execute-api.`region`.amazonaws.com
Content-Type: application/json
Authorization: ...
X-Amz-Date: 20160323T194451Z
{}
`
```
## Get records from and add
records to a stream in Kinesis
After you create a stream in Kinesis, you can add data records to the stream and read
the data from the stream. Adding data records involves calling the [PutRecords](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecords.html#API_PutRecords_Examples) or [PutRecord](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecord.html#API_PutRecord_Examples) action in Kinesis. The former adds multiple records whereas the
latter adds a single record to the stream.
```
`
POST /?Action=PutRecords HTTP/1.1
Host: kinesis.`region`.`domain`
Authorization: AWS4-HMAC-SHA256 Credential=..., ...
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"Records": [
{
"Data": blob,
"ExplicitHashKey": "string",
"PartitionKey": "string"
}
],
"StreamName": "string"
}
`
```
or
```
`
POST /?Action=PutRecord HTTP/1.1
Host: kinesis.`region`.`domain`
Authorization: AWS4-HMAC-SHA256 Credential=..., ...
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"Data": `blob`,
"ExplicitHashKey": `"string"`,
"PartitionKey": `"string"`,
"SequenceNumberForOrdering": `"string"`,
"StreamName": "string"
}
`
```
Here, `StreamName` identifies the target stream to add records.
`StreamName`, `Data`, and `PartitionKey` are
required input data. In our example, we use the default values for all of the optional
input data and will not explicitly specify values for them in the input to the method
request.
Reading data in Kinesis amounts to calling the [GetRecords](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetRecords.html#API_GetRecords_Examples) action:
```
`POST /?Action=GetRecords HTTP/1.1
Host: kinesis.`region`.`domain`
Authorization: AWS4-HMAC-SHA256 Credential=..., ...
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"ShardIterator": `"string"`,
"Limit": `number`
}
`
```
Here, the source stream from which we are getting records is specified in the required
`ShardIterator` value, as is shown in the following Kinesis action to obtain
a shard iterator:
```
`
POST /?Action=GetShardIterator HTTP/1.1
Host: kinesis.`region`.`domain`
Authorization: AWS4-HMAC-SHA256 Credential=..., ...
...
Content-Type: application/x-amz-json-1.1
Content-Length: `PayloadSizeBytes`
{
"ShardId": `"string"`,
"ShardIteratorType": `"string"`,
"StartingSequenceNumber": `"string"`,
"StreamName": `"string"`
}
`
```
For the `GetRecords` and `PutRecords` actions, we expose the
`GET` and `PUT` methods, respectively, on a
`/records` resource that is appended to a named stream resource
(`/{stream-name}`). Similarly, we expose the `PutRecord`
action as a `PUT` method on a `/record` resource.
Because the `GetRecords` action takes as input a
`ShardIterator` value, which is obtained by calling the
`GetShardIterator` helper action, we expose a `GET` helper
method on a `ShardIterator` resource (`/sharditerator`).
###### To create the /record, /records, and /sharditerator resources
1. Choose the **/{stream-name}** resource, and then choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. For **Resource path**, select `/{stream-name}`.
4. For **Resource name**, enter `record`.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
7. Repeat the previous steps to create a **/records** and a **/sharditerator** resource. The final API should look like the following:
![Create Records:GET|PUT|PUT|GET method for the API.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-kinesis-proxy-setup-streams-stream-record-method-new-console.png)
The following four procedures describe how to set up each of the methods, how to map
data from the method requests to the integration requests, and how to test the methods.
###### To
set up and test the `PUT /streams/{stream-name}/record` method to invoke
`PutRecord` in Kinesis:
1. Choose the **/record**, and then choose **Create method**.
2. For **Method type**, select **PUT**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`PutRecord`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **Create method**.
13. In the **Integration request** section, add the following **URL request headers parameters**:
```
`Content-Type: 'x-amz-json-1.1'`
```
The task follows the same procedure to set up the request parameter mapping
for the `GET /streams` method.
14. Add the following body mapping template to map data from the `PUT
/streams/{stream-name}/record` method request to the corresponding
integration request of `POST /?Action=PutRecord`:
```
`{
"StreamName": "$input.params('stream-name')",
"Data": "$util.base64Encode($input.json('$.Data'))",
"PartitionKey": "$input.path('$.PartitionKey')"
}`
```
This mapping template assumes that the method request payload is of the
following format:
```
`{
"Data": "some data",
"PartitionKey": "some key"
}`
```
This data can be modeled by the following JSON schema:
```
`{
"$schema": "http://json-schema.org/draft-04/schema#",
"title": "PutRecord proxy single-record payload",
"type": "object",
"properties": {
"Data": { "type": "string" },
"PartitionKey": { "type": "string" }
}
}`
```
You can create a model to include this schema and use the model to facilitate
generating the mapping template. However, you can generate a mapping template
without using any model.
15. To test the `PUT /streams/{stream-name}/record` method, set the
`stream-name` path variable to the name of an existing stream,
supply a payload of the required format, and then submit the method request. The
successful result is a `200 OK `response with a payload of the
following format:
```
`
{
"SequenceNumber": "49559409944537880850133345460169886593573102115167928386",
"ShardId": "shardId-000000000004"
}
`
```
###### To set up and test the `PUT /streams/{stream-name}/records` method to
invoke `PutRecords` in Kinesis
1. Choose the **/records** resource, and then choose **Create method**.
2. For **Method type**, select **PUT**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`PutRecords`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **Create method**.
13. In the **Integration request** section, add the following **URL request headers parameters**:
```
`Content-Type: 'x-amz-json-1.1'`
```
The task follows the same procedure to set up the request parameter mapping
for the `GET /streams` method.
14. Add the following mapping template to map data from the `PUT
/streams/{stream-name}/records` method request to the corresponding
integration request of `POST /?Action=PutRecords` :
```
`{
"StreamName": "$input.params('stream-name')",
"Records": [
#end
]
}`
```
This mapping template assumes that the method request payload can be modelled
by the following JSON schema:
```
`{
"$schema": "http://json-schema.org/draft-04/schema#",
"title": "PutRecords proxy payload data",
"type": "object",
"properties": {
"records": {
"type": "array",
"items": {
"type": "object",
"properties": {
"data": { "type": "string" },
"partition-key": { "type": "string" }
}
}
}
}
}`
```
You can create a model to include this schema and use the model to facilitate
generating the mapping template. However, you can generate a mapping template
without using any model.
In this tutorial, we used two slightly different payload formats to illustrate
that an API developer can choose to expose the backend data format to the client
or hide it from the client. One format is for the `PUT
/streams/{stream-name}/records` method (above). Another format is used
for the `PUT /streams/{stream-name}/record` method (in the previous
procedure). In production environment, you should keep both formats consistent.
15.
To test the `PUT /streams/{stream-name}/records` method, set the
`stream-name` path variable to an existing stream, supply the
following payload, and submit the method request.
```
`
{
"records": [
{
"data": "some data",
"partition-key": "some key"
},
{
"data": "some other data",
"partition-key": "some key"
}
]
}
`
```
The successful result is a 200 OK response with a payload similar to the
following output:
```
`{
"FailedRecordCount": 0,
"Records": [
{
"SequenceNumber": "49559409944537880850133345460167468741933742152373764162",
"ShardId": "shardId-000000000004"
},
{
"SequenceNumber": "49559409944537880850133345460168677667753356781548470338",
"ShardId": "shardId-000000000004"
}
]
}`
```
###### To set up and test the `GET /streams/{stream-name}/sharditerator`
method invoke `GetShardIterator` in Kinesis
The `GET /streams/{stream-name}/sharditerator` method is a helper
method to acquire a required shard iterator before calling the `GET
/streams/{stream-name}/records` method.
1. Choose the **/sharditerator** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`GetShardIterator`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **URL query string parameters**.
The `GetShardIterator` action requires an input of a ShardId
value. To pass a client-supplied `ShardId` value, we add a
`shard-id` query parameter to the method request, as shown in the
following step.
13. Choose **Add query string**.
14. For **Name**, enter `shard-id`.
15. Keep **Required** and **Caching** turned off.
16. Choose **Create method**.
17. In the **Integration request** section, add the following mapping template to generate the required input
(`ShardId` and `StreamName`) to the
`GetShardIterator` action from the `shard-id` and
`stream-name` parameters of the method request. In addition, the
mapping template also sets `ShardIteratorType` to
`TRIM\_HORIZON` as a default.
```
`{
"ShardId": "$input.params('shard-id')",
"ShardIteratorType": "TRIM\_HORIZON",
"StreamName": "$input.params('stream-name')"
}`
```
18. Using the **Test** option in the API Gateway console, enter an
existing stream name as the `stream-name`
**Path** variable value, set the `shard-id`
**Query string** to an existing `ShardId` value
(e.g., `shard-000000000004`), and choose **Test**.
The successful response payload is similar to the following output:
```
`{
"ShardIterator": "AAAAAAAAAAFYVN3VlFy..."
}`
```
Make note of the `ShardIterator` value. You need it to get records
from a stream.
###### To configure and test the `GET /streams/{stream-name}/records` method
to invoke the `GetRecords` action in Kinesis
1. Choose the **/records** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Kinesis stream.
5. For **AWS service**, select **Kinesis**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose
**POST**.
8. For **Action type**, choose **Use action
name**.
9. For **Action name**, enter
`GetRecords`.
10. For **Execution role**, enter the ARN for your
execution role.
11. Keep the default of **Passthrough** for
**Content Handling**.
12. Choose **HTTP request headers**.
The `GetRecords` action requires an input of a
`ShardIterator` value. To pass a client-supplied
`ShardIterator` value, we add a `Shard-Iterator`
header parameter to the method request.
13. Choose **Add header**.
14. For **Name**, enter `Shard-Iterator`.
15. Keep **Required** and **Caching** turned off.
16. Choose **Create method**.
17. In the **Integration request** section, add the following body mapping template to map the `Shard-Iterator`
header parameter value to the `ShardIterator` property value of the
JSON payload for the `GetRecords` action in Kinesis.
```
`{
"ShardIterator": "$input.params('Shard-Iterator')"
}`
```
18. Using the **Test** option in the API Gateway console, enter an
existing stream name as the `stream-name`
**Path** variable value, set the `Shard-Iterator`
**Header** to the `ShardIterator` value obtained
from the test run of the `GET /streams/{stream-name}/sharditerator`
method (above), and choose **Test**.
The successful response payload is similar to the following output:
```
`{
"MillisBehindLatest": 0,
"NextShardIterator": "AAAAAAAAAAF...",
"Records": [ ... ]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Call the API using a REST API client
OpenAPI
definitions of a sample API as a Kinesis proxy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.