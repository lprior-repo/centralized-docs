---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-custom-domain-ip-address-type.html
title: IP address types for custom domain names in API Gateway
word_count: 476
filtered: true
elements_removed: 0
density_score: 0.87
---

IP address types for custom domain names in API Gateway - Amazon API Gateway
IP address types for custom domain names in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-custom-domain-ip-address-type)
[Considerations for IP address types](#api-gateway-ip-address-type-considerations)[Change the IP address type of a custom domain name](#rest-custom-domain-ip-address-type-change)
# IP address types for custom domain names in API Gateway
When you create a custom domain name, you specify the type of IP addresses that can invoke your domain. You
can choose IPv4 to resolve IPv4 addresses to invoke your domain, or you can choose dualstack to allow both IPv4 and IPv6
addresses to invoke your domain. We recommend that you set the IP address type to dualstack to alleviate IP space
exhaustion or for your security posture. For more information about the benefits
of a dualstack IP address type, see [IPv6 on AWS](https://docs.aws.amazon.com/whitepapers/latest/ipv6-on-aws/internet-protocol-version-6.html).
You can change the IP address type by updating the endpoint configuration of your domain name.
## Considerations for IP address types
The following considerations might impact your use of IP address types.
* The default IP address type for API Gateway custom domain names for public APIs is IPv4.
* Private custom domain names can only have a dualstack IP address type.
* Your custom domain name doesn't need to have the same IP address type for all APIs mapped to it. If you
disable your default API endpoint, this might affect how callers can invoke your domain.
## Change the IP address type of a custom domain name
You can change the IP address type by updating the domain name's endpoint configuration. You can update the
endpoint configuration by using the AWS Management Console, the AWS CLI, CloudFormation, or an AWS SDK.
AWS Management Console
###### To change the IP address type of a custom domain name
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a public custom domain name.
3. Choose **Endpoint configuration**.
4. For IP address type, select either **IPv4** or **Dualstack**.
5. Choose **Save**.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
command updates an API to have an IP address type of dualstack:
```
`aws apigateway update-domain-name \\
--domain-name dualstack.example.com \\
--patch-operations "op='replace',path='/endpointConfiguration/ipAddressType',value='dualstack'"`
```
The output will look like the following:
```
`{
"domainName": "dualstack.example.com",
"certificateUploadDate": "2025-02-04T14:46:10-08:00",
"regionalDomainName": "d-abcd1234.execute-api.us-east-1.amazonaws.com",
"regionalHostedZoneId": "Z3LQWSYCGH4ADY",
"regionalCertificateArn": "arn:aws:acm:us-east-1:111122223333:certificate/a1b2c3d4-5678-90ab-cdef",
"endpointConfiguration": {
"types": [
"REGIONAL"
],
"ipAddressType": "dualstack"
},
"domainNameStatus": "AVAILABLE",
"securityPolicy": "TLS\_1\_2",
"tags": {}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API mappings
Choose a security policy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.