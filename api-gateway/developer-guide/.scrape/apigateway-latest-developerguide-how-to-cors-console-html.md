---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-cors-console.html
title: Enable CORS on a resource using the API Gateway
word_count: 617
filtered: true
elements_removed: 0
density_score: 0.78
---

Enable CORS on a resource using the API Gateway console - Amazon API Gateway
Enable CORS on a resource using the API Gateway console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-cors-console)
# Enable CORS on a resource using the API Gateway
console
You can use the API Gateway console to enable CORS support for one or all methods on a REST
API resource that you have created. After you enable COR support, set the integration passthrough behavior to `NEVER`. In this case, the method request of an
unmapped content type will be rejected with an HTTP 415 Unsupported Media Type response. For more information, see
[Method request behavior for payloads without mapping
templates for REST APIs in API Gateway](./integration-passthrough-behaviors.html)
###### Important
Resources can contain child resources. Enabling CORS support for a resource and
its methods does not recursively enable it for child resources and their
methods.
###### To enable CORS support on a REST API resource
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an API.
3. Choose a resource under **Resources**.
4. In the **Resource details** section,
choose **Enable CORS**.
![In the Resources pane, choose Enable CORS.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/amazon-api-gateway-new-console-enable-cors.png)
5. In the **Enable CORS** box, do the following:
1. (Optional) If you created a custom gateway response and want to enable CORS support for a response, select a gateway response.
2. Select each method to enable CORS support. The `OPTION` method must have CORS enabled.
If you enable CORS support for an `ANY` method, CORS is enabled for all methods.
3. In the **Access-Control-Allow-Headers** input field,
enter a static string of a comma-separated list of headers that the
client must submit in the actual request of the resource. Use the
console-provided header list of
`'Content-Type,X-Amz-Date,Authorization,X-Api-Key,X-Amz-Security-Token'`
or specify your own headers.
4. Use the console-provided value of `'\*'` as the
**Access-Control-Allow-Origin** header value to
allow access requests from all origins, or specify an origin to be
permitted to access the resource.
5. Choose **Save**.
![Choose which headers are allowed](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/amazon-api-gateway-new-console-enable-cors-resources.png)
###### Important
When applying the above instructions to the `ANY` method in a
proxy integration, any applicable CORS headers will not be set. Instead,
your backend must return the applicable CORS headers, such as
`Access-Control-Allow-Origin`.
After CORS is enabled on the `GET` method, an `OPTIONS` method
is added to the resource, if it is not already there. The `200` response of
the `OPTIONS` method is automatically configured to return the three
`Access-Control-Allow-\*` headers to fulfill preflight handshakes. In
addition, the actual (`GET`) method is also configured by default to return
the `Access-Control-Allow-Origin` header in its 200 response as well. For
other types of responses, you will need to manually configure them to return
`Access-Control-Allow-Origin'` header with '\*' or specific origins, if
you do not want to return the `Cross-origin access` error.
After you enable CORS support on your resource, you must deploy or redeploy the API
for the new settings to take effect. For more information, see [Create a deployment](./set-up-deployments.html#create-deployment).
###### Note
If you cannot enable CORS support on your resource after following the procedure, we recommend that you compare your CORS configuration to the
example API `/pets` resource. To learn how to create the example API, see [Tutorial: Create a REST API by importing an
example](./api-gateway-create-api-from-example.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
CORS
Enable CORS using OpenAPI definition
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.