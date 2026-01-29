---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-emr-eks.html
title: Create and manage Amazon EMR clusters on EKS with AWS Step Functions
word_count: 581
filtered: true
elements_removed: 0
density_score: 0.83
---

Create and manage Amazon EMR clusters on EKS with AWS Step Functions - AWS Step Functions
Create and manage Amazon EMR clusters on EKS with AWS Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-emr-eks)
# Create and manage Amazon EMR clusters on EKS with AWS Step Functions
Learn how to integrate AWS Step Functions with Amazon EMR on EKS using the Amazon EMR on EKS service
integration APIs. The service integration APIs are the same as the corresponding Amazon EMR on
EKS APIs, but not all APIs support all integration patterns, as shown in the following
table.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### How the Optimized Amazon EMR on EKS integration is different than the Amazon EMR on EKS AWS SDK
integration
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration
pattern is supported.
* There are no specific optimizations for the [Request Response](./connect-to-resource.html#connect-default) integration pattern.
* The [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token)
integration pattern is not supported.
###### Note
For integration with Amazon EMR, Step Functions has a hard-coded 60 seconds job polling frequency for the first 10 minutes and 300 seconds after that.
|API|Request response|Run a job (.sync)|
|CreateVirtualCluster|Supported|*Not supported*|
|DeleteVirtualCluster|Supported|Supported|
|StartJobRun|Supported|Supported|
Supported Amazon EMR on EKS APIs:
###### Quota for input or result data
When sending or receiving data between services, the maximum input or result for a task is 256 KiB of data as a UTF-8 encoded string. See [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
* [`CreateVirtualCluster`](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_CreateVirtualCluster.html)
* [Request syntax](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_CreateVirtualCluster.html#API_CreateVirtualCluster_RequestSyntax)
* [Supported parameters](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_CreateVirtualCluster.html#API_CreateVirtualCluster_RequestBody)
* [Response syntax](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_CreateVirtualCluster.html#API_CreateVirtualCluster_ResponseSyntax)
* [`DeleteVirtualCluster`](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_DeleteVirtualCluster.html)
* [Request syntax](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_DeleteVirtualCluster.html#API_DeleteVirtualCluster_RequestSyntax)
* [Supported parameters](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_DeleteVirtualCluster.html#API_DeleteVirtualCluster_RequestParameters)
* [Response syntax](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_CreateVirtualCluster.html#API_CreateVirtualCluster_ResponseSyntax)
* [`StartJobRun`](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_StartJobRun.html)
* [Request syntax](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_StartJobRun.html#API_StartJobRun_RequestSyntax)
* [Supported parameters](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_StartJobRun.html#API_StartJobRun_RequestParameters)
* [Response syntax](https://docs.aws.amazon.com/emr-on-eks/latest/APIReference/API_StartJobRun.html#API_StartJobRun_ResponseSyntax)
The following includes a `Task` state that creates a virtual cluster.
```
`"Create\_Virtual\_Cluster": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-containers:createVirtualCluster",
"Arguments": {
"Name": "MyVirtualCluster",
"ContainerProvider": {
"Id": "EKSClusterName",
"Type": "EKS",
"Info": {
"EksInfo": {
"Namespace": "Namespace"
}
}
}
},
"End": true
}`
```
The following includes a `Task` state that submits a job to a virtual cluster
and waits for it to complete.
```
`"Submit\_Job": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-containers:startJobRun.sync",
"Arguments": {
"Name": "MyJobName",
"VirtualClusterId": "{% $VirtualClusterId %}",
"ExecutionRoleArn": "arn:aws:iam::`&lt;accountId&gt;`:role/job-execution-role",
"ReleaseLabel": "emr-6.2.0-latest",
"JobDriver": {
"SparkSubmitJobDriver": {
"EntryPoint": "s3://`&lt;amzn-s3-demo-bucket&gt;`/jobs/trip-count.py",
"EntryPointArguments": [
"60"
],
"SparkSubmitParameters": "--conf spark.driver.cores=2 --conf spark.executor.instances=10 --conf spark.kubernetes.pyspark.pythonVersion=3 --conf spark.executor.memory=10G --conf spark.driver.memory=10G --conf spark.executor.cores=1 --conf spark.dynamicAllocation.enabled=false"
}
},
"ConfigurationOverrides": {
"ApplicationConfiguration": [
{
"Classification": "spark-defaults",
"Properties": {
"spark.executor.instances": "2",
"spark.executor.memory": "2G"
}
}
],
"MonitoringConfiguration": {
"PersistentAppUI": "ENABLED",
"CloudWatchMonitoringConfiguration": {
"LogGroupName": "MyLogGroupName",
"LogStreamNamePrefix": "MyLogStreamNamePrefix"
},
"S3MonitoringConfiguration": {
"LogUri": "s3://`&lt;amzn-s3-demo-logging-bucket1&gt;`"
}
}
},
"Tags": {
`"taskType"`: `"jobName"`
}
},
"End": true
}`
```
The following includes a `Task` state that deletes a virtual cluster and waits
for the deletion to complete.
```
`"Delete\_Virtual\_Cluster": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-containers:deleteVirtualCluster.sync",
"Arguments": {
"Id": "{% $states.input.VirtualClusterId %}",
},
"End": true
}`
```
To learn about configuring IAM permissions when using Step Functions with other AWS services, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon EMR
Amazon EMR Serverless
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.