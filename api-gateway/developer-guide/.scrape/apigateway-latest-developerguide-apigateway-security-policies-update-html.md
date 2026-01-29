---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-security-policies-update.html
title: How to change a security policy
word_count: 511
filtered: true
elements_removed: 0
density_score: 0.81
---

How to change a security policy - Amazon API Gateway
How to change a security policy - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-security-policies-update)
# How to change a security policy
You can change the security policy for your API. If you are sending traffic to your APIs through your custom
domain name, the API and the custom domain name don't need to have the same security policy. When you invoke that
custom domain name, API Gateway uses the security policy of the API to negotiate the TLS handshake. However, for
consistency, we recommend that you use the same security policy for your custom domain name and API.
If you change your security policy, it takes about 15 minutes for the update to complete. You can monitor the
`apiStatus` of your API. As your API updates, the `apiStatus` is
`UPDATING` and when it completes, it will be `AVAILABLE`. When your API is updating, you can
still invoke it.
AWS Management Console
###### To change the security policy of an API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose **API settings**, and then choose
**Edit**.
4. For **Security policy**, select a new policy that starts with `SecurityPolicy\_`.
5. For **Endpoint access mode**, choose **Strict**.
6. Choose **Save changes**.
Redeploy your API for the changes to take effect. Because you changed the endpoint access mode to
strict, it will take about 15 minutes for the changes to fully propagate.
AWS CLI
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command
updates an API to use the `SecurityPolicy\_TLS13\_1\_3\_2025\_09` security policy:
```
`aws apigateway update-rest-api \\
--rest-api-id abcd1234 \\
--patch-operations '[
{
"op": "replace",
"path": "/securityPolicy",
"value": "SecurityPolicy\_TLS13\_1\_3\_2025\_09"
},
{
"op": "replace",
"path": "/endpointAccessMode",
"value": "STRICT"
}
]'`
```
The output will look like the following:
```
`{
"id": "abcd1234",
"name": "MyAPI",
"description": "My API with a new security policy",
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
"securityPolicy": "SecurityPolicy\_TLS13\_1\_3\_2025\_09",
"endpointAccessMode": "STRICT"
"rootResourceId": "efg456"
}`
```
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command
updates a API that was using an enhanced security policy to use the `TLS\_1\_0` security
policy.
```
`aws apigateway update-rest-api \\
--rest-api-id abcd1234 \\
--patch-operations '[
{
"op": "replace",
"path": "/securityPolicy",
"value": "TLS\_1\_0"
},
{
"op": "replace",
"path": "/endpointAccessMode",
"value": ""
}
]'`
```
The output will look like the following:
```
`{
"id": "abcd1234",
"name": "MyAPI",
"description": "My API with a new security policy",
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
"securityPolicy": "TLS\_1\_0",
"rootResourceId": "efg456"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Supported security policies
IP address types for REST APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.