---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-tagging.html
title: Tagging your API Gateway resources
word_count: 383
filtered: true
elements_removed: 0
density_score: 0.80
---

Tagging your API Gateway resources - Amazon API Gateway
Tagging your API Gateway resources - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-tagging)
# Tagging your API Gateway resources
A *tag* is a metadata label that you assign or that AWS assigns to an
AWS resource. Each tag has two parts:
* A *tag key* (for example, `CostCenter`,
`Environment`, or `Project`). Tag keys are case
sensitive.
* An optional field known as a *tag value* (for example,
`111122223333` or `Production`). Omitting the
tag value is the same as using an empty string. Like tag keys, tag values are
case-sensitive.
Tags help you do the following:
* Control access to your resources based on the tags that are assigned to them. You
control access by specifying tag keys and values in the conditions for an AWS Identity and Access Management
(IAM) policy. For more information about tag-based access
control, see [Controlling Access Using Tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html) in the *IAM User
Guide*.
* Track your AWS costs. You activate these tags on the AWS Billing and Cost Management dashboard. AWS uses
the tags to categorize your costs and deliver a monthly cost allocation report to
you. For more information, see [Use Cost Allocation
Tags](https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html) in the [AWS Billing User Guide](https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/).
* Identify and organize your AWS resources. Many AWS services support tagging, so
you can assign the same tag to resources from different services to indicate that
the resources are related. For example, you could assign the same tag to an API Gateway
stage that you assign to a CloudWatch Events rule.
For tips on using tags, see the whitepaper [AWS
Tagging Strategies](https://docs.aws.amazon.com/whitepapers/latest/tagging-best-practices/tagging-best-practices.html).
The following sections provide more information about tags for Amazon API Gateway.
###### Topics
* [API Gateway resources that can be
tagged](./apigateway-tagging-supported-resources.html)
* [Using tags to control access to API Gateway REST API resources](./apigateway-tagging-iam-policy.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Best practices
API Gateway resources that can be
tagged
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.