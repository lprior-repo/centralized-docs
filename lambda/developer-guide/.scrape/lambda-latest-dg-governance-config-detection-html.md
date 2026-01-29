---
url: https://docs.aws.amazon.com/lambda/latest/dg/governance-config-detection.html
title: Detect non-compliant Lambda deployments and configurations with AWS Config
word_count: 961
filtered: true
elements_removed: 0
density_score: 0.85
---

Detect non-compliant Lambda deployments and configurations with AWS Config - AWS Lambda
Detect non-compliant Lambda deployments and configurations with AWS Config - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#governance-config-detection)
[Phase 1: Identify access resources](#governance-config-detective-identify)[Phase 2: Visualize and design](#governance-config-detective-visualize)[Phase 3: Implement and enforce](#governance-config-detective-implement)
# Detect non-compliant Lambda deployments and configurations with AWS Config
In addition to [proactive evaluation](./governance-config.html), AWS Config can also reactively detect resource deployments
and configurations that do not comply with your governance policies. This is important because
governance policies evolve as your organization learns and implements new best
practices.
Consider a scenario where you set a brand new policy when deploying or updating Lambda functions:
All Lambda functions must always use a specific, approved Lambda layer version. You can configure
AWS Config to monitor new or updated functions for layer configurations. If AWS Config detects a function
that is not using an approved layer version, it flags the function as a non-compliant resource.
You can optionally configure AWS Config to automatically remediate the resource by specifying a remediation
action using an AWS Systems Manager automation document. For example, you could write an
automation document in Python using the AWS SDK for Python (Boto3), which updates the
non-compliant function to point to the approved layer version. Thus, AWS Config serves as both a
detective and corrective control, automating compliance management.
Let's break down this process into three important implementation phases:
![The three implementation phases are identify, notify, and deploy remediation.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/governance-config-detective-1.png)
## Phase 1: Identify access resources
Start by activating AWS Config across your accounts and configuring it to record AWS
Lambda functions. This allows AWS Config to observe when Lambda functions are created or updated.
You can then configure
[custom
policy rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_develop-rules_cfn-guard.html) to check for specific policy violations, which use AWS CloudFormation Guard
syntax. Guard rules take the following general form:
```
`rule name when condition { assertion }`
```
Below is a sample rule that checks to ensure that a layer is not set to an old layer version:
```
`rule desiredlayer when configuration.layers !empty {
some configuration.layers[\*].arn != CONFIG\_RULE\_PARAMETERS.OldLayerArn
}`
```
Let's understand the rule syntax and structure:
* **Rule name:** The name of the rule in the provided example is `desiredlayer`.
* **Condition:** This clause specifies the condition under which the rule should be checked. In the
provided example, the condition is `configuration.layers !empty`. This means
the resource should be evaluated only when the `layers` property in the
configuration isn't empty.
* **Assertion:** After the `when` clause, an assertion determines what the rule
checks. The assertion
`some configuration.layers[\*].arn != CONFIG\_RULE\_PARAMETERS.OldLayerArn`
checks if any of the Lambda layer ARNs do not match the `OldLayerArn` value.
If they do not match, the assertion is true and the rule passes; otherwise, it fails.
`CONFIG\_RULE\_PARAMETERS` is a special set of parameters that is configured with
the AWS Config rule. In this case, `OldLayerArn` is a parameter inside
`CONFIG\_RULE\_PARAMETERS`. This allows users to provide a specific ARN value that
they consider old or deprecated, and then the rule checks if any Lambda functions are using this
old ARN.
## Phase 2: Visualize and design
AWS Config gathers
configuration data and stores that data in Amazon Simple Storage Service (Amazon S3) buckets. You can use
[Amazon Athena](https://aws.amazon.com/athena/) to query this data directly
from your S3 buckets. With Athena, you can aggregate this data at the organizational level,
generating a holistic view of your resource configurations across all your accounts. To set up
aggregation of resource configuration data, see
[Visualizing AWS Config data using Athena and Amazon Quick Suite](https://aws.amazon.com/blogs/mt/visualizing-aws-config-data-using-amazon-athena-and-amazon-quicksight/) on the AWS Cloud Operations and Management blog.
The following is a sample Athena query to identify all Lambda functions using a particular layer ARN:
```
`WITH unnested AS (
SELECT
item.awsaccountid AS account\_id,
item.awsregion AS region,
item.configuration AS lambda\_configuration,
item.resourceid AS resourceid,
item.resourcename AS resourcename,
item.configuration AS configuration,
json\_parse(item.configuration) AS lambda\_json
FROM
default.aws\_config\_configuration\_snapshot,
UNNEST(configurationitems) as t(item)
WHERE
"dt" = 'latest'
AND item.resourcetype = 'AWS::Lambda::Function'
)
SELECT DISTINCT
region as Region,
resourcename as FunctionName,
json\_extract\_scalar(lambda\_json, '$.memorySize') AS memory\_size,
json\_extract\_scalar(lambda\_json, '$.timeout') AS timeout,
json\_extract\_scalar(lambda\_json, '$.version') AS version
FROM
unnested
WHERE
lambda\_configuration LIKE '%arn:aws:lambda:us-east-1:111122223333:layer:AnyGovernanceLayer:24%'`
```
Here are results from the query:
![Query results in Athena console.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/governance-config-detective-2.png)
With the AWS Config data aggregated across the organization, you can then create a dashboard using
[Amazon Quick Suite](https://aws.amazon.com/quicksight/). By importing your
Athena results into Quick Suite, you can visualize how well your Lambda functions adhere to the
layer version rule. This dashboard can highlight compliant and non-compliant resources, which
helps you to determine your enforcement policy, as outlined in the [next section](#governance-config-detective-implement). The following image
is an example dashboard that reports on the distribution of layer versions applied to functions
within the organization.
![Example Quick Suite dashboard shows distribution of layer versions in Lambda functions.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/governance-config-detective-3.png)
## Phase 3: Implement and enforce
You can now optionally pair your layer version rule that you created in [phase 1](#governance-config-detective-identify) with a remediation
action via a Systems Manager automation document, which you author as a Python script written
with AWS SDK for Python (Boto3). The script calls the
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionConfiguration.html) API action for each Lambda function, updating the
function configuration with the new layer ARN. Alternatively, you could have the script submit a pull request to the code repository to update the layer ARN. This way future code
deployments are also updated with the correct layer ARN.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Proactive controls with AWS Config
Code signing
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.