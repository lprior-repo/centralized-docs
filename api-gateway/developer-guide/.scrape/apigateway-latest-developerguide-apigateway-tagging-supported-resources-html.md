---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-tagging-supported-resources.html
title: API Gateway resources that can be
word_count: 702
filtered: true
elements_removed: 0
density_score: 0.83
---

API Gateway resources that can be tagged - Amazon API Gateway
API Gateway resources that can be tagged - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-tagging-supported-resources)
[Tag inheritance in the Amazon API Gateway V1 API](#apigateway-tagging-inheritance)[Tag restrictions and usage
conventions](#apigateway-tagging-restrictions)
# API Gateway resources that can be
tagged
Tags can be set on the following HTTP API or WebSocket API resources in the
[Amazon API Gateway V2
API](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/):
* `Api`
* `DomainName`
* `Stage`
* `VpcLink`
In addition, tags can be set on the following REST API resources in the [Amazon API Gateway V1 API](https://docs.aws.amazon.com/apigateway/latest/api/API_Operations.html):
* `ApiKey`
* `ClientCertificate`
* `DomainName`
* `DomainNameAccessAssociation`
* `RestApi`
* `Stage`
* `UsagePlan`
* `VpcLink`
Tags cannot be set directly on other resources. However, in the [Amazon API Gateway V1 API](https://docs.aws.amazon.com/apigateway/latest/api/API_Operations.html), child resources inherit the
tags that are set on parent resources. For example:
* If a tag is set on a `RestApi` resource, that tag is inherited by
the following child resources of that `RestApi` for [Attribute-based access
control](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction_attribute-based-access-control.html):
* `Authorizer`
* `Deployment`
* `Documentation`
* `GatewayResponse`
* `Integration`
* `Method`
* `Model`
* `Resource`
* `ResourcePolicy`
* `Setting`
* `Stage`
* If a tag is set on a `DomainName`, that tag is inherited by any `BasePathMapping`,
`ApiMapping`, and `RoutingRule` resources under it.
* If a tag is set on a `UsagePlan`, that tag is inherited by any
`UsagePlanKey` resources under it.
###### Note
Tag inheritance applies only to [attribute-based access
control](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction_attribute-based-access-control.html). For example, you can't use inherited tags to monitor costs in AWS Cost Explorer. To use tags for cost
allocation, we recommend that you create tags on child resources, such as the `Stage` resource.
API Gateway
doesn't return inherited tags when you call [GetTags](https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-tags.html) for a resource.
## Tag inheritance in the Amazon API Gateway V1 API
Previously it was only possible to set tags on stages. Now that you can also set
them on other resources, a `Stage` can receive a tag two ways:
* The tag can be set directly on the `Stage`.
* The stage can inherit the tag from its parent `RestApi`.
If a stage receives a tag both ways, the tag that was set directly on the stage
takes precedence. For example, suppose a stage inherits the following tags from its
parent REST API:
```
`{
'foo': 'bar',
'x':'y'
}`
```
Suppose it also has the following tags set on it directly:
```
`{
'foo': 'bar2',
'hello': 'world'
}`
```
The net effect would be for the stage to have the following tags, with the
following values:
```
`{
'foo': 'bar2',
'hello': 'world'
'x':'y'
}`
```
## Tag restrictions and usage
conventions
The following restrictions and usage conventions apply to using tags with API Gateway
resources:
* Each resource can have a maximum of 50 tags.
* For each resource, each tag key must be unique, and each tag key can have
only one value.
* The maximum tag key length is 128 Unicode characters in UTF-8.
* The maximum tag value length is 256 Unicode characters in UTF-8.
* Allowed characters for keys and values are letters, numbers, spaces
representable in UTF-8, and the following characters:*** . : + = @ \_ / -***
(hyphen). Amazon EC2 resources allow any characters.
* Tag keys and values are case-sensitive. As a best practice, decide on a
strategy for capitalizing tags, and consistently implement that strategy
across all resource types. For example, decide whether to use
`Costcenter`, `costcenter`, or
`CostCenter`, and use the same convention for all tags. Avoid
using similar tags with inconsistent case treatment.
* The `aws:` prefix is prohibited for tags; it's reserved for
AWS use. You can't edit or delete tag keys or values with this prefix.
Tags with this prefix do not count against your tags per resource
limit.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tagging
Attribute-based access control
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.