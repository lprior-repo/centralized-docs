---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/call-api-with-api-gateway-lambda-authorization.html
title: Call an API with an API Gateway Lambda authorizer
word_count: 488
filtered: true
elements_removed: 0
density_score: 0.83
---

Call an API with an API Gateway Lambda authorizer - Amazon API Gateway
Call an API with an API Gateway Lambda authorizer - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#call-api-with-api-gateway-lambda-authorization)
# Call an API with an API Gateway Lambda authorizer
Having configured the Lambda authorizer (formerly known as the custom authorizer) and deployed the API, you
should test the API with the Lambda authorizer enabled. For this, you need a REST client, such as cURL or [Postman](https://www.postman.com/). For the following examples, we use Postman.
###### Note
When calling an authorizer-enabled method, API Gateway does not log the call to CloudWatch if the required token for
the `TOKEN` authorizer is not set, is null, or is invalidated by the specified **Token
validation expression**. Similarly, API Gateway does not log the call to CloudWatch if any of the required
identity sources for the `REQUEST` authorizer are not set, are null, or are empty.
In the following, we show how to use Postman to call or test an API with a Lambda `TOKEN`
authorizer. The method can be applied to calling an API with a Lambda `REQUEST` authorizer, if you
specify the required path, header, or query string parameters explicitly.
###### To call an API with the custom `TOKEN` authorizer
1. Open **Postman**, choose the **GET** method, and paste the API's
**Invoke URL** into the adjacent URL field.
Add the Lambda authorization token header and set the value to `allow`. Choose
**Send**.
![Call API with Lambda authorization allow token](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/custom-auth-call-api-with-allow-token.png)
The response shows that the API Gateway Lambda authorizer returns a **200 OK** response and
successfully authorizes the call to access the HTTP endpoint (http://httpbin.org/get) integrated with the
method.
2. Still in Postman, change the Lambda authorization token header value to `deny`. Choose
**Send**.
![Call API with Lambda authorization deny token](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/custom-auth-call-api-with-deny-token.png)
The response shows that the API Gateway Lambda authorizer returns a **403 Forbidden** response
without authorizing the call to access the HTTP endpoint.
3. In Postman, change the Lambda authorization token header value to `unauthorized` and choose
**Send**.
![Call API with Lambda authorization unauthorized token](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/custom-auth-call-api-with-unauthorized-token.png)
The response shows that API Gateway returns a **401 Unauthorized** response without
authorizing the call to access the HTTP endpoint.
4. Now, change the Lambda authorization token header value to `fail`. Choose
**Send**.
![Call API with Lambda authorization fail token](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/custom-auth-call-api-with-fail-token.png)
The response shows that API Gateway returns a **500 Internal Server Error** response without
authorizing the call to access the HTTP endpoint.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Output from an API Gateway Lambda authorizer
Configure a cross-account Lambda authorizer
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.