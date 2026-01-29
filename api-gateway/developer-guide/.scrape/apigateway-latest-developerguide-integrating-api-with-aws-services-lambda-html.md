---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/integrating-api-with-aws-services-lambda.html
title: Tutorial: Create a
word_count: 3944
filtered: true
elements_removed: 0
density_score: 0.83
---

Tutorial: Create a calculator REST API with two AWS service integrations and one Lambda non-proxy integration - Amazon API Gateway
Tutorial: Create a calculator REST API with two AWS service integrations and one Lambda non-proxy integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#integrating-api-with-aws-services-lambda)
[Create an assumable IAM
role](#api-as-lambda-proxy-setup-iam-role-policies)[Create a Calc
Lambda function](#api-as-lambda-proxy-create-lambda-function)[Test the Calc
Lambda function](#api-as-lambda-proxy-test-lambda-function-)[Create a Calc
API](#api-as-lambda-proxy-create-api-resources)[Integration 1: Create a GET method with query parameters to call the
Lambda function](#api-as-lambda-proxy-expose-get-method-with-query-strings-to-call-lambda-function)[Integration 2: Create a POST method with a JSON payload to call the
Lambda function](#api-as-lambda-proxy-expose-post-method-with-json-body-to-call-lambda-function)[Integration 3: Create a GET method with path parameters to call the
Lambda function](#api-as-lambda-proxy-expose-get-method-with-path-parameters-to-call-lambda-function)
# Tutorial: Create a
calculator REST API with two AWS service integrations and one Lambda non-proxy integration
The [Tutorial: Create a REST API with a Lambda non-proxy
integration](./getting-started-lambda-non-proxy-integration.html) uses `Lambda Function` integration exclusively.
`Lambda Function` integration is a special case of the `AWS
Service` integration type that performs much of the integration setup for you,
such as automatically adding the required resource-based permissions for invoking the Lambda
function. Here, two of the three integrations use `AWS Service` integration. In
this integration type, you have more control, but you'll need to manually perform tasks like
creating and specifying an IAM role containing appropriate permissions.
In this tutorial, you'll create a `Calc` Lambda function that implements basic
arithmetic operations, accepting and returning JSON-formatted input and output. Then you'll
create a REST API and integrate it with the Lambda function in the following ways:
1. By exposing a `GET` method on the `/calc` resource to invoke
the Lambda function, supplying the input as query string parameters. (`AWS
Service` integration)
2. By exposing a `POST` method on the `/calc` resource to
invoke the Lambda function, supplying the input in the method request payload.
(`AWS Service` integration)
3. By exposing a `GET` on nested
`/calc/{operand1}/{operand2}/{operator}` resources to invoke the
Lambda function, supplying the input as path parameters. (`Lambda Function`
integration)
In addition to trying out this tutorial, you may wish to study the [OpenAPI definition
file](./api-as-lambda-proxy-export-swagger-with-extensions.html) for the `Calc` API, which you can import into API Gateway by following
the instructions in [Develop REST APIs using
OpenAPI in API Gateway](./api-gateway-import-api.html).
###### Topics
* [Create an assumable IAM
role](#api-as-lambda-proxy-setup-iam-role-policies)
* [Create a Calc
Lambda function](#api-as-lambda-proxy-create-lambda-function)
* [Test the Calc
Lambda function](#api-as-lambda-proxy-test-lambda-function-)
* [Create a Calc
API](#api-as-lambda-proxy-create-api-resources)
* [Integration 1: Create a GET method with query parameters to call the
Lambda function](#api-as-lambda-proxy-expose-get-method-with-query-strings-to-call-lambda-function)
* [Integration 2: Create a POST method with a JSON payload to call the
Lambda function](#api-as-lambda-proxy-expose-post-method-with-json-body-to-call-lambda-function)
* [Integration 3: Create a GET method with path parameters to call the
Lambda function](#api-as-lambda-proxy-expose-get-method-with-path-parameters-to-call-lambda-function)
* [OpenAPI definitions of sample API integrated with a Lambda function](./api-as-lambda-proxy-export-swagger-with-extensions.html)
## Create an assumable IAM
role
In order for your API to invoke your `Calc` Lambda function, you'll need to
have an API Gateway assumable IAM role, which is an IAM role with the following trusted
relationship:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "",
"Effect": "Allow",
"Principal": {
"Service": "apigateway.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
`
```
The role you create will need to have Lambda [InvokeFunction](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html) permission. Otherwise, the API caller will receive a
`500 Internal Server Error` response. To give the role this permission,
you'll attach the following IAM policy to it:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
"Resource": "\*"
}
]
}`
`
```
Here's how to accomplish all this:
###### Create an API Gateway assumable IAM role
1. Log in to the IAM console.
2. Choose **Roles**.
3. Choose **Create Role**.
4. Under **Select type of trusted entity**, choose
**AWS Service**.
5. Under **Choose the service that will use this role**, choose
**Lambda**.
6. Choose **Next: Permissions**.
7. Choose **Create Policy**.
A new **Create Policy** console window will open up. In that
window, do the following:
1. In the **JSON** tab, replace the existing policy with
the following:
JSON
****
```
``
{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
"Resource": "\*"
}
]
}
`
`
```
2. Choose **Review policy**.
3. Under **Review Policy**, do the following:
1. For **Name**, type a name such as
`lambda\_execute`.
2. Choose **Create Policy**.
3. In the original **Create Role** console window, do the
following:
1. Under **Attach permissions policies**, choose your
`lambda\_execute` policy from the dropdown
list.
If you don't see your policy in the list, choose the refresh button at
the top of the list. (Don't refresh the browser page!)
2. Choose **Next:Tags**.
3. Choose **Next:Review**.
4. For the **Role name**, type a name such as
`lambda\_invoke\_function\_assume\_apigw\_role`.
5. Choose **Create role**.
6. Choose your `lambda\_invoke\_function\_assume\_apigw\_role`
from the list of roles.
7. Choose the **Trust relationships** tab.
8. Choose **Edit trust relationship**.
9. Replace the existing policy with the following:
JSON
****
```
``
{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "",
"Effect": "Allow",
"Principal": {
"Service": [
"lambda.amazonaws.com",
"apigateway.amazonaws.com"
]
},
"Action": "sts:AssumeRole"
}
]
}
`
`
```
10. Choose **Update Trust Policy**.
11. Make a note of the role ARN for the role you just created. You'll need it
later.
## Create a `Calc`
Lambda function
Next you'll create a Lambda function using the Lambda console.
1. In the Lambda console, choose **Create function**.
2. Choose **Author from Scratch**.
3. For **Name**, enter `Calc`.
4. For **Runtime**, choose either the latest supported **Node.js** or
**Python** runtime.
5. For all other options, use the default setting.
6. Choose **Create function**.
7. Copy the following Lambda function in your preferred runtime and paste it into the code editor in the
Lambda console.
Node.js
```
`export const handler = async function (event, context) {
console.log("Received event:", JSON.stringify(event));
if (
event.a === undefined ||
event.b === undefined ||
event.op === undefined
) {
return "400 Invalid Input";
}
const res = {};
res.a = Number(event.a);
res.b = Number(event.b);
res.op = event.op;
if (isNaN(event.a) || isNaN(event.b)) {
return "400 Invalid Operand";
}
switch (event.op) {
case "+":
case "add":
res.c = res.a + res.b;
break;
case "-":
case "sub":
res.c = res.a - res.b;
break;
case "\*":
case "mul":
res.c = res.a \* res.b;
break;
case "/":
case "div":
if (res.b == 0) {
return "400 Divide by Zero";
} else {
res.c = res.a / res.b;
}
break;
default:
return "400 Invalid Operator";
}
return res;
};`
```
Python
```
`import json
def lambda\_handler(event, context):
print(event)
try:
(event['a']) and (event['b']) and (event['op'])
except KeyError:
return '400 Invalid Input'
try:
res = {
"a": float(
event['a']), "b": float(
event['b']), "op": event['op']}
except ValueError:
return '400 Invalid Operand'
if event['op'] == '+':
res['c'] = res['a'] + res['b']
elif event['op'] == '-':
res['c'] = res['a'] - res['b']
elif event['op'] == '\*':
res['c'] = res['a'] \* res['b']
elif event['op'] == '/':
if res['b'] == 0:
return '400 Divide by Zero'
else:
res['c'] = res['a'] / res['b']
else:
return '400 Invalid Operator'
return res`
```
8. Under Execution role, choose **Choose an existing
role**.
9. Enter the role ARN for the
`lambda\_invoke\_function\_assume\_apigw\_role` role you
created earlier.
10. Choose **Deploy**.
This function requires two operands (`a` and `b`) and an
operator (`op`) from the `event` input parameter. The input is a
JSON object of the following format:
```
`
{
"a": "Number" | "String",
"b": "Number" | "String",
"op": "String"
}
`
```
This function returns the calculated result (`c`) and the input. For an
invalid input, the function returns either the null value or the "Invalid op" string as
the result. The output is of the following JSON format:
```
`
{
"a": "Number",
"b": "Number",
"op": "String",
"c": "Number" | "String"
}
`
```
You should test the function in the Lambda console before integrating it with the API
in the next step.
## Test the `Calc`
Lambda function
Here's how to test your `Calc` function in the Lambda console:
1. Choose the **Test** tab.
2. For the test event name, enter `calc2plus5`.
3. Replace the test event definition with the following:
```
`{
"a": "2",
"b": "5",
"op": "+"
}`
```
4. Choose **Save**.
5. Choose **Test**.
6. Expand **Execution result: succeeded**. You should see the
following:
```
`{
"a": 2,
"b": 5,
"op": "+",
"c": 7
}`
```
## Create a `Calc`
API
The following procedure shows how to create an API for the `Calc` Lambda
function you just created. In subsequent sections, you'll add resources and methods to
it.
###### To create an API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. If this is your first time using API Gateway, you see a page that introduces you to
the features of the service. Under **REST API**, choose **Build**. When the
**Create Example API** popup appears, choose
**OK**.
If this is not your first time using API Gateway, choose **Create
API**. Under **REST API**, choose **Build**.
3. For **API name**, enter `LambdaCalc`.
4. (Optional) For **Description**, enter a description.
5. Keep **API endpoint type** set to **Regional**.
6. For **IP address type**, select **IPv4**.
7. Choose **Create API**.
## Integration 1: Create a `GET` method with query parameters to call the
Lambda function
By creating a `GET` method that passes query string parameters to the Lambda
function, you enable the API to be invoked from a browser. This approach can be useful,
especially for APIs that allow open access.
After you create an API, you create a resource. Typically, API resources are organized in a resource tree
according to the application logic. For this step, you create a **/calc** resource.
###### To create a **/calc** resource
1. Choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. Keep **Resource path** as `/`.
4. For **Resource name**, enter `calc`.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
By creating a `GET` method that passes query string parameters to the Lambda
function, you enable the API to be invoked from a browser. This approach can be useful,
especially for APIs that allow open access.
In this method, Lambda requires that the `POST` request be used to invoke any Lambda function. This example
shows that the HTTP method in a frontend method request can be different from the integration request in the
backend.
###### To create a `GET` method
1. Select the **/calc** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Lambda function.
5. For **AWS service**, select **Lambda**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, select **POST**.
8. For **Action type**, select **Use path override**. This option allows us to specify the ARN of the
[Invoke](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html) action to
execute our `Calc` function.
9. For **Path override**, enter `2015-03-31/functions/arn:aws:lambda:`us-east-2`:`account-id`:function:Calc/invocations`. For `
`account-id``, enter your AWS account ID. For ``us-east-2``, enter the AWS Region where you created your Lambda function.
10. For **Execution role**, enter the role ARN for `lambda\_invoke\_function\_assume\_apigw\_role`.
11. Do not change the settings of **Credential cache** and **Default timeout**.
12. Choose **Method request settings**.
13. For **Request validator**, select **Validate query string parameters and headers**.
This setting will
cause an error message to return if the client does not specify the required parameters.
14. Choose **URL query string parameters**.
Now you set up query string parameters for the **GET** method
on the **/calc** resource so it can receive input on behalf of the backend
Lambda function.
To create the query string parameters do the following:
1. Choose **Add query string**.
2. For **Name**, enter `operand1`.
3. Turn on **Required**.
4. Keep **Caching** turned off.
Repeat the same steps and create a query string named `operand2` and a query string named `operator`.
5. Choose **Create method**.
Now, you create a mapping template to translate the client-supplied query strings to the integration
request payload as required by the `Calc` function. This template maps the three query string
parameters declared in **Method request** into designated property values of the JSON object as the input to the
backend Lambda function. The transformed JSON object will be included as the integration request payload.
###### To map input parameters to the integration request
1. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
2. For **Request body passthrough**, select **When there are no templates defined (recommended)**.
3. Choose **Mapping templates**.
4. Choose **Add mapping template**.
5. For **Content type**, enter `application/json`.
6. For **Template body**, enter the following code:
```
`{
"a": "$input.params('operand1')",
"b": "$input.params('operand2')",
"op": "$input.params('operator')"
}`
```
7. Choose **Save**.
You can now test your `GET` method to verify that it has been
properly set up to invoke the Lambda function.
###### To test the `GET` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Query strings**, enter
`operand1=2&amp;operand2=3&amp;operator=+`.
3. Choose **Test**.
The results should look similar to this:
![Create an API in API Gateway as a Lambda proxy](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/aws_proxy_lambda_calc_get_method_test_new_console.png)
## Integration 2: Create a `POST` method with a JSON payload to call the
Lambda function
By creating a `POST` method with a JSON payload to call the Lambda function,
you make it so that the client must provide the necessary input to the backend function
in the request body. To ensure that the client uploads the correct input data, you'll
enable request validation on the payload.
###### To create a `POST` method with a JSON payload
1. Select the **/calc** resource, and then choose **Create method**.
2. For **Method type**, select **POST**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Lambda function.
5. For **AWS service**, select **Lambda**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, select **POST**.
8. For **Action type**, select **Use path override**. This option allows us to specify the ARN of the
[Invoke](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html) action to
execute our `Calc` function.
9. For **Path override**, enter `2015-03-31/functions/arn:aws:lambda:`us-east-2`:`account-id`:function:Calc/invocations`. For `
`account-id``, enter your AWS account ID. For ``us-east-2``, enter the AWS Region where you created your Lambda function.
10. For **Execution role**, enter the role ARN for `lambda\_invoke\_function\_assume\_apigw\_role`.
11. Do not change the settings of **Credential cache** and **Default timeout**.
12. Choose **Create method**.
Now you create an **input** model to describe the input data structure and validate the incoming request body.
###### To create an input model
1. In the main navigation pane, choose **Models**.
2. Choose **Create model**.
3. For **Name**, enter `input`.
4. For **Content type**, enter `application/json`.
If no matching content type is found, request validation is not performed. To use the same model
regardless of the content type, enter `$default`.
5. For **Model schema**, enter the following model:
```
`{
"type":"object",
"properties":{
"a":{"type":"number"},
"b":{"type":"number"},
"op":{"type":"string"}
},
"title":"input"
}`
```
6. Choose **Create model**.
You now create an **output** model. This model describes the data structure of the calculated
output from the backend. It can be used to map the integration response data to a different model. This tutorial
relies on the passthrough behavior and does not use this model.
###### To create an output model
1. Choose **Create model**.
2. For **Name**, enter `output`.
3. For **Content type**, enter `application/json`.
If no matching content type is found, request validation is not performed. To use the same model
regardless of the content type, enter `$default`.
4. For **Model schema**, enter the following model:
```
`{
"type":"object",
"properties":{
"c":{"type":"number"}
},
"title":"output"
}`
```
5. Choose **Create model**.
You now create a **result** model. This model describes the data structure of the returned response data. It references both the **input** and **output**
schemas defined in your API.
###### To create a result model
1. Choose **Create model**.
2. For **Name**, enter `result`.
3. For **Content type**, enter `application/json`.
If no matching content type is found, request validation is not performed. To use the same model
regardless of the content type, enter `$default`.
4. For **Model schema**, enter the following model with your
`restapi-id`. Your `restapi-id` is listed in parenthesis
at the top of the console in the following flow: `API Gateway &gt; APIs &gt;
LambdaCalc (`abc123`).`
```
`{
"type":"object",
"properties":{
"input":{
"$ref":"https://apigateway.amazonaws.com/restapis/`restapi-id`/models/input"
},
"output":{
"$ref":"https://apigateway.amazonaws.com/restapis/`restapi-id`/models/output"
}
},
"title":"result"
}`
```
5. Choose **Create model**.
You now configure the method request of your POST method to enable request validation on the incoming request body.
###### To enable request validation on the POST method
1. In the main navigation pane, choose **Resources**, and then select the `POST` method from the resource tree.
2. On the **Method request** tab, under **Method request settings**, choose **Edit**.
3. For **Request validator**, select **Validate body**.
4. Choose **Request body**, and then choose **Add model**.
5. For **Content type**, enter `application/json`.
If no matching content type is found, request validation is not performed. To use the same model
regardless of the content type, enter `$default`.
6. For **Model**, select **input**.
7. Choose **Save**.
You can now test your `POST` method to verify that it has been
properly set up to invoke the Lambda function.
###### To test the `POST` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Request body**, enter the following JSON payload.
```
`
{
"a": 1,
"b": 2,
"op": "+"
}
`
```
3. Choose **Test**.
You should see the following output:
```
`
{
"a": 1,
"b": 2,
"op": "+",
"c": 3
}
`
```
## Integration 3: Create a `GET` method with path parameters to call the
Lambda function
Now you'll create a `GET` method on a resource specified by a sequence of
path parameters to call the backend Lambda function. The path parameter values specify
the input data to the Lambda function. You'll use a mapping template to map the incoming path
parameter values to the required integration request payload.
The resulting API resource structure will look like this:
![Create an API in API Gateway as a Lambda proxy](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/aws_proxy_lambda_create_api_resources_new_console.png)
###### To create a **/{operand1}/{operand2}/{operator}** resource
1. Choose **Create resource**.
2. For **Resource path**, select `/calc`.
3. For **Resource name**, enter `{operand1}`.
4. Keep **CORS (Cross Origin Resource Sharing)** turned off.
5. Choose **Create resource**.
6. For **Resource path**, select `/calc/{operand1}/`.
7. For **Resource name**, enter `{operand2}`.
8. Keep **CORS (Cross Origin Resource Sharing)** turned off.
9. Choose **Create resource**.
10. For **Resource path**, select `/calc/{operand1}/{operand2}/`.
11. For **Resource name**, enter `{operator}`.
12. Keep **CORS (Cross Origin Resource Sharing)** turned off.
13. Choose **Create resource**.
This time you'll use the built-in Lambda integration in the API Gateway console to
set up the method integration.
###### To set up a method integration
1. Select the **/{operand1}/{operand2}/{operator}** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **Lambda**.
4. Keep **Lambda proxy integration** turned off.
5. For **Lambda function**, select the AWS Region where you created your Lambda function and enter `Calc`.
6. Keep **Default timeout** turned on.
7. Choose **Create method**.
You now create a mapping template to map the three URL path parameters, declared when the
**/calc/{operand1}/{operand2}/{operator}** resource
was created, into designated property values in the JSON object. Because
URL paths must be URL-encoded, the division operator must be specified
as `%2F` instead of `/`. This template translates
the `%2F` into `'/'` before passing it to the
Lambda function.
###### To create a mapping template
1. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
2. For **Request body passthrough**, select **When there are no templates defined (recommended)**.
3. Choose **Mapping templates**.
4. For **Content type**, enter `application/json`.
5. For **Template body**, enter the following code:
```
`{
"a": "$input.params('operand1')",
"b": "$input.params('operand2')",
"op": #if($input.params('operator')=='%2F')"/"#{else}"$input.params('operator')"#end
}`
```
6. Choose **Save**.
You can now test your `GET` method to verify that it has been
properly set up to invoke the Lambda function and pass the original output through the integration response without mapping.
###### To test the `GET` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For the **Path**, do the following:
1. For **operand1**, enter `1`.
2. For **operand2**, enter `1`.
3. For **operator**, enter `+`.
4. Choose **Test**.
5. The result should look like this:
![Test the GET method in the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/aws_proxy_lambda_calc_get_method_test_path_parm_new_console.png)
Next, you model the data structure of the method response
payload after the `result` schema.
By default, the method response body is assigned an empty model. This will
cause the integration response body to be passed through without mapping.
However, when you generate an SDK for one of the strongly-type languages, such
as Java or Objective-C, your SDK users will receive an empty object as the
result. To ensure that both the REST client and SDK clients receive the desired
result, you must model the response data using a predefined schema. Here you'll
define a model for the method response body and to construct a mapping template
to translate the integration response body into the method response body.
###### To create a method response
1. On the **Method response** tab, under **Response 200**, choose **Edit**.
2. Under **Response body**, choose **Add model**.
3. For **Content type**, enter `application/json`.
4. For **Model**, select **result**.
5. Choose **Save**.
Setting the model for the method response body ensures that the response data
will be cast into the `result` object of a given SDK. To make sure
that the integration response data is mapped accordingly, you'll need a mapping
template.
###### To create a mapping template
1. On the **Integration response** tab, under **Default - Response**, choose **Edit**.
2. Choose **Mapping templates**.
3. For **Content type**, enter `application/json`.
4. For **Template body**, enter the following code:
```
`#set($inputRoot = $input.path('$'))
{
"input" : {
"a" : $inputRoot.a,
"b" : $inputRoot.b,
"op" : "$inputRoot.op"
},
"output" : {
"c" : $inputRoot.c
}
}`
```
5. Choose **Save**.
###### To test the mapping template
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For the **Path**, do the following:
1. For **operand1**, enter `1`.
2. For **operand2**, enter `2`.
3. For **operator**, enter `+`.
4. Choose **Test**.
5. The result will look like the following:
```
`{
"input": {
"a": 1,
"b": 2,
"op": "+"
},
"output": {
"c": 3
}
}`
```
At this point, you can only call the API using the **Test** feature
in the API Gateway console. To make it available to clients, you'll need to deploy your API.
Always be sure to redeploy your API whenever you add, modify, or delete a resource
or method, update a data mapping, or update stage settings. Otherwise, new features
or updates will not be available to clients of your API.
as follows:
###### To deploy the API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `Prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
6. (Optional) Under **Stage details**, for **Invoke URL**, you can choose the copy icon to copy your API's invoke URL. You can use this with tools such as [Postman](https://www.postman.com) and [cURL](https://curl.se/) to test your API.
###### Note
Always redeploy your API whenever you add, modify, or delete a resource
or method, update a data mapping, or update stage settings. Otherwise, new features
or updates will not be available to clients of your API.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with an AWS integration
OpenAPI
definitions of a sample API for a Lambda function
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.