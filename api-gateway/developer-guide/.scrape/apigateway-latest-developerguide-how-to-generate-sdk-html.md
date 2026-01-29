---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-generate-sdk.html
title: Generate SDKs for REST APIs in API Gateway
word_count: 353
filtered: true
elements_removed: 0
density_score: 0.86
---

Generate SDKs for REST APIs in API Gateway - Amazon API Gateway
Generate SDKs for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-generate-sdk)
# Generate SDKs for REST APIs in API Gateway
To call your REST API in a platform- or language-specific way, you must generate the platform- or
language-specific SDK of the API. You generate your SDK after you create, test, and deploy your API to a
stage. Currently, API Gateway supports generating an SDK for an API in Java, JavaScript, Java for Android, Objective-C or
Swift for iOS, and Ruby.
This section explains how to generate an SDK of an API Gateway API. It also demonstrates how to
use the generated SDK in a Java app, a Java for Android app, Objective-C and Swift for iOS
apps, and a JavaScript app.
To facilitate the discussion, we use this API Gateway [API](./simple-calc-lambda-api.html), which exposes this [Simple Calculator](./simple-calc-nodejs-lambda-function.html) Lambda function.
Before proceeding, create or import the API and deploy it at least once in API Gateway. For
instructions, see [Deploy REST APIs in API Gateway](./how-to-deploy-api.html).
###### Topics
* [Simple calculator Lambda function](./simple-calc-nodejs-lambda-function.html)
* [Simple calculator API in API Gateway](./simple-calc-lambda-api.html)
* [Simple calculator API OpenAPI definition](./simple-calc-lambda-api-swagger-definition.html)
* [Generate the Java SDK of an
API in API Gateway](./generate-java-sdk-of-an-api.html)
* [Generate the Android SDK of an API in API Gateway](./generate-android-sdk-of-an-api.html)
* [Generate the iOS SDK of an API in API Gateway](./generate-ios-sdk-of-an-api.html)
* [Generate the JavaScript SDK of a REST API in API Gateway](./generate-javascript-sdk-of-an-api.html)
* [Generate the Ruby SDK of an
API in API Gateway](./generate-ruby-sdk-of-an-api.html)
* [Generate SDKs for an API using AWS CLI commands in API Gateway](./how-to-generate-sdk-cli.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Control
access to API documentation in API Gateway
Simple calculator Lambda function
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.