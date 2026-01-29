---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-regional-api-custom-domain-migrate.html
title: Migrate a custom domain name
word_count: 1555
filtered: true
elements_removed: 0
density_score: 0.85
---

Migrate a custom domain name to a different API endpoint type in API Gateway - Amazon API Gateway
Migrate a custom domain name to a different API endpoint type in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-regional-api-custom-domain-migrate)
[Considerations](#apigateway-regional-api-custom-domain-migration-considerations)[Migrate custom domain names](#apigateway-api-custom-domain-names-migrate-procedure)
# Migrate a custom domain name
to a different API endpoint type in API Gateway
You can migrate your custom domain name between edge-optimized and Regional endpoints. You can't migrate a
public custom domain name to a private custom domain name. You first add the new endpoint configuration type to the
existing `endpointConfiguration.types` list for the custom domain name. Next, you set up a DNS record to
point the custom domain name to the newly provisioned endpoint. Finally, you remove the obsolete custom domain name
endpoint.
## Considerations
The following are considerations for migrating your custom domain between a Regional API endpoint and an edge-optimized API endpoint:
* An edge-optimized custom domain name requires a certificate provided by ACM from the
US East (N. Virginia) – `us-east-1` Region. This certificate is distributed to all the geographic
locations.
* A Regional custom domain name requires a certificate provided by ACM in the same Region hosting the API.
You can migrate an edge-optimized custom domain name that is not in the `us-east-1` Region to a
Regional custom domain name by requesting a new ACM certificate from the Region that is local to the
API.
* It might take up to 60 seconds to complete a migration between an edge-optimized custom domain name and a
Regional custom domain name. The migration time also depends on when you update your DNS records.
* You can only add an additional endpoint configuration if the endpoint access mode is set to
`BASIC`. Once you have two endpoint configurations, you can't change the endpoint access mode.
For more information, see [Endpoint access mode](./apigateway-security-policies.html#apigateway-security-policies-endpoint-access-mode).
* If your custom domain name uses a security policy that starts with `SecurityPolicy\_`, when you
add a new endpoint configuration type, the endpoint access mode is the same across both endpoint types, and
you must choose a security policy that starts with `SecurityPolicy\_` for the new endpoint
configuration type.
###### Note
To complete the migration, make sure that you remove the obsolete endpoint from your custom domain name.
The following procedure shows how to migrate an edge-optimized custom domain name to a Regional custom domain name.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose an edge-optimized custom domain name.
4. For **Endpoint configuration**, choose **Edit**.
5. Choose **Add Regional endpoint**.
6. For **ACM certificate**, choose a certificate.
The Regional certificate must be in the same Region as the Regional API.
7. Choose **Save changes**.
8. Set up a DNS record to point the Regional custom domain name to this Regional hostname. For more information, see [configuring Route53 to route traffic to API Gateway](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html).
9. After you confirm that your DNS configuration is using the correct endpoint, you delete the
edge-optimized endpoint configuration. Choose your custom domain name, and then for
**Edge-optimized endpoint configuration**, choose **Delete**.
10. Confirm your choice and delete the endpoint.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
command migrates an edge-optmized custom domain name to a Regional custom domain name:
```
`aws apigateway update-domain-name \\
--domain-name 'api.example.com' \\
--patch-operations '[
{ "op":"add", "path": "/endpointConfiguration/types","value": "REGIONAL" },
{ "op":"add", "path": "/regionalCertificateArn", "value": "arn:aws:acm:us-west-2:123456789012:certificate/cd833b28-58d2-407e-83e9-dce3fd852149" }
]'`
```
The Regional certificate must be of the same Region as the Regional API.
The output will look like the following:
```
`{
"certificateArn": "arn:aws:acm:us-east-1:123456789012:certificate/34a95aa1-77fa-427c-aa07-3a88bd9f3c0a",
"certificateName": "edge-cert",
"certificateUploadDate": "2017-10-16T23:22:57Z",
"distributionDomainName": "d1frvgze7vy1bf.cloudfront.net",
"domainName": "api.example.com",
"endpointConfiguration": {
"types": [
"EDGE",
"REGIONAL"
]
},
"regionalCertificateArn": "arn:aws:acm:us-west-2:123456789012:certificate/cd833b28-58d2-407e-83e9-dce3fd852149",
"regionalDomainName": "d-fdisjghyn6.execute-api.us-west-2.amazonaws.com"
}`
```
For the migrated Regional custom domain name, the resulting
`regionalDomainName` property returns the Regional API hostname. You
must set up a DNS record to point the Regional custom domain name to this Regional
hostname. This enables the traffic that is bound to the custom domain name to be
routed to the Regional host.
After the DNS record is set, you can remove the edge-optimized custom domain name. The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command removes the
edge-optimized custom domain name:
```
`aws apigateway update-domain-name \\
--domain-name api.example.com \\
--patch-operations '[
{"op":"remove", "path":"/endpointConfiguration/types", "value":"EDGE"},
{"op":"remove", "path":"certificateName"},
{"op":"remove", "path":"certificateArn"}
]'`
```
The following procedure shows how to migrate an edge-optimized custom domain name that uses an enhanced
security policy to a Regional custom domain name that also uses an enhanced security policy.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose an edge-optimized custom domain name.
4. For **Endpoint configuration**, choose **Edit**.
5. Choose **Add Regional endpoint**.
6. For **ACM certificate**, choose a certificate.
The Regional certificate must be in the same Region as the Regional API.
7. For **Security policy**, choose a security policy that starts with
`SecurityPolicy\_`.
8. For **Endpoint access mode**, choose **Basic**.
9. Choose **Save changes**.
10. Set up a DNS record to point the Regional custom domain name to this Regional hostname. For more information, see [configuring Route53 to route traffic to API Gateway](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html).
11. After you confirm that your DNS configuration is using the correct endpoint, you delete the
edge-optimized endpoint configuration. Choose your custom domain name, and then for
**Edge-optimized endpoint configuration**, choose **Delete**.
12. Confirm your choice and delete the endpoint.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
command migrates an edge-optmized custom domain name to a Regional custom domain name:
```
`aws apigateway update-domain-name \\
--domain-name 'api.example.com' \\
--patch-operations '[
{ "op":"add", "path": "/endpointConfiguration/types","value": "REGIONAL" },
{ "op":"replace", "path": "/securityPolicy", "value":"SecurityPolicy\_TLS13\_1\_3\_2025\_09"},
{ "op":"add", "path": "/regionalCertificateArn", "value": "arn:aws:acm:us-west-2:123456789012:certificate/cd833b28-58d2-407e-83e9-dce3fd852149" }
]'`
```
The Regional certificate must be of the same Region as the Regional API.
The output will look like the following:
```
`{
"certificateArn": "arn:aws:acm:us-east-1:123456789012:certificate/34a95aa1-77fa-427c-aa07-3a88bd9f3c0a",
"certificateName": "edge-cert",
"certificateUploadDate": "2017-10-16T23:22:57Z",
"distributionDomainName": "d1frvgze7vy1bf.cloudfront.net",
"domainName": "api.example.com",
"endpointConfiguration": {
"types": [
"EDGE",
"REGIONAL"
]
},
"securityPolicy": "SecurityPolicy\_TLS13\_1\_3\_2025\_09",
"endpointAccessMode": "BASIC",
"regionalCertificateArn": "arn:aws:acm:us-west-2:123456789012:certificate/cd833b28-58d2-407e-83e9-dce3fd852149",
"regionalDomainName": "d-fdisjghyn6.execute-api.us-west-2.amazonaws.com"
}`
```
For the migrated Regional custom domain name, the resulting
`regionalDomainName` property returns the Regional API hostname. You
must set up a DNS record to point the Regional custom domain name to this Regional
hostname. This enables the traffic that is bound to the custom domain name to be
routed to the Regional host.
After the DNS record is set, you can remove the edge-optimized custom domain name. The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command removes the
edge-optimized custom domain name:
```
`aws apigateway update-domain-name \\
--domain-name api.example.com \\
--patch-operations '[
{"op":"remove", "path":"/endpointConfiguration/types", "value":"EDGE"},
{"op":"remove", "path":"certificateName"},
{"op":"remove", "path":"certificateArn"}
]'`
```
The following procedure shows how to migrate a Regional custom domain name to an edge-optimized custom domain name.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose a Regional custom domain name.
4. For **Endpoint configuration**, choose **Edit**.
5. Choose **Add edge-optimized endpoint**.
6. For **ACM certificate**, choose a certificate.
The edge-optimized domain certificate must be created in the `us-east-1` Region.
7. Choose **Save**.
8. Set up a DNS record to point the edge-optimized custom domain name to this edge-optimized hostname. For more information, see [configuring Route53 to route traffic to API Gateway](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html).
9. After you confirm that your DNS configuration is using the correct endpoint, you delete the
Regional endpoint configuration. Choose your custom domain name, and then for **Regional endpoint
configuration**, choose **Delete**.
10. Confirm your choice and delete the endpoint.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
command migrates your Regional custom domain name to an edge-optimized custom domain name:
```
`aws apigateway update-domain-name \\
--domain-name 'api.example.com' \\
--patch-operations '[
{ "op":"add", "path": "/endpointConfiguration/types","value": "EDGE" },
{ "op":"add", "path": "/certificateName", "value": "edge-cert" },
{"op":"add", "path": "/certificateArn", "value": "arn:aws:acm:us-east-1:738575810317:certificate/34a95aa1-77fa-427c-aa07-3a88bd9f3c0a"}
]'`
```
The edge-optimized domain certificate must be created in the
`us-east-1` Region.
The output will look like the following:
```
`{
"certificateArn": "arn:aws:acm:us-east-1:738575810317:certificate/34a95aa1-77fa-427c-aa07-3a88bd9f3c0a",
"certificateName": "edge-cert",
"certificateUploadDate": "2017-10-16T23:22:57Z",
"distributionDomainName": "d1frvgze7vy1bf.cloudfront.net",
"domainName": "api.example.com",
"endpointConfiguration": {
"types": [
"EDGE",
"REGIONAL"
]
},
"regionalCertificateArn": "arn:aws:acm:us-east-1:123456789012:certificate/3d881b54-851a-478a-a887-f6502760461d",
"regionalDomainName": "d-cgkq2qwgzf.execute-api.us-east-1.amazonaws.com"
}`
```
For the specified custom domain name, API Gateway returns the edge-optimized API
hostname as the `distributionDomainName` property value. You must set a
DNS record to point the edge-optimized custom domain name to this distribution
domain name. This enables traffic that is bound to the edge-optimized custom domain
name to be routed to the edge-optimized API hostname.
After the DNS record is set, you can remove the `REGION` endpoint type
of the custom domain name. The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command removes the Regional endpoint type:
```
`aws apigateway update-domain-name \\
--domain-name api.example.com \\
--patch-operations '[
{"op":"remove", "path":"/endpointConfiguration/types", value:"REGIONAL"},
{"op":"remove", "path":"regionalCertificateArn"}
]'`
```
The output looks like the following:
```
`{
"certificateArn": "arn:aws:acm:us-east-1:738575810317:certificate/34a95aa1-77fa-427c-aa07-3a88bd9f3c0a",
"certificateName": "edge-cert",
"certificateUploadDate": "2017-10-16T23:22:57Z",
"distributionDomainName": "d1frvgze7vy1bf.cloudfront.net",
"domainName": "api.example.com",
"endpointConfiguration": {
"types": "EDGE"
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up an edge-optimized custom domain name in API Gateway
Send traffic
to your APIs through your custom domain name in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.