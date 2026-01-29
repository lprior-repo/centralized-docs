---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-edge-optimized-custom-domain-name.html
title: Set up an edge-optimized custom domain name in API Gateway
word_count: 1807
filtered: true
elements_removed: 0
density_score: 0.87
---

Set up an edge-optimized custom domain name in API Gateway - Amazon API Gateway
Set up an edge-optimized custom domain name in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-edge-optimized-custom-domain-name)
[Considerations](#how-to-edge-optimized-custom-domain-name-considerations)[Create an edge-optimize custom domain name](#how-to-custom-domains-console)[Configure base path mapping of an API with a custom
domain name as its hostname](#how-to-custom-domains-mapping-console)[Create a DNS record for your
edge-optimized custom domain name](#how-to-edge-optimized-custom-domain-name-dns-record)[Log custom domain name
creation in CloudTrail](#how-to-custom-domain-log-cloudfront-distribution-update-in-cloudtrail)[Rotate a certificate imported into ACM](#how-to-rotate-custom-domain-certificate)[Call your API with custom domain names when you use a base path mapping](#how-to-custom-domains-call-api-with-sni)
# Set up an edge-optimized custom domain name in API Gateway
When you create a custom domain name for an edge-optimized API, API Gateway sets up a CloudFront distribution and a DNS
record to map the API domain name to the CloudFront distribution domain name. Requests for the API are then routed to
API Gateway through the mapped CloudFront distribution. This mapping is for API requests that are bound for the custom domain
name to be routed to API Gateway through the mapped CloudFront distribution.
## Considerations
The following are considerations for your edge-optimized custom domain name:
* To set up an edge-optimized custom domain name or to update its certificate, you must have a permission
to update CloudFront distributions.
The following permissions are required to update CloudFront distributions:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "AllowCloudFrontUpdateDistribution",
"Effect": "Allow",
"Action": [
"cloudfront:updateDistribution"
],
"Resource": [
"\*"
]
}
]
}`
`
```
* You must request or import a certificate for your edge-optimized custom domain name in the
US East (N. Virginia) – `us-east-1` Region.
* The CloudFront distribution created by API Gateway is owned by a Region-specific account affiliated
with API Gateway. When tracing operations to create and update such a CloudFront distribution in CloudTrail, you must use
this API Gateway account ID. For more information, see [Log custom domain name
creation in CloudTrail](#how-to-custom-domain-log-cloudfront-distribution-update-in-cloudtrail).
* API Gateway supports edge-optimized custom domain names by leveraging Server Name Indication (SNI) on the
CloudFront distribution. For more information on using custom domain names on a CloudFront distribution, including the
required certificate format and the maximum size of a certificate key length, see [ Using Alternate Domain Names and HTTPS](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https-alternate-domain-names.html)
in the *Amazon CloudFront Developer Guide*
* An edge-optimized custom domain names takes about 40 minutes to be ready.
* After you create your edge-optimized custom domain name, you must create a DNS record to map the custom
domain name to the CloudFront distribution name.
## Create an edge-optimize custom domain name
The following procedure describes how to create an edge-optimized custom domain name for an API.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation pane.
3. Choose **Add domain name**.
4. For **Domain name**, enter a domain name.
5. For **Routing mode**, choose **API\_MAPPING\_ONLY**.
6. For **API endpoint type**, choose **Edge-optimized**.
7. Choose a minimum TLS version.
8. Choose an ACM certificate.
9. Choose **Add domain name**.
REST API
1. Call [domainname:create](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateDomainName.html), specifying the
custom domain name and the ARN of a certificate stored in AWS Certificate Manager.
The successful API call returns a `201 Created` response containing the certificate ARN as
well as the associated CloudFront distribution name in its payload.
2. Note the CloudFront distribution domain name shown in the output. You need it in the next step to set the
custom domain's A-record alias target in your DNS.
For code examples of this REST API call, see [domainname:create](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateDomainName.html).
An edge-optimized custom domain names takes about 40 minutes to be ready, but the console immediately
displays the associated CloudFront distribution domain name, in the form of
``distribution-id`.cloudfront.net`, along with the certificate ARN. In
the meantime, you can create a base path mapping or a routing rule and then configure the DNS record alias to map the custom
domain name to the associated CloudFront distribution domain name.
## Configure base path mapping of an API with a custom
domain name as its hostname
Because you set the routing mode to `API\_MAPPING\_ONLY`, you can use base path mapping to use a
single custom domain name as the hostname of multiple APIs. This makes an API accessible through the combination
of the custom domain name and the associated base path.
For example, if in API Gateway, you created an API named `PetStore` and another API named
`Dogs` and then set up a custom domain name of `api.example.com`, you can set the
`PetStore` API's URL as `https://api.example.com`.
This associates the
`PetStore` API with the base path of an empty string. If you set the `PetStore` API's
URL as `https://api.example.com/PetStore`, this associates the `PetStore` API with the
base path of `PetStore`. You can assign a base path of `MyDogList` for the `Dogs` API. The URL of
`https://api.example.com/MyDogList` is then the root URL of the `Dogs` API.
To configure API mappings on multiple levels, you can only use a Regional custom domain name. Edge-optimized
custom domain names are not supported. For more information, see [Use API mappings to connect API stages to a custom domain name for REST APIs](./rest-api-mappings.html).
The following procedure sets up API mappings to map paths from your custom domain name to your API
stages.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the API Gateway console main navigation pane.
3. Choose a custom domain name.
4. Choose **Configure API mappings**.
5. Choose **Add new mapping**.
6. Specify the **API**, **Stage**, and **Path**
(optional) for the mapping.
7. Choose **Save**.
REST API
Call [basepathmapping:create](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateBasePathMapping.html) on a
specific custom domain name, specifying the `basePath`, `restApiId`, and a deployment
`stage` property in the request payload.
The successful API call returns a `201 Created` response.
For code examples of the REST API call, see [basepathmapping:create](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateBasePathMapping.html).
For more flexibility on how you route traffic to your APIs, you can change the routing mode to
`ROUTING\_RULE\_ONLY` or `ROUTING\_RULE\_THEN\_API\_MAPPING` and create a routing rule. For
more information, see [Send traffic
to your APIs through your custom domain name in API Gateway](./rest-api-routing-mode.html).
## Create a DNS record for your
edge-optimized custom domain name
After you initiate the creation of your edge-optimized custom domain name, set up the DNS record alias.
We recommend that you use Route53 to create an A-record alias for your custom domain name and specify the CloudFront
distribution domain name as the alias target. This means that Route53 can route your custom domain name even
if it is a zone apex. For more information, see [Choosing Between Alias and Non-Alias
Resource Record Sets](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/resource-record-sets-choosing-alias-non-alias.html) in the *Amazon Route53 Developer Guide*.
For instructions for Amazon Route53, see [Routing traffic to an Amazon API Gateway API by
using your domain name](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html) in the *Amazon Route53 Developer Guide*.
## Log custom domain name
creation in CloudTrail
When CloudTrail is enabled for logging API Gateway calls made by your account, API Gateway logs the associated CloudFront
distribution updates when a custom domain name is created or updated for an API. These logs are available in `us-east-1`. Because these CloudFront
distributions are owned by API Gateway, each of these reported CloudFront distributions is identified by one of the
following Region-specific API Gateway account IDs, instead of the API owner's account ID.
|
**Region**
|
**Account ID**
|
|us-east-1|392220576650|
|us-east-2|718770453195|
|us-west-1|968246515281|
|us-west-2|109351309407|
|ca-central-1|796887884028|
|eu-west-1|631144002099|
|eu-west-2|544388816663|
|eu-west-3|061510835048|
|eu-central-1|474240146802|
|eu-central-2|166639821150|
|eu-north-1|394634713161|
|eu-south-1|753362059629|
|eu-south-2|359345898052|
|ap-northeast-1|969236854626|
|ap-northeast-2|020402002396|
|ap-northeast-3|360671645888|
|ap-southeast-1|195145609632|
|ap-southeast-2|798376113853|
|ap-southeast-3|652364314486|
|ap-southeast-4|849137399833|
|ap-south-1|507069717855|
|ap-south-2|644042651268|
|ap-east-1|174803364771|
|sa-east-1|287228555773|
|me-south-1|855739686837|
|me-central-1|614065512851|
## Rotate a certificate imported into ACM
ACM automatically handles renewal of certificates it issues. You do not need to rotate any ACM-issued
certificates for your custom domain names. CloudFront handles it on your behalf.
However, if you import a certificate into ACM and use it for a custom domain name, you must rotate the
certificate before it expires. This involves importing a new third-party certificate for the domain name and
rotate the existing certificate to the new one. You need to repeat the process when the newly imported
certificate expires. Alternatively, you can request ACM to issue a new certificate for the domain name and
rotate the existing one to the new ACM-issued certificate. After that, you can leave ACM and CloudFront to handle
the certificate rotation for you automatically. To create or import a new ACM certificate, follow the steps in
[To create or import an SSL/TLS certificate into ACM](./how-to-specify-certificate-for-custom-domain-name.html#how-to-specify-certificate-for-custom-domain-name-setup).
The following procedure describes how to rotate a certificate for a domain name.
###### Note
It takes about 40 minutes
to rotate a certificate imported into ACM.
AWS Management Console
1. Request or import a certificate in ACM.
2. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
3. Choose **Custom domain names** from the API Gateway console main navigation pane.
4. Choose an edge-optimized custom domain name.
5. For **Endpoint configuration**, choose **Edit**.
6. For **ACM certificate**, select a certificate from dropdown list.
7. Choose **Save changes** to begin rotating the certificate for the custom domain name.
REST API
Call [domainname:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDomainName.html) action, specifying
the ARN of the new ACM certificate for the specified domain name.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
updates the ACM certificate for an edge-optimized domain name.
```
`aws apigateway update-domain-name \\
--domain-name edge.example.com \\
--patch-operations "op='replace',path='/certificateArn',value='arn:aws:acm:us-east-2:111122223333:certificate/CERTEXAMPLE123EXAMPLE'"`
```
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
updates the ACM certificate for a Regional domain name.
```
`aws apigateway update-domain-name \\
--domain-name regional.example.com \\
--patch-operations "op='replace',path='/regionalCertificateArn',value='arn:aws:acm:us-east-2:111122223333:certificate/CERTEXAMPLE123EXAMPLE'"`
```
## Call your API with custom domain names when you use a base path mapping
Calling an API with a custom domain name is the same as calling the API with its default domain name,
provided that the correct URL is used.
The following examples compare and contrast a set of default URLs and corresponding custom URLs of two APIs
(`udxjef` and `qf3duz`) in a specified Region
(`us-east-1`), and of a given custom domain name (`api.example.com`).
|API ID|Stage|Default URL|Base path|Custom URL|
|udxjef|prod|https://udxjef.execute-api.us-east-1.amazonaws.com/prod|/petstore|https://api.example.com/petstore|
|udxjef|tst|https://udxjef.execute-api.us-east-1.amazonaws.com/tst|/petdepot|https://api.example.com/petdepot|
|qf3duz|dev|https://qf3duz.execute-api.us-east-1.amazonaws.com/dev|/bookstore|https://api.example.com/bookstore|
|qf3duz|tst|https://qf3duz.execute-api.us-east-1.amazonaws.com/tst|/bookstand|https://api.example.com/bookstand|
For more flexibility on how you route traffic to your APIs, you can create a routing rule. For more
information, see [Send traffic
to your APIs through your custom domain name in API Gateway](./rest-api-routing-mode.html).
API Gateway supports custom domain names for an API by using [Server Name Indication (SNI)](https://en.wikipedia.org/wiki/Server_Name_Indication). You can
invoke the API with a custom domain name using a browser or a client library that supports SNI.
API Gateway enforces SNI on the CloudFront distribution. For information on how CloudFront uses custom domain names, see
[Amazon CloudFront Custom SSL](https://aws.amazon.com/cloudfront/custom-ssl-domains/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a Regional custom domain name
Migrate custom domain names
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.