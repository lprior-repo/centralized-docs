---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-execution-service-limits-table.html
title: Quotas for configuring
word_count: 972
filtered: true
elements_removed: 0
density_score: 0.84
---

Quotas for configuring and running a REST API in API Gateway - Amazon API Gateway
Quotas for configuring and running a REST API in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-execution-service-limits-table)
[Best practices to reduce your quotas](#api-gateway-execution-service-limits-best-practices)
# Quotas for configuring
and running a REST API in API Gateway
The following quotas apply to configuring and running a REST API in Amazon API Gateway. If the quota is per-API, it can
only be increased on specific APIs, not for all the APIs in an account.
For information about account-level quotas
see [Amazon API Gateway quotas](./limits.html).
|Resource or operation|Default quota|Can be increased|
|Public custom domain names per account per Region|120|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-A93447B8)|
|Private custom domain names per account per Region|50|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-24E7E662)|
|Regional APIs per Region|600|No|
|Edge-optimized APIs per Region|120|No|
|Private APIs per account per Region|600|No|
|Domain name access associations per account|100|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-4D98A8A5)|
|Multi-level API mappings per domain|200|No|
|Routing rules per domain|50|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-68B79FF0)|
|Length, in characters, of the URL for an edge-optimized API|8192|No|
|Length, in characters, of the URL for a Regional API|10240|No|
|Length, in characters, of the URL for a private API|8192|No|
|Length, in characters, of API Gateway resource policy|8192|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-8B81B02C)|
|API keys per account per Region|10000|No|
|Client certificates per account per Region|60|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-824C9E42)|
|Authorizers per API (AWS Lambda and Amazon Cognito)|10|
Yes
To increase this quota, contact the [AWS Support Center](https://console.aws.amazon.com/support/home#/)
|
|Documentation parts per API|2000|
Yes
To increase this quota, contact the [AWS Support Center](https://console.aws.amazon.com/support/home#/)
|
|Resources per API|300|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-01C8A9E0)|
|Stages per API|10|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-379E48B0)|
|Stage variables per stage|100|No|
|Length, in characters, of the key in a stage variable|64|No|
|Length, in characters, of the value in a stage variable|512|No|
|Usage plans per account per Region|300|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-E8693075)|
|Usage plans per API key|10|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-985EB478)|
|VPC links per account per Region|20|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-A4C7274F)|
|API caching TTL|300 seconds by default and configurable between 0 and 3600 by an API
owner.|Not for the upper bound (3600)|
|Cached response size|1048576 Bytes. Cache data encryption may increase the size of the item that is being cached.|No|
|Integration timeout for Regional APIs|50 milliseconds - 29 seconds for all integration types, including Lambda,
Lambda proxy, HTTP, HTTP proxy, and AWS integrations.|[Yes \*](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-E5AE38E3)|
|Integration timeout for edge-optimized APIs|50 milliseconds - 29 seconds for all integration types, including Lambda,
Lambda proxy, HTTP, HTTP proxy, and AWS integrations.|No|
|Integration timeout for private APIs|50 milliseconds - 29 seconds for all integration types, including Lambda,
Lambda proxy, HTTP, HTTP proxy, and AWS integrations.|[Yes \*](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-E5AE38E3)|
|Total combined size of all header values, including header names, values, any line terminators, and whitespaces|10240 Bytes|No|
|Total combined size of all header values for a private API|8000 Bytes|No|
|Payload size|10 MB|No|
|Tags per stage|50|No|
|Number of iterations in a `#foreach ... #end` loop in mapping
templates|1000|No|
|Maximum mapping template size|300 KB|No|
|Access log template size|3 KB|No|
|Method ARN length|1600 bytes. If your method contains a path parameter and a client uses a value that exceeds the
ARN length, your API will return a `414 Request URI too long` response.|No|
|Method-level throttling settings for a stage in a usage plan|20|
Yes
To increase this quota, contact the [AWS Support Center](https://console.aws.amazon.com/support/home#/)
|
|Model size per API|400 KB|No|
|Number of certificates in a truststore|1,000 certificates up to 1 MB total object size.|No|
|Idle connection timeout|310 seconds|No|
|Maximum API definition file size when using [restapi:import](https://docs.aws.amazon.com/apigateway/latest/api/API_ImportRestApi.html) or [restapi:put](https://docs.aws.amazon.com/apigateway/latest/api/API_PutRestApi.html)|6 MB|No|
\* You can't set the integration timeout to less than 50 milliseconds. You can raise the integration timeout to
greater than 29 seconds, but this might require a reduction in your Region-level
throttle quota for your account.
## Best practices to reduce your quotas
The following best practices might help reduce your current number of resources to avoid increasing your quota.
Make sure that these suggestions work for your API's architecture.
**APIs per Region**
To reduce the number of APIs per Region, export any unused APIs and then delete them from API Gateway. For more information,
see [Export a REST API from API Gateway](./api-gateway-export-api.html).
**Stages per API**
To reduce the number of stages per API, split up your API into multiple APIs.
**Resources per API**
Use `{proxy+}` paths to reduce the number of resources. For more information, see [Set up a proxy integration with a proxy
resource](./api-gateway-set-up-simple-proxy.html).
**API mappings**
To reduce the number of API mappings for a custom domain name, use API mappings with single levels, such
as `/prod`. API mappings with a single level doesn't count towards the API mappings quota.
**Authorizers per API**
To reduce the number of authorizers per API, reuse authorizers across API methods.
**Documentation parts per API**
To reduce the number of documentation parts per API, use `ignore=documentation` when you
import your API. For more information, see [Import API
documentation](./api-gateway-documenting-api-quick-start-import-export.html).
You can also use content inheritance to
allow some documentation parts to inherit content from an API entity of a more general specification. For more
information, see [Representation of
API documentation in API Gateway](./api-gateway-documenting-api-content-representation.html).
**Length, in characters, of API Gateway resource policy**
To reduce the length of a resource policy, use AWS WAFV2 to protect your API. For more information, see
[Use AWS WAF to protect your REST APIs in API Gateway](./apigateway-control-access-aws-waf.html).
If your policy contains IP addresses, you can also
use ranges instead of specific values to define IP addresses.
**Usage plans per API key**
To reduce the number of usage plans per API key, use one API key per usage plan, and associate your
usage plan with multiple APIs. We don't recommend sharing one API key across multiple usage plans.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Quotas and important notes
HTTP API quotas
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.