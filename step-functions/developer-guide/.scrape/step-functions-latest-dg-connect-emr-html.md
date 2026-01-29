---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-emr.html
title: Create and manage Amazon EMR clusters with Step Functions
word_count: 1699
filtered: true
elements_removed: 0
density_score: 0.77
---

Create and manage Amazon EMR clusters with Step Functions - AWS Step Functions
Create and manage Amazon EMR clusters with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-emr)
[Supported APIs](#connect-emr-api)[Examples](#connect-emr-api-examples)[IAM policies](#emr-iam)
# Create and manage Amazon EMR clusters with Step Functions
Learn how to integrate AWS Step Functions with Amazon EMR using the provided Amazon EMR service integration
APIs. The service integration APIs are similar to the corresponding Amazon EMR APIs, with some
differences in the fields that are passed and in the responses that are returned.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized Amazon EMR integration
* The Optimized Amazon EMR service integration has a customized set of APIs that wrap the
underlying Amazon EMR APIs, described below. Because of this, it differs significantly from
the Amazon EMR AWS SDK service integration.
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration pattern is supported.
Step Functions does not terminate an Amazon EMR cluster automatically if execution is stopped. If your
state machine stops before your Amazon EMR cluster has terminated, your cluster may continue
running indefinitely, and can accrue additional charges. To avoid this, ensure that any
Amazon EMR cluster you create is terminated properly. For more information, see:
* [Control Cluster
Termination](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-termination.html) in the Amazon EMR User Guide.
* The Service Integration Patterns [Run a Job (.sync)](./connect-to-resource.html#connect-sync) section.
###### Note
As of `emr-5.28.0`, you can specify the parameter
`StepConcurrencyLevel` when creating a cluster to allow multiple steps to
run in parallel on a single cluster. You can use the Step Functions `Map` and
`Parallel` states to submit work in parallel to the cluster.
The availability of Amazon EMR service integration is subject to the availability of Amazon EMR
APIs. See [Amazon EMR](https://docs.aws.amazon.com//govcloud-us/latest/UserGuide/govcloud-emr.html) documentation for
limitations in special regions.
###### Note
For integration with Amazon EMR, Step Functions has a hard-coded 60 seconds job polling frequency for the first 10 minutes and 300 seconds after that.
## Optimized Amazon EMR APIs
The following table describes the differences between each Amazon EMR service integration API and corresponding Amazon EMR APIs.
|Amazon EMR Service Integration API|Corresponding EMR API|Differences|
|*createCluster*
Creates and starts running a cluster (job flow).
Amazon EMR
is linked directly to a unique type of IAM role known as a
service-linked role. For `createCluster` and
`createCluster.sync` to work, you must have configured
the necessary permissions to create the service-linked role
`AWSServiceRoleForEMRCleanup`. For more information about
this, including a statement you can add to your IAM permissions
policy, see [Using the Service-Linked Role for Amazon EMR](https://docs.aws.amazon.com/emr/latest/ManagementGuide/using-service-linked-roles.html).
|[runJobFlow](https://docs.aws.amazon.com/emr/latest/APIReference/API_RunJobFlow.html)|`createCluster` uses the same request syntax as [runJobFlow](https://docs.aws.amazon.com/emr/latest/APIReference/API_RunJobFlow.html), except for the following:
* The field `Instances.KeepJobFlowAliveWhenNoSteps`
is mandatory, and must have the Boolean value
`TRUE`.
* The field `Steps` is not allowed.
* The field `Instances.InstanceFleets[index].Name`
should be provided and must be unique if the optional
`modifyInstanceFleetByName` connector API is
used.
* The field `Instances.InstanceGroups[index].Name`
should be provided and must be unique if the optional
`modifyInstanceGroupByName` API is used.
Response is this:
```
`{
"ClusterId": "string"
}`
```
Amazon EMR uses this:
```
`{
"JobFlowId": "string"
}`
```
|
|*createCluster.sync*
Creates and starts running a cluster (job flow).
|[runJobFlow](https://docs.aws.amazon.com/emr/latest/APIReference/API_RunJobFlow.html)|The same as `createCluster`, but waits for the cluster to
reach the `WAITING` state.|
|*setClusterTerminationProtection*
Locks a cluster (job flow) so the EC2 instances in the cluster cannot
be terminated by user intervention, an API call, or a job-flow
error.
|[setTerminationProtection](https://docs.aws.amazon.com/emr/latest/APIReference/API_SetTerminationProtection.html)|Request uses
this:
```
`{
"ClusterId": "string"
}`
```
Amazon EMR uses
this:
```
`{
"JobFlowIds": ["string"]
}`
```
|
|*terminateCluster*
Shuts down a cluster (job flow).
|[terminateJobFlows](https://docs.aws.amazon.com/emr/latest/APIReference/API_TerminateJobFlows.html)|Request uses
this:
```
`{
"ClusterId": "string"
}`
```
Amazon EMR uses
this:
```
`{
"JobFlowIds": ["string"]
}`
```
|
|*terminateCluster.sync*
Shuts down a cluster (job
flow).
|[terminateJobFlows](https://docs.aws.amazon.com/emr/latest/APIReference/API_TerminateJobFlows.html)|The same as `terminateCluster`, but waits for the cluster to
terminate.|
|*addStep*
Adds a new step to a running cluster.
Optionally, you can also specify the
`[ExecutionRoleArn](https://docs.aws.amazon.com/emr/latest/APIReference/API_AddJobFlowSteps.html#EMR-AddJobFlowSteps-request-ExecutionRoleArn)`
parameter while using this API.
|
[addJobFlowSteps](https://docs.aws.amazon.com/emr/latest/APIReference/API_AddJobFlowSteps.html)
|Request uses the key `"ClusterId"`. Amazon EMR uses
`"JobFlowId"`. Request uses a single
step.
```
`{
"Step": &lt;"StepConfig object"&gt;
}`
```
Amazon EMR uses
this:
```
`{
"Steps": [&lt;StepConfig objects&gt;]
}`
```
Response is
this:
```
`{
"StepId": "string"
}`
```
Amazon EMR returns
this:
```
`{
"StepIds": [&lt;strings&gt;]
}`
```
|
|*addStep.sync*
Adds a new step to a running cluster.
Optionally, you can also specify the
`[ExecutionRoleArn](https://docs.aws.amazon.com/emr/latest/APIReference/API_AddJobFlowSteps.html#EMR-AddJobFlowSteps-request-ExecutionRoleArn)`
parameter while using this API.
|
[addJobFlowSteps](https://docs.aws.amazon.com/emr/latest/APIReference/API_AddJobFlowSteps.html)
|The same as `addStep`, but waits for the step to
complete.|
|*cancelStep*
Cancels a pending step in a running cluster.
|[cancelSteps](https://docs.aws.amazon.com/emr/latest/APIReference/API_CancelSteps.html)| Request uses
this:
```
`{
"StepId": "string"
}`
```
Amazon EMR uses
this:
```
`{
"StepIds": [&lt;strings&gt;]
}`
```
Response is
this:
```
`{
"CancelStepsInfo": &lt;CancelStepsInfo object&gt;
}`
```
Amazon EMR uses
this:
```
`{
"CancelStepsInfoList": [&lt;CancelStepsInfo objects&gt;]
}`
```
|
|*modifyInstanceFleetByName*
Modifies the target On-Demand and target Spot capacities for the
instance fleet with the specified
`InstanceFleetName`.
|[modifyInstanceFleet](https://docs.aws.amazon.com/emr/latest/APIReference/API_ModifyInstanceFleet.html)|Request is the same as for `modifyInstanceFleet`, except for
the following:
* The field `Instance.InstanceFleetId` is not
allowed.
* At runtime the `InstanceFleetId` is determined
automatically by the service integration by calling
`ListInstanceFleets` and parsing the
result.
|
|*modifyInstanceGroupByName*
Modifies the number of nodes and configuration settings of an instance
group.
|[modifyInstanceGroups](https://docs.aws.amazon.com/emr/latest/APIReference/API_ModifyInstanceGroups.html)|Request is
this:
```
`{
"ClusterId": "string",
"InstanceGroup": &lt;InstanceGroupModifyConfig object&gt;
}`
```
Amazon EMR uses a list:
```
`{
"ClusterId": ["string"],
"InstanceGroups": [&lt;InstanceGroupModifyConfig objects&gt;]
}`
```
Within the `InstanceGroupModifyConfig` object, the field
`InstanceGroupId` is not allowed.
A new field, `InstanceGroupName`, has been added. At
runtime the `InstanceGroupId` is determined automatically by
the service integration by calling `ListInstanceGroups` and
parsing the result.
|
## Workflow example
The following includes a `Task` state that creates a cluster.
```
`"Create\_Cluster": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:createCluster.sync",
"Arguments": {
"Name": "MyWorkflowCluster",
"VisibleToAllUsers": true,
"ReleaseLabel": "emr-5.28.0",
"Applications": [
{
"Name": "Hive"
}
],
"ServiceRole": "EMR\_DefaultRole",
"JobFlowRole": "EMR\_EC2\_DefaultRole",
"LogUri": "s3n://aws-logs-`account-id`-us-east-1/elasticmapreduce/",
"Instances": {
"KeepJobFlowAliveWhenNoSteps": true,
"InstanceFleets": [
{
"InstanceFleetType": "MASTER",
"Name": "MASTER",
"TargetOnDemandCapacity": 1,
"InstanceTypeConfigs": [
{
"InstanceType": "m4.xlarge"
}
]
},
{
"InstanceFleetType": "CORE",
"Name": "CORE",
"TargetOnDemandCapacity": 1,
"InstanceTypeConfigs": [
{
"InstanceType": "m4.xlarge"
}
]
}
]
}
},
"End": true
}`
```
The following includes a `Task` state that enables termination protection.
```
`"Enable\_Termination\_Protection": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:setClusterTerminationProtection",
"Arguments": {
"ClusterId": "{% $ClusterId %}",
"TerminationProtected": true
},
"End": true
}`
```
The following includes a `Task` state that submits a step to a cluster.
```
`"Step\_One": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:addStep.sync",
"Arguments": {
"ClusterId": "{% $ClusterId %}",
"ExecutionRoleArn": "arn:aws:iam::`account-id`:role/`myEMR-execution-role`",
"Step": {
"Name": "The first step",
"ActionOnFailure": "TERMINATE\_CLUSTER",
"HadoopJarStep": {
"Jar": "command-runner.jar",
"Args": [
"hive-script",
"--run-hive-script",
"--args",
"-f",
"s3://`region`.elasticmapreduce.samples/cloudfront/code/Hive\_CloudFront.q",
"-d",
"INPUT=s3://`region`.elasticmapreduce.samples",
"-d",
"OUTPUT=s3://`&lt;amzn-s3-demo-bucket&gt;`/MyHiveQueryResults/"
]
}
}
},
"End": true
}`
```
The following includes a `Task` state that cancels a step.
```
`"Cancel\_Step\_One": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:cancelStep",
"Arguments": {
"ClusterId": "{% $ClusterId %}",
"StepId": "{% $AddStepsResult.StepId %}"
},
"End": true
}`
```
The following includes a `Task` state that terminates a cluster.
```
`"Terminate\_Cluster": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:terminateCluster.sync",
"Arguments": {
"ClusterId": "{% $ClusterId %}",
},
"End": true
}`
```
The following includes a `Task` state that scales a cluster up or down for an
instance group.
```
`
"ModifyInstanceGroupByName": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:modifyInstanceGroupByName",
"Arguments": {
"ClusterId": "j-`account-id`3",
"InstanceGroupName": "MyCoreGroup",
"InstanceGroup": {
"InstanceCount": 8
}
},
"End": true
}`
```
The following includes a `Task` state that scales a cluster up or down for an
instance fleet.
```
`"ModifyInstanceFleetByName": {
"Type": "Task",
"Resource": "arn:aws:states:::elasticmapreduce:modifyInstanceFleetByName",
"Arguments": {
"ClusterId": "j-`account-id`3",
"InstanceFleetName": "MyCoreFleet",
"InstanceFleet": {
"TargetOnDemandCapacity": 8,
"TargetSpotCapacity": 0
}
},
"End": true
}`
```
## IAM policies for calling Amazon EMR
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
### `addStep`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:AddJobFlowSteps",
"elasticmapreduce:DescribeStep",
"elasticmapreduce:CancelSteps"
],
"Resource": [
"arn:aws:elasticmapreduce:us-east-1:`123456789012`:cluster/`clusterId`"
]
}
]
}`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:AddJobFlowSteps",
"elasticmapreduce:DescribeStep",
"elasticmapreduce:CancelSteps"
],
"Resource": "arn:aws:elasticmapreduce:\*:\*:cluster/\*"
}
]
}`
`
```
### `cancelStep`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "elasticmapreduce:CancelSteps",
"Resource": [
"arn:aws:elasticmapreduce:`us-east-1`:`123456789012`:cluster/myCluster-id"
]
}
]
}`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "elasticmapreduce:CancelSteps",
"Resource": "arn:aws:elasticmapreduce:\*:\*:cluster/\*"
}
]
}`
`
```
### `createCluster`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:RunJobFlow",
"elasticmapreduce:DescribeCluster",
"elasticmapreduce:TerminateJobFlows"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::`123456789012`:role/`myRoleName`"
]
}
]
}`
`
```
### `setClusterTerminationProtection`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "elasticmapreduce:SetTerminationProtection",
"Resource": [
"arn:aws:elasticmapreduce:`us-east-1`:`123456789012`:cluster/myCluster-id"
]
}
]
}`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "elasticmapreduce:SetTerminationProtection",
"Resource": "arn:aws:elasticmapreduce:\*:\*:cluster/\*"
}
]
}`
`
```
### `modifyInstanceFleetByName`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:ModifyInstanceFleet",
"elasticmapreduce:ListInstanceFleets"
],
"Resource": [
"arn:aws:elasticmapreduce:`us-east-1`:`123456789012`:cluster/myCluster-id"
]
}
]
}`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:ModifyInstanceFleet",
"elasticmapreduce:ListInstanceFleets"
],
"Resource": "arn:aws:elasticmapreduce:\*:\*:cluster/\*"
}
]
}`
`
```
### `modifyInstanceGroupByName`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:ModifyInstanceGroups",
"elasticmapreduce:ListInstanceGroups"
],
"Resource": [
"arn:aws:elasticmapreduce:`us-east-1`:`123456789012`:cluster/myCluster-id"
]
}
]
}`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:ModifyInstanceGroups",
"elasticmapreduce:ListInstanceGroups"
],
"Resource": "\*"
}
]
}`
`
```
### `terminateCluster`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:TerminateJobFlows",
"elasticmapreduce:DescribeCluster"
],
"Resource": [
"arn:aws:elasticmapreduce:`us-east-1`:`123456789012`:cluster/myCluster-id"
]
}
]
}`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"elasticmapreduce:TerminateJobFlows",
"elasticmapreduce:DescribeCluster"
],
"Resource": "arn:aws:elasticmapreduce:\*:\*:cluster/\*"
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon EKS
Amazon EMR on EKS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.