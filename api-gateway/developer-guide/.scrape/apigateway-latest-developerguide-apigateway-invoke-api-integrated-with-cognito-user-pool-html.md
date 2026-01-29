---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-invoke-api-integrated-with-cognito-user-pool.html
title: Call a REST API
word_count: 261
filtered: true
elements_removed: 0
density_score: 0.93
---

Call a REST API integrated with an Amazon Cognito user pool - Amazon API Gateway
Call a REST API integrated with an Amazon Cognito user pool - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-invoke-api-integrated-with-cognito-user-pool)
# Call a REST API
integrated with an Amazon Cognito user pool
To call a method with a user pool authorizer configured, the client must do the
following:
* Enable the user to sign up with the user pool.
* Enable the user to sign in to the user pool.
* Obtain an [identity or access token](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html) of the signed-in user from the user pool.
* Include the token in the `Authorization` header (or
another header you specified when you created the authorizer).
You can use [AWS Amplify]() to perform these tasks.
See [Integrating Amazon Cognito With Web and Mobile Apps](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-integrate-apps.html)
for more information.
* For Android, see [Getting Started with Amplify for Android](https://docs.amplify.aws/android/build-a-backend/auth/).
* To use iOS see [Getting started with Amplify for iOS](https://docs.amplify.aws/swift/build-a-backend/auth/).
* To use JavaScript, see [Getting Started with Amplify for Javascript](https://docs.amplify.aws/javascript/build-a-backend/auth/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrate a REST API with an Amazon Cognito user
pool
Configure cross-account Amazon Cognito authorizer for a REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.