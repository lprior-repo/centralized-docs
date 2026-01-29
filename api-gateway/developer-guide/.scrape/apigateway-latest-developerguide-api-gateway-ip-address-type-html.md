---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-ip-address-type.html
title: IP address types for REST APIs in API Gateway
word_count: 510
filtered: true
elements_removed: 0
density_score: 0.83
---

IP address types for REST APIs in API Gateway - Amazon API Gateway
IP address types for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-ip-address-type)
[Considerations for IP address types](#api-gateway-ip-address-type-considerations)
# IP address types for REST APIs in API Gateway
When you create an API, you specify the type of IP addresses that can invoke your API. You can choose IPv4 to
resolve IPv4 addresses to invoke your API, or you can choose dualstack to allow both IPv4 and IPv6 addresses to invoke
your API. We recommend that you set the IP address type to dualstack to alleviate IP space exhaustion or for your security
posture. For more information about the benefits of a dualstack IP address
type, see
[IPv6 on
AWS](https://docs.aws.amazon.com/whitepapers/latest/ipv6-on-aws/internet-protocol-version-6.html).
To restrict your API to only IPv6 traffic, you can create a resource policy and restrict the source IP
addresses to only IPv6 ranges. You can change the IP address type by updating
the API’s configuration. This change will take effect immediately, and you don't need to redeploy your API. For more information, see
[Example:
Deny API traffic based on source IP address or range](./apigateway-resource-policies-examples.html#apigateway-resource-policies-source-ip-address-example)
## Considerations for IP address types
The following considerations might impact your use of IP address types:
* The default IP address type for all Regional and edge-optimized APIs is IPv4.
* Private APIs can only have a dualstack IP address type.
* If you change the IP address type for an existing API from IPv4 to dualstack, confirm that any policies controlling access to your APIs
have been updated to account for IPv6 calls. When you change the IP address type, the change takes effect immediately.
* If you migrate the endpoint type of an API from Regional or edge-optimized to private, API Gateway changes the IP address type to
dualstack. For more information, see [Change a public or private API endpoint type in
API Gateway](./apigateway-api-migration.html).
* If you migrate the endpoint type of an API from private to Regional, you must set the IP address type to
dualstack. After the endpoint migration is complete, you can change the IP address type to IPv4. For more
information, see [Change a public or private API endpoint type in
API Gateway](./apigateway-api-migration.html).
* Your API can be mapped to a custom domain name with a different IP address type than your API. If you
disable your default API endpoint, this might affect how callers can invoke your API.
* You can't use an external definition file to configure your API's IP address type.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
How to change a security policy
Change the IP address type of a REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.