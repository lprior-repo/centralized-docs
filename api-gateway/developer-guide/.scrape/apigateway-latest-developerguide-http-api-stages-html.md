---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-stages.html
title: Stages for HTTP APIs in API Gateway
word_count: 255
filtered: true
elements_removed: 0
density_score: 0.82
---

Stages for HTTP APIs in API Gateway - Amazon API Gateway
Stages for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-stages)
# Stages for HTTP APIs in API Gateway
An API stage is a logical reference to a lifecycle state of your API (for example,
`dev`, `prod`, `beta`, or `v2`). API stages
are identified by their API ID and stage name, and they're included in the URL you use to
invoke the API. Each stage is a named reference to a deployment of the API and is made
available for client applications to call.
You can create a `$default` stage that is served from the base of your API's
URL—for example, `https://{api\_id}.execute-api.{region}.amazonaws.com/`.
You use this URL to invoke an API stage.
A deployment is a snapshot of your API configuration. After you deploy an API to a stage,
it’s available for clients to invoke. You must deploy an API for changes to take effect. If
you enable automatic deployments, changes to an API are automatically released for
you.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Publish
Use stage variables for HTTP APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.