---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-publish.html
title: Publish REST APIs for customers to
word_count: 304
filtered: true
elements_removed: 0
density_score: 0.85
---

Publish REST APIs for customers to invoke - Amazon API Gateway
Publish REST APIs for customers to invoke - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-publish)
# Publish REST APIs for customers to
invoke
Simply creating and developing an API Gateway API doesn't automatically make it callable by your
users. To make it callable, you must deploy your API to a stage. In addition, you might want
to customize the URL that your users will use to access your API. You can give it a domain
that is consistent with your brand or is more memorable than the default URL for your
API.
In this section, you can learn how to deploy your API and customize the URL that you
provide to users to access it.
###### Note
To augment the security of your API Gateway APIs, the `execute-api.{`region`}.amazonaws.com` domain is registered in the [Public Suffix List (PSL)](https://publicsuffix.org/). For further security, we recommend that you use cookies with a `\_\_Host-`
prefix if you ever need to set sensitive cookies in the default domain name for your API Gateway APIs. This practice will help to defend your domain against cross-site request
forgery attempts (CSRF). For more information see the [Set-Cookie](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#cookie_prefixes) page in the Mozilla Developer Network.
###### Topics
* [Deploy REST APIs in API Gateway](./how-to-deploy-api.html)
* [Custom domain name for public
REST APIs in API Gateway](./how-to-custom-domains.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Export a REST API
Deploy REST APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.