---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_RoutingConfigurationListItem.html
title: RoutingConfigurationListItem
word_count: 159
filtered: true
elements_removed: 0
density_score: 0.84
---

RoutingConfigurationListItem - AWS Step Functions
RoutingConfigurationListItem - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_RoutingConfigurationListItem)
[Contents](#API_RoutingConfigurationListItem_Contents)[See Also](#API_RoutingConfigurationListItem_SeeAlso)
# RoutingConfigurationListItem
Contains details about the routing configuration of a state machine alias. In a routing
configuration, you define an array of objects that specify up to two state machine versions.
You also specify the percentage of traffic to be routed to each version.
## Contents
**
stateMachineVersionArn
**
The Amazon Resource Name (ARN) that identifies one or two state machine versions defined in the routing configuration.
If you specify the ARN of a second version, it must belong to the same state machine as the first version.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
weight
**
The percentage of traffic you want to route to a state machine version. The sum of the
weights in the routing configuration must be equal to 100.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 100.
Required: Yes