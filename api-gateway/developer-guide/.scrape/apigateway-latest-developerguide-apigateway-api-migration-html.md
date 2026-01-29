---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-api-migration.html
title: Change a public or private API endpoint type in
word_count: 856
filtered: true
elements_removed: 0
density_score: 0.85
---

Change a public or private API endpoint type in API Gateway - Amazon API Gateway
Change a public or private API endpoint type in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-api-migration)
[Use the API Gateway console to change an API
endpoint type](#migrate-api-using-console)[Use the AWS CLI to change an API endpoint
type](#migrate-api-using-aws-cli)
# Change a public or private API endpoint type in
API Gateway
Changing an API endpoint type requires you to update the API's configuration. You can change an existing API
type using the API Gateway console, the AWS CLI, or an AWS SDK for API Gateway. The endpoint type cannot be changed again until the
current change is completed, but your API will be available.
The following endpoint type changes are supported:
* From edge-optimized to Regional or private
* From Regional to edge-optimized or private
* From private to Regional
You cannot change a private API into an edge-optimized API.
If you are changing a public API from edge-optimized to Regional or vice versa, note that
an edge-optimized API may have different behaviors than a Regional API. For example, an
edge-optimized API removes the `Content-MD5` header. Any MD5 hash value passed to
the backend can be expressed in a request string parameter or a body property. However, the
Regional API passes this header through, although it may remap the header name to some other
name. Understanding the differences helps you decide how to update an edge-optimized API to
a Regional one or from a Regional API to an edge-optimized one.
###### Topics
* [Use the API Gateway console to change an API
endpoint type](#migrate-api-using-console)
* [Use the AWS CLI to change an API endpoint
type](#migrate-api-using-aws-cli)
## Use the API Gateway console to change an API
endpoint type
To change the API endpoint type of your API, perform one of the following sets of
steps:
###### To convert a public endpoint from Regional or edge-optimized and vice
versa
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose **API settings**.
4. In the **API details** section, choose **Edit**.
5. For **API endpoint type**, select either **Edge-optimized** or
**Regional**.
6. Choose **Save changes**.
7. Redeploy your API so that the changes will take effect.
###### To convert a private endpoint to a Regional endpoint
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Edit the resource policy for your API to remove any mention of VPCs or VPC
endpoints so that API calls from outside your VPC as well as inside your VPC
will succeed.
4. Choose **API settings**.
5. In the **API details** section, choose **Edit**.
6. For **API endpoint type**, select
**Regional**.
7. Choose **Save changes**.
8. Remove the resource policy from your API.
9. Redeploy your API so that the changes will take effect.
Because you're migrating the endpoint type from private to Regional, API Gateway changes the IP address type to
IPv4. For more information, see [IP address types for REST APIs in API Gateway](./api-gateway-ip-address-type.html).
###### To convert a Regional endpoint to a private endpoint
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Create a resource policy that grants access to your VPC or VPC endpoint. For more information, see [Step 3: Set up a resource policy
for a private API](./apigateway-private-api-create.html#apigateway-private-api-set-up-resource-policy).
4. Choose **API settings**.
5. In the **API details** section, choose **Edit**.
6. For **API endpoint type**, select
**Private**.
7. (Optional) For **VPC endpoint IDs**, select the VPC endpoint IDs that you want to associate with your private API.
8. Choose **Save changes**.
9. Redeploy your API so that the changes will take effect.
Because you're migrating the endpoint type from Regional to private, API Gateway changes the IP address type to
dualstack. For more information, see [IP address types for REST APIs in API Gateway](./api-gateway-ip-address-type.html).
## Use the AWS CLI to change an API endpoint
type
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command
updates an edge-optimized API to a Regional API:
```
`aws apigateway update-rest-api \\
--rest-api-id a1b2c3 \\
--patch-operations op=replace,path=/endpointConfiguration/types/EDGE,value=REGIONAL`
```
The successful response has a status code of `200 OK` and a payload similar
to the following:
```
`{
"createdDate": "2017-10-16T04:09:31Z",
"description": "Your first API with Amazon API Gateway. This is a sample API that integrates via HTTP with our demo Pet Store endpoints",
"endpointConfiguration": {
"types": "REGIONAL"
},
"id": "a1b2c3",
"name": "PetStore imported as edge-optimized"
}`
```
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command
updates a Regional API to an edge-optimized API:
```
`aws apigateway update-rest-api \\
--rest-api-id a1b2c3 \\
--patch-operations op=replace,path=/endpointConfiguration/types/REGIONAL,value=EDGE`
```
Because [put-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-rest-api.html) is
for updating API definitions, it is not applicable to updating an API endpoint
type.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway endpoint types
Security policies
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.