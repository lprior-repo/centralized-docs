---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-resource-policies-aws-condition-keys.html
title: AWS condition keys
word_count: 360
filtered: true
elements_removed: 0
density_score: 0.91
---

AWS condition keys that can be used in API Gateway resource policies - Amazon API Gateway
AWS condition keys that can be used in API Gateway resource policies - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-resource-policies-aws-condition-keys)
# AWS condition keys
that can be used in API Gateway resource policies
The following table contains AWS condition keys that can be used
in resource policies for APIs in API Gateway for each authorization type.
For more information about AWS condition keys, see [AWS Global Condition
Context Keys](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html).
|
**Condition keys**
|
**Criteria**
|
**Needs `AuthN`?**
|
**Authorization type**
|
|`aws:CurrentTime`|None|No|All|
|`aws:EpochTime`|None|No|All|
|`aws:TokenIssueTime`|Key is present only in requests that are signed using temporary
security credentials.|Yes|IAM|
|`aws:MultiFactorAuthPresent`|Key is present only in requests that are signed using temporary
security credentials.|Yes|IAM|
|`aws:MultiFactorAuthAge`|Key is present only if MFA is present in the requests.|Yes|IAM|
|`aws:PrincipalAccount`|None|Yes|IAM|
|`aws:PrincipalArn`|None|Yes|IAM|
|`aws:PrincipalOrgID`|This key is included in the request context only if the principal is a member of an organization.|Yes|IAM|
|`aws:PrincipalOrgPaths`|This key is included in the request context only if the principal is a member of an organization.|Yes|IAM|
|`aws:PrincipalTag`|This key is included in the request context if the principal is using an IAM user with attached tags. It is included for a
principal using an IAM role with attached tags or session tags.|Yes|IAM|
|`aws:PrincipalType`|None|Yes|IAM|
|`aws:Referer`|Key is present only if the value is provided by the caller in the
HTTP header.|No|All|
|`aws:SecureTransport`|None|No|All|
|`aws:SourceArn`|None|No|All|
|`aws:SourceIp`|None|No|All|
|`aws:SourceVpc`|This key can be used only for private APIs.|No|All|
|`aws:SourceVpce`|This key can be used only for private APIs.|No|All|
|`aws:VpcSourceIp`|This key can be used only for private APIs.|No|All|
|`aws:UserAgent`|Key is present only if the value is provided by the caller in the
HTTP header.|No|All|
|`aws:userid`|None|Yes|IAM|
|`aws:username`|None|Yes|IAM|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create and attach an API Gateway
resource policy to an API
Use IAM permissions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.