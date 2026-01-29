---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtime-management-shared.html
title: Understanding the shared responsibility model for Lambda runtime management
word_count: 539
filtered: true
elements_removed: 0
density_score: 0.84
---

Understanding the shared responsibility model for Lambda runtime management - AWS Lambda
Understanding the shared responsibility model for Lambda runtime management - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtime-management-shared)
# Understanding the shared responsibility model for Lambda runtime management
Lambda is responsible for curating and publishing security updates for all supported
managed runtimes and container images. Responsibility for updating existing functions to use
the latest runtime version varies depending on which runtime update mode you use.
Lambda is responsible for applying runtime updates to all functions configured to use the
**Auto** runtime update mode.
For functions configured with the **Function update** runtime update
mode, you're responsible for regularly updating your function. Lambda is responsible for
applying runtime updates when you make those updates. If you don't update your function, then
Lambda doesn't update the runtime. If you don't regularly update your function, then we
strongly recommend configuring it for automatic runtime updates so that it continues to
receive security updates.
For functions configured to use the **Manual** runtime update mode,
you're responsible for updating your function to use the latest runtime version. We strongly
recommend that you use this mode only to roll back the runtime version as a temporary
mitigation in the rare event of runtime update incompatibility. We also recommend that you
change to **Auto** mode as quickly as possible to minimize the time in which
your functions aren't patched.
If you're [using container images to deploy your
functions](./images-create.html), then Lambda is responsible for publishing updated base images. In this
case, you're responsible for rebuilding your function's container image from the latest base
image and redeploying the container image.
This is summarized in the following table:
|Deployment mode|Lambda's responsibility|Customer's responsibility|
|Managed runtime, **Auto** mode|
Publish new runtime versions containing the latest patches.
Apply runtime patches to existing functions.
|Roll back to a previous runtime version in the rare event of a runtime update
compatibility issue. Follow best practices for [backward compatibility](./runtimes-update.html#runtime-update-compatibility).|
|Managed runtime, **Function update** mode|Publish new runtime versions containing the latest patches.|
Update functions regularly to pick up the latest runtime version.
Switch a function to **Auto** mode when you're not regularly
updating the function.
Roll back to a previous runtime version in the rare event of a runtime update
compatibility issue. Follow best practices for [backward compatibility](./runtimes-update.html#runtime-update-compatibility).
|
|Managed runtime, **Manual** mode|Publish new runtime versions containing the latest patches.|
Use this mode only for temporary runtime rollback in the rare event of a runtime
update compatibility issue.
Switch functions to **Auto** or **Function
update** mode and the latest runtime version as soon as possible.
|
|Container image|Publish new container images containing the latest patches.|Redeploy functions regularly using the latest container base image to pick up the
latest patches.|
For more information about shared responsibility with AWS, see [Shared Responsibility Model](https://aws.amazon.com/compliance/shared-responsibility-model/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtime version updates
Permissions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.