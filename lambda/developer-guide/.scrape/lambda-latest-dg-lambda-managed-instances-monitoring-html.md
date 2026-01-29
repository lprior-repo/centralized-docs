---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-monitoring.html
title: Monitoring Lambda Managed Instances
word_count: 628
filtered: true
elements_removed: 0
density_score: 0.91
---

Monitoring Lambda Managed Instances - AWS Lambda
Monitoring Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-monitoring)
[Available metrics](#lambda-managed-instances-available-metrics)[Metric frequency and retention](#lambda-managed-instances-metric-frequency)[Viewing metrics in CloudWatch](#lambda-managed-instances-viewing-metrics)[Using metrics to optimize performance](#lambda-managed-instances-using-metrics)[Next steps](#lambda-managed-instances-monitoring-next-steps)
# Monitoring Lambda Managed Instances
You can monitor Lambda Managed Instances using CloudWatch metrics. Lambda automatically publishes metrics to CloudWatch to help you monitor resource utilization, track costs, and optimize performance.
## Available metrics
Lambda Managed Instances provides metrics at two levels: capacity provider level and execution environment level.
### Capacity provider level metrics
Capacity provider level metrics provide visibility into overall resource utilization across your instances. These metrics use the following dimensions:
* **CapacityProviderName** - The name of your capacity provider
* **InstanceType** - The EC2 instance type
**Resource utilization metrics:**
* **CPUUtilization** - The percentage of CPU utilization across instances in the capacity provider
* **MemoryUtilization** - The percentage of memory utilization across instances in the capacity provider
* **NetworkOut** - Network traffic sent through the customer ENI (in bytes)
* **NetworkIn** - Network traffic received through the customer ENI (in bytes)
* **DiskReadBytes** - Read traffic from local storage across instances (in bytes)
* **DiskWriteBytes** - Write traffic to local storage across instances (in bytes)
**Capacity metrics:**
* **vCPUAvailable** - The amount of vCPU available on instances for allocation (in count)
* **MemoryAvailable** - The amount of memory available on instances for allocation (in bytes)
* **vCPUAllocated** - The amount of vCPU allocated on instances for execution environments (in count)
* **MemoryAllocated** - The amount of memory allocated on instances for execution environments (in bytes)
### Execution environment level metrics
Execution environment level metrics provide visibility into resource utilization and concurrency for individual functions. These metrics use the following dimensions:
* **CapacityProviderName** - The name of your capacity provider
* **FunctionName** - The name of your Lambda function
**Available execution environment metrics:**
* **ExecutionEnvironmentConcurrency** - The maximum concurrency over a 5-minute sample period
* **ExecutionEnvironmentConcurrencyLimit** - The maximum concurrency limit per execution environment
* **ExecutionEnvironmentCPUUtilization** - The percentage of CPU utilization for the function's execution environments
* **ExecutionEnvironmentMemoryUtilization** - The percentage of memory utilization for the function's execution environments
## Metric frequency and retention
Lambda Managed Instances metrics are published at 5-minute intervals and retained for 15 months.
## Viewing metrics in CloudWatch
**To view Lambda Managed Instances metrics in the CloudWatch console**
1. Open the CloudWatch console at [console.aws.amazon.com/cloudwatch/](http://console.aws.amazon.com/cloudwatch/).
2. In the navigation pane, choose **Metrics**.
3. In the **All metrics** tab, choose **AWS/Lambda**.
4. Choose the metric dimension you want to view:
* For capacity provider level metrics, filter by **CapacityProviderName** and **InstanceType**
* For execution environment level metrics, filter by **CapacityProviderName** and **FunctionName**
* Select the metrics you want to monitor.
## Using metrics to optimize performance
Monitor CPU and memory utilization to understand if your functions are properly sized. High utilization may indicate the need for larger instance types or increased function memory allocation. Track concurrency metrics to understand scaling behavior and identify potential throttling.
Monitor capacity metrics to ensure sufficient resources are available for your workloads. The **vCPUAvailable** and **MemoryAvailable** metrics help you understand remaining capacity on your instances.
## Next steps
* Learn about [scaling Lambda Managed Instances](./lambda-managed-instances-scaling.html)
* Review runtime-specific guides for [Java](./lambda-managed-instances-java-runtime.html), [Node.js](./lambda-managed-instances-nodejs-runtime.html), and [Python](./lambda-managed-instances-python-runtime.html)
* Configure [VPC connectivity for your capacity providers](./lambda-managed-instances-networking.html)
* Understand [security and permissions for Lambda Managed Instances](./lambda-managed-instances-security.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Networking
Quotas
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.