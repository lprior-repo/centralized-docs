---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-regional-api-custom-domain-create.html
title: Set up a Regional custom
word_count: 1257
filtered: true
elements_removed: 0
density_score: 0.84
---

Set up a Regional custom domain name in API Gateway - Amazon API Gateway
Set up a Regional custom domain name in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-regional-api-custom-domain-create)
[Considerations](#regional-custom-domain-names)[Create a Regional custom domain name ](#apigateway-regional-api-custom-domain-create-procedure)[Create a routing rule for your Regional custom domain name](#apigateway-regional-api-custom-domain-base-path-mapping)[Create a DNS record for your Regional custom domain name ](#apigateway-regional-api-custom-domain-dns-record)
# Set up a Regional custom
domain name in API Gateway
Use a Regional custom domain name to create a user-friendly API base URL. With a Regional custom domain name,
you can map HTTP and REST API stages to the same custom domain name and use mutual TLS authentication.
## Considerations
The following are considerations for your Regional custom domain name:
* You must provide a Region-specific ACM certificate. This certificate must be in the same Region as your API. For more
information about creating or uploading a custom domain name certificate, see [Get certificates ready in AWS Certificate Manager](./how-to-specify-certificate-for-custom-domain-name.html).
* When you create a Regional custom domain name (or migrate one) with an ACM certificate,
API Gateway creates a service-linked role in your account. The
service-linked role is required to attach your ACM certificate to your Regional endpoint.
The role is named **AWSServiceRoleForAPIGateway** and will have the
**APIGatewayServiceRolePolicy** managed policy attached to it. For more
information about use of the service-linked role, see [Using Service-Linked Roles](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html).
* After your create your Regional custom domain name, you must create a DNS record to point the custom
domain name to the Regional domain. This enables the traffic that is bound to the custom domain name to be routed to the
API's Regional hostname.
The DNS record can be the CNAME or an A Alias record. If you use Route53 as your DNS provider, create an A
Alias record. If you use a third-party DNS provider, use a CNAME record. If
you use a CNAME record and create an API Gateway interface
VPC endpoint with private DNS enabled for a private API, you can't resolve the custom
domain name within the VPC that hosts your private API.
## Create a Regional custom domain name
The following procedure shows how to create a Regional custom domain name. After you complete this procedure,
you create a routing rule to route stages of your API to your custom domain name.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose **Create**.
4. For **Domain name**, enter a domain name.
5. For **Routing mode**, choose **Routing rules only**.
In this routing mode, you can only send traffic from your custom domain name to your APIs by using
routing rules. For more information, see [Send traffic
to your APIs through your custom domain name in API Gateway](./rest-api-routing-mode.html).
6. For **Minimum TLS version**, select a version.
7. Under **Endpoint configuration**, for **API endpoint type**, choose **Regional**.
8. Choose an ACM certificate. The certificate must be in the same Region as the API.
9. Choose **Create**.
AWS CLI
The following [create-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-domain-name.html)
command creates a custom domain name:
```
`aws apigatewayv2 create-domain-name \\
--domain-name 'regional.example.com' \\
--domain-name-configurations CertificateArn=arn:aws:acm:`us-west-2`:`123456789012`:certificate/`123456789012-1234-1234-1234-12345678` \\
--routing-mode ROUTING\_RULE\_ONLY`
```
The output will look like the following:
```
`{
"ApiMappingSelectionExpression": "$request.basepath",
"DomainName": "regional.example.com",
"DomainNameConfigurations": [
{
"ApiGatewayDomainName": "d-numh1z56v6.execute-api.us-west-2.amazonaws.com",
"CertificateArn": "arn:aws:acm:us-west-2:`123456789012`:certificate/`123456789012-1234-1234-1234-12345678`",
"DomainNameStatus": "AVAILABLE",
"EndpointType": "REGIONAL",
"HostedZoneId": "Z2OJLYMUO9EFXC",
"SecurityPolicy": "TLS\_1\_2"
}
"RoutingMode": "ROUTING\_RULE\_ONLY"
]
}`
```
The `DomainNameConfigurations` property value returns the Regional API's hostname. You must
create a DNS record to point your custom domain name to this Regional domain name. This enables the traffic
that is bound to the custom domain name to be routed to this Regional API's hostname.
## Create a routing rule for your Regional custom domain name
After you create your custom domain name, you configure how traffic is routed from your custom domain name to
your APIs. Because you set the routing mode to `ROUTING\_RULE\_ONLY`, you use routing rules to route
incoming requests to your custom domain name to your APIs.
In this example, you create a catch-all rule that routes all incoming requests to your
custom domain name to one stage of your API. You can also configure routing rules based on
different header and path conditions. For more information, see [Routing rules to connect API stages to a custom domain name for REST APIs](./rest-api-routing-rules.html).
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a custom domain name.
3. On the **Routing details** tab, choose
**Add routing rule**.
4. Choose **Add a new condition** to add a new condition.
5. Keep this rule without any conditions. This routes all requests to your custom domain name to your
target API and target stage.
6. For **Action**, use the dropdown to select your target API and target stage.
7. Choose **Next**.
8. In the priority field, enter `100`.
API Gateway evaluates rules in priority order, from the lowest value to the highest value. Because this is
a catch-all rule, you use a high priority so API Gateway can match any additional rules you create first.
9. Choose **Create routing rule**.
AWS CLI
The following `create-routing-rule`
command creates a catch-all routing rule:
```
`aws apigatewayv2 create-routing-rule \\
--domain-name 'regional.example.com' \\
--priority 100 \\
--conditions \\
--actions '[{
"InvokeApi": {
"ApiId": "a1b2c3",
"Stage": "prod"
}
}]'`
```
You can change the routing mode and create new rules at any time. For more information, see [Send traffic
to your APIs through your custom domain name in API Gateway](./rest-api-routing-mode.html).
## Create a DNS record for your Regional custom domain name
After you create your custom domain name and create base path mappings, you create a DNS record to
point your custom domain name your newly created Regional domain name.
AWS Management Console
To use the AWS Management Console, follow the Route53 documentation on [configuring Route53 to route traffic to API Gateway](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html).
AWS CLI
To configure your DNS records to map the Regional custom domain name to its
hostname of the given hosted zone ID, first create a JSON file that contains the
configuration for setting up a DNS record for the Regional domain name.
The
following `setup-dns-record.json` shows how to create a DNS `A` record to map a
Regional custom domain name (`regional.example.com`) to its Regional
hostname (`d-numh1z56v6.execute-api.us-west-2.amazonaws.com`)
provisioned as part of the custom domain name creation. The `DNSName`
and `HostedZoneId` properties of `AliasTarget` can take
the `regionalDomainName` and `regionalHostedZoneId`
values, respectively, of the custom domain name. You can also get the Regional
Route 53 Hosted Zone IDs in [Amazon API Gateway Endpoints and Quotas](https://docs.aws.amazon.com/general/latest/gr/apigateway.html).
```
`{
"Changes": [
{
"Action": "CREATE",
"ResourceRecordSet": {
"Name": "regional.example.com",
"Type": "A",
"AliasTarget": {
"DNSName": "d-numh1z56v6.execute-api.us-west-2.amazonaws.com",
"HostedZoneId": "Z2OJLYMUO9EFXC",
"EvaluateTargetHealth": false
}
}
}
]
}
`
```
The following [change-resource-record-sets](https://docs.aws.amazon.com/cli/latest/reference/route53/change-resource-record-sets.html)
creates a DNS record for your Regional custom domain name:
```
`aws route53 change-resource-record-sets \\
--hosted-zone-id Z2OJLYMUO9EFXC \\
--change-batch file://`path/to/your/setup-dns-record.json``
```
Replace the`hosted-zone-id` with the Route53 Hosted
Zone ID of the DNS record set in your account. The `change-batch`
parameter value points to a JSON file
(`setup-dns-record.json`) in a folder
(`path/to/your`).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Get certificates ready in AWS Certificate Manager
Set up an edge-optimized custom domain name in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.