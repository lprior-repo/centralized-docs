---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ProvisionedPollerConfig.html
title: ProvisionedPollerConfig
word_count: 248
filtered: true
elements_removed: 0
density_score: 0.81
---

ProvisionedPollerConfig - AWS Lambda
ProvisionedPollerConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ProvisionedPollerConfig)
[Contents](#API_ProvisionedPollerConfig_Contents)[See Also](#API_ProvisionedPollerConfig_SeeAlso)
# ProvisionedPollerConfig
The [
provisioned mode](https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventsourcemapping.html#invocation-eventsourcemapping-provisioned-mode) configuration for the event source. Use Provisioned Mode to customize the minimum and maximum number of event pollers
for your event source.
## Contents
**
MaximumPollers
**
The maximum number of event pollers this event source can scale up to. For Amazon SQS events source mappings, default is 200, and minimum value allowed is 2. For Amazon MSK and self-managed Apache Kafka event source mappings, default is 200, and minimum value allowed is 1.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 2000.
Required: No
**
MinimumPollers
**
The minimum number of event pollers this event source can scale down to. For Amazon SQS events source mappings, default is 2, and minimum 2 required. For Amazon MSK and self-managed Apache Kafka event source mappings, default is 1.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 200.
Required: No
**
PollerGroupName
**
(Amazon MSK and self-managed Apache Kafka) The name of the provisioned poller group. Use this option to group multiple ESMs within the event source's VPC to share Event Poller Unit (EPU) capacity. You can use this option to optimize Provisioned mode costs for your ESMs. You can group up to 100 ESMs per poller group and aggregate maximum pollers across all ESMs in a group cannot exceed 2000.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 128.
Pattern: `[a-zA-Z0-9-\_]\*`
Required: No