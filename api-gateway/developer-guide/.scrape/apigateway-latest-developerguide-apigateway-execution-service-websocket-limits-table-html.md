---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-execution-service-websocket-limits-table.html
title: Quotas for configuring
word_count: 402
filtered: true
elements_removed: 0
density_score: 0.90
---

Quotas for configuring and running a WebSocket in API Gateway - Amazon API Gateway
Quotas for configuring and running a WebSocket in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-execution-service-websocket-limits-table)
# Quotas for configuring
and running a WebSocket in API Gateway
The following quotas apply to configuring and running a WebSocket API in Amazon API Gateway. If the quota is per-API, it can
only be increased on specific APIs, not for all the APIs in an account. For information about account-level quotas
see [Amazon API Gateway quotas](./limits.html)
|Resource or operation|Default quota|Can be increased|
|New connections per second per account (across all WebSocket APIs) per Region|500|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-9ED1E49A)|
|Concurrent connections|Not applicable \*|Not applicable|
|AWS Lambda authorizers per API|10|
Yes
To increase this quota, contact the [AWS Support Center](https://console.aws.amazon.com/support/home#/)
|
|AWS Lambda authorizer result size|8 KB|No|
|Routes per API|300|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-01C8A9E0)|
|Integrations per API|300|
Yes
To increase this quota, contact the [AWS Support Center](https://console.aws.amazon.com/support/home#/)
|
|Integration timeout|50 milliseconds - 29 seconds for all integration types, including Lambda,
Lambda proxy, HTTP, HTTP proxy, and AWS integrations.|No|
|Stages per API|10|[Yes](https://console.aws.amazon.com/servicequotas/home/services/apigateway/quotas/L-379E48B0)|
|WebSocket frame size|32 KB|No|
|Message payload size|128 KB \*\*|No|
|Maximum mapping template size|300 KB|No|
|Connection duration for WebSocket API|2 hours|No|
|Idle Connection Timeout|10 minutes|No|
|Length, in characters, of the URL for a WebSocket API|4096|No|
|Access log template size|3 KB|No|
\* API Gateway doesn't enforce a quota on concurrent connections. The maximum number of concurrent connections is
determined by the rate of new connections per second and maximum connection duration of two hours. For example, with
the default quota of 500 new connections per second, if clients connect at the maximum rate over two hours, API Gateway can
serve up to 3,600,000 concurrent connections.
\*\* Because of the WebSocket frame-size quota of 32 KB, a message larger than 32 KB must be split into multiple
frames, each 32 KB or smaller. This applies to `@connections` commands. If a larger message (or larger frame size) is received, the connection is closed with
code 1009.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Portal quotas
Important notes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.