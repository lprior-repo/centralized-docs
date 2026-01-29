---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-api-step-by-step.html
title: Tutorial: Create a REST API with an HTTP non-proxy
word_count: 1821
filtered: true
elements_removed: 0
density_score: 0.86
---

Tutorial: Create a REST API with an HTTP non-proxy integration - Amazon API Gateway
Tutorial: Create a REST API with an HTTP non-proxy integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-create-api-step-by-step)
[Create an API with HTTP
custom integration](#api-gateway-create-resource-and-methods)[(Optional) Map request parameters](#api-gateway-create-resources-and-methods-next-steps)
# Tutorial: Create a REST API with an HTTP non-proxy
integration
In this tutorial, you create an API from scratch using the Amazon API Gateway console. You can
think of the console as an API design studio and use it to scope the API features, to
experiment with its behaviors, to build the API, and to deploy your API in stages.
###### Topics
* [Create an API with HTTP
custom integration](#api-gateway-create-resource-and-methods)
* [(Optional) Map request parameters](#api-gateway-create-resources-and-methods-next-steps)
## Create an API with HTTP
custom integration
This section walks you through the steps to create resources, expose methods on a
resource, configure a method to achieve the desired API behaviors, and to test and
deploy the API.
In this step, you create an empty API. In the following steps you create resources and methods to connect your
API to the `http://petstore-demo-endpoint.execute-api.com/petstore/pets` endpoint, using a non-proxy HTTP integration.
###### To create an API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. If this is your first time using API Gateway, you see a page that introduces you to
the features of the service. Under **REST API**, choose **Build**. When the
**Create Example API** popup appears, choose
**OK**.
If this is not your first time using API Gateway, choose **Create
API**. Under **REST API**, choose **Build**.
3. For **API name**, enter `HTTPNonProxyAPI`.
4. (Optional) For **Description**, enter a description.
5. Keep **API endpoint type** set to **Regional**.
6. For **IP address type**, select **IPv4**.
7. Choose **Create API**.
The **Resources** tree
shows the root resource (`/`) without any methods. In this exercise,
we will build the API with the HTTP custom integration of the PetStore website
(http://petstore-demo-endpoint.execute-api.com/petstore/pets.) For illustration
purposes, we will create a `/pets` resource as a child of the root
and expose a GET method on this resource for a client to retrieve a list of
available Pets items from the PetStore website.
###### To create a /pets resource
1. Choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. Keep **Resource path** as `/`.
4. For **Resource name**, enter `pets`.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
In this step, you create a `GET` method on the **/pets** resource. The
`GET` method is integrated with the
`http://petstore-demo-endpoint.execute-api.com/petstore/pets` website. Other options for an API
method include the following:
* **POST**, primarily used to create child
resources.
* **PUT**, primarily used to update existing
resources (and, although not recommended, can be used to create
child resources).
* **DELETE**, used to delete resources.
* **PATCH**, used to update resources.
* **HEAD**, primarily used in testing scenarios. It
is the same as GET but does not return the resource
representation.
* **OPTIONS**, which can be used by callers to get
information about available communication options for the target
service.
For the integration request's **HTTP method**, you must
choose one supported by the backend. For `HTTP` or `Mock
integration`, it makes sense that the method request and the
integration request use the same HTTP verb. For other integration types the
method request will likely use an HTTP verb different from the integration
request. For example, to call a Lambda function, the integration request must
use `POST` to invoke the function, whereas the method request may
use any HTTP verb depending on the logic of the Lambda function.
###### To create a `GET` method on the **/pets** resource
1. Select the **/pets** resource.
2. Choose **Create method**.
3. For **Method type**, select **GET**.
4. For **Integration type**, select **HTTP integration**.
5. Keep **HTTP proxy integration** turned off.
6. For **HTTP method**, select **GET**.
7. For **Endpoint URL**, enter `http://petstore-demo-endpoint.execute-api.com/petstore/pets`.
The PetStore website allows you to retrieve a list of `Pet` items by the pet type, such as "Dog" or "Cat", on a given page.
8. For **Content handling**, select **Passthrough**.
9. Choose **URL query string parameters**.
The PetStore website uses the `type` and `page` query string parameters to accept an
input. You add query string parameters to the method request and map them into corresponding query string
parameters of the integration request.
10. To add the query string parameters, do the following:
1. Choose **Add query string**.
2. For **Name**, enter `type`
3. Keep **Required** and **Caching** turned off.
Repeat the previous steps to create an additional query string with the name `page`.
4. Choose **Create method**.
The client can now supply a pet type and a page number as query string
parameters when submitting a request. These input parameters must be mapped into
the integration's query string parameters to forward the input values to our
PetStore website in the backend.
###### To map input parameters to the Integration request
1. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
2. Choose **URL query string parameters**, and then do the following:
1. Choose **Add query string parameter**.
2. For **Name**, enter `type`.
3. For **Mapped from**, enter `method.request.querystring.type`
4. Keep **Caching** turned off.
5. Choose **Add query string parameter**.
6. For **Name**, enter `page`.
7. For **Mapped from**, enter `method.request.querystring.page`
8. Keep **Caching** turned off.
9. Choose **Save**.
###### To test the API
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Query strings**, enter `type=Dog&amp;page=2`.
3. Choose **Test**.
The result is similar to the following:
![Test-invoke GET on pets method result](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-create-api-step-by-step-test-invoke-get-on-pets-result-new-console.png)
Now that the test is successful, we can deploy the API to make it publicly
available.
4. Choose **Deploy API**.
5. For **Stage**, select **New stage**.
6. For **Stage name**, enter `Prod`.
7. (Optional) For **Description**, enter a description.
8. Choose **Deploy**.
9. (Optional) Under **Stage details**, for **Invoke URL**, you can choose the copy icon to copy your API's invoke URL. You can use this with tools such as [Postman](https://www.postman.com) and [cURL](https://curl.se/) to test your API.
If you use an SDK to create a client, you can call the methods exposed by the
SDK to sign the request. For implementation details, see the [AWS SDK](https://aws.amazon.com/developer/tools/) of your choosing.
###### Note
When changes are made to your API, you must redeploy the API to make the
new or updated features available before invoking the request URL again.
### Map request parameters for an API Gateway API
This tutorial shows how to create a path parameter of `{petId}` on the API's method request
to specify an item ID, map it to the `{id}` path parameter in the
integration request URL, and send the request to the HTTP endpoint.
###### Note
If you enter the incorrect case of a letter, such as lowercase letter instead of an uppercase letter, this will cause
errors later in the walkthrough.
#### Step 1: Create
resources
In this step, you create a resource with a path parameter {petId}.
###### To create the {petId} resource
1. Select the **/pets** resource, and then choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. For **Resource path**, select **/pets/**.
4. For **Resource name**, enter `{petId}`.
Use the curly braces
(`{ }`) around `petId` so that
**/pets/{petId}**
is displayed.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
#### Step 2: Create and test the
methods
In this step, you create a `GET` method with a `{petId}` path parameter.
###### To set up GET method
1. Select the **/{petId}** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **HTTP integration**.
4. Keep **HTTP proxy integration** turned off.
5. For **HTTP method**, select **GET**.
6. For **Endpoint URL**, enter `http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}`
7. For **Content handling**, select **Passthrough**.
8. Keep the **Default timeout** turned on.
9. Choose **Create method**.
Now you map the `{petId}` path parameter that you just created to the `{id}` path parameter
in the HTTP endpoint URL of the integration request. The HTTP endpoint URL was
`http://petstore-demo-endpoint.execute-api.com/petstore/pets/{id}`.
###### To map the `{petId}` path parameter
1. On the **Integration request** tab, under **Integration request
settings**, choose **Edit**.
2. Choose **URL path parameters**.
3. API Gateway creates a path parameter for the integration request named **petId**, however this
path parameter isn't valid for the HTTP endpoint URL that you set as the backend integration. The HTTP
endpoint uses `{id}` as the path parameter. For **Name**, delete **petId** and enter
`id`.
This maps the method request's path parameter of `petId` to the
integration request's path parameter of `id`.
4. Choose **Save**.
Now you test the method.
###### To test the method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. Under **Path** for **petId**, enter `4`.
3. Choose **Test**.
If successful, **Response body** displays the following:
```
`{
"id": 4,
"type": "bird",
"price": 999.99
}`
```
#### Step 3: Deploy the API
In this step, you deploy the API so that you can begin calling it outside of the API Gateway
console.
###### To deploy the API
1. Choose **Deploy API**.
2. For **Stage**, select **Prod**.
3. (Optional) For **Description**, enter a description.
4. Choose **Deploy**.
#### Step 4: Test the API
In this step, you go outside of the API Gateway console and use your API to access the HTTP
endpoint.
1. In the main navigation pane, choose **Stage**.
2. Under **Stage details**, choose the copy icon to copy your API's invoke URL.
It should look something like
this:
```
`https://`my-api-id`.execute-api.`region-id`.amazonaws.com/prod`
```
3. Enter this URL in the address box of a new browser tab and append `/pets/4` to the URL before you submit your request.
4. The browser will return the following:
```
`{
"id": 4,
"type": "bird",
"price": 999.99
}`
```
#### Next steps
You can further customize your API by turning on request validation, transforming data, or creating custom gateway responses.
To explore more ways to customize your API, see the following tutorials:
* For more information about request validation, see [Set up basic request validation
in API Gateway](./api-gateway-request-validation-set-up.html).
* For information about how to transform request and response payloads, see [Tutorial: Modify the integration request and response
for integrations to AWS services](./set-up-data-transformations-in-api-gateway.html).
* For information about how to create custom gateway responses see, [Set up a gateway response
for a REST API using the API Gateway console](./set-up-gateway-response-using-the-console.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with an HTTP
proxy integration
Tutorial: Create a REST API with a private integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.