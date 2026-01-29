---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals.html
title: API Gateway portals
word_count: 394
filtered: true
elements_removed: 0
density_score: 0.84
---

API Gateway portals - Amazon API Gateway
API Gateway portals - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals)
[Considerations](#apigateway-portals-considerations)
# API Gateway portals
You can use an API Gateway portal to share your APIs and API documentation with API consumers in a web page. A portal
contains a set of *portal products*. Each portal product is a logical grouping of REST APIs and
contains the documentation that you create and publish for your API consumers. Product pages within a portal contain
the custom documentation at the portal product level. *Product REST endpoint pages* contain the
documentation for each of the REST APIs and the path, method, and stage of a REST API. The combination of product
pages and product REST endpoint pages provide the complete documentation to allow API consumers to learn how to start
using your REST APIs.
Use a portal to do the following:
* Use API Gateway for your entire API lifecycle, from creating your APIs to documenting and distributing
them.
* Share portal products across AWS accounts.
* Customize and share a central location for your portal products and provide product pages.
## Considerations
The following considerations might impact your use of API Gateway portals:
* You can only create portals to share REST APIs.
* Portals are created at the AWS Region level. Your portal can only contain REST APIs that are
in the same Region where you create the portal.
* You can only use the AWS Management Console, AWS CLI, or AWS SDKs to create and share your portals.
* You can only control access to your portal using Amazon Cognito user pools. Your portal consumers can either sign
in directly through a user pool, or they can federate through a third-party identity provider (IdP). If you
use an Amazon Cognito user pool, you cannot set a client secret for your user pool. For more information, see [Amazon Cognito user
pools](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
X-Ray traces
Portal product
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.