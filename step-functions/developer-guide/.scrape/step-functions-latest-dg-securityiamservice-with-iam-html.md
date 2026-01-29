---
url: https://docs.aws.amazon.com/step-functions/latest/dg/security_iam_service-with-iam.html
title: How AWS Step Functions works with IAM
word_count: 1576
filtered: true
elements_removed: 0
density_score: 0.81
---

How AWS Step Functions works with IAM - AWS Step Functions
How AWS Step Functions works with IAM - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#security_iam_service-with-iam)
[Identity-based policies](#security_iam_service-with-iam-id-based-policies)[Resource-based policies](#security_iam_service-with-iam-resource-based-policies)[Policy actions](#security_iam_service-with-iam-id-based-policies-actions)[Policy resources](#security_iam_service-with-iam-id-based-policies-resources)[Policy condition keys](#security_iam_service-with-iam-id-based-policies-conditionkeys)[ACLs](#security_iam_service-with-iam-acls)[ABAC](#security_iam_service-with-iam-tags)[Temporary credentials](#security_iam_service-with-iam-roles-tempcreds)[Principal permissions](#security_iam_service-with-iam-principal-permissions)[Service roles](#security_iam_service-with-iam-roles-service)[Service-linked roles](#security_iam_service-with-iam-roles-service-linked)
# How AWS Step Functions works with IAM
Before you use IAM to manage access to Step Functions, learn what IAM features are
available to use with Step Functions.
The following table lists IAM features that you can use with AWS Step Functions:
|IAM feature|Step Functions support|
|
[Identity-based policies](#security_iam_service-with-iam-id-based-policies)
|
Yes
|
|
[Resource-based policies](#security_iam_service-with-iam-resource-based-policies)
|
No
|
|
[Policy actions](#security_iam_service-with-iam-id-based-policies-actions)
|
Yes
|
|
[Policy resources](#security_iam_service-with-iam-id-based-policies-resources)
|
Yes
|
|
[Policy condition keys (service-specific)](#security_iam_service-with-iam-id-based-policies-conditionkeys)
|
Yes
|
|
[ACLs](#security_iam_service-with-iam-acls)
|
No
|
|
[ABAC (tags in
policies)](#security_iam_service-with-iam-tags)
|
Partial
|
|
[Temporary
credentials](#security_iam_service-with-iam-roles-tempcreds)
|
Yes
|
|
[Principal permissions](#security_iam_service-with-iam-principal-permissions)
|
Yes
|
|
[Service
roles](#security_iam_service-with-iam-roles-service)
|
Yes
|
|
[Service-linked roles](#security_iam_service-with-iam-roles-service-linked)
|
No
|
To get a high-level view of how Step Functions and other AWS services work with most IAM
features, see [AWS services that work with IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html) in the
*IAM User Guide*.
## Identity-based policies for Step Functions
**Supports identity-based policies:**
Yes
Identity-based policies are JSON permissions policy documents that you can attach to an identity, such as an IAM user, group of users, or role. These
policies control what actions users and roles can perform, on which resources, and under what conditions. To learn how to create an identity-based
policy, see [Define custom IAM permissions with customer managed policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_create.html) in the
*IAM User Guide*.
With IAM identity-based policies, you can specify allowed or denied actions and
resources as well as the conditions under which actions are allowed or denied. To learn about all of the elements that you can use in a
JSON policy, see [IAM JSON
policy elements reference](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements.html) in the
*IAM User Guide*.
###
Identity-based policy examples for Step Functions
To view examples of Step Functions identity-based policies, see [Identity-based policy examples
for AWS Step Functions](./security_iam_id-based-policy-examples.html).
## Resource-based
policies within Step Functions
**Supports resource-based policies:**
No
Resource-based policies are JSON policy documents that you attach to a resource. Examples of resource-based policies are
IAM *role trust policies* and Amazon S3 *bucket policies*. In services that support resource-based policies, service
administrators can use them to control access to a specific resource. For the resource where the policy is attached, the policy defines what actions
a specified principal can perform on that resource and under what conditions. You must [specify a principal](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html) in a resource-based policy. Principals
can include accounts, users, roles, federated users, or AWS services.
To enable cross-account access, you can specify an entire account or IAM entities
in another account as the principal in a resource-based policy. For more information, see [Cross account resource access in IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies-cross-account-resource-access.html) in the
*IAM User Guide*.
## Policy actions
for Step Functions
**Supports policy actions:**
Yes
Administrators can use AWS JSON policies to specify who has access to what. That is, which **principal** can perform
**actions** on what **resources**, and under what **conditions**.
The `Action` element of a JSON policy describes the
actions that you can use to allow or deny access in a policy. Include actions in a policy to grant permissions to perform the associated operation.
To see a list of Step Functions actions, see [Resources Defined by AWS Step Functions](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awsstepfunctions.html) in the
*Service Authorization Reference*.
Policy actions in Step Functions use the following prefix before the action:
```
`states`
```
To specify multiple actions in a single statement, separate them with commas.
```
`"Action": [
"states:`action1`",
"states:`action2`"
]`
```
To view examples of Step Functions identity-based policies, see [Identity-based policy examples
for AWS Step Functions](./security_iam_id-based-policy-examples.html).
## Policy
resources for Step Functions
**Supports policy resources:**
Yes
Administrators can use AWS JSON policies to specify who has access to what. That is, which **principal** can perform
**actions** on what **resources**, and under what **conditions**.
The `Resource` JSON policy element specifies the object or objects to which the action applies. As a best practice, specify a resource using its [Amazon Resource Name (ARN)](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html). For actions that don't support resource-level permissions, use a wildcard (\*) to indicate that the statement applies to all resources.
```
`"Resource": "\*"`
```
To see a list of Step Functions resource types and their ARNs, see [Actions Defined by AWS Step Functions](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awsstepfunctions.html)
in the *Service Authorization Reference*. To learn with which actions you can
specify the ARN of each resource, see [Resources Defined by AWS Step Functions](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awsstepfunctions.html).
To view examples of Step Functions identity-based policies, see [Identity-based policy examples
for AWS Step Functions](./security_iam_id-based-policy-examples.html).
## Policy
condition keys for Step Functions
**Supports service-specific policy condition keys:**
Yes
Administrators can use AWS JSON policies to specify who has access to what. That is, which **principal** can perform
**actions** on what **resources**, and under what **conditions**.
The `Condition` element specifies when statements execute based on defined criteria. You can create conditional expressions that use [condition
operators](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition_operators.html), such as equals or less than, to match the condition in the
policy with values in the request. To see all AWS global
condition keys, see [AWS global condition context keys](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html) in the
*IAM User Guide*.
To see a list of Step Functions condition keys, see [Condition Keys for AWS Step Functions](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awsstepfunctions.html) in the
*Service Authorization Reference*. To learn with which actions and resources you
can use a condition key, see [Resources Defined by AWS Step Functions](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awsstepfunctions.html).
If your policy must depend on the Step Functions service principal name, we recommend you check for the
existence or non-existence of `states.amazonaws.com` in the
`aws:PrincipalServiceNamesList`
[
multivalued context key](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-single-vs-multi-valued-context-keys.html#reference_policies_condition-multi-valued-context-keys), rather than the `aws:PrincipalServiceName` condition key.
The `aws:PrincipalServiceName` condition key contains only one entry from the list of
service principal names and it may not always be `states.amazonaws.com`.
The following condition block demonstrates checking for the existence of `states.amazonaws.com`.
```
`{
"Condition": {
"ForAnyValue:StringEquals": {
"aws:PrincipalServiceNamesList": "states.amazonaws.com"
}
}
}`
```
To view examples of Step Functions identity-based policies, see [Identity-based policy examples
for AWS Step Functions](./security_iam_id-based-policy-examples.html).
## ACLs in Step Functions
**Supports ACLs:**
No
Access control lists (ACLs) control which principals (account members, users, or roles) have permissions to access a resource. ACLs are
similar to resource-based policies, although they do not use the JSON policy document format.
## ABAC with Step Functions
**Supports ABAC (tags in policies):**
Partial
Attribute-based access control (ABAC) is an authorization strategy that defines permissions
based on attributes called tags. You can attach tags to IAM entities and AWS resources, then design ABAC policies to allow operations when the principal's tag matches the tag on the resource.
To control access based on tags, you provide tag information in the [condition element](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html) of a policy using the `aws:ResourceTag/`key-name``,
`aws:RequestTag/`key-name``, or `aws:TagKeys` condition keys.
If a service supports all three condition keys for every resource type, then the value is **Yes** for the service. If a service supports all three condition keys for only some resource types, then the value is **Partial**.
For more information about ABAC, see [Define permissions with ABAC authorization](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction_attribute-based-access-control.html) in the *IAM User Guide*. To view a tutorial with steps for setting up ABAC, see
[Use attribute-based access control (ABAC)](https://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_attribute-based-access-control.html) in the *IAM User Guide*.
## Using temporary
credentials with Step Functions
**Supports temporary credentials:**
Yes
Temporary credentials provide short-term access to AWS resources and are automatically created when you use federation or switch roles. AWS recommends that you
dynamically generate temporary credentials instead of using long-term access keys. For
more information, see [Temporary
security credentials in IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html) and [AWS services
that work with IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html) in the *IAM User Guide*.
## Cross-service
principal permissions for Step Functions
**Supports forward access sessions (FAS):**
Yes
Forward access sessions (FAS) use the permissions of the principal calling an AWS service, combined with the requesting AWS service to make requests to downstream services. For policy details
when making FAS requests, see [Forward access sessions](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_forward_access_sessions.html).
## Service roles for
Step Functions
**Supports service roles:**
Yes
A service role is an [IAM role](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html) that a service assumes to perform
actions on your behalf. An IAM administrator can create, modify, and delete a service role from within IAM. For
more information, see [Create a role to delegate permissions to an AWS service](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-service.html) in the *IAM User Guide*.
###### Warning
Changing the permissions for a service role might break Step Functions functionality.
Edit service roles only when Step Functions provides guidance to do so.
## Service-linked
roles for Step Functions
**Supports service-linked roles:**
No
A service-linked role is a type of service role that is linked to an AWS service. The service can assume the role to perform an action on your behalf.
Service-linked roles appear in your AWS account and are owned by the service. An IAM administrator can view,
but not edit the permissions for service-linked roles.
For details about creating or managing service-linked roles, see [AWS services
that work with IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html). Find a service in the table that includes a
`Yes` in the **Service-linked role** column. Choose the
**Yes** link to view the service-linked role documentation for that
service.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Identity and Access Management
Identity-based policy examples
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.