---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtime-management-hc-applications.html
title: Controlling Lambda runtime update permissions for high-compliance applications
word_count: 240
filtered: true
elements_removed: 0
density_score: 0.84
---

Controlling Lambda runtime update permissions for high-compliance applications - AWS Lambda
Controlling Lambda runtime update permissions for high-compliance applications - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtime-management-hc-applications)
# Controlling Lambda runtime update permissions for high-compliance applications
To meet patching requirements, Lambda customers typically rely on automatic runtime
updates. If your application is subject to strict patching freshness requirements, you may
want to limit use of earlier runtime versions. You can restrict Lambda's runtime management
controls by using AWS Identity and Access Management (IAM) to deny users in your AWS account access to the
[PutRuntimeManagementConfig](https://docs.aws.amazon.com/lambda/latest/api/API_PutRuntimeManagementConfig.html) API operation.
This operation is used to choose the runtime update mode for a function. Denying access to
this operation causes all functions to default to the **Auto** mode. You can
apply this restriction across your organization by using a [service control policies
(SCP)](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scps.html). If you must roll back a function to an earlier runtime
version, you can grant a policy exception on a case-by-case basis.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Shared responsibility model
Get data about functions by runtime
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.