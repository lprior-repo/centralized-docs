---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-cross-account-cognito-authorizer.html
title: Configure cross-account
word_count: 471
filtered: true
elements_removed: 0
density_score: 0.76
---

Configure cross-account Amazon Cognito authorizer for a REST API using the API Gateway console - Amazon API Gateway
Configure cross-account Amazon Cognito authorizer for a REST API using the API Gateway console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-cross-account-cognito-authorizer)
[Create
a cross-account Amazon Cognito authorizer for a REST API](#apigateway-configure-cross-account-cognito-authorizer)
# Configure cross-account
Amazon Cognito authorizer for a REST API using the API Gateway console
You can now also use a Amazon Cognito user pool from a different AWS account as your API
authorizer. The Amazon Cognito
user pool can use bearer token authentication strategies such as OAuth or SAML. This
makes it easy to centrally manage and share a central Amazon Cognito user pool authorizer across
multiple API Gateway APIs.
In this section, we show how to configure a cross-account Amazon Cognito user pool using the
Amazon API Gateway console.
These instructions assume that you already have an API Gateway API in one AWS account and
a Amazon Cognito user pool in another account.
## Create
a cross-account Amazon Cognito authorizer for a REST API
Log in to the Amazon API Gateway console in the account that has your API in it, and then do the following:
1. Create a new API, or select an existing API in API Gateway.
2. In the main navigation pane, choose **Authorizers**.
3. Choose **Create authorizer**.
4. To configure the new authorizer to use a user pool, do the following:
1. For **Authorizer name**, enter a name.
2. For **Authorizer type**, select **Cognito**.
3. For **Cognito user pool**, enter the full ARN for
the user pool that you have in your second account.
###### Note
In the Amazon Cognito console, you can find the ARN for your user pool in the
**Pool ARN** field of the **General
Settings** pane.
4. For **Token source**, enter
`Authorization` as the header name to pass the identity
or access token that's returned by Amazon Cognito when a user signs in
successfully.
5. (Optional) Enter a regular expression in the **Token
validation** field to validate the `aud`
(audience) field of the identity token before the request is authorized
with Amazon Cognito. Note that when using an access token this validation rejects
the request due to the access token not containing the `aud`
field.
6. Choose **Create authorizer**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Call a REST API integrated with a user pool
Create an Amazon Cognito authorizer for a REST API using CloudFormation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.