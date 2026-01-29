---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-api-as-simple-proxy-for-http.html
title: Tutorial: Create a REST API with an HTTP
word_count: 1298
filtered: true
elements_removed: 0
density_score: 0.82
---

Tutorial: Create a REST API with an HTTP proxy integration - Amazon API Gateway
Tutorial: Create a REST API with an HTTP proxy integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-create-api-as-simple-proxy-for-http)
[Create an API
with HTTP proxy integration using the API Gateway console](#api-gateway-create-api-as-simple-proxy-for-http-build)[Test an API with
HTTP proxy integration](#api-gateway-create-api-as-simple-proxy-for-http-test)
# Tutorial: Create a REST API with an HTTP
proxy integration
HTTP proxy integration is a simple, powerful, and versatile mechanism to build an API that
allows a web application to access multiple resources or features of the integrated HTTP
endpoint, for example the entire website, with a streamlined setup of a single API method.
In HTTP proxy integration, API Gateway passes the client-submitted method request to the backend.
The request data that is passed through includes the request headers, query string
parameters, URL path variables, and payload. The backend HTTP endpoint or the web server
parses the incoming request data to determine the response that it returns. HTTP proxy
integration makes the client and backend interact directly with no intervention from API Gateway
after the API method is set up, except for known issues such as unsupported characters,
which are listed in [Amazon API Gateway important notes](./api-gateway-known-issues.html).
With the all-encompassing proxy resource `{proxy+}`, and the catch-all
`ANY` verb for the HTTP method, you can use an HTTP proxy integration to
create an API of a single API method. The method exposes the entire set of the publicly
accessible HTTP resources and operations of a website. When the backend web server opens
more resources for public access, the client can use these new resources with the same API
setup. To enable this, the website developer must communicate clearly to the client
developer what the new resources are and what operations are applicable for each of
them.
As a quick introduction, the following tutorial demonstrates the HTTP proxy integration.
In the tutorial, we create an API using the API Gateway console to integrate with the PetStore
website through a generic proxy resource `{proxy+}`, and create the HTTP method
placeholder of `ANY`.
###### Topics
* [Create an API
with HTTP proxy integration using the API Gateway console](#api-gateway-create-api-as-simple-proxy-for-http-build)
* [Test an API with
HTTP proxy integration](#api-gateway-create-api-as-simple-proxy-for-http-test)
## Create an API
with HTTP proxy integration using the API Gateway console
The following procedure walks you through the steps to create and test an API with a
proxy resource for an HTTP backend using the API Gateway console. The HTTP backend is the
`PetStore` website
(`http://petstore-demo-endpoint.execute-api.com/petstore/pets`) from
[Tutorial: Create a REST API with an HTTP non-proxy
integration](./api-gateway-create-api-step-by-step.html), in which screenshots are used
as visual aids to illustrate the API Gateway UI elements. If you are new to using the API Gateway
console to create an API, you may want to follow that section first.
###### To create an API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. If this is your first time using API Gateway, you see a page that introduces you to
the features of the service. Under **REST API**, choose **Build**. When the
**Create Example API** popup appears, choose
**OK**.
If this is not your first time using API Gateway, choose **Create
API**. Under **REST API**, choose **Build**.
3. For **API name**, enter `HTTPProxyAPI`.
4. (Optional) For **Description**, enter a description.
5. Keep **API endpoint type** set to **Regional**.
6. For **IP address type**, select **IPv4**.
7. Choose **Create API**.
In this step, you create a proxy resource path of `{proxy+}`. This is the
placeholder of any of the backend endpoints under
`http://petstore-demo-endpoint.execute-api.com/`. For example, it can be
`petstore`, `petstore/pets`, and
`petstore/pets/{petId}`. API Gateway creates the `ANY` method when you create the `{proxy+}` resource and serves as a
placeholder for any of the supported HTTP verbs at run time.
###### To create a **/{proxy+}** resource
1. Choose your API.
2. In the main navigation pane, choose **Resources**.
3. Choose **Create resource**.
4. Turn on **Proxy resource**.
5. Keep **Resource path** as `/`.
6. For **Resource name**, enter `{proxy+}`.
7. Keep **CORS (Cross Origin Resource Sharing)** turned off.
8. Choose **Create resource**.
![Create a child resource.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-simple-proxy-create-proxy-resource-new-console.png)
In this step, you integrate the `ANY` method with a backend HTTP endpoint, using a proxy
integration. In a proxy integration, API Gateway passes the client-submitted method request to the backend with no
intervention from API Gateway.
###### To create an `ANY` method
1. Choose the **/{proxy+}** resource.
2. Choose the **ANY** method.
3. Under the warning symbol, choose **Edit integration**. You cannot deploy an API that has a method without an integration.
4. For **Integration type**, select **HTTP**.
5. Turn on **HTTP proxy integration**.
6. For **HTTP method**, select **ANY**.
7. For **Endpoint URL**, enter `http://petstore-demo-endpoint.execute-api.com/{proxy}`.
8. Choose **Save**.
## Test an API with
HTTP proxy integration
Whether a particular client request succeeds depends on the following:
* If the backend has made the corresponding backend endpoint available and, if
so, has granted the required access permissions.
* If the client supplies the correct input.
For example, the PetStore API used here does not expose the `/petstore`
resource. As such, you get a `404 Resource Not Found` response containing the
error message of `Cannot GET /petstore`.
In addition, the client must be able to handle the output format of the backend in
order to parse the result correctly. API Gateway does not mediate to facilitate interactions
between the client and backend.
###### To test an API integrated with the PetStore website using HTTP proxy integration
through the proxy resource
1. Select the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Method type**, select `GET`.
3. For **Path**, under **proxy**, enter `petstore/pets`.
4. For **Query strings**, enter `type=fish`.
5. Choose **Test**.
![Use the test feature to test a method.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/api-gateway-simple-proxy-petstore-call-proxy-resource-new-console.png)
Because the backend website supports the `GET
/petstore/pets?type=fish` request, it returns a successful response
similar to the following:
```
`[
{
"id": 1,
"type": "fish",
"price": 249.99
},
{
"id": 2,
"type": "fish",
"price": 124.99
},
{
"id": 3,
"type": "fish",
"price": 0.99
}
]`
```
If you try to call `GET /petstore`, you get a `404`
response with an error message of `Cannot GET /petstore`. This is
because the backend does not support the specified operation. If you call
`GET /petstore/pets/1`, you get a `200 OK` response
with the following payload, because the request is supported by the PetStore
website.
```
`{
"id": 1,
"type": "dog",
"price": 249.99
}`
```
You can also use a browser to test your API. Deploy your API and associate it to a stage to create your API's Invoke URL.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `test`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
Now clients can call your API.
###### To invoke your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. In the main navigation pane, choose **Stage**.
4. Under **Stage details**, choose the copy icon to copy your API's invoke URL.
Enter your API's invoke URL in a web browser.
The full URL should look like
`https://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/`test`/petstore/pets?type=fish`.
Your browser sends a `GET` request to the API.
5. The result should be the same as returned when you use **Test**
in the API Gateway console.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Choose an HTTP integration tutorial
Tutorial: Create a REST API with an HTTP non-proxy integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.