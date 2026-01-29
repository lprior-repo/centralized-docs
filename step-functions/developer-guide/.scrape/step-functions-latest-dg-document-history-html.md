---
url: https://docs.aws.amazon.com/step-functions/latest/dg/document-history.html
title: Document history
word_count: 7568
filtered: true
elements_removed: 0
density_score: 0.87
---

Document history - AWS Step Functions
Document history - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#document-history)
# Document history
This section lists major changes to the *AWS Step Functions Developer Guide*.
|Change|Description|Date changed|
|New feature|
Step Functions now supports additional data sources and observability metrics for Distributed Map.
Step Functions supports additional data sources and new observability metrics for Distributed Map. With this update, Distributed Map now supports additional data inputs, so you can orchestrate large-scale analytics and ETL workflows. You can now process Athena data manifest and Parquet files directly, iterate over Amazon S3 objects under a specified prefix using `S3ListObjectsV2`, read from JSON objects and natively extract array data from JSON object from Amazon S3 or state input, eliminating the need for custom pre-processing. You also now get visibility into your Distributed Map usage with new metrics, including: Approximate Open Map Runs Count, Open Map Run Limit, and Approximate Map Runs Backlog Size. To learn more, see [ItemReader (Map)](./input-output-itemreader.html) and [Monitoring Step Functions metrics using Amazon CloudWatch](./procedure-cw-metrics.html).
|Sep 18, 2025|
|Documentation-only update|
Simplified and re-organized the integrating services topics based on customer feedback. To see the updates, check out the [Integrating services with Step Functions](./integrate-services.html) chapter.
|Sep 4, 2025|
|Updates|
Step Functions will now auto-create roles and policy for optimized integrations with MediaConvert.
For integrations with MediaConvert, Step Functions will now automatically create the necessary roles and policies required by your state machine. To learn more, see [Create an AWS Elemental MediaConvert job with Step Functions](./connect-mediaconvert.html) and [Integrating optimized services with Step Functions](./integrate-optimized.html).
|March 14, 2025|
|New feature|
Step Functions expands data source and output options for Distributed Map.
Distributed map can process data from JSON Lines (JSONL) and a broader range of delimited file formats, such as semicolon-delimited files and
tab-delimited files. Additionally, Distributed Map offers output transformations for greater control over result formatting. To learn more, [ItemReader (Map)](./input-output-itemreader.html) and [ResultWriter (Map)](./input-output-resultwriter.html).
|February 7, 2025|
|Documentation-only update|
Replaced the [Getting started tutorial](./getting-started.html) with content from workshop presented at re:Invent
2024.
|Dec 23, 2024|
|New feature|
Manage state and transform data with Step Functions workflow variables and JSONata.
With variables, you can pass data between the steps of your workflows. With JSONata, you gain an open source query and expression language to
select and transform data in your workflows. To learn more, see [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata in Step Functions](./transforming-data.html).
|November 22, 2024|
|New feature|
Step Functions adds Infrastructure as Code (IaC) template generation
The AWS Step Functions console provides the ability to export and download saved workflows as AWS CloudFormation or AWS SAM (SAM) templates. For AWS Regions
that support AWS Infrastructure Composer, it additionally provides the ability to export your workflows to Infrastructure Composer and navigates to the Infrastructure Composer console, where you can
continue to work with the newly generated template. To learn more, see [Exporting your workflow to IaC templates](./exporting-iac-templates.html).
|November 14, 2024|
|New feature|
Step Functions adds the option to use AWS KMS and customer managed keys to encrypt your data
You can add another layer of security by choosing a customer managed key to encrypt workflows, activities, and logs. To learn more, see [Data at rest encryption in Step Functions](./encryption-at-rest.html).
|July 25, 2024|
|Updates|
Document structure update
With page view data and depth analysis, documentation sections were restructured to increase visibility of important topics. The navigation
was updated to reduce overall depth. Related topics were consolidated. Redirects were added so that bookmarks should lead to the updated
locations. Send feedback if you notice errors or omissions after this massive update. Thank you!
|July 24, 2024|
|Updates|
AWS managed policy updates - new permission: `states:ValidateStateMachineDefinition`
Added information about new permission to check the syntax of a state machine that you provide. To learn more, see [AWS managed policies for AWS Step Functions](./security-iam-awsmanpol.html).
|April 29, 2024|
|New feature|
Step Functions adds optimized integration for AWS Elemental MediaConvert
AWS Elemental MediaConvert provides broadcast-grade video and audio file transcoding, which customers can automate with code to suit their media workflows.
With the optimized integration for AWS Step Functions in MediaConvert, it is now possible to orchestrate using the low-code visual tool Workflow Studio. To
learn more, see the documentation to [Manage AWS Elemental MediaConvert with
Step Functions](https://docs.aws.amazon.com/step-functions/latest/dg/connect-mediaconvert.html).
|April 12, 2024|
|Updates|
AWS managed policy updates - Update to an existing policy: `AWSStepFunctionsReadOnlyAccess`
Added information about new read-only permissions for tags, distributed maps, and versions and aliases. To learn more, see [AWS managed policies for AWS Step Functions](./security-iam-awsmanpol.html).
|April 02, 2024|
|Updates|
Step Functions adds support for Open Workflow metrics
With open workflow metrics, you now have account-level visibility into the number of standard workflows in progress as well as your open
workflow limit. You can manage workloads across all workflows, regardless of how they're started, to ensure smooth workflow operations. You can
set CloudWatch alarms to monitor your workflows and proactively receive alerts as you approach your limits. Once alerted, you can effectively manage
your workflows by taking actions such as stopping specific workflows or requesting a limit increase.
Open workflow metrics is available to use in CloudWatch for standard workflows with no additional configuration required. To learn more, see [Execution Metrics](./procedure-cw-metrics.html#cloudwatch-step-functions-execution-metrics).
|February 29, 2024|
|Updates|
Service integration additions and updates. For the list of new and updated AWS SDK integrations, see [Learning to use AWS service SDK integrations in
Step Functions](./supported-services-awssdk.html). For the full list of services, see [Supported AWS SDK service integrations](./supported-services-awssdk.html#supported-services-awssdk-list).
|January 18, 2024|
|New feature|
Use Workflow Studio in Infrastructure Composer to build serverless workflows using AWS CloudFormation templates. For more information, see [Using Workflow Studio in Infrastructure Composer to build Step Functions workflows](./use-wfs-in-app-composer.html).
|November 27, 2023|
|New feature|
Step Functions now lets you directly invoke public HTTPS endpoints and test individual states using a new Test State API. For more
information, see:
* [Call HTTPS APIs in Step Functions
workflows](./call-https-apis.html)
* [Testing state machines with TestState API](./test-state-isolation.html)
|November 26, 2023|
|New feature|Step Functions now integrates with Amazon Bedrock. For more information, see the following topics:
* [Invoke and customize Amazon Bedrock models with Step Functions](./connect-bedrock.html)
* [IAM permissions for Amazon Bedrock](./connect-bedrock.html#bedrock-iam)
* [Perform AI prompt-chaining with Amazon Bedrock](./sample-bedrock-prompt-chaining.html)
* [Integrating services with Step Functions](./integrate-services.html)
|November 26, 2023|
|New feature|Step Functions now lets you redrive workflow executions of type Standard from their point of failure. For more information, see [Restarting state machine executions with redrive in
Step Functions](./redrive-executions.html) and [Redriving Map Runs in Step Functions executions](./redrive-map-run.html).|November 15, 2023|
|Documentation-only update|
Published a new topic that explains how to run state machines on a schedule using Amazon EventBridge Scheduler. For more information, see [Using Amazon EventBridge Scheduler to start a Step Functions state machine execution](./using-eventbridge-scheduler.html).
|October 16, 2023|
|New feature|Step Functions now integrates with Amazon EMR Serverless. For more information, see the following topics:
* [Create and manage Amazon EMR Serverless applications with Step Functions](./connect-emr-serverless.html)
* [Run an EMR Serverless job](./sample-emr-serverless-job.html)
* [Integrating services with Step Functions](./integrate-optimized.html)
* [Integrating services with Step Functions](./integrate-services.html)
|October 12, 2023|
|Documentation-only update|Added information about running state machines on a schedule using Amazon EventBridge Scheduler. For more information, see [Using EventBridge Scheduler](./using-eventbridge-scheduler.html).|October 05, 2023|
|Update|
Reorganized and updated the *Distributed Map state* topics for clarity, brevity, and establishing a clear journey map for new users. For more information,
see [Using Map state in Distributed mode for large-scale parallel workloads in Step Functions](./state-map-distributed.html).
|October 6, 2023|
|Fixes|Fixed code samples in a tutorial to use AWS CDK v2. For more information, see [Using AWS CDK to create a Standard workflow in Step Functions](./tutorial-lambda-state-machine-cdk.html).|September 19, 2023|
|Update|Added information about the enhanced error handling capabilities that Step Functions has introduced to identify errors clearly and implement retries
with greater control. For more information, see [Fail workflow state](./state-fail.html) and [Retrying after an error](./concepts-error-handling.html#error-handling-retrying-after-an-error).|September 07, 2023|
|Update|Step Functions has added enhancements to Workflow Studio for streamlining workflow authoring experience. For more information, see [Developing workflows in Step Functions Workflow Studio](./workflow-studio.html).|August 31, 2023|
|Documentation-only update|Added information about twice the actual metric count reported for the `ExecutionsStarted` metric. For more information, see [Metrics that report a
count](./procedure-cw-metrics.html#monitoring-using-cloudwatch-count-metrics).|July 25, 2023|
|Documentation-only update|Step Functions has added two new sample projects that demonstrate the following common use cases for the *Distributed Map state*:
* [Processing a CSV file](./sample-dist-map-csv-process.html)
* [Processing data in an Amazon S3 bucket](./sample-dist-map-s3data-process.html)
|July 17, 2023|
|Documentation-only update|
Published a new topic about deploying state machines using Terraform. For more information, see [Using Terraform to deploy state machines in Step Functions](./terraform-sfn.html).
|July 5, 2023|
|Documentation-only update|
Updated the following procedures to match changes to the Amazon EventBridge interface.
* [Automate event delivery](./eventbridge-integration.html)
* [Starting a Step Functions workflow in response to events](./tutorial-cloudwatch-events-s3.html)
|June 26, 2023|
|New feature|Step Functions now provides the ability to create multiple state machine versions and aliases for improved resiliency while deploying
serverless workflows. For more information, see [
Manage continuous deployments with versions and aliases in Step Functions](./concepts-cd-aliasing-versioning.html).|June 22, 2023|
|Documentation-only update|
Improved the description of `TimeoutSeconds` and `HeartbeatSeconds` fields to describe how they're different from each
other. For more information, see [Task state fields](./state-task.html#task-state-fields).
|June 22, 2023|
|Documentation-only update|
Published a new section that describes how to flatten an array of arrays typically returned as result for Parallel and Map states. For more
information, see [Flattening an array of arrays](./amazon-states-language-paths.html#flatten-array-of-arrays).
|June 20, 2023|
|
Update
|
Step Functions has expanded support for AWS SDK integrations by adding seven AWS services and 468 new API actions. For more information, see [Supported AWS SDK service integrations](./supported-services-awssdk.html#supported-services-awssdk-list) and [Learning to use AWS service SDK integrations in
Step Functions](./supported-services-awssdk.html).
|June 16, 2023|
|Documentation-only update|Published a new topic that lists the AWS Regions in which recently launched Step Functions features are available. For more information, see [Recent feature launches](./recent-launches.html).|June 16, 2023|
|
Documentation-only update
|Step Functions now includes a section about AWS User Notifications, an AWS service that acts as a central location for your AWS notifications in the
AWS Management Console. For more information, see [Events using User Notifications](./using-user-notifications-sfn.html).|May 4, 2023|
|Documentation-only update|
Added a new section that explains about the permissions needed to write child workflow execution results to an Amazon S3 bucket encrypted with an
AWS Key Management Service
(AWS KMS) key. For more information, see [IAM permissions for AWS KMS key encrypted Amazon S3 bucket](./iam-policies-eg-dist-map.html#multiupload-dmap-result-policy).
|April 29, 2023|
|Documentation-only update|
Added a new topic that explains about the [ Data flow
simulator](https://console.aws.amazon.com/states/home?region=us-east-1#/simulator) feature. For more information, see [Data flow simulator (unsupported)](./test-and-debug.html#use-data-flow-simulator).
|April 14, 2023|
|Quota update|
Added information about default quota of 1000 for open Map Runs in each account. For more information, see [Quotas related to accounts](./service-quotas.html#service-limits-accounts).
|April 05, 2023|
|Documentation-only update|
Added a Note about unavailability of X-Ray tracing for the [Distributed Map state](./state-map-distributed.html). For more information, see
[Trace Step Functions request data in AWS X-Ray](./concepts-xray-tracing.html).
|March 21, 2023|
|Documentation-only update|
Added information about how Step Functions handles tag-based authorization. For more information, see [Tagging state machines and activities in Step Functions](./sfn-best-practices.html#concepts-tagging) and [Creating tag-based IAM policies in Step Functions](./tag-based-policies.html).
|March 15, 2023|
|Documentation-only update|
Added information about how Step Functions parses CSV files used as input in *Distributed Map state*. For more information, see [CSV file in an Amazon S3 bucket](./input-output-itemreader.html#itemsource-example-csv-data).
|March 14, 2023|
|Documentation-only update|
Added information about how Step Functions handles [cross-account](./concepts-access-cross-acct-resources.html) invocations for the Run a
Job (.sync) pattern. For more information, see [Run a Job (.sync)](./connect-to-resource.html#connect-sync).
|March 01, 2023|
|Documentation-only update|
Reduce the history retention period of your completed workflow executions from 90 days to 30 days. For more information about adjusting the
retention period, see [Execution guarantees in Step Functions workflows](./choosing-workflow-type.html#express-at-least-once-execution) and [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
|February 21, 2023|
|Update|
Step Functions has expanded support for AWS SDK integrations by adding 35 AWS services and 1100 new API actions. For more information, see [Supported AWS SDK service integrations](./supported-services-awssdk.html#supported-services-awssdk-list) and [Learning to use AWS service SDK integrations in
Step Functions](./supported-services-awssdk.html).
|February 17, 2023|
|Documentation-only update|
Published a Getting Started tutorial series that walks you through the process of creating a workflow for credit card application using Step Functions.
For more information, see [Learn how to get started with Step Functions](./getting-started.html).
|December 30, 2022|
|New feature|
Step Functions adds support to orchestrate large-scale parallel workflows for data processing using a new Distributed mode for `Map`
state. For more information, see [Using Map state in Distributed mode for large-scale parallel workloads in Step Functions](./state-map-distributed.html).
|December 01, 2022|
|New feature|Step Functions now supports access to cross-account AWS resources configured in other accounts. For more information, see
* [Accessing resources in other AWS accounts in Step Functions](./concepts-access-cross-acct-resources.html)
* [Accessing cross-account AWS resources in Step Functions](./tutorial-access-cross-acct-resources.html)
* [Task state](./state-task.html#task-cred-field)
|November 18, 2022|
|Update|
Step Functions now provides a new console experience for viewing and debugging Express workflow executions. For more information see:
* [Standard and Express console experience differences](./concepts-view-execution-details.html#console-exp-differences)
* [Viewing execution details in the Step Functions console](./concepts-view-execution-details.html)
|October 18, 2022|
|Update|
Added support to optionally specify the `ExecutionRoleArn` parameter while using the `addStep` and
`addStep.sync` APIs for the Amazon EMR optimized service integration. For more information, see [Create and manage Amazon EMR clusters with Step Functions](./connect-emr.html).
|September 20, 2022|
|Documentation-only update|
Added a new topic that provides recommendations about optimizing cost while building serverless workflows using Step Functions. For more information,
see [Optimizing costs using Express Workflows](./sfn-best-practices.html#cost-opt-exp-workflows).
|September 15, 2022|
|
Update
|
Step Functions adds support for 14 new intrinsic functions for performing data processing tasks, such as array manipulations, data encoding and
decoding, hash calculations, JSON data manipulation, math function operations, and unique identifier generation.
###### Documentation-only update:
Grouped all the existing and newly introduced intrinsic functions into the following categories based on the type of data processing task
they help you perform:
* [Intrinsics for arrays](./intrinsic-functions.html#asl-intrsc-func-arrays)
* [Intrinsics for data encoding and decoding](./intrinsic-functions.html#asl-intrsc-func-data-encode-decode)
* [Intrinsic for hash calculation](./intrinsic-functions.html#asl-intrsc-func-hash-calc)
* [Intrinsics for JSON data manipulation](./intrinsic-functions.html#asl-intrsc-func-json-manipulate)
* [Intrinsics for Math operations](./intrinsic-functions.html#asl-intrsc-func-math-operation)
* [Intrinsic for String operation](./intrinsic-functions.html#asl-intrsc-func-string-operation)
* [Intrinsic for unique identifier generation](./intrinsic-functions.html#asl-intrsc-func-uuid-generate)
* [Intrinsic for generic operation](./intrinsic-functions.html#asl-intrsc-func-generic)
For more information, see [Intrinsic functions for JSONPath states in Step Functions ](./intrinsic-functions.html).
|
August 31, 2022
|
|Update|
Step Functions has expanded support for AWS SDK integrations by adding three more AWS services – AWS Billing Conductor,
Amazon GameSparks, and Amazon Pinpoint SMS and Voice V2. For more information, see [Learning to use AWS service SDK integrations in
Step Functions](./supported-services-awssdk.html).
|July 26, 2022|
|Documentation-only update|
Added a new topic to include a summary of all the updates made to AWS SDK integrations supported by Step Functions. For more information, see [Learning to use AWS service SDK integrations in
Step Functions](./supported-services-awssdk.html)
|July 26, 2022|
|Documentation-only update|
*AWS Step Functions Developer Guide* now includes details about the execution metrics that are emitted specifically for Express
Workflows. For more information, see [Execution metrics for Express Workflows](./procedure-cw-metrics.html#cloudwatch-step-functions-execution-metrics-express-wf).
|June 09, 2022|
|Update|###### Step Functions console enhancements
The console now features a redesigned **Execution Details** page that includes the following enhancements:
* Ability to identify the reason for a failed execution at a glance.
* Two new modes of visualizations for your state machine – **Table view** and **Event view**. These
views also provide you the ability to apply filters to only view the information of interest. In addition, you can sort the **Event
view** contents based on the event timestamps.
* Switch between the different iterations of `Map` state in the **Graph view** mode using a dropdown list or in
the **Table view** mode's tree view for `Map` states.
* View in-depth information about each state in the workflow, including the complete input and output data transfer path and retry attempts
for `Task` or `Parallel` states.
* Miscellaneous enhancements including the option to copy the state machine's execution Amazon Resource Name, view the count of total state
machine transitions, and export the execution details in JSON format.
###### Documentation-only updates
Added a new topic to explain the various types of information displayed in the **Execution Details** page. Also, added a
tutorial to show how to examine this information. For more information, see:
* [Viewing execution details in the Step Functions console](./concepts-view-execution-details.html)
* [Examining state machine executions in Step Functions](./debug-sm-exec-using-ui.html)
|May 09, 2022|
|
Update
|
Step Functions now provides a workaround to prevent the confused deputy security issue, which arises when an entity (a service or an account) is
coerced by a different entity to perform an action. For more information, see:
* [Prevent cross-service confused deputy issue](./procedure-create-iam-role.html#prevent-cross-service-confused-deputy)
|
May 02, 2022
|
|
Update
|
* Step Functions has expanded support for AWS SDK integrations by adding 21 more AWS services. For more information, see: [Supported AWS SDK service integrations](./supported-services-awssdk.html#supported-services-awssdk-list).
* Documentation-only updates:
* Added a list of all the exception prefixes present in the exceptions that are generated when you erroneously perform an AWS SDK
service integration with Step Functions. For more information, see: [Supported AWS SDK service integrations](./supported-services-awssdk.html#supported-services-awssdk-list).
|
April 19, 2022
|
|
New feature
|
Step Functions Local now supports AWS SDK integration and mocking of service integrations. For more information, see:
* [Using mocked service integrations for testing in Step Functions Local](./sfn-local-test-sm-exec.html)
|
January 28, 2022
|
|
New feature
|
AWS Step Functions now supports creating an Amazon API Gateway REST API with synchronous express state machine as backend integration using the AWS Cloud Development Kit (AWS CDK). For
more information, see:
* [Using AWS CDK to create an Express workflow in Step Functions](./tutorial-step-functions-rest-api-integration-cdk.html)
|
December 10, 2021
|
|
Update
|
Step Functions has added three new sample projects that demonstrate the integration of Step Functions and Amazon Athena's upgraded console. For more information,
see:
* [Execute queries in sequence and parallel using Athena](./run-multiple-queries.html)
* [Query large datasets using an AWS Glue crawler](./sample-query-large-datasets.html)
* [Keep data in a target table updated with AWS Glue and Athena](./sample-keep-data-updated.html)
|
November 22, 2021
|
|
New feature
|
Step Functions has added Amazon VPC endpoints support for Synchronous Express Workflows. For more information, see:
* [Creating Amazon VPC endpoints for Step Functions](./vpc-endpoints.html)
|
November 15, 2021
|
|
Update
|
AWS Step Functions has added three new sample projects that demonstrate how to use the Step Functions AWS Batch integration. For more information, see:
* [Fan out batch jobs with Map state](./sample-batch-fan-out.html)
* [Run an AWS Batch job with Lambda](./sample-batch-lambda.html)
* [Manage a batch job with AWS Batch and Amazon SNS](./batch-job-notification.html)
|
October 14, 2021
|
|
New feature
|
AWS Step Functions has added AWS SDK integrations, letting you use the API actions for all of the more than two hundred AWS services. For more
information, see:
* [Learning to use AWS service SDK integrations in
Step Functions](./supported-services-awssdk.html)
* [Gather Amazon S3 bucket info using AWS SDK service integrations](./tutorial-gather-s3-info.html)
|
September 30, 2021
|
|
New feature
|
AWS Step Functions has added a visual workflow designer, the AWS Step Functions Workflow Studio. For more information, see:
* [Developing workflows in Step Functions Workflow Studio](./workflow-studio.html)
|
June 17, 2021
|
|
Update
|
AWS Step Functions has added four new APIs, `StartBuildBatch`, `StopBuildBatch`, `RetryBuildBatch` and
`DeleteBuildBatch`, to the CodeBuild integration. For more information, see:
* [Manage AWS CodeBuild builds with Step Functions](./connect-codebuild.html)
|
June 4, 2021
|
|
New feature
|
AWS Step Functions now integrates with Amazon EventBridge. For more information, see:
* [Add EventBridge events with Step Functions](./connect-eventbridge.html)
* IAM policies for Step Functions and [IAM policies for calling EventBridge](./connect-eventbridge.html#eventbridge-iam)
* A sample project that shows how to [Send a custom event to an EventBridge event bus](./sample-eventbridge-custom-event.html)
|
May 14, 2021
|
|
Update
|
AWS Step Functions has added a new sample project that shows how to use Step Functions and the Amazon Redshift Data API to run an ETL/ELT workflow. For more
information, see:
* [Run an ETL/ELT workflow using Step Functions and the Amazon Redshift API](./sample-etl-orchestration.html)
|
April 16, 2021
|
|
New feature
|
AWS Step Functions has a new data flow simulator in the console. For more information, see:
* [Data flow simulator (unsupported)](./test-and-debug.html#use-data-flow-simulator)
|
April 8, 2021
|
|
New feature
|
AWS Step Functions now integrates with Amazon EMR on EKS. For more information, see:
* [Create and manage Amazon EMR clusters on EKS with AWS Step Functions](./connect-emr-eks.html)
|
March 29, 2021
|
|
Update
|
YAML support for state machine definitions has been added to AWS Toolkit for Visual Studio Code and CloudFormation. For more information, see:
* [AWS Toolkit for Visual Studio Code](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/building-stepfunctions.html)
|
March 4, 2021
|
|
New feature
|
AWS Step Functions now integrates with AWS Glue DataBrew. For more information, see:
* [Start AWS Glue DataBrew jobs with Step Functions](./connect-databrew.html)
* [What is AWS Glue DataBrew?](https://docs.aws.amazon.com/databrew/latest/dg/what-is.html) in the DataBrew developer guide.
|
January 6, 2021
|
|
New feature
|
AWS Step Functions Synchronous Express Workflows are now available, giving you an easy way to orchestrate microservices. For more information,
see:
* [Synchronous and Asynchronous Express
Workflows in Step Functions](./choosing-workflow-type.html#concepts-express-synchronous)
* A sample project that shows how to [Invoke Synchronous Express Workflows through API Gateway](./synchronous-execution.html)
* The [StartSyncExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartSyncExecution.html) API documentation.
|
November 24, 2020
|
|
New feature
|
AWS Step Functions now integrates with Amazon API Gateway. For more information, see:
* [Create API Gateway REST APIs with Step Functions](./connect-api-gateway.html)
* IAM policies for Step Functions and [IAM policies for calls to Amazon API Gateway](./connect-api-gateway.html#api-gateway-iam)
* A sample project that shows how to [Interact with an API managed by API Gateway](./sample-apigateway-workflow.html)
|
November 17, 2020
|
|
New feature
|
AWS Step Functions now integrates with Amazon Elastic Kubernetes Service. For more information, see:
* [Create and manage Amazon EKS clusters with Step Functions](./connect-eks.html)
* IAM policies for Step Functions and [IAM policies for calling Amazon EKS](./connect-eks.html#eks-iam)
* A sample project that shows how to [Create and manage an Amazon EKS cluster with a node group](./sample-eks-cluster.html)
|
November 16, 2020
|
|
New feature
|
AWS Step Functions now integrates with Amazon Athena. For more information, see:
* [Run Athena queries with Step Functions](./connect-athena.html)
* IAM policies for Step Functions and [IAM policies for calling Amazon Athena](./connect-athena.html#athena-iam)
* A sample project that shows how to [Start an Athena query and send a results notification](./sample-athena-query.html)
|
October 22, 2020
|
|
New feature
|
AWS Step Functions now supports tracing end-to-end workflows with AWS X-Ray, giving you full visibility across state machine executions and making it
easier to analyze and debug your distributed applications. For more information, see:
* [Trace Step Functions request data in AWS X-Ray](./concepts-xray-tracing.html)
* IAM policies for Step Functions and [IAM policies using AWS X-Ray in Step Functions](./concepts-xray-tracing.html#xray-iam)
* [AWS Step Functions API Reference](https://docs.aws.amazon.com/step-functions/latest/apireference/)
* [TracingConfiguration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TracingConfiguration.html)
|
September 14, 2020
|
|Update|
AWS Step Functions now supports payload sizes up to 256 KiB of data as a UTF-8 encoded string. This lets you process larger payloads in both Standard and Express
workflows.
Your existing state machines do not need to be changed in order to use the larger payloads. However, you will need to update to the latest
versions of the Step Functions SDK and Local Runner to use the updated APIs. For more information, see:
* [Step Functions service quotas](./service-quotas.html)
* [Using Amazon S3 ARNs instead of passing large payloads in Step Functions](./sfn-best-practices.html#avoid-exec-failures)
* [States.DataLimitExceeded](./concepts-error-handling.html#error-handling-error-representation)
* [CloudWatch Logs payloads](./cw-logs.html#cloudwatch-payload)
* [AWS Step Functions API Reference](https://docs.aws.amazon.com/step-functions/latest/apireference/)
* [CloudWatchEventsExecutionDataDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CloudWatchEventsExecutionDataDetails.html)
* [HistoryEventExecutionDataDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_HistoryEventExecutionDataDetails.html)
* [GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html)
* [ActivityScheduledEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivityScheduledEventDetails.html)
* [ActivitySucceededEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivitySucceededEventDetails.html)
* [CloudWatchEventsExecutionDataDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CloudWatchEventsExecutionDataDetails.html)
* [ExecutionSucceededEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ExecutionSucceededEventDetails.html)
* [LambdaFunctionScheduledEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_LambdaFunctionScheduledEventDetails.html)
* [ExecutionSucceededEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ExecutionSucceededEventDetails.html)
* [StateEnteredEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StateEnteredEventDetails.html)
* [StateExitedEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StateExitedEventDetails.html)
* [TaskSubmittedEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TaskSubmittedEventDetails.html)
* [TaskSucceededEventDetails](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TaskSucceededEventDetails.html)
|September 3, 2020|
|
Update
|
The Amazon States Language has been updated as follows:
* [Choice Rules (JSONata)](./state-choice.html#state-choice-rules) has added
* A null comparison operator, `IsNull`. `IsNull` tests against the JSON null value, and can be used to detect if
the output of a previous state is null or not.
* Four other new operators have been added, IsBoolean, IsNumeric, IsString and IsTimestamp.
* A test for the existence or non-existence of a field using the `IsPresent` operator. `IsPresent` can be used to
prevent `States.Runtime` errors when there is an attempt to access a non-existent key.
* Wildcard pattern matching to support string comparison against patterns with one or more wildcards.
* Comparison between two variables for supported comparison operators.
* Timeout and heartbeat values in a `Task` state can now be provided dynamically from the state input instead of a fixed value
using the `TimeoutSecondsPath` and `HeartbeatSecondsPath` fields. See the [Task workflow state](./state-task.html) state for more information.
* The new [ResultSelector](./input-output-inputpath-params.html#input-output-resultselector) field provides a way to manipulate a
state’s result before `ResultPath` is applied. The `ResultSelector` field is an optional field in the [Map workflow state](./state-map.html), [Parallel workflow state](./state-parallel.html), and [Task workflow state](./state-task.html) states.
* [Intrinsic functions for JSONPath states in Step Functions ](./intrinsic-functions.html) have been added to allow basic operations without
`Task` states. Intrinsic functions can be used within the `Parameters` and `ResultSelector` fields.
|
August 13, 2020
|
|Update|
AWS Step Functions now supports the Amazon SageMaker AI `CreateProcessingJob` API call. For more information, see:
* [Create and manage Amazon SageMaker AI jobs with Step Functions](./connect-sagemaker.html)
* [Preprocess data and train a machine learning model with Amazon SageMaker AI](./sample-preprocess-feature-transform.html), a sample project that
demonstrates `CreateProcessingJob`.
|August 4, 2020|
|
New feature
|
AWS Step Functions is now supported by AWS Serverless Application Model, making it easier to integrate workflow orchestration into your serverless applications. For more
information, see:
* [Using AWS SAM to build Step Functions workflows](./concepts-sam-sfn.html)
* [AWS::Serverless::StateMachine](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-statemachine.html)
* [AWS SAM Policy Templates](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-policy-templates.html)
|
May 27, 2020
|
|New feature|AWS Step Functions has introduced a new synchronous invocation for nesting Step Functions executions. The new invocation,
`arn:aws:states:::states:startExecution.sync:2`, returns a JSON object. The original invocation,
`arn:aws:states:::states:startExecution.sync`, continues to be supported, and returns a JSON-escaped string. For more information, see:
* [Start a new AWS Step Functions state machine from a running execution](./connect-stepfunctions.html)
|May 19, 2020|
|
New feature
|
AWS Step Functions now integrates with AWS CodeBuild. For more information, see:
* [Integrating services with Step Functions](./integrate-services.html)
* [Manage AWS CodeBuild builds with Step Functions](./connect-codebuild.html)
* [Integrating services with Step Functions](./integrate-optimized.html)
|
May 5, 2020
|
|New feature|
Step Functions is now supported in [AWS Toolkit for Visual Studio Code](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/building-stepfunctions.html), making it easier to
create and visualize state machine based workflows without leaving your code editor.
|March 31, 2020|
|
Update
|
You can now configure logging to Amazon CloudWatch Logs for Standard workflows. For more information, see:
* [Using CloudWatch Logs to log execution history in Step Functions](./cw-logs.html)
|
February 25, 2020
|
|
New feature
|
AWS Step Functions can now be accessed without requiring a public IP address, directly from Amazon Virtual Private Cloud (VPC). For more information, see:
* [Creating Amazon VPC endpoints for Step Functions](./vpc-endpoints.html)
|
December 23, 2019
|
|
New feature
|
Express Workflows are a new workflow type, suitable for high-volume event processing workloads such as IoT data ingestion, streaming data
processing and transformation, and mobile application backends.
For more information, review the following new and updated topics.
* [Choosing workflow type in Step Functions](./choosing-workflow-type.html)
* [Execution guarantees in Step Functions workflows](./choosing-workflow-type.html#express-at-least-once-execution)
* [Integrating services with Step Functions](./integrate-services.html)
* [Integrating services with Step Functions](./integrate-optimized.html)
* [Process high-volume messages from Amazon SQS
with Step Functions Express workflows](./sample-project-express-high-volume-sqs.html)
* [Perform selective checkpointing using Standard and Express workflows](./sample-project-express-selective-checkpointing.html)
* [Step Functions service quotas](./service-quotas.html)
* [Step Functions service quotas](./service-quotas.html)
* [Using CloudWatch Logs to log execution history in Step Functions](./cw-logs.html)
* [AWS Step Functions API Reference](https://docs.aws.amazon.com/step-functions/latest/apireference/)
* [CreateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html)
* [UpdateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachine.html)
* [DescribeStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachine.html)
* [DescribeStateMachineForExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachineForExecution.html)
* [StopExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StopExecution.html)
* [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html)
* [GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html)
* [ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html)
* [ListStateMachines](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachines.html)
* [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)
* [CloudWatchLogsLogGroup](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CloudWatchLogsLogGroup.html)
* [LogDestination](https://docs.aws.amazon.com/step-functions/latest/apireference/API_LogDestination.html)
* [LoggingConfiguration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_LoggingConfiguration.html)
|
December 3, 2019
|
|
New feature
|
AWS Step Functions now integrates with Amazon EMR. For more information, see:
* [Integrating services with Step Functions](./integrate-services.html)
* [Create and manage Amazon EMR clusters with Step Functions](./connect-emr.html)
* [Integrating services with Step Functions](./integrate-optimized.html)
|
November 19, 2019
|
|
Update
|
AWS Step Functions has released the AWS Step Functions Data Science SDK. For more information, see the following.
* [Project on Github](https://github.com/aws/aws-step-functions-data-science-sdk-python)
* [SDK Documentation](https://aws-step-functions-data-science-sdk.readthedocs.io/)
* The following [Example Notebooks](https://docs.aws.amazon.com/sagemaker/latest/dg/howitworks-nbexamples.html), which are available in the [SageMaker AI console](https://console.aws.amazon.com/sagemaker/) and the related [GitHub project](https://github.com/awslabs/amazon-sagemaker-examples/tree/master/step-functions-data-science-sdk).
* `hello\_world\_workflow.ipynb`
* `machine\_learning\_workflow\_abalone.ipynb`
* `training\_pipeline\_pytorch\_mnist.ipynb`
|
November 7, 2019
|
|
Update
|
Step Functions now supports more API actions for Amazon SageMaker AI, and includes two new sample projects to demonstrate the functionality. For more information,
see the following.
* [Create and manage Amazon SageMaker AI jobs with Step Functions](./connect-sagemaker.html)
* [Integrating services with Step Functions](./integrate-services.html)
* [Train a machine learning model using Amazon SageMaker AI](./sample-train-model.html)
* [Tune the hyperparameters of a machine learning model in SageMaker AI](./sample-hyper-tuning.html)
|
October 3, 2019
|
|New feature|
Step Functions supports starting new workflow executions by calling `StartExecution` as an integrated service API. See:
* [Start workflow executions from a task state in Step Functions](./concepts-nested-workflows.html)
* [Start a new AWS Step Functions state machine from a running execution](./connect-stepfunctions.html)
* [Integrating services with Step Functions](./integrate-services.html)
* [IAM Policies for Starting Step Functions Workflow Executions](./connect-stepfunctions.html#stepfunctions-iam)
|August 12, 2019|
|New feature|
Step Functions includes the ability to pass a task token to integrated services, and pause the execution until that task token is returned with
`SendTaskSuccess` or `SendTaskFailure`. See:
* [Discover service integration patterns in Step Functions](./connect-to-resource.html)
* [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token)
* [Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda](./callback-task-sample-sqs.html)
* [Integrating services with Step Functions](./integrate-optimized.html)
* [Deploying a workflow that waits for human approval in Step Functions](./tutorial-human-approval.html)
* [Service
Integration Metrics](./procedure-cw-metrics.html#cloudwatch-step-functions-service-integration-metrics)
Step Functions now provides a way to access dynamic information about your current execution directly in the `"Parameters"` field of a state
definition. See:
* [Accessing execution data from the Context object
in Step Functions ](./input-output-contextobject.html)
* [Pass Context object nodes as parameters](./connect-parameters.html#connect-parameters-context)
|May 23, 2019|
|New feature|
Step Functions supports CloudWatch Events for execution status changes, see:
* [Automating Step Functions event delivery with EventBridge](./eventbridge-integration.html)
|May 8, 2019|
|New feature|
Step Functions supports IAM permissions using tags. For more information, see:
* [Tagging state machines and activities in Step Functions](./sfn-best-practices.html#concepts-tagging)
* [Creating tag-based IAM policies in Step Functions](./tag-based-policies.html)
|March 5, 2019|
|New feature|
Step Functions Local is now available. You can run Step Functions on your local machine for testing and development. Step Functions Local is available for download
as either a Java application, or as a Docker image. See [Testing state machines with Step Functions Local (unsupported)](./sfn-local.html).
|February 4, 2019|
|New feature|
AWS Step Functions is now available in the Beijing and Ningxia regions.
|January 15, 2018|
|New feature|
Step Functions supports resource tagging to help track your cost allocation. You can tag state machines on the **Details** page,
or through API actions. See [Tagging state machines and activities in Step Functions](./sfn-best-practices.html#concepts-tagging).
|January 7, 2019|
|New feature|
AWS Step Functions is now available in the Europe (Paris), and South America (São Paulo) regions.
|December 13, 2018|
|New feature|
AWS Step Functions is now available the Europe (Stockholm) region.
|December 12, 2018|
|
New feature
|
Step Functions now integrates with some AWS services. You can now directly call and pass parameters to the API of these integrated services
from a task state in the Amazon States Language. For more information, see:
* [Integrating services with Step Functions](./integrate-services.html)
* [Passing parameters to a service API in Step Functions](./connect-parameters.html)
* [Integrating services with Step Functions](./integrate-optimized.html)
|
November 29, 2018
|
|
Update
|
Improved the description of `TimeoutSeconds` and `HeartbeatSeconds` in the documentation for task states. See [Task workflow state](./state-task.html).
|
October 24, 2018
|
|
Update
|
Improved the description for the *Maximum execution history size* limit and provided a link to the related best practices
topic.
* [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions)
* [Starting new executions to avoid reaching the history quota in Step Functions](./sfn-best-practices.html#bp-history-limit)
|
October 17, 2018
|
|
Update
|
Added a new tutorial to the AWS Step Functions documentation: See [Starting a Step Functions workflow in response to events](./tutorial-cloudwatch-events-s3.html).
|
September 25, 2018
|
|
Update
|
Removed the entry *Maximum executions displayed in Step Functions console* from the limits documentation. See [Step Functions service quotas](./service-quotas.html).
|
September 13, 2018
|
|
Update
|
Added a best practices topic to the AWS Step Functions documentation on improving latency when polling for activity tasks. See [Avoiding latency when polling for activity tasks](./sfn-best-practices.html#bp-activity-pollers).
|
August 30, 2018
|
|
Update
|
Improved the AWS Step Functions topic on activities and activity workers. See [Learn about Activities in Step Functions](./concepts-activities.html).
|
August 29, 2018
|
|
Update
|
Improved the AWS Step Functions topic on CloudTrail integration. See [Recording Step Functions API calls with AWS CloudTrail](./procedure-cloud-trail.html).
|
August 7, 2018
|
|
Update
|
Added JSON examples to CloudFormation tutorial. See [Using CloudFormation to create a workflow in Step Functions](./tutorial-lambda-state-machine-cloudformation.html).
|
June 23, 2018
|
|
Update
|
Added a new topic on handling Lambda service errors. See [Handle transient Lambda service exceptions](./sfn-best-practices.html#bp-lambda-serviceexception).
|
June 20, 2018
|
|New feature|
AWS Step Functions is now available the Asia Pacific (Mumbai) region.
|June 28, 2018|
|New feature|
AWS Step Functions is now available the AWS GovCloud (US-West) region. For information about using Step Functions in the AWS GovCloud (US-West) Region, see
[AWS GovCloud (US)](https://docs.aws.amazon.com/govcloud-us/latest/UserGuide/using-govcloud-endpoints.html).
|June 28, 2018|
|
Update
|
Improved documentation on error handling for `Parallel` states. See [Error Handling](./state-parallel.html#error-handling).
|
June 20, 2018
|
|
Update
|
Improved documentation about Input and Output processing in Step Functions. Learn how to use `InputPath`, `ResultPath`, and
`OutputPath` to control the flow of JSON through your workflows, states, and tasks. See:
* [Processing input and output in Step Functions](./concepts-input-output-filtering.html)
* [Specifying state output using ResultPath in Step Functions](./input-output-resultpath.html)
|
June 7, 2018
|
|
Update
|
Improved code examples for parallel states. See [Parallel workflow state](./state-parallel.html).
|
June 4, 2018
|
|
New feature
|
You can now monitor API and Service metrics in CloudWatch. See [Monitoring Step Functions metrics using Amazon CloudWatch](./procedure-cw-metrics.html).
|
May 25, 2018
|
|Update|`StartExecution`, `StopExecution`, and `StateTransition` now have increased throttling limits in the following
regions:
* US East (N. Virginia)
* US West (Oregon)
* Europe (Ireland)
For more information see [Step Functions service quotas](./service-quotas.html). |May 16, 2018|
|New feature|
AWS Step Functions is now available the US West (N. California) and Asia Pacific (Seoul) regions. See [AWS Services by Region](https://aws.amazon.com/about-aws/global-infrastructure/regional-product-services/) for a list of supported
regions.
|May 5, 2018|
|Update|
Updated procedures and images to match changes to the interface.
|April 25, 2018|
|Update|
Added a new tutorial that shows how to start a new execution to continue your work. See [Continue long-running workflows using Step Functions API (recommended)](./tutorial-continue-new.html). This tutorial describes a design pattern that can help avoid some service limitations. See [Starting new executions to avoid reaching the history quota in Step Functions](./sfn-best-practices.html#bp-history-limit).
|April 19, 2018|
|Update|
Improved introduction to states documentation by adding conceptual information about state machines. See [Discovering workflow states to use in Step Functions](./workflow-states.html).
|March 9, 2018|
|New feature|
* When you create a new state machine, you must acknowledge that AWS Step Functions will create an IAM role which allows access to your Lambda
functions.
* Updated the following tutorials to reflect the minor changes in the state machine creation workflow:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
* [Iterate a loop with a Lambda function in Step Functions](./tutorial-create-iterate-pattern-section.html)
|February 19, 2018|
|Update|Added a topic that describes an example activity worker written in Ruby. This implementation can be used to create a Ruby activity worker
directly, or as a design pattern for creating an activity worker in another language.
See [Example: Activity Worker in Ruby](./concepts-activities.html#example-ruby-activity-worker).
|February 6, 2018|
|Update|Added a new tutorial describing a design pattern that uses a Lambda function to iterate a count.
See [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html).
|January 31, 2018|
|Update|Updated content on IAM permissions to include `DescribeStateMachineForExecution` and `UpdateStateMachine`
APIs.
See [Creating granular permissions for non-admin users in Step Functions](./concept-create-iam-advanced.html).
|January 26, 2018|
|Update|Added newly available regions: Canada (Central), Asia Pacific (Singapore).|January 25, 2018|
|Update|Updated tutorials and procedures to reflect that IAM allows you to select *Step Functions* as a role.|January 24, 2018|
|Update|Added a new *Best Practices* topic that suggests not passing large payloads between states.
See [Using Amazon S3 ARNs instead of passing large payloads in Step Functions](./sfn-best-practices.html#avoid-exec-failures).
|January 23, 2018|
|Update|Corrected procedures to match updated interface for creating a state machine:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
|January 17, 2018|
|New Feature|You can use *Sample Projects* to quickly provision state machines and all related AWS resources. See [Deploy a state machine using a starter template for Step Functions](./starter-templates.html),
Available sample projects include:
* [Poll for job status with Lambda and AWS Batch](./sample-project-job-poller.html)
* [Create a task timer with Lambda and Amazon SNS](./task-timer-sample.html)
###### Note
These sample projects and related documentation replace tutorials that described implementing the same functionality.
|January 11, 2018|
|Update|Added a *Best Practices* section that includes information on avoiding stuck executions. See [Best practices for Step Functions](./sfn-best-practices.html).|January 5, 2018|
|Update|Added a note on how retries can affect pricing:
###### Note
Retries are treated as state transitions. For information about how state transitions affect billing, see [Step Functions Pricing](https://aws.amazon.com/step-functions/pricing/).
|December 8, 2017|
|Update|Added information related to resource names:
###### Note
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
|December 6, 2017|
|Update|Improved security overview information and added a topic on granular IAM permissions. See [Security in AWS Step Functions](./security.html)
and [Creating granular permissions for non-admin users in Step Functions](./concept-create-iam-advanced.html).|November 27, 2017|
|Update|
* Added new screenshots for state machine execution results to reflect changes in the Step Functions console. Rewrote the Lambda instructions in the
following tutorials to reflect changes in the Lambda console:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* Creating a Job Status Poller
* Creating a Task Timer
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
* Corrected and clarified information about creating state machines in the following sections:
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
|October 6, 2017|
|Update|
Rewrote the IAM instructions in the following sections to reflect changes in the IAM console:
* [Creating an IAM role for your state machine in Step Functions](./procedure-create-iam-role.html)
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* Creating a Job Status Poller
* Creating a Task Timer
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
* [Creating a Step Functions API using API Gateway](./tutorial-api-gateway.html)
|October 5, 2017|
|Update|
Rewrote the [State Machine Data](./concepts-statemachines.html#concepts-state-machine-data) section.
|September 28, 2017|
|New feature|
The [limits related to API action throttling](./service-quotas.html#service-limits-api-action-throttling-general) are increased for all regions
where Step Functions is available.
|
September 18, 2017
|
|Update|
* Corrected and clarified information about starting new executions in all tutorials.
* Corrected and clarified information in the [Quotas related to accounts](./service-quotas.html#service-limits-accounts)
section.
|
September 14, 2017
|
|Update|
Rewrote the following tutorials to reflect changes in the Lambda console:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
* Creating a Job Status Poller
|
August 28, 2017
|
|New feature|
Step Functions is available in Europe (London).
|
August 23, 2017
|
|New feature|
The visual workflows of state machines let you zoom in, zoom out, and center the graph.
|
August 21, 2017
|
|
New feature
|
###### Important
An execution can't use the name of another execution for 90 days.
When you make multiple `StartExecution` calls with the same name, the new execution doesn't run.
For more information, see the [`name`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html#API_StartExecution_RequestParameters)
request parameter of the `StartExecution` API action in the *AWS Step Functions API Reference*.
|August 18, 2017|
|Update|
Added information about an alternative way of passing the state machine ARN to the [Creating a Step Functions API using API Gateway](./tutorial-api-gateway.html) tutorial.
|
August 17, 2017
|
|Update|Added the new *Creating a Job Status Poller* tutorial.|
August 10, 2017
|
|
New feature
|
* Step Functions emits the `ExecutionThrottled` CloudWatch metric. For more information, see [Monitoring Step Functions metrics using Amazon CloudWatch](./procedure-cw-metrics.html).
* Added the [Quotas related to state
throttling](./service-quotas.html#service-limits-api-state-throttling) section.
|August 3, 2017|
|
Update
|
Updated the instructions in the [Step 1: Create an IAM Role for API Gateway](./tutorial-api-gateway.html#api-gateway-step-1) section.
|
July 18, 2017
|
|
Update
|
Corrected and clarified information in the [Choice workflow state](./state-choice.html) section.
|June 23, 2017|
|
Update
|
Added information about using resources under other AWS accounts to the following tutorials:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* [Using CloudFormation to create a workflow in Step Functions](./tutorial-lambda-state-machine-cloudformation.html)
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
|
June 22, 2017
|
|
Update
|
Corrected and clarified information in the following sections:
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
* [Discovering workflow states to use in Step Functions](./workflow-states.html)
* [Handling errors in Step Functions workflows](./concepts-error-handling.html)
|
June 21, 2017
|
|
Update
|
Rewrote all tutorials to match the Step Functions console refresh.
|June 12, 2017|
|
New feature
|
Step Functions is available in Asia Pacific (Sydney).
|
June 8, 2017
|
|
Update
|
Restructured the [Using Amazon States Language to define Step Functions workflows](./concepts-amazon-states-language.html) section.
|
June 7, 2017
|
|
Update
|
Corrected and clarified information in the [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html) section.
|
June 6, 2017
|
|
Update
|
Corrected the code examples in the [State machine examples using Retry and Catch](./concepts-error-handling.html#error-handling-examples) section.
|
June 5, 2017
|
|
Update
|
Restructured this guide using AWS documentation standards.
|
May 31, 2017
|
|
Update
|
Corrected and clarified information in the [Parallel workflow state](./state-parallel.html) section.
|
May 25, 2017
|
|
Update
|
Merged the Paths and Filters sections into the [Processing input and output in Step Functions](./concepts-input-output-filtering.html) section.
|
May 24, 2017
|
|
Update
|
Corrected and clarified information in the [Monitoring Step Functions metrics using Amazon CloudWatch](./procedure-cw-metrics.html) section.
|
May 15, 2017
|
|
Update
|
Updated the `GreeterActivities.java` worker code in the [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html) tutorial.
|
May 9, 2017
|
|
Update
|
Added an introductory video to the [What is Step Functions?](./welcome.html) section.
|
April 19, 2017
|
|
Update
|
Corrected and clarified information in the following tutorials:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
|
April 19, 2017
|
|
Update
|
Added information about Lambda templates to the [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html) and [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html) tutorials.
|
April 6, 2017
|
|
Update
|
Changed the "Maximum input or result data size" limit to "Maximum input or result data size for a task, state, or execution" (32,768
characters). For more information, see [Quotas related to task
executions](./service-quotas.html#service-limits-task-executions).
|
March 31, 2017
|
|
New feature
|
* Step Functions supports executing state machines by setting Step Functions as Amazon CloudWatch Events targets.
|
March 21, 2017
|
|
New feature
|
* Step Functions allows Lambda function error handling as the preferred error handling method.
* Updated the [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html) tutorial and the [Handling errors in Step Functions workflows](./concepts-error-handling.html) section.
|
March 16, 2017
|
|
New feature
|
Step Functions is available in Europe (Frankfurt).
|
March 7, 2017
|
|
Update
|
Reorganized the topics in the table of contents and updated the following tutorials:
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
|
February 23, 2017
|
|
New feature
|
* The **State Machines** page of the Step Functions console includes the **Copy to New** and
**Delete** buttons.
* Updated the screenshots to match the console changes.
|
February 23, 2017
|
|
New feature
|
* Step Functions supports creating APIs using API Gateway.
* Added the [Creating a Step Functions API using API Gateway](./tutorial-api-gateway.html) tutorial.
|
February 14, 2017
|
|
New feature
|
* Step Functions supports integration with CloudFormation.
* Added the [Using CloudFormation to create a workflow in Step Functions](./tutorial-lambda-state-machine-cloudformation.html)
tutorial.
|
February 10, 2017
|
|
Update
|
Clarified the current behavior of the `ResultPath` and `OutputPath` fields in relation to `Parallel`
states.
|
February 6, 2017
|
|
Update
|
* Clarified state machine naming restrictions in tutorials.
* Corrected some code examples.
|
January 5, 2017
|
|
Update
|
Updated Lambda function examples to use the latest programming model.
|
December 9, 2016
|
|
Initial release
|
Initial release of AWS Step Functions.
|
December 1, 2016
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Recent feature launches
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.