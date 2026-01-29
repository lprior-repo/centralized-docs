---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-ip-address-type-change.html
title: Change the IP address type of a REST API
word_count: 329
filtered: true
elements_removed: 0
density_score: 0.86
---

Change the IP address type of a REST API - Amazon API Gateway
Change the IP address type of a REST API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-ip-address-type-change)
# Change the IP address type of a REST API
You can change the IP address type by updating the API’s configuration. You can update the API's configuration
by using the AWS Management Console, the AWS CLI, CloudFormation, or an AWS SDK. If you change the API’s IP address type, you don't redeploy
your API for the changes to take effect. Before you change the IP address type, confirm that any policies
controlling access to your APIs have been updated to account for IPv6 calls.
AWS Management Console
###### To change the IP address type of a REST API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose **API settings**, and then choose
**Edit**.
4. For IP address type, select either **IPv4** or **Dualstack**.
5. Choose **Save changes**.
The change to your API's configuration will take effect immediately.
AWS CLI
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html)
command updates an API to have an IP address type of dualstack:
```
`aws apigateway update-rest-api \\
--rest-api-id abcd1234 \\
--patch-operations "op='replace',path='/endpointConfiguration/ipAddressType',value='dualstack'"`
```
The output will look like the following:
```
`{
"id": "abcd1234",
"name": "MyAPI",
"description": "My API with a dualstack IP address type",
"createdDate": "2025-02-04T11:47:06-08:00",
"apiKeySource": "HEADER",
"endpointConfiguration": {
"types": [
"REGIONAL"
],
"ipAddressType": "dualstack"
},
"tags": {},
"disableExecuteApiEndpoint": false,
"rootResourceId": "efg456"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IP address types for REST APIs in API Gateway
Methods
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.