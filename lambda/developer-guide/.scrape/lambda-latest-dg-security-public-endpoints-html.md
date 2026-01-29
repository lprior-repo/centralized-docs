---
url: https://docs.aws.amazon.com/lambda/latest/dg/security-public-endpoints.html
title: Securing workloads with public endpoints
word_count: 705
filtered: true
elements_removed: 0
density_score: 0.81
---

Securing workloads with public endpoints - AWS Lambda
Securing workloads with public endpoints - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#security-public-endpoints)
[Authentication and authorization](#authentication)[Protecting API endpoints](#api-endpoints)
# Securing workloads with public endpoints
For workloads that are accessible publicly, AWS provides a number of features and services that can help
mitigate certain risks. This section covers authentication and authorization of application users and
protecting API endpoints.
## Authentication and authorization
Authentication relates to identity and authorization refers to actions. Use authentication to control
who can invoke a Lambda function, and then use authorization to control what they can do. For many applications,
IAM is sufficient for managing both control mechanisms.
For applications with external users, such as web or mobile applications, it is common to use
[JSON Web Tokens](https://jwt.io/introduction/) (JWTs) to manage authentication and
authorization. Unlike traditional, server-based password management, JWTs are passed from the client on
every request. They are a cryptographically secure way to verify identity and claims using data passed from
the client. For Lambda-based applications, this allows you to secure every call to each API endpoint without
relying on a central server for authentication.
You can
[
implement JWTs with Amazon Cognito](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html), a user directory service that can handle registration, authentication,
account recovery, and other common account management operations.
[Amplify Framework](https://docs.amplify.aws/start/getting-started/auth/q/integration/react)
provides libraries to simplify integrating this service into your frontend application. You can also
consider third-party partner services like [Auth0](https://auth0.com/).
Given the critical security role of an identity provider service, it’s important to use professional tooling
to safeguard your application. It’s not recommended that you write your own services to handle authentication
or authorization. Any vulnerabilities in custom libraries may have significant implications for the security
of your workload and its data.
## Protecting API endpoints
For serverless applications, the preferred way to serve a backend application publicly is to use Amazon API Gateway.
This can help you protect an API from malicious users or spikes in traffic.
API Gateway offers two endpoint types for serverless developers:
[REST APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-rest-api.html)
and [HTTP APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api.html).
Both support [
authorization using AWS Lambda](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-use-lambda-authorizer.html), IAM, or Amazon Cognito. When using IAM or Amazon Cognito, incoming requests are
evaluated and if they are missing a required token or contain invalid authentication, the request is rejected.
You are not charged for these requests and they do not count towards any throttling quotas.
Unauthenticated API routes may be accessed by anyone on the public internet so it’s recommended that you
limit the use of unauthenticated APIs. If you must use unauthenticated APIs, it’s important to protect these
against common risks, such as [
denial-of-service](https://en.wikipedia.org/wiki/Denial-of-service_attack) (DoS) attacks.
[
Applying AWS WAF](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-control-access-aws-waf.html) to these APIs can help protect your application from SQL injection and cross-site
scripting (XSS) attacks. API Gateway also implements
[
throttling](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-request-throttling.html) at the AWS account-level and per-client level when API keys are used.
In many cases, the functionality provided by unauthenticated API can be achieved with an alternative approach.
For example, a web application may provide a list of customer retail stores from a DynamoDB table to users who are
not logged in. This request may originate from a frontend web application or from any other source that calls the
URL endpoint. This diagram compares three solutions:
![security ops figure 5](https://docs.aws.amazon.com/images/lambda/latest/dg/images/security-ops-figure-5.png)
1. This unauthenticated API can be called by anyone on the internet. In a denial of service attack,
it’s possible to exhaust API throttling limits, Lambda concurrency, or DynamoDB provisioned read capacity
on an underlying table.
2. A CloudFront distribution in front of the API endpoint with an appropriate
[
time-to-live](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html) (TTL) configuration would absorb most of the traffic in a DoS attack, without
changing the underlying solution for fetching the data.
3. Alternatively, for static data that rarely changes, the CloudFront distribution could serve the data'
from an Amazon S3 bucket.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Infrastructure security
Code signing
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.