---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_UsagePlan.html
title: UsagePlan
word_count: 222
filtered: true
elements_removed: 0
density_score: 0.82
---

UsagePlan - Amazon API Gateway
UsagePlan - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_UsagePlan)
[Contents](#API_UsagePlan_Contents)[See Also](#API_UsagePlan_SeeAlso)
# UsagePlan
Represents a usage plan used to specify who can assess associated API stages. Optionally, target request rate and quota limits can be set.
In some cases clients can exceed the targets that you set. Don’t rely on usage plans to control costs.
Consider using [AWS Budgets](https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-managing-costs.html) to monitor costs
and [AWS WAF](https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html) to manage API requests.
## Contents
**
apiStages
**
The associated API stages of a usage plan.
Type: Array of [ApiStage](./API_ApiStage.html) objects
Required: No
**
description
**
The description of a usage plan.
Type: String
Required: No
**
id
**
The identifier of a UsagePlan resource.
Type: String
Required: No
**
name
**
The name of a usage plan.
Type: String
Required: No
**
productCode
**
The AWS Marketplace product identifier to associate with the usage plan as a SaaS product on the AWS Marketplace.
Type: String
Required: No
**
quota
**
The target maximum number of permitted requests per a given unit time interval.
Type: [QuotaSettings](./API_QuotaSettings.html) object
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No
**
throttle
**
A map containing method level throttling information for API stage in a usage plan.
Type: [ThrottleSettings](./API_ThrottleSettings.html) object
Required: No