---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-update.html
title: Understanding how Lambda manages runtime version updates
word_count: 1444
filtered: true
elements_removed: 0
density_score: 0.85
---

Understanding how Lambda manages runtime version updates - AWS Lambda
Understanding how Lambda manages runtime version updates - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtimes-update)
[Backward compatibility](#runtime-update-compatibility)[Runtime update modes](#runtime-management-controls)[Two-phase runtime version rollout](#runtime-management-two-phase)
# Understanding how Lambda manages runtime version updates
Lambda keeps each managed runtime up to date with security updates, bug fixes, new features,
performance enhancements, and support for minor version releases. These runtime updates are
published as *runtime versions*. Lambda applies runtime updates to functions
by migrating the function from an earlier runtime version to a new runtime version.
By default, for functions using managed runtimes, Lambda applies runtime updates automatically.
With automatic runtime updates, Lambda takes on the operational burden of patching the
runtime versions. For most customers, automatic updates are the right choice. You can change
this default behavior by [configuring
runtime management settings](./runtime-management-configure-settings.html).
Lambda also publishes each new runtime version as a container image. To update runtime
versions for container-based functions, you must [create a new
container image](./images-create.html) from the updated base image and redeploy your function.
Each runtime version is associated with a version number and an ARN (Amazon Resource Name).
Runtime version numbers use a numbering scheme that Lambda defines, independent of the version
numbers that the programming language uses. Runtime version numbers are not always sequential. For example, version 42 might be followed by version 45. The runtime version ARN is a unique identifier for
each runtime version. You can view the ARN of your function's current runtime version in the
Lambda console, or the [INIT\_START
line of your function logs](./runtime-management-identify.html).
Runtime versions should not be confused with runtime identifiers. Each runtime has a unique
**runtime identifier**, such as `python3.14` or
`nodejs24.x`. These correspond to each major programming language release. Runtime
versions describe the patch version of an individual runtime.
###### Note
The ARN for the same runtime version number can vary between AWS Regions and CPU
architectures.
###### Topics
* [Backward compatibility](#runtime-update-compatibility)
* [Runtime update modes](#runtime-management-controls)
* [Two-phase runtime version rollout](#runtime-management-two-phase)
* [Configuring Lambda runtime management settings](./runtime-management-configure-settings.html)
* [Rolling back a Lambda runtime version](./runtime-management-rollback.html)
* [Identifying Lambda runtime version changes](./runtime-management-identify.html)
* [Understanding the shared responsibility model for Lambda runtime management](./runtime-management-shared.html)
* [Controlling Lambda runtime update permissions for high-compliance applications](./runtime-management-hc-applications.html)
## Backward compatibility
Lambda strives to provide runtime updates that are backward compatible with existing functions. However, as with software patching, there are rare cases in which a runtime update can negatively impact an existing function. For example, security patches can expose an underlying issue with an existing function that depends on the previous, insecure behavior.
When building and deploying your function, it is important to understand how to manage your dependencies to avoid potential incompatibilities with a future runtime update. For example, suppose your function has a dependency on package A, which in turn depends on package B. Both packages are included in the Lambda runtime (for example, they could be parts of the SDK or its dependencies, or parts of the runtime system libraries).
Consider the following scenarios:
|Deployment|Patching compatible|Reason|
|
* **Package A:** Use from runtime
* **Package B:** Use from runtime
|Yes|Future runtime updates to packages A and B are backward compatible.|
|
* **Package A:** In deployment package
* **Package B:** In deployment package
|Yes|Your deployment takes precedence, so future runtime updates to packages A and B
have no effect.|
|
* **Package A:** In deployment package
* **Package B:** Use from runtime
|Yes\*|
Future runtime updates to package B are backward compatible.
\*If A and B are tightly coupled, compatibility issues can occur. For example, the
`boto3` and `botocore` packages in the AWS SDK for Python should be deployed
together.
|
|
* **Package A:** Use from runtime
* **Package B:** In deployment package
|No|Future runtime updates to package A might require an updated version of package B.
However, the deployed version of package B takes precedence, and might not be
forward compatible with the updated version of package A.|
To maintain compatibility with future runtime updates, follow these best practices:
* **When possible, package all dependencies:** Include all required libraries, including the AWS SDK and its dependencies, in your deployment package. This ensures a stable, compatible set of components.
* **Use runtime-provided SDKs sparingly:** Only rely on the runtime-provided SDK when you can't include additional packages (for example, when using the Lambda console code editor or inline code in an AWS CloudFormation template).
* **Avoid overriding system libraries:** Don't deploy custom operating system libraries that may conflict with future runtime updates.
## Runtime update modes
Lambda strives to provide runtime updates that are backward compatible with existing
functions. However, as with software patching, there are rare cases in which a runtime update
can negatively impact an existing function. For example, security patches can expose an
underlying issue with an existing function that depends on the previous, insecure behavior.
Lambda runtime management controls help reduce the risk of impact to your workloads in the rare
event of a runtime version incompatibility. For each [function version](./configuration-versions.html) (`$LATEST` or published version), you can choose one of
the following runtime update modes:
* Auto (default) – Automatically update to the most recent
and secure runtime version using [Two-phase runtime version rollout](#runtime-management-two-phase). We recommend this mode for most
customers so that you always benefit from runtime updates.
* Function update – Update to the most recent and secure runtime version when you update your function. When you update your function, Lambda
updates the runtime of your function to the most recent and secure runtime version. This
approach synchronizes runtime updates with function deployments, giving you control over
when Lambda applies runtime updates. With this mode, you can detect and mitigate rare
runtime update incompatibilities early. When using this mode, you must regularly update
your functions to keep their runtime up to date.
* Manual – Manually update your runtime version. You specify a runtime version in your function
configuration. The function uses this runtime version indefinitely. In the rare case in
which a new runtime version is incompatible with an existing function, you can use this
mode to roll back your function to an earlier runtime version. We recommend against using
**Manual** mode to try to achieve runtime consistency across
deployments. For more information, see
[Rolling back a Lambda runtime version](./runtime-management-rollback.html).
Responsibility for applying runtime updates to your functions varies according to which runtime update mode you choose. For more information, see
[Understanding the shared responsibility model for Lambda runtime management](./runtime-management-shared.html).
## Two-phase runtime version rollout
Lambda introduces new runtime versions in the following order:
1. In the first
phase, Lambda applies the new runtime version whenever you create or update a function. A function gets updated when you call the
[UpdateFunctionCode](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionCode.html) or [UpdateFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionConfiguration.html) API operations.
2. In the second phase, Lambda updates any function that uses the **Auto** runtime update mode and that hasn't already
been updated to the new runtime version.
The overall duration of the rollout process varies
according to multiple factors, including the severity of any security patches included in the
runtime update.
If you're actively developing and deploying your functions, you will most likely pick up
new runtime versions during the first phase. This synchronizes runtime updates with function
updates. In the rare event that the latest runtime version negatively impacts your
application, this approach lets you take prompt corrective action. Functions that aren't
in active development still receive the operational benefit of automatic runtime updates
during the second phase.
This approach doesn't affect functions set to **Function update** or
**Manual** mode. Functions using **Function update** mode
receive the latest runtime updates only when you create or update them. Functions using
**Manual** mode don't receive runtime updates.
Lambda publishes new runtime versions in a gradual, rolling fashion across AWS Regions.
If your functions are set to **Auto** or **Function update**
modes, it's possible that functions deployed at the same time to different Regions, or at
different times in the same Region, will pick up different runtime versions. Customers who
require guaranteed runtime version consistency across their environments should [use container images to deploy their Lambda functions](./images-create.html).
The **Manual** mode is designed as a temporary mitigation to enable runtime version
rollback in the rare event that a runtime version is incompatible with your function.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda runtimes
Configuring runtime management
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.