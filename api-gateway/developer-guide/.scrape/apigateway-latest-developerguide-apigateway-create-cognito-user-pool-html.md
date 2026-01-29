---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-create-cognito-user-pool.html
title: Create an Amazon Cognito user pool for a REST API
word_count: 388
filtered: true
elements_removed: 0
density_score: 0.82
---

Create an Amazon Cognito user pool for a REST API - Amazon API Gateway
Create an Amazon Cognito user pool for a REST API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-create-cognito-user-pool)
# Create an Amazon Cognito user pool for a REST API
Before integrating your API with a user pool, you must create the user pool in Amazon Cognito. Your user pool
configuration must follow all
[resource quotas for Amazon Cognito](https://docs.aws.amazon.com/cognito/latest/developerguide/limits.html). All user-defined Amazon Cognito variables such as groups, users,
and roles should use only alphanumeric characters. For instructions on how to create a user pool, see [Tutorial: Creating a user pool](https://docs.aws.amazon.com/cognito/latest/developerguide/tutorial-create-user-pool.html) in the *Amazon Cognito
Developer Guide*.
Note the user pool ID, client ID, and any client secret. The client must provide
them to Amazon Cognito for the user to register with the user pool, to sign in to the user
pool, and to obtain an identity or access token to be included in requests to call
API methods that are configured with the user pool. Also, you must specify the user
pool name when you configure the user pool as an authorizer in API Gateway, as described
next.
If you're using access tokens to authorize API method calls, be sure to configure the
app integration with the user pool to set up the custom scopes that you want on a given
resource server. For more information about using tokens with Amazon Cognito user pools, see [Using Tokens with User Pools](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html). For more information about resource servers, see [Defining Resource
Servers for Your User Pool](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-define-resource-servers.html).
Note the configured resource server identifiers and custom scope names. You need them
to construct the access scope full names for **OAuth Scopes**,
which
is used by the `COGNITO\_USER\_POOLS` authorizer.
![Amazon Cognito user pool resource servers and scopes](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/cognito-user-pool-custom-scopes-new-console.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use Amazon Cognito user pool as authorizer for a REST API
Integrate a REST API with an Amazon Cognito user
pool
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.