---
url: https://docs.aws.amazon.com/lambda/latest/api/API_EventSourceMappingMetricsConfig.html
title: EventSourceMappingMetricsConfig
word_count: 104
filtered: true
elements_removed: 0
density_score: 0.93
---

EventSourceMappingMetricsConfig - AWS Lambda
EventSourceMappingMetricsConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_EventSourceMappingMetricsConfig)
[Contents](#API_EventSourceMappingMetricsConfig_Contents)[See Also](#API_EventSourceMappingMetricsConfig_SeeAlso)
# EventSourceMappingMetricsConfig
The metrics configuration for your event source. Use this configuration object to define which metrics you want your
event source mapping to produce.
## Contents
**
Metrics
**
The metrics you want your event source mapping to produce. Include `EventCount` to receive event source mapping
metrics related to the number of events processed by your event source mapping. For more information about these metrics,
see [
Event source mapping metrics](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-metrics-types.html#event-source-mapping-metrics).
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 1 item.
Valid Values: `EventCount`
Required: No