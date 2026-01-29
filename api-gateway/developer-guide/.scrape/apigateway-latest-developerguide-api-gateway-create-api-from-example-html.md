---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-api-from-example.html
title: Tutorial: Create a REST API by importing an
word_count: 1489
filtered: true
elements_removed: 0
density_score: 0.78
---

Tutorial: Create a REST API by importing an example - Amazon API Gateway
Tutorial: Create a REST API by importing an example - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-create-api-from-example)
# Tutorial: Create a REST API by importing an
example
You can use the Amazon API Gateway console to create and test a simple REST API with the HTTP
integration for a PetStore website. The API definition is preconfigured as a OpenAPI 2.0
file. After loading the API definition into API Gateway, you can use the API Gateway console to examine
the API's basic structure or simply deploy and test the API.
The PetStore example API supports the following methods for a client to access the HTTP
backend website of `http://petstore-demo-endpoint.execute-api.com/petstore/pets`.
###### Note
This tutorial uses an HTTP endpoint as an example. When you create your own APIs, we recommend you use HTTPS
endpoints for your HTTP integrations.
* `GET /`: for read access of the API's root resource that is not
integrated with any backend endpoint. API Gateway responds with an overview of the
PetStore website. This is an example of the `MOCK` integration
type.
* `GET /pets`: for read access to the API's `/pets` resource
that is integrated with the like-named backend `/pets` resource. The
backend returns a page of available pets in the PetStore. This is an example of the
`HTTP` integration type. The URL of the integration endpoint is
`http://petstore-demo-endpoint.execute-api.com/petstore/pets`.
* `POST /pets`: for write access to the API's `/pets` resource
that is integrated with the backend `/petstore/pets` resource. Upon
receiving a correct request, the backend adds the specified pet to the PetStore and
returns the result to the caller. The integration is also `HTTP`.
* `GET /pets/{petId}`: for read access to a pet as identified by a
`petId` value as specified as a path variable of the incoming request
URL. This method also has the `HTTP` integration type. The backend
returns the specified pet found in the PetStore. The URL of the backend HTTP
endpoint is
`http://petstore-demo-endpoint.execute-api.com/petstore/pets/`n``,
where `n` is an integer as the identifier of the queried pet.
The API supports CORS access via the `OPTIONS` methods of the
`MOCK` integration type. API Gateway returns the required headers supporting CORS
access.
The following procedure walks you through the steps to create and test an API from an
example using the API Gateway Console.
###### To import, build, and test the example API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Do one of the following:
* To create your first API, for **REST API**, choose **Build**.
* If you've created an API before, choose **Create API**, and then choose
**Build** for **REST API**.
* Under **Create REST API**, choose **Example
API** and then choose **Create API** to create the example
API.
![Example REST API in the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-by-importing-example-new-console.png)
You can scroll down the OpenAPI definition for details of this example API before
choosing **Create API**.
* In the main navigation pane, choose **Resources**. The newly created API is shown as follows:
![The example API after importing it into the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-by-importing-example-result-new-console.png)
The **Resources** pane shows the structure of the created API as a tree of nodes. API
methods defined on each resource are edges of the tree. When a resource is selected, all of its methods are
listed in the **Methods** table on the right. Displayed with each method is the method type,
integration type, authorization type, and API key requirement.
* To view the details of a method, to modify its set-up, or to test the method
invocation, choose the method name from either the method list or the resource tree.
Here, we choose the `POST /pets` method as an illustration:
![The POST /pets method for the example API in the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-by-importing-example-post-method-execution-new-console.png)
The resulting **Method execution** pane presents a logical view
of the chosen (`POST /pets`) method's structure and behaviors.
The **Method request** and **Method response** represent the API's interface with the frontend, and the **Integration request** and
**Integration response** represent the API's interface with the backend.
A
client uses the API to access a backend feature through the **Method
request**. API Gateway translates the client request, if necessary, into the
form acceptable to the backend in **Integration request** before
forwarding the incoming request to the backend. The transformed request is known as
the integration request. Similarly, the backend returns the response to API Gateway in
**Integration response**. API Gateway then routes it to
**Method Response** before sending it to the client. Again, if
necessary, API Gateway can map the backend response data to a form expected by the client.
For the `POST` method on an API resource, the method request payload
can be passed through to the integration request without modification, if the method
request's payload is of the same format as the integration request's payload.
The `GET /` method request uses the `MOCK` integration type
and is not tied to any real backend endpoint. The corresponding
**Integration response** is set up to return a static HTML
page. When the method is called, the API Gateway simply accepts the request and
immediately returns the configured integration response to the client by way of
**Method response**. You can use the mock integration to test
an API without requiring a backend endpoint. You can also use it to serve a local
response, generated from a response body-mapping template.
As an API developer, you control the behaviors of your API's frontend interactions
by configuring the method request and a method response. You control the behaviors
of your API's backend interactions by setting up the integration request and
integration response. These involve data mappings between a method and its
corresponding integration. For now, we focus on
testing the API to provide an end-to-end user experience.
* Select the **Test** tab. You might need to choose the right arrow button to show the tab.
* For example, to test the `POST /pets` method, enter
the following `{"type": "dog","price": 249.99}` payload into the **Request body**, and then choose
**Test**.
![Test the POST method in the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-by-importing-example-post-method-test-new-console.png)
The input specifies the attributes of the pet that we want to add to the list of
pets on the PetStore website.
* The results display as follows:
![The result of testing the POST method in the API Gateway console.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-by-importing-example-post-method-test-result-new-console.png)
The **Log** entry of the output shows the state changes from
the method request to the integration request, and from the integration response to
the method response. This can be useful for troubleshooting any mapping errors that
cause the request to fail. In this example, no mapping is applied: the method
request payload is passed through the integration request to the backend and,
similarly, the backend response is passed through the integration response to the
method response.
To test the API using a client other than the API Gateway test-invoke-request feature,
you must first deploy the API to a stage.
* To deploy the sample API,
choose **Deploy API**.
![Use the deploy button to deploy your API, so API callers can invoke your API.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-by-importing-example-deploy-api-new-console.png)
* For **Stage**, select **New stage**, and then enter `test`.
* (Optional) For **Description**, enter a description.
* Choose **Deploy**.
* In the resulting **Stages** pane, under **Stage details**, the **Invoke
URL** displays the URL to invoke the API's `GET /` method
request.
![After you create your REST API, the console shows your API's invoke URL.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/getting-started-rest-invoke-url.png)
* Choose the copy icon to copy your API's invoke URL, and then enter your API's invoke URL in a web browser. A successful
response return the result, generated from the mapping template in the integration
response.
* In the **Stages** navigation pane, expand the
**test** stage, select **GET** on
`/pets/{petId}`, and then copy the **Invoke URL**
value of
`https://`api-id`.execute-api.`region`.amazonaws.com/test/pets/{petId}`.
`{petId}` stands for a path variable.
Paste the **Invoke URL** value (obtained in the previous step)
into the address bar of a browser, replacing `{petId}` by, for example,
`1`, and press Enter to submit the request. A 200 OK response should
return with the following JSON payload:
```
`{
"id": 1,
"type": "dog",
"price": 249.99
}`
```
Invoking the API method as shown is possible because its
**Authorization** type is set to `NONE`. If the
`AWS\_IAM` authorization were used, you would sign the request using
the
[Signature Version 4](https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html) (SigV4) or [Signature Version 4a](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_sigv.html#how-sigv4a-works) (SigV4a)
protocols. For an example of such a request, see [Tutorial: Create a REST API with an HTTP non-proxy
integration](./api-gateway-create-api-step-by-step.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with a cross-account Lambda proxy integration
Choose an HTTP integration tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.