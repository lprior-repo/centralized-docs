---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-request-validation-set-up.html
title: Set up basic request validation
word_count: 3545
filtered: true
elements_removed: 0
density_score: 0.78
---

Set up basic request validation in API Gateway - Amazon API Gateway
Set up basic request validation in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-request-validation-set-up)
[Set up request validation using the
API Gateway console](#api-gateway-request-validation-setup-in-console)[Set up basic request validation using the AWS CLI](#api-gateway-request-validation-setup-cli)[Set up
basic request validation using an OpenAPI definition](#api-gateway-request-validation-setup-importing-swagger)
# Set up basic request validation
in API Gateway
This section shows how to set up request validation for API Gateway using the console, AWS CLI, and an OpenAPI definition.
###### Topics
* [Set up request validation using the
API Gateway console](#api-gateway-request-validation-setup-in-console)
* [Set up basic request validation using the AWS CLI](#api-gateway-request-validation-setup-cli)
* [Set up
basic request validation using an OpenAPI definition](#api-gateway-request-validation-setup-importing-swagger)
## Set up request validation using the
API Gateway console
You can use the API Gateway console to validate a request by selecting one of three validators for an API
request:
* **Validate body**.
* **Validate query string parameters and headers**.
* **Validate body, query string parameters, and headers**.
When you apply one of the validators on an API method, the API Gateway console adds the validator to the API's
[RequestValidators](https://docs.aws.amazon.com/apigateway/latest/api/API_RequestValidator.html) map.
To follow this tutorial, you'll use an CloudFormation template to create an incomplete API Gateway API. This API
has a `/validator` resource with `GET` and `POST` methods. Both methods are
integrated with the `http://petstore-demo-endpoint.execute-api.com/petstore/pets` HTTP endpoint. You
will configure two kinds of request validation:
* In the `GET` method, you will configure request validation for URL query string
parameters.
* In the `POST` method, you will configure request validation for the request body.
This will allow only certain API calls to pass through to the API.
Download and unzip [the app creation template for CloudFormation](samples/request-validation-tutorial-console.zip).
You'll use this template to create an incomplete API. You will finish the
rest of the steps in the API Gateway console.
###### To create an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Choose **Create stack** and then choose **With new resources
(standard)**.
3. For **Specify template**, choose **Upload a template file**.
4. Select the template that you downloaded.
5. Choose **Next**.
6. For **Stack name**, enter `request-validation-tutorial-console` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the resources specified in the template. It can take a few minutes to finish provisioning
your resources. When the status of your CloudFormation stack is **CREATE\_COMPLETE**, you're ready to move
on to the next step.
###### To select your newly created API
1. Select the newly created `request-validation-tutorial-console` stack.
2. Choose **Resources**.
3. Under **Physical ID**, choose your API. This link will direct you to the API Gateway console.
Before you modify the `GET` and `POST` methods, you must create a model.
###### To create a model
1. A model is required to use request validation on the body of an incoming request. To create a model, in
the main navigation pane, choose **Models**.
2. Choose **Create model**.
3. For **Name**, enter `PetStoreModel`.
4. For **Content Type**, enter `application/json`. If no matching content type is found, request validation is not performed. To use the same model regardless of the content type, enter `$default`.
5. For **Description**, enter `My PetStore Model` as the model
description.
6. For **Model schema**, paste the following model into the code editor, and choose
**Create**.
```
`{
"type" : "object",
"required" : [ "name", "price", "type" ],
"properties" : {
"id" : {
"type" : "integer"
},
"type" : {
"type" : "string",
"enum" : [ "dog", "cat", "fish" ]
},
"name" : {
"type" : "string"
},
"price" : {
"type" : "number",
"minimum" : 25.0,
"maximum" : 500.0
}
}
}`
```
For more information about the model, see [Data models for REST APIs](./models-mappings-models.html).
###### To configure request validation for a `GET` method
1. In the main navigation pane, choose **Resources**, and then select the **GET** method.
2. On the **Method request** tab, under **Method request settings**, choose **Edit**.
3. For **Request validator**, select **Validate query string parameters and headers**.
4. Under **URL query string parameters**, do the following:
1. Choose **Add query string**.
2. For **Name**, enter `petType`.
3. Turn on **Required**.
4. Keep **Caching** turned off.
5. Choose **Save**.
6. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
7. Under **URL query string parameters**, do the following:
1. Choose **Add query string**.
2. For **Name**, enter `petType`.
3. For **Mapped from**, enter `method.request.querystring.petType`. This maps the
`petType` to the pet's type.
For more information about data mapping, see [ the data mapping
tutorial](./set-up-data-transformations-in-api-gateway.html#mapping-example-console).
4. Keep **Caching** turned off.
5. Choose **Save**.
###### To test request validation for the `GET` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Query strings**, enter `petType=dog`, and then choose
**Test**.
3. The method test will return `200 OK` and a list of dogs.
For information about how to transform this output data, see the [data mapping tutorial.](./set-up-data-transformations-in-api-gateway.html#mapping-example-console)
4. Remove `petType=dog` and choose **Test**.
5. The method test will return a `400` error with the following error message:
```
`{
"message": "Missing required request parameters: [petType]"
}`
```
###### To configure request validation for the `POST` method
1. In the main navigation pane, choose **Resources**, and then select the **POST** method.
2. On the **Method request** tab, under **Method request settings**, choose **Edit**.
3. For **Request validator**, select **Validate
body**.
4. Under **Request body**, choose **Add model**.
5. For **Content type**, enter `application/json`. If no matching
content type is found, request validation is not performed. To use the same model regardless of the content
type, enter `$default`.
For **Model**, select
**PetStoreModel**.
6. Choose **Save**.
###### To test request validation for a `POST` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Request body** paste the following into the code editor:
```
`{
"id": 2,
"name": "Bella",
"type": "dog",
"price": 400
}`
```
Choose **Test**.
3. The method test will return `200 OK` and a success message.
4. For **Request body** paste the following into the code editor:
```
`{
"id": 2,
"name": "Bella",
"type": "dog",
"price": 4000
}`
```
Choose **Test**.
5. The method test will return a `400` error with the following error message:
```
`{
"message": "Invalid request body"
}`
```
At the bottom of the test logs, the reason for the invalid request body is returned. In this case, the
price of the pet was outside the maximum specified in the model.
###### To delete an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Select your CloudFormation stack.
3. Choose **Delete** and then confirm your choice.
### Next steps
* For information about how to transform output data and perform more data mapping, see the [data mapping tutorial](./set-up-data-transformations-in-api-gateway.html#mapping-example-console).
* Follow the [ Set up basic request validation using the AWS CLI](#api-gateway-request-validation-setup-cli) tutorial, to do similar steps using the AWS CLI.
## Set up basic request validation using the AWS CLI
You can create a validator to set up request validation using the AWS CLI. To follow this tutorial, you'll use an CloudFormation template to create an incomplete API Gateway API.
###### Note
This is not the same CloudFormation template as the console tutorial.
Using a pre-exposed `/validator`resource, you will create `GET` and
`POST` methods. Both methods will be integrated with the
`http://petstore-demo-endpoint.execute-api.com/petstore/pets` HTTP endpoint. You will configure the
following two request validations:
* On the `GET` method, you will create a `params-only` validator to validate URL
query string parameters.
* On the `POST` method, you will create a `body-only` validator to validate the
request body.
This will allow only certain API
calls to pass through to the API.
###### To create an CloudFormation stack
Download and unzip [the app creation template for CloudFormation](samples/request-validation-tutorial-cli.zip).
To complete the following tutorial, you need the [AWS Command Line Interface (AWS CLI) version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
For long commands, an escape character (`\\`) is used to split a command over multiple lines.
###### Note
In Windows, some Bash CLI commands that you commonly use (such as `zip`) are not supported by the operating system's built-in terminals.
To get a Windows-integrated version of Ubuntu and Bash, [install the Windows Subsystem for Linux](https://learn.microsoft.com/en-us/windows/wsl/install).
Example CLI commands in this guide use Linux formatting. Commands which include inline JSON documents must be reformatted if you are using the Windows CLI.
1. Use the following command to create the CloudFormation stack.
```
`aws cloudformation create-stack --stack-name request-validation-tutorial-cli --template-body file://request-validation-tutorial-cli.zip --capabilities CAPABILITY\_NAMED\_IAM `
```
2. CloudFormation provisions the resources specified in the template. It can take a few minutes to finish provisioning
your resources. Use the following command to see the status of your CloudFormation stack.
```
`aws cloudformation describe-stacks --stack-name request-validation-tutorial-cli`
```
3. When the status of your CloudFormation stack is `StackStatus: "CREATE\_COMPLETE"`, use the following
command to retrieve relevant output values for future steps.
```
` aws cloudformation describe-stacks --stack-name request-validation-tutorial-cli --query "Stacks[\*].Outputs[\*].{OutputKey: OutputKey, OutputValue: OutputValue, Description: Description}"`
```
The output values are the following:
* ApiId, which is the ID for the API. For this tutorial, the API ID is `abc123`.
* ResourceId, which is the ID for the validator resource where the `GET` and `POST` methods are exposed. For this tutorial, the Resource ID is `efg456`
###### To create the request validators and import a model
1. A validator is required to use request validation with the AWS CLI. Use the following command to create a
validator that validates only request parameters.
```
`aws apigateway create-request-validator --rest-api-id `abc123` \\
--no-validate-request-body \\
--validate-request-parameters \\
--name params-only`
```
Note the ID of the `params-only` validator.
2. Use the following command to
create a validator that validates only the request body.
```
`aws apigateway create-request-validator --rest-api-id `abc123` \\
--validate-request-body \\
--no-validate-request-parameters \\
--name body-only`
```
Note the ID of the `body-only` validator.
3. A model is required to use request validation on the body of an incoming request. Use the following
command to import a model.
```
`aws apigateway create-model --rest-api-id `abc123` --name PetStoreModel --description 'My PetStore Model' --content-type 'application/json' --schema '{"type": "object", "required" : [ "name", "price", "type" ], "properties" : { "id" : {"type" : "integer"},"type" : {"type" : "string", "enum" : [ "dog", "cat", "fish" ]},"name" : { "type" : "string"},"price" : {"type" : "number","minimum" : 25.0, "maximum" : 500.0}}}}' `
```
If no matching content type is found, request validation is not performed. To use the same model regardless of the content type, specify `$default` as the key.
###### To create the `GET` and `POST` methods
1. Use the following command to add the `GET` HTTP method on the `/validate`
resource. This command creates the `GET`method, adds the `params-only` validator, and
sets the query string `petType` as required.
```
`aws apigateway put-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET \\
--authorization-type "NONE" \\
--request-validator-id `aaa111` \\
--request-parameters "method.request.querystring.petType=true"`
```
Use the following command to add the `POST` HTTP method on the `/validate`
resource. This command creates the `POST`method, adds the `body-only` validator, and
attaches the model to the body-only validator.
```
`aws apigateway put-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--authorization-type "NONE" \\
--request-validator-id `bbb222` \\
--request-models 'application/json'=PetStoreModel`
```
2. Use the following command to set up the `200 OK`
response of the `GET /validate` method.
```
`aws apigateway put-method-response --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET \\
--status-code 200`
```
Use the following command to set up the `200 OK` response of the `POST
/validate` method.
```
`aws apigateway put-method-response --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--status-code 200`
```
3. Use the following command to set up an `Integration` with a specified HTTP endpoint for the
`GET /validation` method.
```
`aws apigateway put-integration --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET \\
--type HTTP \\
--integration-http-method GET \\
--request-parameters '{"integration.request.querystring.type" : "method.request.querystring.petType"}' \\
--uri 'http://petstore-demo-endpoint.execute-api.com/petstore/pets'`
```
Use the following command to set up an `Integration` with a specified HTTP endpoint for
the `POST /validation` method.
```
`aws apigateway put-integration --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--type HTTP \\
--integration-http-method GET \\
--uri 'http://petstore-demo-endpoint.execute-api.com/petstore/pets'`
```
4. Use the following command to set up an integration response for the `GET /validation`
method.
```
`aws apigateway put-integration-response --rest-api-id `abc123` \\
--resource-id `efg456`\\
--http-method GET \\
--status-code 200 \\
--selection-pattern ""`
```
Use the following command to set up an integration response for the `POST /validation`
method.
```
`aws apigateway put-integration-response --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--status-code 200 \\
--selection-pattern ""`
```
###### To test the API
1. To test the `GET` method, which will perform request validation for the query strings, use the following
command:
```
`aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET \\
--path-with-query-string '/validate?petType=dog'`
```
The result will return a `200 OK` and list of dogs.
2. Use the following command to test without including the query string `petType`
```
`aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET`
```
The result will return a `400` error.
3. To test the `POST` method, which will perform request validation for the request body, use the following
command:
```
` aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--body '{"id": 1, "name": "bella", "type": "dog", "price" : 400 }'`
```
The result will return a `200 OK` and a success message.
4. Use the following command to test using an invalid body.
```
` aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--body '{"id": 1, "name": "bella", "type": "dog", "price" : 1000 }'`
```
The result will return a `400` error, as the price of the dog is over the maximum price defined by the model.
###### To delete an CloudFormation stack
* Use the following command to delete your CloudFormation resources.
```
`aws cloudformation delete-stack --stack-name request-validation-tutorial-cli`
```
## Set up
basic request validation using an OpenAPI definition
You can declare a request validator at the API level by specifying a set of the [x-amazon-apigateway-request-validators.requestValidator object](./api-gateway-swagger-extensions-request-validators.requestValidator.html) objects in the [x-amazon-apigateway-request-validators object](./api-gateway-swagger-extensions-request-validators.html) map to select what part of the request
will be validated. In the example OpenAPI definition, there are two validators:
* `all` validator which validates both the
body, using the `RequestBodyModel` data model, and the parameters.
The `RequestBodyModel` data model requires that the input JSON object contains the
`name`, `type`, and `price` properties. The `name` property
can be any string, `type` must be one of the specified enumeration fields (`["dog", "cat",
"fish"]`), and `price` must range between 25 and 500. The `id` parameter is
not required.
* `param-only` which validates only the parameters.
To turn a request validator on all methods of an API, specify an [x-amazon-apigateway-request-validator property](./api-gateway-swagger-extensions-request-validator.html) property at the API level of the OpenAPI
definition. In the example OpenAPI definition, the `all` validator is used on all API methods, unless
otherwise overridden. When using a model to validate the body, if no matching content type is found, request
validation is not performed. To use the same model regardless of the content type, specify `$default`
as the key.
To turn on a request validator on an individual method, specify the `x-amazon-apigateway-request-validator` property at the method level. In the example, OpenAPI
definition, the `param-only` validator overwrites the `all` validator on the
`GET` method.
To import the OpenAPI example into API Gateway, see the following instructions to [Import a Regional API into API Gateway](./import-export-api-endpoints.html) or to
[Import an edge-optimized API into API Gateway](./import-edge-optimized-api.html).
OpenAPI 3.0
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "ReqValidators Sample",
"version" : "1.0.0"
},
"servers" : [ {
"url" : "/{basePath}",
"variables" : {
"basePath" : {
"default" : "/v1"
}
}
} ],
"paths" : {
"/validation" : {
"get" : {
"parameters" : [ {
"name" : "q1",
"in" : "query",
"required" : true,
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"200" : {
"description" : "200 response",
"headers" : {
"test-method-response-header" : {
"schema" : {
"type" : "string"
}
}
},
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/ArrayOfError"
}
}
}
}
},
"x-amazon-apigateway-request-validator" : "params-only",
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "400",
"responseParameters" : {
"method.response.header.test-method-response-header" : "'static value'"
},
"responseTemplates" : {
"application/xml" : "xml 400 response template",
"application/json" : "json 400 response template"
}
},
"2\\\\d{2}" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.querystring.type" : "method.request.querystring.q1"
},
"passthroughBehavior" : "when\_no\_match",
"type" : "http"
}
},
"post" : {
"parameters" : [ {
"name" : "h1",
"in" : "header",
"required" : true,
"schema" : {
"type" : "string"
}
} ],
"requestBody" : {
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/RequestBodyModel"
}
}
},
"required" : true
},
"responses" : {
"200" : {
"description" : "200 response",
"headers" : {
"test-method-response-header" : {
"schema" : {
"type" : "string"
}
}
},
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/ArrayOfError"
}
}
}
}
},
"x-amazon-apigateway-request-validator" : "all",
"x-amazon-apigateway-integration" : {
"httpMethod" : "POST",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "400",
"responseParameters" : {
"method.response.header.test-method-response-header" : "'static value'"
},
"responseTemplates" : {
"application/xml" : "xml 400 response template",
"application/json" : "json 400 response template"
}
},
"2\\\\d{2}" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.header.custom\_h1" : "method.request.header.h1"
},
"passthroughBehavior" : "when\_no\_match",
"type" : "http"
}
}
}
},
"components" : {
"schemas" : {
"RequestBodyModel" : {
"required" : [ "name", "price", "type" ],
"type" : "object",
"properties" : {
"id" : {
"type" : "integer"
},
"type" : {
"type" : "string",
"enum" : [ "dog", "cat", "fish" ]
},
"name" : {
"type" : "string"
},
"price" : {
"maximum" : 500.0,
"minimum" : 25.0,
"type" : "number"
}
}
},
"ArrayOfError" : {
"type" : "array",
"items" : {
"$ref" : "#/components/schemas/Error"
}
},
"Error" : {
"type" : "object"
}
}
},
"x-amazon-apigateway-request-validators" : {
"all" : {
"validateRequestParameters" : true,
"validateRequestBody" : true
},
"params-only" : {
"validateRequestParameters" : true,
"validateRequestBody" : false
}
}
}`
```
OpenAPI 2.0
```
`{
"swagger" : "2.0",
"info" : {
"version" : "1.0.0",
"title" : "ReqValidators Sample"
},
"basePath" : "/v1",
"schemes" : [ "https" ],
"paths" : {
"/validation" : {
"get" : {
"produces" : [ "application/json", "application/xml" ],
"parameters" : [ {
"name" : "q1",
"in" : "query",
"required" : true,
"type" : "string"
} ],
"responses" : {
"200" : {
"description" : "200 response",
"schema" : {
"$ref" : "#/definitions/ArrayOfError"
},
"headers" : {
"test-method-response-header" : {
"type" : "string"
}
}
}
},
"x-amazon-apigateway-request-validator" : "params-only",
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "400",
"responseParameters" : {
"method.response.header.test-method-response-header" : "'static value'"
},
"responseTemplates" : {
"application/xml" : "xml 400 response template",
"application/json" : "json 400 response template"
}
},
"2\\\\d{2}" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.querystring.type" : "method.request.querystring.q1"
},
"passthroughBehavior" : "when\_no\_match",
"type" : "http"
}
},
"post" : {
"consumes" : [ "application/json" ],
"produces" : [ "application/json", "application/xml" ],
"parameters" : [ {
"name" : "h1",
"in" : "header",
"required" : true,
"type" : "string"
}, {
"in" : "body",
"name" : "RequestBodyModel",
"required" : true,
"schema" : {
"$ref" : "#/definitions/RequestBodyModel"
}
} ],
"responses" : {
"200" : {
"description" : "200 response",
"schema" : {
"$ref" : "#/definitions/ArrayOfError"
},
"headers" : {
"test-method-response-header" : {
"type" : "string"
}
}
}
},
"x-amazon-apigateway-request-validator" : "all",
"x-amazon-apigateway-integration" : {
"httpMethod" : "POST",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "400",
"responseParameters" : {
"method.response.header.test-method-response-header" : "'static value'"
},
"responseTemplates" : {
"application/xml" : "xml 400 response template",
"application/json" : "json 400 response template"
}
},
"2\\\\d{2}" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.header.custom\_h1" : "method.request.header.h1"
},
"passthroughBehavior" : "when\_no\_match",
"type" : "http"
}
}
}
},
"definitions" : {
"RequestBodyModel" : {
"type" : "object",
"required" : [ "name", "price", "type" ],
"properties" : {
"id" : {
"type" : "integer"
},
"type" : {
"type" : "string",
"enum" : [ "dog", "cat", "fish" ]
},
"name" : {
"type" : "string"
},
"price" : {
"type" : "number",
"minimum" : 25.0,
"maximum" : 500.0
}
}
},
"ArrayOfError" : {
"type" : "array",
"items" : {
"$ref" : "#/definitions/Error"
}
},
"Error" : {
"type" : "object"
}
},
"x-amazon-apigateway-request-validators" : {
"all" : {
"validateRequestParameters" : true,
"validateRequestBody" : true
},
"params-only" : {
"validateRequestParameters" : true,
"validateRequestBody" : false
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data models for REST APIs
AWS CloudFormation template of a sample API with basic request validation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.