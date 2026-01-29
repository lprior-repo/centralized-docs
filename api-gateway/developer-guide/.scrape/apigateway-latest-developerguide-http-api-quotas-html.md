---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-quotas.html
title: Quotas for configuring
word_count: 440
filtered: true
elements_removed: 0
density_score: 0.92
---

Quotas for configuring and running an HTTP API in API Gateway - Amazon API Gateway
Quotas for configuring and running an HTTP API in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-quotas)
[Best practices to reduce your quotas](#http-api-quotas-best-practices)
# Quotas for configuring
and running an HTTP API in API Gateway
The following quotas apply to configuring and running an HTTP API in Amazon API Gateway. If the quota is per-API, it can
only be increased on specific APIs, not for all the APIs in an account. For information about account-level quotas
see [Amazon API Gateway quotas](./limits.html)
|Resource or operation|Default quota|Can be increased|
|Routes per API|300|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-01C8A9E0)|
|Integrations per API|300|No|
|Maximum integration timeout|30 seconds|No|
|Stages per API|10|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-379E48B0)|
|Multi-level API mappings per domain|200 API mappings. This limit doesn't include API mapping with single levels, such as
`/prod`.
|No|
|Tags per stage |50|No|
|Total combined size of request line and header values|10240 bytes|No|
|Payload size|10 MB|No|
|Custom domains per account per Region|120|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-A93447B8)|
|Access log template size |3 KB|No|
|Amazon CloudWatch Logs log entry |1 MB|No|
|Authorizers per API |10|
Yes
To increase this quota, contact the [AWS Support Center](https://console.aws.amazon.com/support/home#/)
|
|Audiences per authorizer |50|No|
|Scopes per route |10|No|
|Timeout for JSON Web Key Set endpoint |1500 ms|No|
|Response size from JSON Web Key Set endpoint|150000 bytes|No|
|Timeout for OpenID Connect discovery endpoint |1500 ms|No|
|Timeout for Lambda authorizer response|10000 ms|No|
|VPC links per account per Region|10|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-608BDCD4)|
|Subnets per VPC link |10|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-668C9B28)|
|Stage variables per stage|100|No|
|Length, in characters, of the key in a stage variable|64|No|
|Length, in characters, of the value in a stage variable|512|No|
## Best practices to reduce your quotas
The following best practices might help reduce your current number of resources to avoid increasing your quota.
Make sure that these suggestions work for your API's architecture.
**APIs per Region**
To reduce the number of APIs per Region, export any unused APIs and then delete them from API Gateway. For more information,
see [Export HTTP APIs from API Gateway](./http-api-export.html).
**Stages per API**
To reduce the number of stages per API, split up your API into multiple APIs.
**Authorizers per API**
To reduce the number of authorizers per API, reuse authorizers across API methods.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
REST API quotas
Portal quotas
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.