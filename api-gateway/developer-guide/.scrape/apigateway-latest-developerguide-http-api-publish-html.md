---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-publish.html
title: Publish HTTP APIs for customers to invoke
word_count: 311
filtered: true
elements_removed: 0
density_score: 0.85
---

Publish HTTP APIs for customers to invoke - Amazon API Gateway
Publish HTTP APIs for customers to invoke - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-publish)
# Publish HTTP APIs for customers to invoke
You can use stages and custom domain names to publish your API for clients to
invoke.
An API stage is a logical reference to a lifecycle state of your API (for example,
`dev`, `prod`, `beta`, or `v2`). Each stage
is a named reference to a deployment of the API and is made available for client
applications to call. You can configure different integrations and settings for each stage
of an API.
You can use custom domain names to provide a simpler, more intuitive URL for clients to
invoke your API than the default URL,
`https://`api-id`.execute-api.`region`.amazonaws.com/`stage``.
###### Note
To augment the security of your API Gateway APIs, the `execute-api.{`region`}.amazonaws.com` domain is registered in the [Public Suffix List (PSL)](https://publicsuffix.org/). For further security, we recommend that you use cookies with a `\_\_Host-`
prefix if you ever need to set sensitive cookies in the default domain name for your API Gateway APIs. This practice will help to defend your domain against cross-site request
forgery attempts (CSRF). For more information see the [Set-Cookie](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#cookie_prefixes) page in the Mozilla Developer Network.
###### Topics
* [Stages for HTTP APIs in API Gateway](./http-api-stages.html)
* [Security policy for HTTP APIs in API Gateway](./http-api-ciphers.html)
* [Custom domain names for HTTP APIs in API Gateway](./http-api-custom-domain-names.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Export
Stages
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.