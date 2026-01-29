---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-api-cli-sdk.html
title: Tutorial: Create a REST API using
word_count: 4093
filtered: true
elements_removed: 0
density_score: 0.86
---

Tutorial: Create a REST API using AWS SDKs or the AWS CLI - Amazon API Gateway
Tutorial: Create a REST API using AWS SDKs or the AWS CLI - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-create-api-cli-sdk)
[Set up an edge-optimized PetStore API](#api-gateway-create-api-cli-sdk-tutorial)[Automate the setup of your API](#api-gateway-create-api-cli-sdk-iac)
# Tutorial: Create a REST API using
AWS SDKs or the AWS CLI
Use the following tutorial to create a PetStore API supporting the `GET /pets` and `GET
/pets/{petId}` methods. The methods are integrated with an HTTP endpoint. You can follow this tutorial using
the AWS SDK for JavaScript, the SDK for Python (Boto3), or the AWS CLI. You use the following functions or commands to set up your API:
JavaScript v3
* [
CreateRestApiCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/CreateRestApiCommand/)
* [
CreateResourceCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/CreateResourceCommand/)
* [
PutMethodCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/PutMethodCommand/)
* [
PutMethodResponseCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/PutMethodResponseCommand/)
* [
PutIntegrationCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/PutIntegrationCommand/)
* [
PutIntegrationResponseCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/PutIntegrationResponseCommand/)
* [
CreateDeploymentCommand](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/api-gateway/command/CreateDeploymentCommand/)
Python
* [
create\_rest\_api](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/create_rest_api.html)
* [
create\_resource](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/create_resource.html)
* [
put\_method](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/put_method.html)
* [
put\_method\_response](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/put_method_response.html)
* [
put\_integration](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/put_integration.html)
* [
put\_integration\_response](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/put_integration_response.html)
* [
create\_deployment](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/apigateway/client/create_deployment.html)
AWS CLI
* [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html)
* [create-resource](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-resource.html)
* [put-method](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-method.html)
* [put-method-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-method-response.html)
* [put-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration.html)
* [put-integration-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration-response.html)
* [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html)
For more information about the AWS SDK for JavaScript v3, see [What's the AWS SDK for JavaScript?](https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/welcome.html). For more information
about the SDK for Python (Boto3), see [AWS SDK for Python (Boto3)](https://docs.aws.amazon.com/pythonsdk). For more information about the AWS CLI, see [What is the AWS CLI?](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html).
## Set up an edge-optimized PetStore API
In this tutorial, example commands use placeholder values for value IDs such as API ID and resource ID. As you
complete the tutorial, replace these values with your own.
###### To set up an edge-optimized PetStore API using AWS SDKs
1. Use the following example to create a
`RestApi` entity:
JavaScript v3
```
`import {APIGatewayClient, CreateRestApiCommand} from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new CreateRestApiCommand({
name: "Simple PetStore (JavaScript v3 SDK)",
description: "Demo API created using the AWS SDK for JavaScript v3",
version: "0.00.001",
binaryMediaTypes: [
'\*']
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.error(Couldn't create API:\\n", err)
}
})();`
```
A successful call returns your API ID and the root resource ID of your API in an output like the
following:
```
`{
id: 'abc1234',
name: 'PetStore (JavaScript v3 SDK)',
description: 'Demo API created using the AWS SDK for node.js',
createdDate: 2017-09-05T19:32:35.000Z,
version: '0.00.001',
rootResourceId: 'efg567'
binaryMediaTypes: [ '\*' ]
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.create\_rest\_api(
name='Simple PetStore (Python SDK)',
description='Demo API created using the AWS SDK for Python',
version='0.00.001',
binaryMediaTypes=[
'\*'
]
)
except botocore.exceptions.ClientError as error:
logger.exception("Couldn't create REST API %s.", error)
raise
attribute=["id","name","description","createdDate","version","binaryMediaTypes","apiKeySource","endpointConfiguration","disableExecuteApiEndpoint","rootResourceId"]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns your API ID and the root resource ID of your API in an output like the
following:
```
`{'id': 'abc1234', 'name': 'Simple PetStore (Python SDK)', 'description': 'Demo API created using the AWS SDK for Python', 'createdDate': datetime.datetime(2024, 4, 3, 14, 31, 39, tzinfo=tzlocal()), 'version': '0.00.001', 'binaryMediaTypes': ['\*'], 'apiKeySource': 'HEADER', 'endpointConfiguration': {'types': ['EDGE']}, 'disableExecuteApiEndpoint': False, 'rootResourceId': 'efg567'}`
```
AWS CLI
```
`aws apigateway create-rest-api --name 'Simple PetStore (AWS CLI)' --region us-west-2`
```
The following is the output of this command:
```
`{
"id": "abcd1234",
"name": "Simple PetStore (AWS CLI)",
"createdDate": "2022-12-15T08:07:04-08:00",
"apiKeySource": "HEADER",
"endpointConfiguration": {
"types": [
"EDGE"
]
},
"disableExecuteApiEndpoint": false,
"rootResourceId": "efg567"
}`
```
The API you created has an API ID of `abcd1234` and a root resource ID of
`efg567`. You use these values in the set up of your API.
2. Next, you append a child resource under the root, you specify the
`RootResourceId` as the `parentId` property value. Use the following example to create a `/pets` resource for your API:
JavaScript v3
```
`import {APIGatewayClient, CreateResourceCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new CreateResourceCommand({
restApiId: 'abcd1234',
parentId: 'efg567',
pathPart: 'pets'
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The '/pets' resource setup failed:\\n", err)
}
})();`
```
A successful call returns information about your resource in an output like the
following:
```
`{
"path": "/pets",
"pathPart": "pets",
"id": "aaa111",
"parentId": "efg567'"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.create\_resource(
restApiId='abcd1234',
parentId='efg567',
pathPart='pets'
)
except botocore.exceptions.ClientError as error:
logger.exception("The '/pets' resource setup failed: %s.", error)
raise
attribute=["id","parentId", "pathPart", "path",]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns information about your resource in an output like the
following:
```
`{'id': '`aaa111`', 'parentId': 'efg567', 'pathPart': 'pets', 'path': '/pets'}`
```
AWS CLI
```
`aws apigateway create-resource --rest-api-id abcd1234 \\
--region us-west-2 \\
--parent-id efg567 \\
--path-part pets`
```
The following is the output of this command:
```
`{
"id": "aaa111",
"parentId": "efg567",
"pathPart": "pets",
"path": "/pets"
}`
```
The `/pets` resource you created has a resource ID of `aaa111`. You use this value in the set up of your API.
3. Next, you append a child resource under the `/pets` resource. This resource,
`/{petId}` has a path parameter for the `{petId}`.To make a path part a path parameter, enclose it in a pair of curly
brackets, `{ }`. Use the following example to create a
`/pets/{petId}` resource for your API:
JavaScript v3
```
`import {APIGatewayClient, CreateResourceCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new CreateResourceCommand({
restApiId: 'abcd1234',
parentId: 'aaa111',
pathPart: '{petId}'
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The '/pets/{petId}' resource setup failed:\\n", err)
}
})();`
```
A successful call returns information about your resource in an output like the
following:
```
`{
"path": "/pets/{petId}",
"pathPart": "{petId}",
"id": "bbb222",
"parentId": "aaa111'"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.create\_resource(
restApiId='abcd1234',
parentId='aaa111',
pathPart='{petId}'
)
except botocore.exceptions.ClientError as error:
logger.exception("The '/pets/{petId}' resource setup failed: %s.", error)
raise
attribute=["id","parentId", "pathPart", "path",]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns information about your resource in an output like the
following:
```
`{'id': 'bbb222', 'parentId': 'aaa111', 'pathPart': '{petId}', 'path': '/pets/{petId}'}`
```
AWS CLI
```
`aws apigateway create-resource --rest-api-id abcd1234 \\
--region us-west-2 \\
--parent-id aaa111 \\
--path-part '{petId}'`
```
The following is the output of this command:
```
`{
"id": "bbb222",
"parentId": "aaa111",
"path": "/pets/{petId}",
"pathPart": "{petId}"
}`
```
The `/pets/{petId}` resource you created has a resource ID of `bbb222`. You use this value in the set up of your API.
4. In the following two steps, you add HTTP methods to your resources. In this tutorial, you set the methods to have open access by setting the `authorization-type` to
set to `NONE`. To permit only authenticated users to call the
method, you can use IAM roles and policies, a Lambda authorizer (formerly
known as a custom authorizer), or an Amazon Cognito user pool. For more information,
see [Control and manage access to
REST APIs in API Gateway](./apigateway-control-access-to-api.html).
Use the following example to add the `GET` HTTP method on the `/pets` resource:
JavaScript v3
```
`import {APIGatewayClient, PutMethodCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutMethodCommand({
restApiId: 'abcd1234',
resourceId: 'aaa111',
httpMethod: 'GET',
authorizationType: 'NONE'
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The 'GET /pets' method setup failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"apiKeyRequired": false,
"httpMethod": "GET",
"authorizationType": "NONE"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_method(
restApiId='abcd1234',
resourceId='aaa111',
httpMethod='GET',
authorizationType='NONE'
)
except botocore.exceptions.ClientError as error:
logger.exception("The 'GET /pets' method setup failed: %s", error)
raise
attribute=["httpMethod","authorizationType","apiKeyRequired"]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns the following output:
```
`{'httpMethod': 'GET', 'authorizationType': 'NONE', 'apiKeyRequired': False}`
```
AWS CLI
```
`aws apigateway put-method --rest-api-id abcd1234 \\
--resource-id aaa111 \\
--http-method GET \\
--authorization-type "NONE" \\
--region us-west-2`
```
The following is the output of this command:
```
`{
"httpMethod": "GET",
"authorizationType": "NONE",
"apiKeyRequired": false
}`
```
5. Use the following example to add the `GET` HTTP method on the `/pets/{petId}` resource
and to set the `requestParameters` property to pass the client-supplied `petId` value to
the backend:
JavaScript v3
```
`import {APIGatewayClient, PutMethodCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutMethodCommand({
restApiId: 'abcd1234',
resourceId: 'bbb222',
httpMethod: 'GET',
authorizationType: 'NONE'
requestParameters: {
"method.request.path.petId" : true
}
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The 'GET /pets/{petId}' method setup failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"apiKeyRequired": false,
"httpMethod": "GET",
"authorizationType": "NONE",
"requestParameters": {
"method.request.path.petId": true
}
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_method(
restApiId='abcd1234',
resourceId='bbb222',
httpMethod='GET',
authorizationType='NONE',
requestParameters={
"method.request.path.petId": True
}
)
except botocore.exceptions.ClientError as error:
logger.exception("The 'GET /pets/{petId}' method setup failed: %s", error)
raise
attribute=["httpMethod","authorizationType","apiKeyRequired", "requestParameters" ]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns the following output:
```
`{'httpMethod': 'GET', 'authorizationType': 'NONE', 'apiKeyRequired': False, 'requestParameters': {'method.request.path.petId': True}}`
```
AWS CLI
```
`aws apigateway put-method --rest-api-id abcd1234 \\
--resource-id bbb222 --http-method GET \\
--authorization-type "NONE" \\
--region us-west-2 \\
--request-parameters method.request.path.petId=true`
```
The following is the output of this command:
```
`{
"httpMethod": "GET",
"authorizationType": "NONE",
"apiKeyRequired": false,
"requestParameters": {
"method.request.path.petId": true
}
}`
```
6. Use the following example to add the 200 OK method response for the `GET /pets` method:
JavaScript v3
```
`import {APIGatewayClient, PutMethodResponseCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutMethodResponseCommand({
restApiId: 'abcd1234',
resourceId: 'aaa111',
httpMethod: 'GET',
statusCode: '200'
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("Set up the 200 OK response for the 'GET /pets' method failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"statusCode": "200"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_method\_response(
restApiId='abcd1234',
resourceId='aaa111',
httpMethod='GET',
statusCode='200'
)
except botocore.exceptions.ClientError as error:
logger.exception("Set up the 200 OK response for the 'GET /pets' method failed %s.", error)
raise
attribute=["statusCode"]
filtered\_result ={key:result[key] for key in attribute}
logger.info(filtered\_result)`
```
A successful call returns the following output:
```
`{'statusCode': '200'}`
```
AWS CLI
```
`aws apigateway put-method-response --rest-api-id abcd1234 \\
--resource-id aaa111 --http-method GET \\
--status-code 200 --region us-west-2`
```
The following is the output of this command:
```
`{
"statusCode": "200"
}`
```
7. Use the following example to add the 200 OK method response for the `GET /pets/{petId}` method:
JavaScript v3
```
`import {APIGatewayClient, PutMethodResponseCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutMethodResponseCommand({
restApiId: 'abcd1234',
resourceId: 'bbb222',
httpMethod: 'GET',
statusCode: '200'
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("Set up the 200 OK response for the 'GET /pets/{petId}' method failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"statusCode": "200"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_method\_response(
restApiId='abcd1234',
resourceId='bbb222',
httpMethod='GET',
statusCode='200'
)
except botocore.exceptions.ClientError as error:
logger.exception("Set up the 200 OK response for the 'GET /pets/{petId}' method failed %s.", error)
raise
attribute=["statusCode"]
filtered\_result ={key:result[key] for key in attribute}
logger.info(filtered\_result)`
```
A successful call returns the following output:
```
`{'statusCode': '200'}`
```
AWS CLI
```
`aws apigateway put-method-response --rest-api-id abcd1234 \\
--resource-id bbb222 --http-method GET \\
--status-code 200 --region us-west-2`
```
The following is the output of this command:
```
`{
"statusCode": "200"
}`
```
8. Use the following example to configure an integration for the `GET /pets` method with an HTTP
endpoint. The HTTP endpoint is
`http://petstore-demo-endpoint.execute-api.com/petstore/pets`.
JavaScript v3
```
`import {APIGatewayClient, PutIntegrationCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutIntegrationCommand({
restApiId: 'abcd1234',
resourceId: 'aaa111',
httpMethod: 'GET',
type: 'HTTP',
integrationHttpMethod: 'GET',
uri: 'http://petstore-demo-endpoint.execute-api.com/petstore/pets'
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("Set up the integration of the 'GET /pets' method of the API failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"httpMethod": "GET",
"passthroughBehavior": "WHEN\_NO\_MATCH",
"cacheKeyParameters": [],
"type": "HTTP",
"uri": "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"cacheNamespace": "ccc333"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_integration(
restApiId='abcd1234',
resourceId='aaa111',
httpMethod='GET',
type='HTTP',
integrationHttpMethod='GET',
uri='http://petstore-demo-endpoint.execute-api.com/petstore/pets'
)
except botocore.exceptions.ClientError as error:
logger.exception("Set up the integration of the 'GET /' method of the API failed %s.", error)
raise
attribute=["httpMethod","passthroughBehavior","cacheKeyParameters", "type", "uri", "cacheNamespace"]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns the following output:
```
`{'httpMethod': 'GET', 'passthroughBehavior': 'WHEN\_NO\_MATCH', 'cacheKeyParameters': [], 'type': 'HTTP', 'uri': 'http://petstore-demo-endpoint.execute-api.com/petstore/pets', 'cacheNamespace': 'ccc333'}`
```
AWS CLI
```
`aws apigateway put-integration --rest-api-id abcd1234 \\
--resource-id aaa111 --http-method GET --type HTTP \\
--integration-http-method GET \\
--uri 'http://petstore-demo-endpoint.execute-api.com/petstore/pets' \\
--region us-west-2`
```
The following is the output of this command:
```
`{
"type": "HTTP",
"httpMethod": "GET",
"uri": "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"connectionType": "INTERNET",
"passthroughBehavior": "WHEN\_NO\_MATCH",
"timeoutInMillis": 29000,
"cacheNamespace": "6sxz2j",
"cacheKeyParameters": []
}`
```
9. Use the following example to configure an integration for the `GET /pets/{petId}` method with an
HTTP endpoint. The HTTP endpoint is
`http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}`. In this step, you map the
path parameter `petId` to the integration endpoint path parameter of `id`.
JavaScript v3
```
`import {APIGatewayClient, PutIntegrationCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutIntegrationCommand({
restApiId: 'abcd1234',
resourceId: 'bbb222',
httpMethod: 'GET',
type: 'HTTP',
integrationHttpMethod: 'GET',
uri: 'http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}'
requestParameters: {
"integration.request.path.id": "method.request.path.petId"
}
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("Set up the integration of the 'GET /pets/{petId}' method of the API failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"httpMethod": "GET",
"passthroughBehavior": "WHEN\_NO\_MATCH",
"cacheKeyParameters": [],
"type": "HTTP",
"uri": "http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}",
"cacheNamespace": "ddd444",
"requestParameters": {
"integration.request.path.id": "method.request.path.petId"
}
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_integration(
restApiId='ieps9b05sf',
resourceId='t8zeb4',
httpMethod='GET',
type='HTTP',
integrationHttpMethod='GET',
uri='http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}',
requestParameters={
"integration.request.path.id": "method.request.path.petId"
}
)
except botocore.exceptions.ClientError as error:
logger.exception("Set up the integration of the 'GET /pets/{petId}' method of the API failed %s.", error)
raise
attribute=["httpMethod","passthroughBehavior","cacheKeyParameters", "type", "uri", "cacheNamespace", "requestParameters"]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns the following output:
```
`{'httpMethod': 'GET', 'passthroughBehavior': 'WHEN\_NO\_MATCH', 'cacheKeyParameters': [], 'type': 'HTTP', 'uri': 'http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}', 'cacheNamespace': 'ddd444', 'requestParameters': {'integration.request.path.id': 'method.request.path.petId'}}}`
```
AWS CLI
```
`aws apigateway put-integration --rest-api-id abcd1234 \\
--resource-id bbb222 --http-method GET --type HTTP \\
--integration-http-method GET \\
--uri 'http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}' \\
--request-parameters '{"integration.request.path.id":"method.request.path.petId"}' \\
--region us-west-2`
```
The following is the output of this command:
```
`{
"type": "HTTP",
"httpMethod": "GET",
"uri": "http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}",
"connectionType": "INTERNET",
"requestParameters": {
"integration.request.path.id": "method.request.path.petId"
},
"passthroughBehavior": "WHEN\_NO\_MATCH",
"timeoutInMillis": 29000,
"cacheNamespace": "rjkmth",
"cacheKeyParameters": []
}`
```
10. Use the following example to add the integration response for the `GET /pets` integration:
JavaScript v3
```
`import {APIGatewayClient, PutIntegrationResponseCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutIntegrationResponseCommand({
restApiId: 'abcd1234',
resourceId: 'aaa111',
httpMethod: 'GET',
statusCode: '200',
selectionPattern: ''
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The 'GET /pets' method integration response setup failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"selectionPattern": "",
"statusCode": "200"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_integration\_response(
restApiId='abcd1234',
resourceId='aaa111',
httpMethod='GET',
statusCode='200',
selectionPattern='',
)
except botocore.exceptions.ClientError as error:
logger.exception("Set up the integration response of the 'GET /pets' method of the API failed: %s", error)
raise
attribute=["selectionPattern","statusCode"]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns the following output:
```
`{'selectionPattern': "", 'statusCode': '200'}`
```
AWS CLI
```
`aws apigateway put-integration-response --rest-api-id abcd1234 \\
--resource-id aaa111 --http-method GET \\
--status-code 200 --selection-pattern "" \\
--region us-west-2`
```
The following is the output of this command:
```
`{
"statusCode": "200",
"selectionPattern": ""
}`
```
11. Use the following example to add the integration response for the `GET /pets/{petId}` integration:
JavaScript v3
```
`import {APIGatewayClient, PutIntegrationResponseCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new PutIntegrationResponseCommand({
restApiId: 'abcd1234',
resourceId: 'bbb222',
httpMethod: 'GET',
statusCode: '200',
selectionPattern: ''
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The 'GET /pets/{petId}' method integration response setup failed:\\n", err)
}
})();`
```
A successful call returns the following output:
```
`{
"selectionPattern": "",
"statusCode": "200"
}`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.put\_integration\_response(
restApiId='abcd1234',
resourceId='bbb222',
httpMethod='GET',
statusCode='200',
selectionPattern='',
)
except botocore.exceptions.ClientError as error:
logger.exception("Set up the integration response of the 'GET /pets/{petId}' method of the API failed: %s", error)
raise
attribute=["selectionPattern","statusCode"]
filtered\_result ={key:result[key] for key in attribute}
print(filtered\_result)`
```
A successful call returns the following output:
```
`{'selectionPattern': "", 'statusCode': '200'}`
```
AWS CLI
```
`aws apigateway put-integration-response --rest-api-id abcd1234 \\
--resource-id bbb222 --http-method GET
--status-code 200 --selection-pattern ""
--region us-west-2`
```
The following is the output of this command:
```
`{
"statusCode": "200",
"selectionPattern": ""
}`
```
After you create the integration response, your API can query available pets on the PetStore website and to
view an individual pet of a specified identifier. Before your API is callable by your customers, you must deploy
it. We recommend that before you deploy your API, you test it.
12. Use the following example to test the `GET /pets` method:
JavaScript v3
```
`import {APIGatewayClient, TestInvokeMethodCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new TestInvokeMethodCommand({
restApiId: 'abcd1234',
resourceId: 'aaa111',
httpMethod: 'GET',
pathWithQueryString: '/',
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The test on 'GET /pets' method failed:\\n", err)
}
})();`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.test\_invoke\_method(
restApiId='abcd1234',
resourceId='aaa111',
httpMethod='GET',
pathWithQueryString='/',
)
except botocore.exceptions.ClientError as error:
logger.exception("Test invoke method on 'GET /pets' failed: %s", error)
raise
print(result)`
```
AWS CLI
```
`aws apigateway test-invoke-method --rest-api-id abcd1234 /
--resource-id aaa111 /
--http-method GET /
--path-with-query-string '/'`
```
13. Use the following example to test the `GET /pets/{petId}` method with a `petId` of 3:
JavaScript v3
```
`import {APIGatewayClient, TestInvokeMethodCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new TestInvokeMethodCommand({
restApiId: 'abcd1234',
resourceId: 'bbb222',
httpMethod: 'GET',
pathWithQueryString: '/pets/3',
});
try {
const results = await apig.send(command)
console.log(results)
} catch (err) {
console.log("The test on 'GET /pets/{petId}' method failed:\\n", err)
}
})();`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.test\_invoke\_method(
restApiId='abcd1234',
resourceId='bbb222',
httpMethod='GET',
pathWithQueryString='/pets/3',
)
except botocore.exceptions.ClientError as error:
logger.exception("Test invoke method on 'GET /pets/{petId}' failed: %s", error)
raise
print(result)`
```
AWS CLI
```
`aws apigateway test-invoke-method --rest-api-id abcd1234 /
--resource-id bbb222 /
--http-method GET /
--path-with-query-string '/pets/3'`
```
After you successfully test your API, you can deploy it to a stage.
14. Use the following example to deploy your API to a stage named `test`. When you deploy your API to a stage, API callers can invoke your API.
JavaScript v3
```
`import {APIGatewayClient, CreateDeploymentCommand } from "@aws-sdk/client-api-gateway";
(async function (){
const apig = new APIGatewayClient({region:"us-east-1"});
const command = new CreateDeploymentCommand({
restApiId: 'abcd1234',
stageName: 'test',
stageDescription: 'test deployment'
});
try {
const results = await apig.send(command)
console.log("Deploying API succeeded\\n", results)
} catch (err) {
console.log("Deploying API failed:\\n", err)
}
})();`
```
Python
```
`import botocore
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
try:
result = apig.create\_deployment(
restApiId='ieps9b05sf',
stageName='test',
stageDescription='my test stage',
)
except botocore.exceptions.ClientError as error:
logger.exception("Error deploying stage %s.", error)
raise
print('Deploying API succeeded')
print(result)`
```
AWS CLI
```
`aws apigateway create-deployment --rest-api-id abcd1234 \\
--region us-west-2 \\
--stage-name test \\
--stage-description 'Test stage' \\
--description 'First deployment'`
```
The following is the output of this command:
```
`{
"id": "ab1c1d",
"description": "First deployment",
"createdDate": "2022-12-15T08:44:13-08:00"
}`
```
Your API is now callable by customers. You can test this API by entering the
`https://abcd1234.execute-api.us-west-2.amazonaws.com/test/pets`
URL in a browser, and substituting `abcd1234` with the identifier of
your API.
For more examples of how to create or update an API using AWS SDKs or the AWS CLI, see
[Actions for
API Gateway using AWS SDKs](https://docs.aws.amazon.com/code-library/latest/ug/api-gateway_code_examples_actions.html).
## Automate the setup of your API
Instead of creating your API step-by-step, you can automate the creation and cleanup of AWS resources by
using OpenAPI, CloudFormation, or Terraform to create your API.
You can import an OpenAPI defintion into API Gateway. For more information, see [Develop REST APIs using
OpenAPI in API Gateway](./api-gateway-import-api.html).
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "Simple PetStore (OpenAPI)",
"description" : "Demo API created using OpenAPI",
"version" : "2024-05-24T20:39:34Z"
},
"servers" : [ {
"url" : "{basePath}",
"variables" : {
"basePath" : {
"default" : "Prod"
}
}
} ],
"paths" : {
"/pets" : {
"get" : {
"responses" : {
"200" : {
"description" : "200 response",
"content" : { }
}
},
"x-amazon-apigateway-integration" : {
"type" : "http",
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"passthroughBehavior" : "when\_no\_match",
"timeoutInMillis" : 29000
}
}
},
"/pets/{petId}" : {
"get" : {
"parameters" : [ {
"name" : "petId",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"200" : {
"description" : "200 response",
"content" : { }
}
},
"x-amazon-apigateway-integration" : {
"type" : "http",
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.path.id" : "method.request.path.petId"
},
"passthroughBehavior" : "when\_no\_match",
"timeoutInMillis" : 29000
}
}
}
},
"components" : { }
}`
```
To deploy your CloudFormation template, see [Creating a stack on the AWS CloudFormation
console](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-console-create-stack.html).
```
`AWSTemplateFormatVersion: 2010-09-09
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Name: Simple PetStore (AWS CloudFormation)
PetsResource:
Type: 'AWS::ApiGateway::Resource'
Properties:
RestApiId: !Ref Api
ParentId: !GetAtt Api.RootResourceId
PathPart: 'pets'
PetIdResource:
Type: 'AWS::ApiGateway::Resource'
Properties:
RestApiId: !Ref Api
ParentId: !Ref PetsResource
PathPart: '{petId}'
PetsMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref PetsResource
HttpMethod: GET
AuthorizationType: NONE
Integration:
Type: HTTP
IntegrationHttpMethod: GET
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/
IntegrationResponses:
- StatusCode: '200'
MethodResponses:
- StatusCode: '200'
PetIdMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref PetIdResource
HttpMethod: GET
AuthorizationType: NONE
RequestParameters:
method.request.path.petId: true
Integration:
Type: HTTP
IntegrationHttpMethod: GET
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}
RequestParameters:
integration.request.path.id: method.request.path.petId
IntegrationResponses:
- StatusCode: '200'
MethodResponses:
- StatusCode: '200'
ApiDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn:
- PetsMethodGet
Properties:
RestApiId: !Ref Api
StageName: Prod
Outputs:
ApiRootUrl:
Description: Root Url of the API
Value: !Sub 'https://${Api}.execute-api.${AWS::Region}.amazonaws.com/Prod'`
```
For more information about Terraform, see [Terraform](https://developer.hashicorp.com/terraform/intro).
```
`provider "aws" {
region = "us-east-1" # Update with your desired region
}
resource "aws\_api\_gateway\_rest\_api" "Api" {
name = "Simple PetStore (Terraform)"
description = "Demo API created using Terraform"
}
resource "aws\_api\_gateway\_resource" "petsResource"{
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
parent\_id = aws\_api\_gateway\_rest\_api.Api.root\_resource\_id
path\_part = "pets"
}
resource "aws\_api\_gateway\_resource" "petIdResource"{
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
parent\_id = aws\_api\_gateway\_resource.petsResource.id
path\_part = "{petId}"
}
resource "aws\_api\_gateway\_method" "petsMethodGet" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petsResource.id
http\_method = "GET"
authorization = "NONE"
}
resource "aws\_api\_gateway\_method\_response" "petsMethodResponseGet" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petsResource.id
http\_method = aws\_api\_gateway\_method.petsMethodGet.http\_method
status\_code ="200"
}
resource "aws\_api\_gateway\_integration" "petsIntegration" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petsResource.id
http\_method = aws\_api\_gateway\_method.petsMethodGet.http\_method
type = "HTTP"
uri = "http://petstore-demo-endpoint.execute-api.com/petstore/pets"
integration\_http\_method = "GET"
depends\_on = [aws\_api\_gateway\_method.petsMethodGet]
}
resource "aws\_api\_gateway\_integration\_response" "petsIntegrationResponse" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petsResource.id
http\_method = aws\_api\_gateway\_method.petsMethodGet.http\_method
status\_code = aws\_api\_gateway\_method\_response.petsMethodResponseGet.status\_code
}
resource "aws\_api\_gateway\_method" "petIdMethodGet" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petIdResource.id
http\_method = "GET"
authorization = "NONE"
request\_parameters = {"method.request.path.petId" = true}
}
resource "aws\_api\_gateway\_method\_response" "petIdMethodResponseGet" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petIdResource.id
http\_method = aws\_api\_gateway\_method.petIdMethodGet.http\_method
status\_code ="200"
}
resource "aws\_api\_gateway\_integration" "petIdIntegration" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petIdResource.id
http\_method = aws\_api\_gateway\_method.petIdMethodGet.http\_method
type = "HTTP"
uri = "http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}"
integration\_http\_method = "GET"
request\_parameters = {"integration.request.path.id" = "method.request.path.petId"}
depends\_on = [aws\_api\_gateway\_method.petIdMethodGet]
}
resource "aws\_api\_gateway\_integration\_response" "petIdIntegrationResponse" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
resource\_id = aws\_api\_gateway\_resource.petIdResource.id
http\_method = aws\_api\_gateway\_method.petIdMethodGet.http\_method
status\_code = aws\_api\_gateway\_method\_response.petIdMethodResponseGet.status\_code
}
resource "aws\_api\_gateway\_deployment" "Deployment" {
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
depends\_on = [aws\_api\_gateway\_integration.petsIntegration,aws\_api\_gateway\_integration.petIdIntegration ]
}
resource "aws\_api\_gateway\_stage" "Stage" {
stage\_name = "Prod"
rest\_api\_id = aws\_api\_gateway\_rest\_api.Api.id
deployment\_id = aws\_api\_gateway\_deployment.Deployment.id
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
OpenAPI
definitions of a sample API as a Kinesis proxy
Tutorial: Create a private REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.