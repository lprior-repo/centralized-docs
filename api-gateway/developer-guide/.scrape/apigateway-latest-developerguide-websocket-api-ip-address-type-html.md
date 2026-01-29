---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-ip-address-type.html
title: IP address types for WebSocket APIs in API Gateway
word_count: 504
filtered: true
elements_removed: 0
density_score: 0.86
---

IP address types for WebSocket APIs in API Gateway - Amazon API Gateway
IP address types for WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-ip-address-type)
[Considerations for IP address types](#websocket-api-ip-address-type-considerations)[Change the IP address type of an WebSocket API](#websocket-api-ip-address-type-change)
# IP address types for WebSocket APIs in API Gateway
When you create an API, you specify the type of IP addresses that can invoke your API. You can choose IPv4 to
resolve IPv4 addresses to invoke your API, or you can choose dualstack to allow both IPv4 and IPv6 addresses to invoke
your API. We recommend that you set the IP address type to dualstack to alleviate IP space exhaustion or for your security
posture. For more information about the benefits of a dualstack IP address
type, see
[IPv6 on
AWS](https://docs.aws.amazon.com/whitepapers/latest/ipv6-on-aws/internet-protocol-version-6.html).
## Considerations for IP address types
The following considerations might impact your use of IP address types:
* The default IP address type for all WebSocket APIs is IPv4.
* If you change the IP address type for an existing API from IPv4 to dualstack, confirm that any policies controlling access to your APIs
have been updated to account for IPv6 calls. When you change the IP address type, the change takes effect immediately.
* Your API can be mapped to a custom domain name with a different IP address type than your API. If you
disable your default API endpoint, this might affect how callers can invoke your API.
## Change the IP address type of an WebSocket API
You can change the IP address type by updating the API’s configuration. You can update the API's configuration
by using the AWS Management Console, the AWS CLI, CloudFormation, or an AWS SDK. If you change the API’s IP address type, you don't redeploy
your API for the changes to take effect.
AWS Management Console
###### To change the IP address type of a WebSocket API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a WebSocket API.
3. Choose **API settings**, and then choose **Edit**.
4. For IP address type, select either **IPv4** or **Dualstack**.
5. Choose **Save**.
The change to your API's configuration will take effect immediately.
AWS CLI
The following [update-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-api.html)
command updates an API to have an IP address type of dualstack:
```
`aws apigatewayv2 update-api \\
--api-id abcd1234 \\
--ip-address-type dualstack`
```
The output will look like the following:
```
`{
"ApiEndpoint": "https://abcd1234.execute-api.us-east-1.amazonaws.com",
"ApiId": "abcd1234",
"ApiKeySelectionExpression": "$request.header.x-api-key",
"CreatedDate": "2025-02-04T22:20:20+00:00",
"DisableExecuteApiEndpoint": false,
"Name": "My-WebSocket-API",
"ProtocolType": "WEBSOCKET",
"RouteSelectionExpression": "$request.method $request.path",
"Tags": {},
"NotificationUris": [],
"IpAddressType": "dualstack"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create and configure
Routes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.