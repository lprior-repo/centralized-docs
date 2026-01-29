---
url: https://docs.aws.amazon.com/lambda/latest/dg/attribute-based-access-control.html
title: Using attribute-based access control in Lambda
word_count: 462
filtered: true
elements_removed: 0
density_score: 0.90
---

Using attribute-based access control in Lambda - AWS Lambda
Using attribute-based access control in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#attribute-based-access-control)
# Using attribute-based access control in Lambda
With [attribute-based access control (ABAC)](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction_attribute-based-access-control.html), you can use tags to control access to your Lambda resources. You
can attach tags to certain Lambda resources, attach them to certain API requests, or attach them to the AWS Identity and Access Management (IAM) principal
making the request. For more information about how AWS grants attribute-based access, see [Controlling access to AWS resources using
tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html) in the *IAM User Guide*.
You can use ABAC to [grant least privilege](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#grant-least-privilege) without
specifying an Amazon Resource Name (ARN) or ARN pattern in the IAM policy. Instead, you can specify a tag in the
[condition
element](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html) of an IAM policy to control access. Scaling is easier with ABAC because you don't have to update
your IAM policies when you create new resources. Instead, add tags to the new resources to control access.
In Lambda, tags work on the following resources:
* Functions–For more information on tagging functions see, [Using tags on Lambda functions](./configuration-tags.html).
* Code signing configurations–For more information on tagging code signing configurations, see [Using tags on code signing configurations](./tags-csc.html).
* Event source mappings–For more information on tagging event source mappings, see [Using tags on event source mappings](./tags-esm.html).
Tags aren't supported for layers.
You can use the following condition keys to write IAM policy rules based on tags:
* [aws:ResourceTag/tag-key](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-resourcetag): Control access based on the tags that are attached to a Lambda resource.
* [aws:RequestTag/tag-key](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-requesttag): Require tags to be present in a request, such as when creating a new function.
* [aws:PrincipalTag/tag-key](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-principaltag): Control what the IAM principal (the person making the request) is allowed to do based on the tags that
are attached to their IAM [user](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags_users.html)
or [role](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags_roles.html).
* [aws:TagKeys](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-tagkeys): Control whether specific tag keys can be used in a request.
You can only specify conditions for actions that support them. For a list of conditions supported by each Lambda action, see
[Actions, resources,
and condition keys for AWS Lambda](https://docs.aws.amazon.com//service-authorization/latest/reference/list_awslambda.html) in the Service Authorization Reference. For **aws:ResourceTag/tag-key** support, refer to "Resource types defined by AWS Lambda." For **aws:RequestTag/tag-key** and **aws:TagKeys** support, refer to
"Actions defined by AWS Lambda."
###### Topics
* [Secure your functions by tag](./attribute-based-access-control-example.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layer access for other accounts
Secure your functions by tag
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.