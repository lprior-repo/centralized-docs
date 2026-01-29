---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-eks.html
title: Create and manage Amazon EKS clusters with Step Functions
word_count: 2910
filtered: true
elements_removed: 0
density_score: 0.77
---

Create and manage Amazon EKS clusters with Step Functions - AWS Step Functions
Create and manage Amazon EKS clusters with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-eks)
[Kubernetes API integrations](#connect-eks-kubernetes-apis)[Optimized Amazon EKS APIs](#connect-eks-apis)[Permissions](#connect-eks-permissions)[IAM policies](#eks-iam)
# Create and manage Amazon EKS clusters with Step Functions
Learn how to integrate Step Functions with Amazon EKS to manage Kubernetes clusters. Step Functions provides two types of service integration APIs for integrating with Amazon Elastic Kubernetes Service. One
lets you use the Amazon EKS APIs to create and manage an Amazon EKS cluster. The other lets you
interact with your cluster using the Kubernetes API and run jobs as part of your
application’s workflow.
You can use the Kubernetes API integrations with Amazon EKS clusters
created using Step Functions, with Amazon EKS clusters created by the **eksctl** tool or
the [Amazon EKS console](https://console.aws.amazon.com/eks/home), or similar methods. For
more information, see [Creating an Amazon EKS
cluster](https://docs.aws.amazon.com/eks/latest/userguide/create-cluster.html) in the Amazon EKS User Guide.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized Amazon EKS integration
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration
pattern is supported.
* There are no specific optimizations for the [Request Response](./connect-to-resource.html#connect-default) integration pattern.
* The [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token)
integration pattern is not supported.
###### Note
The Step Functions EKS integration supports only Kubernetes APIs with public endpoint access. By default, EKS clusters API server endpoints have public access. For more information, see [Amazon EKS cluster endpoint access control](https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html) in the **Amazon EKS User Guide**.
Step Functions does not terminate an Amazon EKS cluster automatically if execution is stopped. If your
state machine stops before your Amazon EKS cluster has terminated, your cluster may continue
running indefinitely, and can accrue additional charges. To avoid this, ensure that any
Amazon EKS cluster you create is terminated properly. For more information, see:
* [Deleting a cluster](https://docs.aws.amazon.com/eks/latest/userguide/create-cluster.html) in the
Amazon EKS User Guide.
* [Run a Job (.sync)](./connect-to-resource.html#connect-sync) in Service Integration
Patterns.
###### Quota for input or result data
When sending or receiving data between services, the maximum input or result for a task is 256 KiB of data as a UTF-8 encoded string. See [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
## Kubernetes API integrations
Step Functions supports the following Kubernetes APIs:
### RunJob
The `eks:runJob` service integration allows you to run a job on your
Amazon EKS cluster. The `eks:runJob.sync` variant allows you to wait for the
job to complete, and, optionally retrieve logs.
Your Kubernetes API server must grant permissions to the IAM role used by your
state machine. For more information, see [Permissions](#connect-eks-permissions).
For the **Run a Job** (`.sync`) pattern, the status of
the job is determined by polling. Step Functions initially polls at a rate of approximately 1
poll per minute. This rate eventually slows to approximately 1 poll every 5 minutes.
If you require more frequent polling, or require more control over the polling
strategy, you can use the `eks:call` integration to query the status of
the job.
The `eks:runJob` integration is specific to `batch/v1`
Kubernetes Jobs. For more information, see [Jobs](https://kubernetes.io/docs/concepts/workloads/controllers/job/) in the Kubernetes documentation. If you want to manage other
Kubernetes resources, including custom resources, use the `eks:call`
service integration. You can use Step Functions to build polling loops, as demonstrated in
the [Poll for job status with Lambda and AWS Batch](./sample-project-job-poller.html) sample project.
Supported parameters include:
* `ClusterName`: The name of the Amazon EKS cluster you want to
call.
* `Type`: `String`
* Required: yes
* `CertificateAuthority`: The Base64-encoded certificate data
required to communicate with your cluster. You can obtain this value from
the [Amazon EKS console](https://console.aws.amazon.com/eks/home) or by
using the Amazon EKS [DescribeCluster](https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeCluster.html) API.
* `Type`: `String`
* Required: yes
* `Endpoint`: The endpoint URL for your Kubernetes API server.
You can obtain this value from the [Amazon EKS console](https://console.aws.amazon.com/eks/home) or by using the Amazon EKS [DescribeCluster](https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeCluster.html)
API.
* `Type`: `String`
* Required: yes
* `Namespace`: The namespace in which to run the job. If not
provided, the namespace `default` is used.
* `Type`: `String`
* Required: no
* `Job`: The definition of the Kubernetes Job. See [Jobs](https://kubernetes.io/docs/concepts/workloads/controllers/job/) in the Kubernetes documentation.
* `Type`: `JSON` or `String`
* Required: yes
* `LogOptions`: A set of options to control the optional
retrieval of logs. Only applicable if the Run a Job (.sync) service
integration pattern is used to wait for the completion of the job.
* `Type`: `JSON`
* Required: no
* Logs are included in the response under the key `logs`.
There may be multiple pods within the job, each with multiple
containers.
```
`{
...
"logs": {
"pods": {
"pod1": {
"containers": {
"container1": {
"log": `&lt;log&gt; `
},
...
}
},
...
}
}`
```
* Log retrieval is performed on a best-effort basis. If there is an
error retrieving a log, in place of the `log` field there
will be the fields `error` and `cause`.
* `LogOptions.RetrieveLogs`: Enable log retrieval after the job
completes. By default, logs are not retrieved.
* `Type`: `Boolean`
* Required: no
* `LogOptions.RawLogs`: If `RawLogs` is set to true,
logs will be returned as raw strings without attempting to parse them into
JSON. By default, logs are deserialized into JSON if possible. In some cases
such parsing can introduce unwanted changes, such as limiting the precision
of numbers containing many digits.
* `Type`: `Boolean`
* Required: no
* `LogOptions.LogParameters`: The Kubernetes API’s Read Log API
supports query parameters to control log retrieval. For example, you can use
`tailLines` or `limitBytes` to limit the size of
retrieved logs and remain within the Step Functions data size quota. For more
information, see the [Read Log](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#read-log-pod-v1-core) section of the Kubernetes API Reference.
* `Type: `Map of `String` to `List of
Strings`
* Required: no
* Example:
```
`"LogParameters": {
"tailLines": [ "6" ]
}`
```
The following example includes a `Task` state that runs a job, waits
for it to complete, then retrieves the job’s logs:
```
`{
"StartAt": "Run a job on EKS",
"States": {
"Run a job on EKS": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:runJob.sync",
"Arguments": {
"ClusterName": "MyCluster",
"CertificateAuthority": "ANPAJ2UCCR6DPCEXAMPLE",
"Endpoint": "https://AKIAIOSFODNN7EXAMPLE.yl4.us-east-1.eks.amazonaws.com",
"LogOptions": {
"RetrieveLogs": true
},
"Job": {
"apiVersion": "batch/v1",
"kind": "Job",
"metadata": {
"name": "example-job"
},
"spec": {
"backoffLimit": 0,
"template": {
"metadata": {
"name": "example-job"
},
"spec": {
"containers": [
{
"name": "pi-2000",
"image": "perl",
"command": [ "perl" ],
"args": [
"-Mbignum=bpi",
"-wle",
"print bpi(2000)"
]
}
],
"restartPolicy": "Never"
}
}
}
}
},
"End": true
}
}
}`
```
### `Call`
The `eks:call` service integration allows you to use the Kubernetes API
to read and write Kubernetes resource objects via a Kubernetes API endpoint.
Your Kubernetes API server must grant permissions to the IAM role used by your
state machine. For more information, see [Permissions](#connect-eks-permissions).
For more information about the available operations, see the [Kubernetes API
Reference](https://kubernetes.io/docs/reference/kubernetes-api/).
Supported parameters for `Call` include:
* `ClusterName`: The name of the Amazon EKS cluster you want to
call.
* `Type`: String
* Required: Yes
* `CertificateAuthority`: The Base64-encoded certificate data
required to communicate with your cluster. You can obtain this value from
the [Amazon EKS console](https://console.aws.amazon.com/eks/home) or by
using the Amazon EKS [DescribeCluster](https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeCluster.html) API.
* `Type`: `String`
* Required: Yes
* `Endpoint`: The endpoint URL for your Kubernetes API server.
You can find this value on the [Amazon EKS console](https://console.aws.amazon.com/eks/home) or by using Amazon EKS’ DescribeCluster API.
* `Type`: `String`
* Required: Yes
* `Method`: The HTTP method of your request. One of:
`GET`, `POST`, `PUT`,
`DELETE`, `HEAD`, or `PATCH`.
* `Type`: `String`
* Required: Yes
* `Path`: The HTTP path of the Kubernetes REST API
operation.
* `Type`: `String`
* Required: Yes
* `QueryParameters`: The HTTP query parameters of the Kubernetes
REST API operation.
* `Type: `Map of `String` to `List of
Strings`
* Required: No
* Example:
```
`"QueryParameters": {
"labelSelector": [ "job-name=example-job" ]
}`
```
* `RequestBody`: The HTTP message body of the Kubernetes REST API
operation.
* `Type`: `JSON` or `String`
* Required: No
The following includes a `Task` state that uses `eks:call`
to list the pods belonging to the job `example-job`.
```
`{
"StartAt": "Call EKS",
"States": {
"Call EKS": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:call",
"Arguments": {
"ClusterName": "MyCluster",
"CertificateAuthority": "ANPAJ2UCCR6DPCEXAMPLE",
"Endpoint": "https://444455556666.yl4.us-east-1.eks.amazonaws.com",
"Method": "GET",
"Path": "/api/v1/namespaces/default/pods",
"QueryParameters": {
"labelSelector": [
"job-name=example-job"
]
}
},
"End": true
}
}
}`
```
The following includes a `Task` state that uses `eks:call`
to delete the job `example-job`, and sets the
`propagationPolicy` to ensure the job's pods are also deleted.
```
`{
"StartAt": "Call EKS",
"States": {
"Call EKS": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:call",
"Arguments": {
"ClusterName": "MyCluster",
"CertificateAuthority": "ANPAJ2UCCR6DPCEXAMPLE",
"Endpoint": "https://444455556666.yl4.us-east-1.eks.amazonaws.com",
"Method": "DELETE",
"Path": "/apis/batch/v1/namespaces/default/jobs/example-job",
"QueryParameters": {
"propagationPolicy": [
"Foreground"
]
}
},
"End": true
}
}
}`
```
## Optimized Amazon EKS APIs
Supported Amazon EKS APIs and syntax include:
* [`CreateCluster`](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateCluster.html)
* [Request syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateCluster.html#API_CreateCluster_RequestSyntax)
* [Response syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateCluster.html#API_CreateCluster_ResponseSyntax)
When an Amazon EKS cluster is created using the
`eks:createCluster` service integration, the IAM role
is added to the Kubernetes RBAC authorization table as the administrator
(with system:masters permissions). Initially, only that IAM entity can
make calls to the Kubernetes API server. For more information,
see:
* [Managing users
or IAM roles for your cluster](https://docs.aws.amazon.com/eks/latest/userguide/add-user-role.html) in the
*Amazon EKS User Guide*
* The [Permissions](#connect-eks-permissions) section
Amazon EKS uses service-linked roles which contain the permissions Amazon EKS
requires to call other services on your behalf. If these service-linked
roles do not exist in your account already, you must add the
`iam:CreateServiceLinkedRole` permission to the IAM
role used by Step Functions. For more information, see [Using
Service-Linked Roles](https://docs.aws.amazon.com/eks/latest/userguide/using-service-linked-roles.html) in the *Amazon EKS User Guide*.
The IAM role used by Step Functions must have `iam:PassRole`
permissions to pass the cluster IAM role to Amazon EKS. For more
information, see [Amazon
EKS cluster IAM role](https://docs.aws.amazon.com/eks/latest/userguide/service_IAM_role.html) in the *Amazon EKS User Guide*.
* [`DeleteCluster`](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteCluster.html)
* [Request syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteCluster.html#API_DeleteCluster_RequestSyntax)
* [Response syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteCluster.html#API_DeleteCluster_ResponseSyntax)
You must delete any Fargate profiles or node groups before deleting
a cluster.
* [`CreateFargateProfile`](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateFargateProfile.html)
* [Request syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateFargateProfile.html#API_CreateFargateProfile_RequestSyntax)
* [Response syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateFargateProfile.html#API_CreateFargateProfile_ResponseSyntax)
Amazon EKS uses service-linked roles which contain the permissions Amazon EKS
requires to call other services on your behalf. If these service-linked
roles do not exist in your account already, you must add the
`iam:CreateServiceLinkedRole` permission to the IAM
role used by Step Functions. For more information, see [Using
Service-Linked Roles](https://docs.aws.amazon.com/eks/latest/userguide/using-service-linked-roles.html) in the *Amazon EKS User Guide*.
Amazon EKS on Fargate may not be available in all regions. For
information on region availability, see the section on [Fargate](https://docs.aws.amazon.com/eks/latest/userguide/fargate.html) in the
*Amazon EKS User Guide*.
The IAM role used by Step Functions must have `iam:PassRole`
permissions to pass the pod execution IAM role to Amazon EKS. For more
information, see [Pod
execution role](https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html) in the *Amazon EKS User Guide*.
* [`DeleteFargateProfile`](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteFargateProfile.html)
* [Request syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteFargateProfile.html#API_DeleteFargateProfile_RequestSyntax)
* [Response syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteFargateProfile.html#API_DeleteFargateProfile_ResponseSyntax)
* [`CreateNodegroup`](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateNodegroup.html)
* [Request syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateNodegroup.html#API_CreateNodegroup_RequestSyntax)
* [Response syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateNodegroup.html#API_CreateNodegroup_ResponseSyntax)
Amazon EKS uses service-linked roles which contain the permissions Amazon EKS
requires to call other services on your behalf. If these service-linked
roles do not exist in your account already, you must add the
`iam:CreateServiceLinkedRole` permission to the IAM
role used by Step Functions. For more information, see [Using
Service-Linked Roles](https://docs.aws.amazon.com/eks/latest/userguide/using-service-linked-roles.html) in the *Amazon EKS User Guide*.
The IAM role used by Step Functions must have `iam:PassRole`
permissions to pass the node IAM role to Amazon EKS. For more information,
see [Using
Service-Linked Roles](https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html) in the *Amazon EKS User Guide*.
* [`DeleteNodegroup`](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteNodegroup.html)
* [Request syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteNodegroup.html#API_DeleteNodegroup_RequestSyntax)
* [Response syntax](https://docs.aws.amazon.com/eks/latest/APIReference/API_DeleteNodegroup.html#API_DeleteNodegroup_ResponseSyntax)
The following includes a `Task` that creates an Amazon EKS cluster.
```
`{
"StartAt": "CreateCluster.sync",
"States": {
"CreateCluster.sync": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:createCluster.sync",
"Arguments": {
"Name": "MyCluster",
"ResourcesVpcConfig": {
"SubnetIds": [
"subnet-053e7c47012341234",
"subnet-027cfea4b12341234"
]
},
"RoleArn": "arn:aws:iam::`account-id`:role/MyEKSClusterRole"
},
"End": true
}
}
}`
```
The following includes a `Task` state that deletes an Amazon EKS cluster.
```
`{
"StartAt": "DeleteCluster.sync",
"States": {
"DeleteCluster.sync": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:deleteCluster.sync",
"Arguments": {
"Name": "MyCluster"
},
"End": true
}
}
}`
```
The following includes a `Task` state that creates a Fargate
profile.
```
`{
"StartAt": "CreateFargateProfile.sync",
"States": {
"CreateFargateProfile.sync": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:createFargateProfile.sync",
"Arguments": {
"ClusterName": "MyCluster",
"FargateProfileName": "MyFargateProfile",
"PodExecutionRoleArn": "arn:aws:iam::`account-id`:role/MyFargatePodExecutionRole",
"Selectors": [{
"Namespace": "my-namespace",
"Labels": { "my-label": "my-value" }
}]
},
"End": true
}
}
}`
```
The following includes a `Task` state that deletes a Fargate
profile.
```
`{
"StartAt": "DeleteFargateProfile.sync",
"States": {
"DeleteFargateProfile.sync": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:deleteFargateProfile.sync",
"Arguments": {
"ClusterName": "MyCluster",
"FargateProfileName": "MyFargateProfile"
},
"End": true
}
}
}`
```
The following includes a `Task` state that creates a node group.
```
`{
"StartAt": "CreateNodegroup.sync",
"States": {
"CreateNodegroup.sync": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:createNodegroup.sync",
"Arguments": {
"ClusterName": "MyCluster",
"NodegroupName": "MyNodegroup",
"NodeRole": "arn:aws:iam::`account-id`:role/MyNodeInstanceRole",
"Subnets": ["subnet-09fb51df01234", "subnet-027cfea4b1234"]
},
"End": true
}
}
}`
```
The following includes a `Task` state that deletes a node group.
```
`{
"StartAt": "DeleteNodegroup.sync",
"States": {
"DeleteNodegroup.sync": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:deleteNodegroup.sync",
"Arguments": {
"ClusterName": "MyCluster",
"NodegroupName": "MyNodegroup"
},
"End": true
}
}
}`
```
## Permissions
When an Amazon EKS cluster is created using the `eks:createCluster` service
integration, the IAM role is added to the Kubernetes RBAC authorization table as the
administrator, with `system:masters` permissions. Initially, only that IAM
entity can make calls to the Kubernetes API server. For example, you will not be able to
use **kubectl** to interact with your Kubernetes API server, unless you
assume the same role as your Step Functions state machine, or if you configure Kubernetes to
grant permissions to additional IAM entities. For more information, see [Managing users or IAM roles for your
cluster](https://docs.aws.amazon.com/eks/latest/userguide/add-user-role.html) in the *Amazon EKS User Guide*.
You can add permission for additional IAM entities, such as users or roles, by
adding them to the `aws-auth`
`ConfigMap` in the kube-system namespace. If you are creating your cluster
from Step Functions, use the `eks:call` service integration.
The following includes a `Task` state that creates an `aws-auth`
`ConfigMap` and grants `system:masters` permission to the user `arn:aws:iam::`account-id`:user/my-user` and the IAM role
`arn:aws:iam::`account-id`:role/my-role`.
```
`{
"StartAt": "Add authorized user",
"States": {
"Add authorized user": {
"Type": "Task",
"Resource": "arn:aws:states:::eks:call",
"Arguments": {
"ClusterName": "MyCluster",
"CertificateAuthority": "LS0tLS1CRUd...UtLS0tLQo=",
"Endpoint": "https://444455556666.yl4.`region`.eks.amazonaws.com",
"Method": "POST",
"Path": "/api/v1/namespaces/kube-system/configmaps",
"RequestBody": {
"apiVersion": "v1",
"kind": "ConfigMap",
"metadata": {
"name": "aws-auth",
"namespace": "kube-system"
},
"data": {
"mapUsers": "[{ \\"userarn\\": \\"arn:aws:iam::`account-id`:user/my-user\\", \\"username\\": \\"my-user\\", \\"groups\\": [ \\"system:masters\\" ] } ]",
"mapRoles": "[{ \\"rolearn\\": \\"arn:aws:iam::`account-id`:role/my-role\\", \\"username\\": \\"my-role\\", \\"groups\\": [ \\"system:masters\\" ] } ]"
}
}
},
"End": true
}
}`
```
###### Note
You may see the ARN for an IAM role displayed in a format that includes the path
**/service-role/**, such as
`arn:aws:iam::`account-id`:role/`service-role`/my-role`.
This **service-role** path token should not be included when
listing the role in `aws-auth`.
When your cluster is first created the `aws-auth`
`ConfigMap` will not exist, but will be added automatically if you create a
Fargate profile. You can retrieve the current value of `aws-auth`, add the
additional permissions, and `PUT` a new version. It is usually easier to
create `aws-auth` before the Fargate profile.
If your cluster was created outside of Step Functions, you can configure
**kubectl** to communicate with your Kubernetes API server. Then,
create a new `aws-auth`
`ConfigMap` using `kubectl apply -f aws-auth.yaml` or edit one
that already exists using `kubectl edit -n kube-system configmap/aws-auth`.
For more information, see:
* [Create a kubeconfig for
Amazon EKS](https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html) in the *Amazon EKS User Guide*.
* [Managing users or IAM roles for
your cluster](https://docs.aws.amazon.com/eks/latest/userguide/add-user-role.html) in the *Amazon EKS User Guide*.
If your IAM role does not have sufficient permissions in Kubernetes, the
`eks:call` or `eks:runJob` service integrations will fail with
the following error:
```
`Error:
EKS.401
Cause:
{
"ResponseBody": {
"kind": "Status",
"apiVersion": "v1",
"metadata": {},
"status": "Failure",
"message": "Unauthorized",
"reason": "Unauthorized",
"code": 401
},
"StatusCode": 401,
"StatusText": "Unauthorized"
}
`
```
## IAM policies for calling Amazon EKS
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
### `CreateCluster`
*Resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"eks:CreateCluster"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"eks:DescribeCluster",
"eks:DeleteCluster"
],
"Resource": "arn:aws:eks:sa-east-1:444455556666:cluster/\*"
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::444455556666:role/StepFunctionsSample-EKSClusterManag-EKSServiceRole-ANPAJ2UCCR6DPCEXAMPLE"
],
"Condition": {
"StringEquals": {
"iam:PassedToService": "eks.amazonaws.com"
}
}
}
]
}`
`
```
```
`
`
```
### `CreateNodeGroup`
*Resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"ec2:DescribeSubnets",
"eks:CreateNodegroup"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"eks:DescribeNodegroup",
"eks:DeleteNodegroup"
],
"Resource": "arn:aws:eks:sa-east-1:444455556666:nodegroup/\*"
},
{
"Effect": "Allow",
"Action": [
"iam:GetRole",
"iam:ListAttachedRolePolicies"
],
"Resource": "arn:aws:iam::444455556666:role/\*"
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::444455556666:role/StepFunctionsSample-EKSClusterMan-NodeInstanceRole-ANPAJ2UCCR6DPCEXAMPLE"
],
"Condition": {
"StringEquals": {
"iam:PassedToService": "eks.amazonaws.com"
}
}
}
]
}`
`
```
### `DeleteCluster`
*Resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"eks:DeleteCluster",
"eks:DescribeCluster"
],
"Resource": [
"arn:aws:eks:sa-east-1:444455556666:cluster/ExampleCluster"
]
}
]
}`
`
```
### `DeleteNodegroup`
*Resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"eks:DeleteNodegroup",
"eks:DescribeNodegroup"
],
"Resource": [
"arn:aws:eks:sa-east-1:444455556666:nodegroup/ExampleCluster/ExampleNodegroup/\*"
]
}
]
}`
`
```
For more information about using Amazon EKS with Step Functions, see [Create and manage Amazon EKS clusters with Step Functions](./connect-eks.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon ECS/Fargate
Amazon EMR
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.