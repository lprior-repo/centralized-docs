---
url: https://docs.aws.amazon.com/step-functions/latest/dg/supported-services-awssdk.html
title: Learning to use AWS service SDK integrations in
word_count: 4705
filtered: true
elements_removed: 0
density_score: 0.97
---

Learning to use AWS service SDK integrations in Step Functions - AWS Step Functions
Learning to use AWS service SDK integrations in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#supported-services-awssdk)
[Using service integrations](#use-awssdk-integ)[Supported service integrations](#supported-services-awssdk-list)[Deprecated service integrations](#deprecated-aws-sdk-integ)
# Learning to use AWS service SDK integrations in
Step Functions
With Step Functions' AWS SDK integration, your workflows can call almost any AWS service's API actions. The services or SDKs that are not available might be recently released, require customized configuration, or are not suitable for use in a workflow, such as SDKs for streaming audio or video.
###### Topics
* [Using service integrations](#use-awssdk-integ)
* [Supported service integrations](#supported-services-awssdk-list)
* [Deprecated service integrations](#deprecated-aws-sdk-integ)
## Using AWS SDK service integrations
To use AWS SDK integrations, you specify the service name and API call and, optionally,
a service integration pattern (For more information, see [Service integration patterns](./connect-to-resource.html).
###### API naming conventions
* Parameters in Step Functions are expressed in PascalCase, even if the native
service API is in camelCase. For example, you could use the Step Functions API
action `startSyncExecution` and specify its parameter as
`StateMachineArn`.
* For API actions that accept enumerated parameters, such as the `[DescribeLaunchTemplateVersions](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLaunchTemplateVersions.html)` API action for Amazon EC2, specify a
plural version of the parameter name. For example, specify `Filters` for the
`Filter.N` parameter of the `DescribeLaunchTemplateVersions` API
action.
* To work with certain language naming convention requirements, API fields for
**equals** will be referred to in your state
machines as: **EqualsValue**.
You can call AWS SDK services directly from the Amazon States Language in the `Resource` field
of a task state. To do this, use the following syntax:
`arn:aws:states:::aws-sdk:`serviceName`:`apiAction`.`[serviceIntegrationPattern]``
For example, you might use
`arn:aws:states:::aws-sdk:ec2:describeInstances` to return output as
defined for the [Amazon EC2
describeInstances](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstances.html#API_DescribeInstances_ResponseElements) API call.
If an AWS SDK integration encounters an error, the resulting Error field will be
composed of the service name and the error name, separated by a period:
``ServiceName`.`ErrorName``.
Both the service name and error name are in Pascal case. You can also see the service name in
the Task state's Resource field in lowercase. The target service's API reference documentation
lists the potential error names.
Consider an example where you might use the AWS SDK integration
`arn:aws:states:::aws-sdk:acmpca:deleteCertificateAuthority`. The [AWS Private Certificate Authority API Reference](https://docs.aws.amazon.com/privateca/latest/APIReference/API_DeleteCertificateAuthority.html#API_DeleteCertificateAuthority_Errors) indicates that the `DeleteCertificateAuthority` API
action can result in an error named `ResourceNotFoundException`. To handle this
error you would specify the Error `AcmPca.ResourceNotFoundException` in your Task
state's Retriers or Catchers.
###### Note
Some AWS services don't include the *Exception* suffix in the The
API Reference documentation. Despite this alternate naming convention, always include the
*Exception* suffix for the potential error names in your AWS Step Functions
integration. Do so even when the suffix is not already part of the error name provided by the service.
Consider another error name, this time the [CreateBucket](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html) action. The
*Amazon Simple Storage Service API Reference* lists the error `BucketAlreadyExists`. Note
that it doesn't have the *Exception* suffix. To handle this
error in Step Functions, refer to it as `S3.BucketAlreadyExists**Exception**`. The S3 service error naming convention differs from the
errors in the previously mentioned [AWS Private Certificate Authority API Reference](https://docs.aws.amazon.com/privateca/latest/APIReference/API_DeleteCertificateAuthority.html#API_DeleteCertificateAuthority_Errors). Regardless, in both cases you must include the
*Exception* suffix for potential errors in the Step Functions integration.
For more information about error handling, see [Handling errors in Step Functions workflows](./concepts-error-handling.html).
Step Functions cannot autogenerate IAM policies for AWS SDK integrations. After you create your
state machine, you will need to navigate to the IAM console and configure your role
policies. See [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) for more information.
See the [Gather Amazon S3 bucket info using AWS SDK service integrations](./tutorial-gather-s3-info.html)
tutorial for an example of how to use AWS SDK integrations.
## Supported AWS SDK service integrations
The *Task state resource* (Resource) shows the syntax to call a
specific API action for the service. The *Exception prefix*
is present in the exceptions that are generated when you erroneously perform an AWS SDK
service integration with Step Functions.
###### Important
New actions and updates to already supported actions, such as new parameters, will not be immediately available after service SDK updates.
**Amazon A2I**
Task state resource: `arn:aws:states:::aws-sdk:sagemakera2iruntime:`[apiAction]``
Exception prefix: `SageMakerA2IRuntime`
**API Gateway V1**
Task state resource: `arn:aws:states:::aws-sdk:apigateway:`[apiAction]``
Exception prefix: `ApiGateway`
**API Gateway V2**
Task state resource: `arn:aws:states:::aws-sdk:apigatewayv2:`[apiAction]``
Exception prefix: `ApiGatewayV2`
**AWS Account Management**
Task state resource: `arn:aws:states:::aws-sdk:account:`[apiAction]``
Exception prefix: `Account`
**AWS Amplify**
Task state resource: `arn:aws:states:::aws-sdk:amplify:`[apiAction]``
Exception prefix: `Amplify`
**Amplify Backend**
Task state resource: `arn:aws:states:::aws-sdk:amplifybackend:`[apiAction]``
Exception prefix: `AmplifyBackend`
**Amplify UI Builder**
Task state resource: `arn:aws:states:::aws-sdk:amplifyuibuilder:`[apiAction]``
Exception prefix: `AmplifyUiBuilder`
**AWS App Mesh**
Task state resource: `arn:aws:states:::aws-sdk:appmesh:`[apiAction]``
Exception prefix: `AppMesh`
**AWS App Runner**
Task state resource: `arn:aws:states:::aws-sdk:apprunner:`[apiAction]``
Exception prefix: `AppRunner`
**AWS AppConfig**
Task state resource: `arn:aws:states:::aws-sdk:appconfig:`[apiAction]``
Exception prefix: `AppConfig`
**AWS AppConfig Data**
Task state resource: `arn:aws:states:::aws-sdk:appconfigdata:`[apiAction]``
Exception prefix: `AppConfigData`
**AWS AppFabric**
Task state resource: `arn:aws:states:::aws-sdk:appfabric:`[apiAction]``
Exception prefix: `AppFabric`
**AppIntegrations**
Task state resource: `arn:aws:states:::aws-sdk:appintegrations:`[apiAction]``
Exception prefix: `AppIntegrations`
**Amazon AppStream**
Task state resource: `arn:aws:states:::aws-sdk:appstream:`[apiAction]``
Exception prefix: `AppStream`
**AWS AppSync**
Task state resource: `arn:aws:states:::aws-sdk:appsync:`[apiAction]``
Exception prefix: `AppSync`
**Amazon Appflow**
Task state resource: `arn:aws:states:::aws-sdk:appflow:`[apiAction]``
Exception prefix: `Appflow`
**Application Auto Scaling**
Task state resource: `arn:aws:states:::aws-sdk:applicationautoscaling:`[apiAction]``
Exception prefix: `ApplicationAutoScaling`
**Application Cost Profiler**
Task state resource: `arn:aws:states:::aws-sdk:applicationcostprofiler:`[apiAction]``
Exception prefix: `ApplicationCostProfiler`
**Application Discovery Service**
Task state resource: `arn:aws:states:::aws-sdk:applicationdiscovery:`[apiAction]``
Exception prefix: `ApplicationDiscovery`
**Unsupported operations:** `DescribeExportConfigurations`, `ExportConfigurations`
**Application Migration Service**
Task state resource: `arn:aws:states:::aws-sdk:mgn:`[apiAction]``
Exception prefix: `Mgn`
**Amazon Athena**
Task state resource: `arn:aws:states:::aws-sdk:athena:`[apiAction]``
Exception prefix: `Athena`
**Audit Manager**
Task state resource: `arn:aws:states:::aws-sdk:auditmanager:`[apiAction]``
Exception prefix: `AuditManager`
**Amazon Aurora DSQL**
Task state resource: `arn:aws:states:::aws-sdk:dsql:`[apiAction]``
Exception prefix: `Dsql`
**AWS Auto Scaling**
Task state resource: `arn:aws:states:::aws-sdk:autoscalingplans:`[apiAction]``
Exception prefix: `AutoScalingPlans`
**B2B Data Interchange**
Task state resource: `arn:aws:states:::aws-sdk:b2bi:`[apiAction]``
Exception prefix: `B2Bi`
**AWS Backup**
Task state resource: `arn:aws:states:::aws-sdk:backup:`[apiAction]``
Exception prefix: `Backup`
**AWS Backup Gateway**
Task state resource: `arn:aws:states:::aws-sdk:backupgateway:`[apiAction]``
Exception prefix: `BackupGateway`
**AWS Backup Search**
Task state resource: `arn:aws:states:::aws-sdk:backupsearch:`[apiAction]``
Exception prefix: `BackupSearch`
**AWS Batch**
Task state resource: `arn:aws:states:::aws-sdk:batch:`[apiAction]``
Exception prefix: `Batch`
**Amazon Bedrock**
Task state resource: `arn:aws:states:::aws-sdk:bedrock:`[apiAction]``
Exception prefix: `Bedrock`
**Amazon Bedrock Agents**
Task state resource: `arn:aws:states:::aws-sdk:bedrockagent:`[apiAction]``
Exception prefix: `BedrockAgent`
**Amazon Bedrock Runtime**
Task state resource: `arn:aws:states:::aws-sdk:bedrockruntime:`[apiAction]``
Exception prefix: `BedrockRuntime`
**Unsupported operations:** `InvokeModelWithResponseStream`, `ConverseStream`
**Amazon Bedrock Runtime Agents**
Task state resource: `arn:aws:states:::aws-sdk:bedrockagentruntime:`[apiAction]``
Exception prefix: `BedrockAgentRuntime`
**Unsupported operations:** `InvokeAgent`, `InvokeFlow`, `InvokeInlineAgent`, `OptimizePrompt`, `RetrieveAndGenerateStream`
**AWS Billing**
Task state resource: `arn:aws:states:::aws-sdk:billing:`[apiAction]``
Exception prefix: `Billing`
**AWS Billing Conductor**
Task state resource: `arn:aws:states:::aws-sdk:billingconductor:`[apiAction]``
Exception prefix: `Billingconductor`
**AWS Billing and Cost Management Pricing Calculator**
Task state resource: `arn:aws:states:::aws-sdk:bcmpricingcalculator:`[apiAction]``
Exception prefix: `BcmPricingCalculator`
**Amazon Braket**
Task state resource: `arn:aws:states:::aws-sdk:braket:`[apiAction]``
Exception prefix: `Braket`
**AWS Budgets**
Task state resource: `arn:aws:states:::aws-sdk:budgets:`[apiAction]``
Exception prefix: `Budgets`
**Certificate Manager**
Task state resource: `arn:aws:states:::aws-sdk:acm:`[apiAction]``
Exception prefix: `Acm`
**Certificate Manager PCA**
Task state resource: `arn:aws:states:::aws-sdk:acmpca:`[apiAction]``
Exception prefix: `AcmPca`
**Amazon Chime**
Task state resource: `arn:aws:states:::aws-sdk:chime:`[apiAction]``
Exception prefix: `Chime`
**Amazon Chime Identity**
Task state resource: `arn:aws:states:::aws-sdk:chimesdkidentity:`[apiAction]``
Exception prefix: `ChimeSdkIdentity`
**Amazon Chime Media Pipelines**
Task state resource: `arn:aws:states:::aws-sdk:chimesdkmediapipelines:`[apiAction]``
Exception prefix: `ChimeSdkMediaPipelines`
**Amazon Chime Meetings**
Task state resource: `arn:aws:states:::aws-sdk:chimesdkmeetings:`[apiAction]``
Exception prefix: `ChimeSdkMeetings`
**Amazon Chime Messaging**
Task state resource: `arn:aws:states:::aws-sdk:chimesdkmessaging:`[apiAction]``
Exception prefix: `ChimeSdkMessaging`
**Amazon Chime Voice**
Task state resource: `arn:aws:states:::aws-sdk:chimesdkvoice:`[apiAction]``
Exception prefix: `ChimeSdkVoice`
**AWS Clean Rooms**
Task state resource: `arn:aws:states:::aws-sdk:cleanrooms:`[apiAction]``
Exception prefix: `CleanRooms`
**AWS Clean Rooms ML**
Task state resource: `arn:aws:states:::aws-sdk:cleanroomsml:`[apiAction]``
Exception prefix: `CleanRoomsMl`
**AWS Cloud Control**
Task state resource: `arn:aws:states:::aws-sdk:cloudcontrol:`[apiAction]``
Exception prefix: `CloudControl`
**Cloud Directory**
Task state resource: `arn:aws:states:::aws-sdk:clouddirectory:`[apiAction]``
Exception prefix: `CloudDirectory`
**AWS Cloud Map**
Task state resource: `arn:aws:states:::aws-sdk:servicediscovery:`[apiAction]``
Exception prefix: `ServiceDiscovery`
**AWS Cloud9**
Task state resource: `arn:aws:states:::aws-sdk:cloud9:`[apiAction]``
Exception prefix: `Cloud9`
**CloudFormation**
Task state resource: `arn:aws:states:::aws-sdk:cloudformation:`[apiAction]``
Exception prefix: `CloudFormation`
**CloudFront**
Task state resource: `arn:aws:states:::aws-sdk:cloudfront:`[apiAction]``
Exception prefix: `CloudFront`
**Amazon CloudFront KeyValueStore**
Task state resource: `arn:aws:states:::aws-sdk:cloudfrontkeyvaluestore:`[apiAction]``
Exception prefix: `CloudFrontKeyValueStore`
**CloudHSM V1**
Task state resource: `arn:aws:states:::aws-sdk:cloudhsm:`[apiAction]``
Exception prefix: `CloudHsm`
**CloudHSM V2**
Task state resource: `arn:aws:states:::aws-sdk:cloudhsmv2:`[apiAction]``
Exception prefix: `CloudHsmV2`
**CloudSearch**
Task state resource: `arn:aws:states:::aws-sdk:cloudsearch:`[apiAction]``
Exception prefix: `CloudSearch`
**CloudTrail**
Task state resource: `arn:aws:states:::aws-sdk:cloudtrail:`[apiAction]``
Exception prefix: `CloudTrail`
**CloudTrail Data**
Task state resource: `arn:aws:states:::aws-sdk:cloudtraildata:`[apiAction]``
Exception prefix: `CloudTrailData`
**CloudWatch**
Task state resource: `arn:aws:states:::aws-sdk:cloudwatch:`[apiAction]``
Exception prefix: `CloudWatch`
**CloudWatch Application Insights**
Task state resource: `arn:aws:states:::aws-sdk:applicationinsights:`[apiAction]``
Exception prefix: `ApplicationInsights`
**Amazon CloudWatch Application Signals**
Task state resource: `arn:aws:states:::aws-sdk:applicationsignals:`[apiAction]``
Exception prefix: `ApplicationSignals`
**CloudWatch Internet Monitor**
Task state resource: `arn:aws:states:::aws-sdk:internetmonitor:`[apiAction]``
Exception prefix: `InternetMonitor`
**CloudWatch Logs**
Task state resource: `arn:aws:states:::aws-sdk:cloudwatchlogs:`[apiAction]``
Exception prefix: `CloudWatchLogs`
**Unsupported operations:** `StartLiveTail`
**CloudWatch Observability Access Manager**
Task state resource: `arn:aws:states:::aws-sdk:oam:`[apiAction]``
Exception prefix: `Oam`
**CloudWatch Observability Admin Service**
Task state resource: `arn:aws:states:::aws-sdk:observabilityadmin:`[apiAction]``
Exception prefix: `ObservabilityAdmin`
**CloudWatch RUM**
Task state resource: `arn:aws:states:::aws-sdk:rum:`[apiAction]``
Exception prefix: `Rum`
**CloudWatch Synthetics**
Task state resource: `arn:aws:states:::aws-sdk:synthetics:`[apiAction]``
Exception prefix: `Synthetics`
**CodeArtifact**
Task state resource: `arn:aws:states:::aws-sdk:codeartifact:`[apiAction]``
Exception prefix: `Codeartifact`
**CodeBuild**
Task state resource: `arn:aws:states:::aws-sdk:codebuild:`[apiAction]``
Exception prefix: `CodeBuild`
**Amazon CodeCatalyst**
Task state resource: `arn:aws:states:::aws-sdk:codecatalyst:`[apiAction]``
Exception prefix: `CodeCatalyst`
**CodeCommit**
Task state resource: `arn:aws:states:::aws-sdk:codecommit:`[apiAction]``
Exception prefix: `CodeCommit`
**AWS CodeConnections**
Task state resource: `arn:aws:states:::aws-sdk:codeconnections:`[apiAction]``
Exception prefix: `CodeConnections`
**CodeDeploy**
Task state resource: `arn:aws:states:::aws-sdk:codedeploy:`[apiAction]``
Exception prefix: `CodeDeploy`
**Unsupported operations:** `BatchGetDeploymentInstances`, `GetDeploymentInstance`, `ListDeploymentInstances`, `SkipWaitTimeForInstanceTermination`
**CodeGuru Profiler**
Task state resource: `arn:aws:states:::aws-sdk:codeguruprofiler:`[apiAction]``
Exception prefix: `CodeGuruProfiler`
**CodeGuru Reviewer**
Task state resource: `arn:aws:states:::aws-sdk:codegurureviewer:`[apiAction]``
Exception prefix: `CodeGuruReviewer`
**CodeGuru Security**
Task state resource: `arn:aws:states:::aws-sdk:codegurusecurity:`[apiAction]``
Exception prefix: `CodeGuruSecurity`
**CodePipeline**
Task state resource: `arn:aws:states:::aws-sdk:codepipeline:`[apiAction]``
Exception prefix: `CodePipeline`
**AWS CodeStar Connections**
Task state resource: `arn:aws:states:::aws-sdk:codestarconnections:`[apiAction]``
Exception prefix: `CodeStarConnections`
**AWS CodeStar Notifications**
Task state resource: `arn:aws:states:::aws-sdk:codestarnotifications:`[apiAction]``
Exception prefix: `CodestarNotifications`
**Cognito Identity Pools**
Task state resource: `arn:aws:states:::aws-sdk:cognitoidentity:`[apiAction]``
Exception prefix: `CognitoIdentity`
**Cognito Sync**
Task state resource: `arn:aws:states:::aws-sdk:cognitosync:`[apiAction]``
Exception prefix: `CognitoSync`
**Cognito User Pools**
Task state resource: `arn:aws:states:::aws-sdk:cognitoidentityprovider:`[apiAction]``
Exception prefix: `CognitoIdentityProvider`
**Amazon Comprehend**
Task state resource: `arn:aws:states:::aws-sdk:comprehend:`[apiAction]``
Exception prefix: `Comprehend`
**Amazon Comprehend Medical**
Task state resource: `arn:aws:states:::aws-sdk:comprehendmedical:`[apiAction]``
Exception prefix: `ComprehendMedical`
**Unsupported operations:** `DetectEntities`
**Compute Optimizer**
Task state resource: `arn:aws:states:::aws-sdk:computeoptimizer:`[apiAction]``
Exception prefix: `ComputeOptimizer`
**AWS Config**
Task state resource: `arn:aws:states:::aws-sdk:config:`[apiAction]``
Exception prefix: `Config`
**Amazon Connect**
Task state resource: `arn:aws:states:::aws-sdk:connect:`[apiAction]``
Exception prefix: `Connect`
**Amazon Connect Campaigns**
Task state resource: `arn:aws:states:::aws-sdk:connectcampaigns:`[apiAction]``
Exception prefix: `ConnectCampaigns`
**Amazon Connect Campaigns V2**
Task state resource: `arn:aws:states:::aws-sdk:connectcampaignsv2:`[apiAction]``
Exception prefix: `ConnectCampaignsV2`
**Amazon Connect Cases**
Task state resource: `arn:aws:states:::aws-sdk:connectcases:`[apiAction]``
Exception prefix: `ConnectCases`
**Amazon Connect Contact Lens**
Task state resource: `arn:aws:states:::aws-sdk:connectcontactlens:`[apiAction]``
Exception prefix: `ConnectContactLens`
**Amazon Connect Customer Profiles**
Task state resource: `arn:aws:states:::aws-sdk:customerprofiles:`[apiAction]``
Exception prefix: `CustomerProfiles`
**Amazon Connect Participant**
Task state resource: `arn:aws:states:::aws-sdk:connectparticipant:`[apiAction]``
Exception prefix: `ConnectParticipant`
**Amazon Connect Voice ID**
Task state resource: `arn:aws:states:::aws-sdk:voiceid:`[apiAction]``
Exception prefix: `VoiceId`
**Amazon Connect Wisdom**
Task state resource: `arn:aws:states:::aws-sdk:wisdom:`[apiAction]``
Exception prefix: `Wisdom`
**AWS Control Catalog**
Task state resource: `arn:aws:states:::aws-sdk:controlcatalog:`[apiAction]``
Exception prefix: `ControlCatalog`
**AWS Control Tower**
Task state resource: `arn:aws:states:::aws-sdk:controltower:`[apiAction]``
Exception prefix: `ControlTower`
**AWS Cost Explorer**
Task state resource: `arn:aws:states:::aws-sdk:costexplorer:`[apiAction]``
Exception prefix: `CostExplorer`
**Cost Optimization Hub**
Task state resource: `arn:aws:states:::aws-sdk:costoptimizationhub:`[apiAction]``
Exception prefix: `CostOptimizationHub`
**AWS Cost and Usage Report**
Task state resource: `arn:aws:states:::aws-sdk:costandusagereport:`[apiAction]``
Exception prefix: `CostAndUsageReport`
**Data Automation for Amazon Bedrock**
Task state resource: `arn:aws:states:::aws-sdk:bedrockdataautomation:`[apiAction]``
Exception prefix: `BedrockDataAutomation`
**AWS Data Exchange**
Task state resource: `arn:aws:states:::aws-sdk:dataexchange:`[apiAction]``
Exception prefix: `DataExchange`
**Unsupported operations:** `SendApiAsset`
**AWS Data Exports**
Task state resource: `arn:aws:states:::aws-sdk:bcmdataexports:`[apiAction]``
Exception prefix: `BcmDataExports`
**Amazon Data Lifecycle Manager**
Task state resource: `arn:aws:states:::aws-sdk:dlm:`[apiAction]``
Exception prefix: `Dlm`
**Data Pipeline**
Task state resource: `arn:aws:states:::aws-sdk:datapipeline:`[apiAction]``
Exception prefix: `DataPipeline`
**DataSync**
Task state resource: `arn:aws:states:::aws-sdk:datasync:`[apiAction]``
Exception prefix: `DataSync`
**Amazon DataZone**
Task state resource: `arn:aws:states:::aws-sdk:datazone:`[apiAction]``
Exception prefix: `DataZone`
**AWS Database Migration Service**
Task state resource: `arn:aws:states:::aws-sdk:databasemigration:`[apiAction]``
Exception prefix: `DatabaseMigration`
**AWS Deadline Cloud**
Task state resource: `arn:aws:states:::aws-sdk:deadline:`[apiAction]``
Exception prefix: `Deadline`
**Detective**
Task state resource: `arn:aws:states:::aws-sdk:detective:`[apiAction]``
Exception prefix: `Detective`
**DevOps Guru**
Task state resource: `arn:aws:states:::aws-sdk:devopsguru:`[apiAction]``
Exception prefix: `DevOpsGuru`
**Device Farm**
Task state resource: `arn:aws:states:::aws-sdk:devicefarm:`[apiAction]``
Exception prefix: `DeviceFarm`
**Direct Connect**
Task state resource: `arn:aws:states:::aws-sdk:directconnect:`[apiAction]``
Exception prefix: `DirectConnect`
**Unsupported operations:** `AllocateConnectionOnInterconnect`, `DescribeConnectionLoa`, `DescribeConnectionsOnInterconnect`, `DescribeInterconnectLoa`
**Directory Service**
Task state resource: `arn:aws:states:::aws-sdk:directory:`[apiAction]``
Exception prefix: `Directory`
**AWS Directory Service Data**
Task state resource: `arn:aws:states:::aws-sdk:directoryservicedata:`[apiAction]``
Exception prefix: `DirectoryServiceData`
**Amazon DocumentDB**
Task state resource: `arn:aws:states:::aws-sdk:docdb:`[apiAction]``
Exception prefix: `DocDb`
**Amazon DocumentDB Elastic Clusters**
Task state resource: `arn:aws:states:::aws-sdk:docdbelastic:`[apiAction]``
Exception prefix: `DocDbElastic`
**DynamoDB**
Task state resource: `arn:aws:states:::aws-sdk:dynamodb:`[apiAction]``
Exception prefix: `DynamoDb`
**DynamoDB Accelerator**
Task state resource: `arn:aws:states:::aws-sdk:dax:`[apiAction]``
Exception prefix: `Dax`
**DynamoDB Streams**
Task state resource: `arn:aws:states:::aws-sdk:dynamodbstreams:`[apiAction]``
Exception prefix: `DynamoDbStreams`
**Amazon EBS**
Task state resource: `arn:aws:states:::aws-sdk:ebs:`[apiAction]``
Exception prefix: `Ebs`
**Amazon EC2**
Task state resource: `arn:aws:states:::aws-sdk:ec2:`[apiAction]``
Exception prefix: `Ec2`
**EC2 Auto Scaling**
Task state resource: `arn:aws:states:::aws-sdk:autoscaling:`[apiAction]``
Exception prefix: `AutoScaling`
**EC2 Image Builder**
Task state resource: `arn:aws:states:::aws-sdk:imagebuilder:`[apiAction]``
Exception prefix: `Imagebuilder`
**AWS EC2 Instance Connect**
Task state resource: `arn:aws:states:::aws-sdk:ec2instanceconnect:`[apiAction]``
Exception prefix: `Ec2InstanceConnect`
**Amazon ECR**
Task state resource: `arn:aws:states:::aws-sdk:ecr:`[apiAction]``
Exception prefix: `Ecr`
**Amazon ECR Public**
Task state resource: `arn:aws:states:::aws-sdk:ecrpublic:`[apiAction]``
Exception prefix: `EcrPublic`
**Amazon ECS**
Task state resource: `arn:aws:states:::aws-sdk:ecs:`[apiAction]``
Exception prefix: `Ecs`
**Amazon EFS**
Task state resource: `arn:aws:states:::aws-sdk:efs:`[apiAction]``
Exception prefix: `Efs`
**Unsupported operations:** `CreateTags`
**Amazon EKS**
Task state resource: `arn:aws:states:::aws-sdk:eks:`[apiAction]``
Exception prefix: `Eks`
**Amazon EKS Auth**
Task state resource: `arn:aws:states:::aws-sdk:eksauth:`[apiAction]``
Exception prefix: `EksAuth`
**Amazon EMR**
Task state resource: `arn:aws:states:::aws-sdk:emr:`[apiAction]``
Exception prefix: `Emr`
**Unsupported operations:** `DescribeJobFlows`
**Amazon EMR Containers**
Task state resource: `arn:aws:states:::aws-sdk:emrcontainers:`[apiAction]``
Exception prefix: `EmrContainers`
**Amazon EMR Serverless**
Task state resource: `arn:aws:states:::aws-sdk:emrserverless:`[apiAction]``
Exception prefix: `EmrServerless`
**ElastiCache**
Task state resource: `arn:aws:states:::aws-sdk:elasticache:`[apiAction]``
Exception prefix: `ElastiCache`
**Elastic Beanstalk**
Task state resource: `arn:aws:states:::aws-sdk:elasticbeanstalk:`[apiAction]``
Exception prefix: `ElasticBeanstalk`
**Elastic Disaster Recovery**
Task state resource: `arn:aws:states:::aws-sdk:drs:`[apiAction]``
Exception prefix: `Drs`
**Elastic Inference**
Task state resource: `arn:aws:states:::aws-sdk:elasticinference:`[apiAction]``
Exception prefix: `ElasticInference`
**Elastic Load Balancing V1**
Task state resource: `arn:aws:states:::aws-sdk:elasticloadbalancing:`[apiAction]``
Exception prefix: `ElasticLoadBalancing`
**Elastic Load Balancing V2**
Task state resource: `arn:aws:states:::aws-sdk:elasticloadbalancingv2:`[apiAction]``
Exception prefix: `ElasticLoadBalancingV2`
**Elastic Transcoder**
Task state resource: `arn:aws:states:::aws-sdk:elastictranscoder:`[apiAction]``
Exception prefix: `ElasticTranscoder`
**Unsupported operations:** `TestRole`
**Amazon ElasticSearch**
Task state resource: `arn:aws:states:::aws-sdk:elasticsearch:`[apiAction]``
Exception prefix: `Elasticsearch`
**AWS End User Messaging Social**
Task state resource: `arn:aws:states:::aws-sdk:socialmessaging:`[apiAction]``
Exception prefix: `SocialMessaging`
**AWS Entity Resolution**
Task state resource: `arn:aws:states:::aws-sdk:entityresolution:`[apiAction]``
Exception prefix: `EntityResolution`
**Amazon EventBridge**
Task state resource: `arn:aws:states:::aws-sdk:eventbridge:`[apiAction]``
Exception prefix: `EventBridge`
**EventBridge Pipes**
Task state resource: `arn:aws:states:::aws-sdk:pipes:`[apiAction]``
Exception prefix: `Pipes`
**EventBridge Scheduler**
Task state resource: `arn:aws:states:::aws-sdk:scheduler:`[apiAction]``
Exception prefix: `Scheduler`
**EventBridge Schema Registry**
Task state resource: `arn:aws:states:::aws-sdk:schemas:`[apiAction]``
Exception prefix: `Schemas`
**Evidently**
Task state resource: `arn:aws:states:::aws-sdk:evidently:`[apiAction]``
Exception prefix: `Evidently`
**AWS FIS**
Task state resource: `arn:aws:states:::aws-sdk:fis:`[apiAction]``
Exception prefix: `Fis`
**Amazon FSx**
Task state resource: `arn:aws:states:::aws-sdk:fsx:`[apiAction]``
Exception prefix: `FSx`
**FinSpace Data**
Task state resource: `arn:aws:states:::aws-sdk:finspacedata:`[apiAction]``
Exception prefix: `FinspaceData`
**FinSpace Management**
Task state resource: `arn:aws:states:::aws-sdk:finspace:`[apiAction]``
Exception prefix: `Finspace`
**Firewall Manager**
Task state resource: `arn:aws:states:::aws-sdk:fms:`[apiAction]``
Exception prefix: `Fms`
**Amazon Forecast**
Task state resource: `arn:aws:states:::aws-sdk:forecast:`[apiAction]``
Exception prefix: `Forecast`
**Amazon Forecast Query**
Task state resource: `arn:aws:states:::aws-sdk:forecastquery:`[apiAction]``
Exception prefix: `Forecastquery`
**Amazon Fraud Detector**
Task state resource: `arn:aws:states:::aws-sdk:frauddetector:`[apiAction]``
Exception prefix: `FraudDetector`
**AWS Free Tier**
Task state resource: `arn:aws:states:::aws-sdk:freetier:`[apiAction]``
Exception prefix: `FreeTier`
**Amazon GameLift**
Task state resource: `arn:aws:states:::aws-sdk:gamelift:`[apiAction]``
Exception prefix: `GameLift`
**AWS Glue**
Task state resource: `arn:aws:states:::aws-sdk:glue:`[apiAction]``
Exception prefix: `Glue`
**AWS Glue DataBrew**
Task state resource: `arn:aws:states:::aws-sdk:databrew:`[apiAction]``
Exception prefix: `DataBrew`
**AWS Ground Station**
Task state resource: `arn:aws:states:::aws-sdk:groundstation:`[apiAction]``
Exception prefix: `GroundStation`
**Amazon GuardDuty**
Task state resource: `arn:aws:states:::aws-sdk:guardduty:`[apiAction]``
Exception prefix: `GuardDuty`
**AWS Health**
Task state resource: `arn:aws:states:::aws-sdk:health:`[apiAction]``
Exception prefix: `Health`
**AWS Health Imaging**
Task state resource: `arn:aws:states:::aws-sdk:medicalimaging:`[apiAction]``
Exception prefix: `MedicalImaging`
**Amazon HealthLake**
Task state resource: `arn:aws:states:::aws-sdk:healthlake:`[apiAction]``
Exception prefix: `HealthLake`
**Amazon Honeycode**
Task state resource: `arn:aws:states:::aws-sdk:honeycode:`[apiAction]``
Exception prefix: `Honeycode`
**IAM**
Task state resource: `arn:aws:states:::aws-sdk:iam:`[apiAction]``
Exception prefix: `Iam`
**IAM Access Analyzer**
Task state resource: `arn:aws:states:::aws-sdk:accessanalyzer:`[apiAction]``
Exception prefix: `AccessAnalyzer`
**IAM Roles Anywhere**
Task state resource: `arn:aws:states:::aws-sdk:rolesanywhere:`[apiAction]``
Exception prefix: `RolesAnywhere`
**Amazon IVS**
Task state resource: `arn:aws:states:::aws-sdk:ivs:`[apiAction]``
Exception prefix: `Ivs`
**Amazon IVS Chat**
Task state resource: `arn:aws:states:::aws-sdk:ivschat:`[apiAction]``
Exception prefix: `Ivschat`
**Amazon IVS RealTime**
Task state resource: `arn:aws:states:::aws-sdk:ivsrealtime:`[apiAction]``
Exception prefix: `IvsRealTime`
**Incident Manager**
Task state resource: `arn:aws:states:::aws-sdk:ssmincidents:`[apiAction]``
Exception prefix: `SsmIncidents`
**Incident Manager Contacts**
Task state resource: `arn:aws:states:::aws-sdk:ssmcontacts:`[apiAction]``
Exception prefix: `SsmContacts`
**Amazon Inspector Scan**
Task state resource: `arn:aws:states:::aws-sdk:inspectorscan:`[apiAction]``
Exception prefix: `InspectorScan`
**Amazon Inspector V1**
Task state resource: `arn:aws:states:::aws-sdk:inspector:`[apiAction]``
Exception prefix: `Inspector`
**Amazon Inspector V2**
Task state resource: `arn:aws:states:::aws-sdk:inspector2:`[apiAction]``
Exception prefix: `Inspector2`
**AWS Invoicing**
Task state resource: `arn:aws:states:::aws-sdk:invoicing:`[apiAction]``
Exception prefix: `Invoicing`
**AWS IoT**
Task state resource: `arn:aws:states:::aws-sdk:iot:`[apiAction]``
Exception prefix: `Iot`
**Unsupported operations:** `AttachPrincipalPolicy`, `ListPrincipalPolicies`, `DetachPrincipalPolicy`, `ListPolicyPrincipals`, `DetachPrincipalPolicy`
**AWS IoT Analytics**
Task state resource: `arn:aws:states:::aws-sdk:iotanalytics:`[apiAction]``
Exception prefix: `IoTAnalytics`
**AWS IoT Device Advisor**
Task state resource: `arn:aws:states:::aws-sdk:iotdeviceadvisor:`[apiAction]``
Exception prefix: `IotDeviceAdvisor`
**Unsupported operations:** `ListTestCases`
**AWS IoT Events**
Task state resource: `arn:aws:states:::aws-sdk:iotevents:`[apiAction]``
Exception prefix: `IotEvents`
**AWS IoT Events Data**
Task state resource: `arn:aws:states:::aws-sdk:ioteventsdata:`[apiAction]``
Exception prefix: `IotEventsData`
**AWS IoT Fleet Hub**
Task state resource: `arn:aws:states:::aws-sdk:iotfleethub:`[apiAction]``
Exception prefix: `IoTFleetHub`
**AWS IoT FleetWise**
Task state resource: `arn:aws:states:::aws-sdk:iotfleetwise:`[apiAction]``
Exception prefix: `IoTFleetWise`
**AWS IoT Greengrass V1**
Task state resource: `arn:aws:states:::aws-sdk:greengrass:`[apiAction]``
Exception prefix: `Greengrass`
**AWS IoT Greengrass V2**
Task state resource: `arn:aws:states:::aws-sdk:greengrassv2:`[apiAction]``
Exception prefix: `GreengrassV2`
**AWS IoT Jobs Data**
Task state resource: `arn:aws:states:::aws-sdk:iotjobsdataplane:`[apiAction]``
Exception prefix: `IotJobsDataPlane`
**AWS IoT Secure Tunneling**
Task state resource: `arn:aws:states:::aws-sdk:iotsecuretunneling:`[apiAction]``
Exception prefix: `IoTSecureTunneling`
**AWS IoT SiteWise**
Task state resource: `arn:aws:states:::aws-sdk:iotsitewise:`[apiAction]``
Exception prefix: `IoTSiteWise`
**Unsupported operations:** `InvokeAssistant`
**AWS IoT Things Graph**
Task state resource: `arn:aws:states:::aws-sdk:iotthingsgraph:`[apiAction]``
Exception prefix: `IoTThingsGraph`
**AWS IoT TwinMaker**
Task state resource: `arn:aws:states:::aws-sdk:iottwinmaker:`[apiAction]``
Exception prefix: `IoTTwinMaker`
**AWS IoT Wireless**
Task state resource: `arn:aws:states:::aws-sdk:iotwireless:`[apiAction]``
Exception prefix: `IotWireless`
**AWS KMS**
Task state resource: `arn:aws:states:::aws-sdk:kms:`[apiAction]``
Exception prefix: `Kms`
**Amazon Kendra**
Task state resource: `arn:aws:states:::aws-sdk:kendra:`[apiAction]``
Exception prefix: `Kendra`
**Amazon Kendra Intelligent Ranking**
Task state resource: `arn:aws:states:::aws-sdk:kendraranking:`[apiAction]``
Exception prefix: `KendraRanking`
**Amazon Keyspaces**
Task state resource: `arn:aws:states:::aws-sdk:keyspaces:`[apiAction]``
Exception prefix: `Keyspaces`
**Kinesis Data Analytics V1**
Task state resource: `arn:aws:states:::aws-sdk:kinesisanalytics:`[apiAction]``
Exception prefix: `KinesisAnalytics`
**Kinesis Data Analytics V2**
Task state resource: `arn:aws:states:::aws-sdk:kinesisanalyticsv2:`[apiAction]``
Exception prefix: `KinesisAnalyticsV2`
**Kinesis Data Firehose**
Task state resource: `arn:aws:states:::aws-sdk:firehose:`[apiAction]``
Exception prefix: `Firehose`
**Kinesis Data Streams**
Task state resource: `arn:aws:states:::aws-sdk:kinesis:`[apiAction]``
Exception prefix: `Kinesis`
**Unsupported operations:** `SubscribeToShard`
**Kinesis Video Signaling Channels**
Task state resource: `arn:aws:states:::aws-sdk:kinesisvideosignaling:`[apiAction]``
Exception prefix: `KinesisVideoSignaling`
**Kinesis Video Streams**
Task state resource: `arn:aws:states:::aws-sdk:kinesisvideo:`[apiAction]``
Exception prefix: `KinesisVideo`
**Kinesis Video Streams Archived Media**
Task state resource: `arn:aws:states:::aws-sdk:kinesisvideoarchivedmedia:`[apiAction]``
Exception prefix: `KinesisVideoArchivedMedia`
**Kinesis Video Streams Media**
Task state resource: `arn:aws:states:::aws-sdk:kinesisvideomedia:`[apiAction]``
Exception prefix: `KinesisVideoMedia`
**Kinesis Video WebRTC Storage**
Task state resource: `arn:aws:states:::aws-sdk:kinesisvideowebrtcstorage:`[apiAction]``
Exception prefix: `KinesisVideoWebRtcStorage`
**AWS Lake Formation**
Task state resource: `arn:aws:states:::aws-sdk:lakeformation:`[apiAction]``
Exception prefix: `LakeFormation`
**AWS Lambda**
Task state resource: `arn:aws:states:::aws-sdk:lambda:`[apiAction]``
Exception prefix: `Lambda`
**Unsupported operations:** `InvokeAsync`, `InvokeWithResponseStream`
**AWS Launch Wizard**
Task state resource: `arn:aws:states:::aws-sdk:launchwizard:`[apiAction]``
Exception prefix: `LaunchWizard`
**Amazon Lex Model Building V1**
Task state resource: `arn:aws:states:::aws-sdk:lexmodelbuilding:`[apiAction]``
Exception prefix: `LexModelBuilding`
**Amazon Lex Model Building V2**
Task state resource: `arn:aws:states:::aws-sdk:lexmodelsv2:`[apiAction]``
Exception prefix: `LexModelsV2`
**Amazon Lex Runtime V1**
Task state resource: `arn:aws:states:::aws-sdk:lexruntime:`[apiAction]``
Exception prefix: `LexRuntime`
**Amazon Lex Runtime V2**
Task state resource: `arn:aws:states:::aws-sdk:lexruntimev2:`[apiAction]``
Exception prefix: `LexRuntimeV2`
**Unsupported operations:** `StartConversation`
**AWS License Manager**
Task state resource: `arn:aws:states:::aws-sdk:licensemanager:`[apiAction]``
Exception prefix: `LicenseManager`
**License Manager Linux Subscriptions**
Task state resource: `arn:aws:states:::aws-sdk:licensemanagerlinuxsubscriptions:`[apiAction]``
Exception prefix: `LicenseManagerLinuxSubscriptions`
**License Manager User Subscriptions**
Task state resource: `arn:aws:states:::aws-sdk:licensemanagerusersubscriptions:`[apiAction]``
Exception prefix: `LicenseManagerUserSubscriptions`
**Amazon Lightsail**
Task state resource: `arn:aws:states:::aws-sdk:lightsail:`[apiAction]``
Exception prefix: `Lightsail`
**Amazon Location**
Task state resource: `arn:aws:states:::aws-sdk:location:`[apiAction]``
Exception prefix: `Location`
**Amazon Location Service Maps V2**
Task state resource: `arn:aws:states:::aws-sdk:geomaps:`[apiAction]``
Exception prefix: `GeoMaps`
**Amazon Location Service Places V2**
Task state resource: `arn:aws:states:::aws-sdk:geoplaces:`[apiAction]``
Exception prefix: `GeoPlaces`
**Amazon Location Service Routes V2**
Task state resource: `arn:aws:states:::aws-sdk:georoutes:`[apiAction]``
Exception prefix: `GeoRoutes`
**Lookout for Equipment**
Task state resource: `arn:aws:states:::aws-sdk:lookoutequipment:`[apiAction]``
Exception prefix: `LookoutEquipment`
**Lookout for Metrics**
Task state resource: `arn:aws:states:::aws-sdk:lookoutmetrics:`[apiAction]``
Exception prefix: `LookoutMetrics`
**Lookout for Vision**
Task state resource: `arn:aws:states:::aws-sdk:lookoutvision:`[apiAction]``
Exception prefix: `LookoutVision`
**Amazon MQ**
Task state resource: `arn:aws:states:::aws-sdk:mq:`[apiAction]``
Exception prefix: `Mq`
**Amazon MSK**
Task state resource: `arn:aws:states:::aws-sdk:kafka:`[apiAction]``
Exception prefix: `Kafka`
**Amazon MSK Connect**
Task state resource: `arn:aws:states:::aws-sdk:kafkaconnect:`[apiAction]``
Exception prefix: `KafkaConnect`
**Amazon MWAA**
Task state resource: `arn:aws:states:::aws-sdk:mwaa:`[apiAction]``
Exception prefix: `Mwaa`
**Amazon Macie V2**
Task state resource: `arn:aws:states:::aws-sdk:macie2:`[apiAction]``
Exception prefix: `Macie2`
**MailManager**
Task state resource: `arn:aws:states:::aws-sdk:mailmanager:`[apiAction]``
Exception prefix: `MailManager`
**AWS Mainframe Modernization**
Task state resource: `arn:aws:states:::aws-sdk:m2:`[apiAction]``
Exception prefix: `M2`
**AWS Mainframe Modernization Application Testing**
Task state resource: `arn:aws:states:::aws-sdk:apptest:`[apiAction]``
Exception prefix: `AppTest`
**Managed Blockchain**
Task state resource: `arn:aws:states:::aws-sdk:managedblockchain:`[apiAction]``
Exception prefix: `ManagedBlockchain`
**Managed Blockchain Query**
Task state resource: `arn:aws:states:::aws-sdk:managedblockchainquery:`[apiAction]``
Exception prefix: `ManagedBlockchainQuery`
**Amazon Managed Grafana**
Task state resource: `arn:aws:states:::aws-sdk:grafana:`[apiAction]``
Exception prefix: `Grafana`
**AWS Marketplace Catalog**
Task state resource: `arn:aws:states:::aws-sdk:marketplacecatalog:`[apiAction]``
Exception prefix: `MarketplaceCatalog`
**AWS Marketplace Commerce Analytics**
Task state resource: `arn:aws:states:::aws-sdk:marketplacecommerceanalytics:`[apiAction]``
Exception prefix: `MarketplaceCommerceAnalytics`
**AWS Marketplace Entitlement Service**
Task state resource: `arn:aws:states:::aws-sdk:marketplaceentitlement:`[apiAction]``
Exception prefix: `MarketplaceEntitlement`
**AWS Marketplace Metering**
Task state resource: `arn:aws:states:::aws-sdk:marketplacemetering:`[apiAction]``
Exception prefix: `MarketplaceMetering`
**AWS Marketplace Reporting Service**
Task state resource: `arn:aws:states:::aws-sdk:marketplacereporting:`[apiAction]``
Exception prefix: `MarketplaceReporting`
**Amazon Mechanical Turk**
Task state resource: `arn:aws:states:::aws-sdk:mturk:`[apiAction]``
Exception prefix: `MTurk`
**MediaConnect**
Task state resource: `arn:aws:states:::aws-sdk:mediaconnect:`[apiAction]``
Exception prefix: `MediaConnect`
**MediaConvert**
Task state resource: `arn:aws:states:::aws-sdk:mediaconvert:`[apiAction]``
Exception prefix: `MediaConvert`
**MediaLive**
Task state resource: `arn:aws:states:::aws-sdk:medialive:`[apiAction]``
Exception prefix: `MediaLive`
**MediaPackage V1**
Task state resource: `arn:aws:states:::aws-sdk:mediapackage:`[apiAction]``
Exception prefix: `MediaPackage`
**Unsupported operations:** `RotateChannelCredentials`
**MediaPackage V2**
Task state resource: `arn:aws:states:::aws-sdk:mediapackagev2:`[apiAction]``
Exception prefix: `MediaPackageV2`
**MediaPackage VOD**
Task state resource: `arn:aws:states:::aws-sdk:mediapackagevod:`[apiAction]``
Exception prefix: `MediaPackageVod`
**MediaStore**
Task state resource: `arn:aws:states:::aws-sdk:mediastore:`[apiAction]``
Exception prefix: `MediaStore`
**MediaTailor**
Task state resource: `arn:aws:states:::aws-sdk:mediatailor:`[apiAction]``
Exception prefix: `MediaTailor`
**Amazon MemoryDB**
Task state resource: `arn:aws:states:::aws-sdk:memorydb:`[apiAction]``
Exception prefix: `MemoryDb`
**Migration Hub**
Task state resource: `arn:aws:states:::aws-sdk:migrationhub:`[apiAction]``
Exception prefix: `MigrationHub`
**Migration Hub Home Region**
Task state resource: `arn:aws:states:::aws-sdk:migrationhubconfig:`[apiAction]``
Exception prefix: `MigrationHubConfig`
**Migration Hub Orchestrator**
Task state resource: `arn:aws:states:::aws-sdk:migrationhuborchestrator:`[apiAction]``
Exception prefix: `MigrationHubOrchestrator`
**Migration Hub Refactor Spaces**
Task state resource: `arn:aws:states:::aws-sdk:migrationhubrefactorspaces:`[apiAction]``
Exception prefix: `MigrationHubRefactorSpaces`
**Migration Hub Strategy Recommendations**
Task state resource: `arn:aws:states:::aws-sdk:migrationhubstrategy:`[apiAction]``
Exception prefix: `MigrationHubStrategy`
**Amazon Neptune**
Task state resource: `arn:aws:states:::aws-sdk:neptune:`[apiAction]``
Exception prefix: `Neptune`
**Amazon Neptune Graph**
Task state resource: `arn:aws:states:::aws-sdk:neptunegraph:`[apiAction]``
Exception prefix: `NeptuneGraph`
**Network Firewall**
Task state resource: `arn:aws:states:::aws-sdk:networkfirewall:`[apiAction]``
Exception prefix: `NetworkFirewall`
**Network Flow Monitor**
Task state resource: `arn:aws:states:::aws-sdk:networkflowmonitor:`[apiAction]``
Exception prefix: `NetworkFlowMonitor`
**Network Manager**
Task state resource: `arn:aws:states:::aws-sdk:networkmanager:`[apiAction]``
Exception prefix: `NetworkManager`
**Network Monitor**
Task state resource: `arn:aws:states:::aws-sdk:networkmonitor:`[apiAction]``
Exception prefix: `NetworkMonitor`
**Amazon Omics**
Task state resource: `arn:aws:states:::aws-sdk:omics:`[apiAction]``
Exception prefix: `Omics`
**Amazon OpenSearch**
Task state resource: `arn:aws:states:::aws-sdk:opensearch:`[apiAction]``
Exception prefix: `OpenSearch`
**Amazon OpenSearch Ingestion**
Task state resource: `arn:aws:states:::aws-sdk:osis:`[apiAction]``
Exception prefix: `Osis`
**OpenSearch Serverless**
Task state resource: `arn:aws:states:::aws-sdk:opensearchserverless:`[apiAction]``
Exception prefix: `OpenSearchServerless`
**OpsWorks**
Task state resource: `arn:aws:states:::aws-sdk:opsworks:`[apiAction]``
Exception prefix: `OpsWorks`
**OpsWorks CM**
Task state resource: `arn:aws:states:::aws-sdk:opsworkscm:`[apiAction]``
Exception prefix: `OpsWorksCm`
**AWS Organizations**
Task state resource: `arn:aws:states:::aws-sdk:organizations:`[apiAction]``
Exception prefix: `Organizations`
**AWS Outposts**
Task state resource: `arn:aws:states:::aws-sdk:outposts:`[apiAction]``
Exception prefix: `Outposts`
**AWS Panorama**
Task state resource: `arn:aws:states:::aws-sdk:panorama:`[apiAction]``
Exception prefix: `Panorama`
**AWS Parallel Computing Service**
Task state resource: `arn:aws:states:::aws-sdk:pcs:`[apiAction]``
Exception prefix: `Pcs`
**Partner Central Selling API**
Task state resource: `arn:aws:states:::aws-sdk:partnercentralselling:`[apiAction]``
Exception prefix: `PartnerCentralSelling`
**Payment Cryptography**
Task state resource: `arn:aws:states:::aws-sdk:paymentcryptography:`[apiAction]``
Exception prefix: `PaymentCryptography`
**Payment Cryptography Data**
Task state resource: `arn:aws:states:::aws-sdk:paymentcryptographydata:`[apiAction]``
Exception prefix: `PaymentCryptographyData`
**Amazon Personalize**
Task state resource: `arn:aws:states:::aws-sdk:personalize:`[apiAction]``
Exception prefix: `Personalize`
**Amazon Personalize Events**
Task state resource: `arn:aws:states:::aws-sdk:personalizeevents:`[apiAction]``
Exception prefix: `PersonalizeEvents`
**Amazon Personalize Runtime**
Task state resource: `arn:aws:states:::aws-sdk:personalizeruntime:`[apiAction]``
Exception prefix: `PersonalizeRuntime`
**Amazon Pinpoint**
Task state resource: `arn:aws:states:::aws-sdk:pinpoint:`[apiAction]``
Exception prefix: `Pinpoint`
**Amazon Pinpoint Email Service**
Task state resource: `arn:aws:states:::aws-sdk:pinpointemail:`[apiAction]``
Exception prefix: `PinpointEmail`
**Amazon Pinpoint SMS and Voice V1**
Task state resource: `arn:aws:states:::aws-sdk:pinpointsmsvoice:`[apiAction]``
Exception prefix: `PinpointSmsVoice`
**Amazon Pinpoint SMS and Voice V2**
Task state resource: `arn:aws:states:::aws-sdk:pinpointsmsvoicev2:`[apiAction]``
Exception prefix: `PinpointSmsVoiceV2`
**Amazon Polly**
Task state resource: `arn:aws:states:::aws-sdk:polly:`[apiAction]``
Exception prefix: `Polly`
**AWS Price List**
Task state resource: `arn:aws:states:::aws-sdk:pricing:`[apiAction]``
Exception prefix: `Pricing`
**AWS Private 5G**
Task state resource: `arn:aws:states:::aws-sdk:privatenetworks:`[apiAction]``
Exception prefix: `PrivateNetworks`
**Private CA Connector for Active Directory**
Task state resource: `arn:aws:states:::aws-sdk:pcaconnectorad:`[apiAction]``
Exception prefix: `PcaConnectorAd`
**Private CA Connector for SCEP**
Task state resource: `arn:aws:states:::aws-sdk:pcaconnectorscep:`[apiAction]``
Exception prefix: `PcaConnectorScep`
**Amazon Prometheus**
Task state resource: `arn:aws:states:::aws-sdk:amp:`[apiAction]``
Exception prefix: `Amp`
**AWS Proton**
Task state resource: `arn:aws:states:::aws-sdk:proton:`[apiAction]``
Exception prefix: `Proton`
**Amazon Q Apps**
Task state resource: `arn:aws:states:::aws-sdk:qapps:`[apiAction]``
Exception prefix: `QApps`
**Amazon Q Business**
Task state resource: `arn:aws:states:::aws-sdk:qbusiness:`[apiAction]``
Exception prefix: `QBusiness`
**Unsupported operations:** `Chat`
**Amazon Q Connect**
Task state resource: `arn:aws:states:::aws-sdk:qconnect:`[apiAction]``
Exception prefix: `QConnect`
**Amazon QLDB**
Task state resource: `arn:aws:states:::aws-sdk:qldb:`[apiAction]``
Exception prefix: `Qldb`
**Amazon QLDB Session**
Task state resource: `arn:aws:states:::aws-sdk:qldbsession:`[apiAction]``
Exception prefix: `QldbSession`
**Amazon QuickSight**
Task state resource: `arn:aws:states:::aws-sdk:quicksight:`[apiAction]``
Exception prefix: `QuickSight`
**Amazon RDS**
Task state resource: `arn:aws:states:::aws-sdk:rds:`[apiAction]``
Exception prefix: `Rds`
**Amazon RDS Data**
Task state resource: `arn:aws:states:::aws-sdk:rdsdata:`[apiAction]``
Exception prefix: `RdsData`
**Unsupported operations:** `ExecuteSql`
**Amazon RDS Performance Insights**
Task state resource: `arn:aws:states:::aws-sdk:pi:`[apiAction]``
Exception prefix: `Pi`
**Recycle Bin for EBS**
Task state resource: `arn:aws:states:::aws-sdk:rbin:`[apiAction]``
Exception prefix: `Rbin`
**Amazon Redshift**
Task state resource: `arn:aws:states:::aws-sdk:redshift:`[apiAction]``
Exception prefix: `Redshift`
**Amazon Redshift Data**
Task state resource: `arn:aws:states:::aws-sdk:redshiftdata:`[apiAction]``
Exception prefix: `RedshiftData`
**Amazon Redshift Serverless**
Task state resource: `arn:aws:states:::aws-sdk:redshiftserverless:`[apiAction]``
Exception prefix: `RedshiftServerless`
**Amazon Rekognition**
Task state resource: `arn:aws:states:::aws-sdk:rekognition:`[apiAction]``
Exception prefix: `Rekognition`
**Resilience Hub**
Task state resource: `arn:aws:states:::aws-sdk:resiliencehub:`[apiAction]``
Exception prefix: `Resiliencehub`
**AWS Resource Access Manager**
Task state resource: `arn:aws:states:::aws-sdk:ram:`[apiAction]``
Exception prefix: `Ram`
**AWS Resource Explorer**
Task state resource: `arn:aws:states:::aws-sdk:resourceexplorer2:`[apiAction]``
Exception prefix: `ResourceExplorer2`
**Resource Groups**
Task state resource: `arn:aws:states:::aws-sdk:resourcegroups:`[apiAction]``
Exception prefix: `ResourceGroups`
**Resource Groups Tagging**
Task state resource: `arn:aws:states:::aws-sdk:resourcegroupstaggingapi:`[apiAction]``
Exception prefix: `ResourceGroupsTaggingApi`
**AWS RoboMaker**
Task state resource: `arn:aws:states:::aws-sdk:robomaker:`[apiAction]``
Exception prefix: `RoboMaker`
**Route 53**
Task state resource: `arn:aws:states:::aws-sdk:route53:`[apiAction]``
Exception prefix: `Route53`
**Route 53 ARC Zonal Shift**
Task state resource: `arn:aws:states:::aws-sdk:arczonalshift:`[apiAction]``
Exception prefix: `ArcZonalShift`
**Route 53 Domains**
Task state resource: `arn:aws:states:::aws-sdk:route53domains:`[apiAction]``
Exception prefix: `Route53Domains`
**Route 53 Profiles**
Task state resource: `arn:aws:states:::aws-sdk:route53profiles:`[apiAction]``
Exception prefix: `Route53Profiles`
**Route 53 Recovery Control Config**
Task state resource: `arn:aws:states:::aws-sdk:route53recoverycontrolconfig:`[apiAction]``
Exception prefix: `Route53RecoveryControlConfig`
**Route 53 Recovery Readiness**
Task state resource: `arn:aws:states:::aws-sdk:route53recoveryreadiness:`[apiAction]``
Exception prefix: `Route53RecoveryReadiness`
**Route 53 Resolver**
Task state resource: `arn:aws:states:::aws-sdk:route53resolver:`[apiAction]``
Exception prefix: `Route53Resolver`
**Route 53 Routing Control**
Task state resource: `arn:aws:states:::aws-sdk:route53recoverycluster:`[apiAction]``
Exception prefix: `Route53RecoveryCluster`
**Runtime for Amazon Bedrock Data Automation**
Task state resource: `arn:aws:states:::aws-sdk:bedrockdataautomationruntime:`[apiAction]``
Exception prefix: `BedrockDataAutomationRuntime`
**Amazon S3**
Task state resource: `arn:aws:states:::aws-sdk:s3:`[apiAction]``
Exception prefix: `S3`
**Unsupported operations:** `SelectObjectContent`
**Amazon S3 Control**
Task state resource: `arn:aws:states:::aws-sdk:s3control:`[apiAction]``
Exception prefix: `S3Control`
**Unsupported operations:** `SelectObjectContent`
**Amazon S3 Glacier**
Task state resource: `arn:aws:states:::aws-sdk:glacier:`[apiAction]``
Exception prefix: `Glacier`
**Amazon S3 Tables**
Task state resource: `arn:aws:states:::aws-sdk:s3tables:`[apiAction]``
Exception prefix: `S3Tables`
**Amazon S3 on Outposts**
Task state resource: `arn:aws:states:::aws-sdk:s3outposts:`[apiAction]``
Exception prefix: `S3Outposts`
**Amazon SES V1**
Task state resource: `arn:aws:states:::aws-sdk:ses:`[apiAction]``
Exception prefix: `Ses`
**Amazon SES V2**
Task state resource: `arn:aws:states:::aws-sdk:sesv2:`[apiAction]``
Exception prefix: `SesV2`
**Amazon SNS**
Task state resource: `arn:aws:states:::aws-sdk:sns:`[apiAction]``
Exception prefix: `Sns`
**Amazon SQS**
Task state resource: `arn:aws:states:::aws-sdk:sqs:`[apiAction]``
Exception prefix: `Sqs`
**AWS SSO**
Task state resource: `arn:aws:states:::aws-sdk:sso:`[apiAction]``
Exception prefix: `Sso`
**AWS SSO**
Task state resource: `arn:aws:states:::aws-sdk:identitystore:`[apiAction]``
Exception prefix: `Identitystore`
**AWS SSO Admin**
Task state resource: `arn:aws:states:::aws-sdk:ssoadmin:`[apiAction]``
Exception prefix: `SsoAdmin`
**AWS SSO OIDC**
Task state resource: `arn:aws:states:::aws-sdk:ssooidc:`[apiAction]``
Exception prefix: `SsoOidc`
**Amazon SWF**
Task state resource: `arn:aws:states:::aws-sdk:swf:`[apiAction]``
Exception prefix: `Swf`
**SageMaker**
Task state resource: `arn:aws:states:::aws-sdk:sagemaker:`[apiAction]``
Exception prefix: `SageMaker`
**SageMaker Edge Manager**
Task state resource: `arn:aws:states:::aws-sdk:sagemakeredge:`[apiAction]``
Exception prefix: `SagemakerEdge`
**SageMaker Feature Store**
Task state resource: `arn:aws:states:::aws-sdk:sagemakerfeaturestoreruntime:`[apiAction]``
Exception prefix: `SageMakerFeatureStoreRuntime`
**SageMaker Geospatial**
Task state resource: `arn:aws:states:::aws-sdk:sagemakergeospatial:`[apiAction]``
Exception prefix: `SageMakerGeospatial`
**SageMaker Metrics**
Task state resource: `arn:aws:states:::aws-sdk:sagemakermetrics:`[apiAction]``
Exception prefix: `SageMakerMetrics`
**SageMaker Runtime**
Task state resource: `arn:aws:states:::aws-sdk:sagemakerruntime:`[apiAction]``
Exception prefix: `SageMakerRuntime`
**Unsupported operations:** `InvokeEndpointWithResponseStream`
**AWS Savings Plans**
Task state resource: `arn:aws:states:::aws-sdk:savingsplans:`[apiAction]``
Exception prefix: `Savingsplans`
**AWS Secrets Manager**
Task state resource: `arn:aws:states:::aws-sdk:secretsmanager:`[apiAction]``
Exception prefix: `SecretsManager`
**AWS Security Hub**
Task state resource: `arn:aws:states:::aws-sdk:securityhub:`[apiAction]``
Exception prefix: `SecurityHub`
**Security Incident Response**
Task state resource: `arn:aws:states:::aws-sdk:securityir:`[apiAction]``
Exception prefix: `SecurityIr`
**Amazon Security Lake**
Task state resource: `arn:aws:states:::aws-sdk:securitylake:`[apiAction]``
Exception prefix: `SecurityLake`
**Unsupported operations:** `GetDatalake`, `GetDatalakeAutoEnable`, `GetDatalakeExceptionsExpiry`, `GetDatalakeExceptionsSubscription`, `GetDatalakeStatus`, `CreateSubscriptionNotificationConfiguration`, `CreateDatalake`, `CreateDatalakeAutoEnable`, `CreateDatalakeDelegatedAdmin`, `CreateDatalakeExceptionsSubscription`, `DeleteDatalake`, `UpdateDatalake`, `UpdateSubscriptionNotificationConfiguration`, `UpdateDatalakeExceptionsExpiry`, `UpdateDatalakeExceptionsSubscription`, `DeleteDatalakeAutoEnable`, `DeleteDatalakeDelegatedAdmin`, `DeleteDatalakeExceptionsSubscription`, `DeleteSubscriptionNotificationConfiguration`, `ListDatalakeExceptions`
**AWS Security Token Service**
Task state resource: `arn:aws:states:::aws-sdk:sts:`[apiAction]``
Exception prefix: `Sts`
**Unsupported operations:** `AssumeRole`, `AssumeRoleWithSAML`, `AssumeRoleWithWebIdentity`
**AWS Server Migration Service**
Task state resource: `arn:aws:states:::aws-sdk:sms:`[apiAction]``
Exception prefix: `Sms`
**AWS Serverless Application Repository**
Task state resource: `arn:aws:states:::aws-sdk:serverlessapplicationrepository:`[apiAction]``
Exception prefix: `ServerlessApplicationRepository`
**AWS Service Catalog**
Task state resource: `arn:aws:states:::aws-sdk:servicecatalog:`[apiAction]``
Exception prefix: `ServiceCatalog`
**AWS Service Catalog App Registry**
Task state resource: `arn:aws:states:::aws-sdk:servicecatalogappregistry:`[apiAction]``
Exception prefix: `ServiceCatalogAppRegistry`
**Service Quotas**
Task state resource: `arn:aws:states:::aws-sdk:servicequotas:`[apiAction]``
Exception prefix: `ServiceQuotas`
**AWS Shield**
Task state resource: `arn:aws:states:::aws-sdk:shield:`[apiAction]``
Exception prefix: `Shield`
**Unsupported operations:** `DeleteSubscription`
**AWS Signer**
Task state resource: `arn:aws:states:::aws-sdk:signer:`[apiAction]``
Exception prefix: `Signer`
**AWS SimSpace Weaver**
Task state resource: `arn:aws:states:::aws-sdk:simspaceweaver:`[apiAction]``
Exception prefix: `SimSpaceWeaver`
**AWS Snow Device Management**
Task state resource: `arn:aws:states:::aws-sdk:snowdevicemanagement:`[apiAction]``
Exception prefix: `SnowDeviceManagement`
**AWS Snowball**
Task state resource: `arn:aws:states:::aws-sdk:snowball:`[apiAction]``
Exception prefix: `Snowball`
**AWS Step Functions**
Task state resource: `arn:aws:states:::aws-sdk:sfn:`[apiAction]``
Exception prefix: `Sfn`
**AWS Storage Gateway**
Task state resource: `arn:aws:states:::aws-sdk:storagegateway:`[apiAction]``
Exception prefix: `StorageGateway`
**AWS Supply Chain**
Task state resource: `arn:aws:states:::aws-sdk:supplychain:`[apiAction]``
Exception prefix: `SupplyChain`
**AWS Support**
Task state resource: `arn:aws:states:::aws-sdk:support:`[apiAction]``
Exception prefix: `Support`
**AWS Support App**
Task state resource: `arn:aws:states:::aws-sdk:supportapp:`[apiAction]``
Exception prefix: `SupportApp`
**Systems Manager**
Task state resource: `arn:aws:states:::aws-sdk:ssm:`[apiAction]``
Exception prefix: `Ssm`
**AWS Systems Manager QuickSetup**
Task state resource: `arn:aws:states:::aws-sdk:ssmquicksetup:`[apiAction]``
Exception prefix: `SsmQuickSetup`
**Systems Manager for SAP**
Task state resource: `arn:aws:states:::aws-sdk:ssmsap:`[apiAction]``
Exception prefix: `SsmSap`
**Tax Settings**
Task state resource: `arn:aws:states:::aws-sdk:taxsettings:`[apiAction]``
Exception prefix: `TaxSettings`
**AWS Telco Network Builder**
Task state resource: `arn:aws:states:::aws-sdk:tnb:`[apiAction]``
Exception prefix: `Tnb`
**Amazon Textract**
Task state resource: `arn:aws:states:::aws-sdk:textract:`[apiAction]``
Exception prefix: `Textract`
**Timestream InfluxDB**
Task state resource: `arn:aws:states:::aws-sdk:timestreaminfluxdb:`[apiAction]``
Exception prefix: `TimestreamInfluxDb`
**Amazon Timestream Query**
Task state resource: `arn:aws:states:::aws-sdk:timestreamquery:`[apiAction]``
Exception prefix: `TimestreamQuery`
**Amazon Timestream Write**
Task state resource: `arn:aws:states:::aws-sdk:timestreamwrite:`[apiAction]``
Exception prefix: `TimestreamWrite`
**Amazon Transcribe**
Task state resource: `arn:aws:states:::aws-sdk:transcribe:`[apiAction]``
Exception prefix: `Transcribe`
**AWS Transfer Family**
Task state resource: `arn:aws:states:::aws-sdk:transfer:`[apiAction]``
Exception prefix: `Transfer`
**Amazon Translate**
Task state resource: `arn:aws:states:::aws-sdk:translate:`[apiAction]``
Exception prefix: `Translate`
**Trusted Advisor**
Task state resource: `arn:aws:states:::aws-sdk:trustedadvisor:`[apiAction]``
Exception prefix: `TrustedAdvisor`
**AWS User Notifications Contacts**
Task state resource: `arn:aws:states:::aws-sdk:notificationscontacts:`[apiAction]``
Exception prefix: `NotificationsContacts`
**Amazon VPC Lattice**
Task state resource: `arn:aws:states:::aws-sdk:vpclattice:`[apiAction]``
Exception prefix: `VpcLattice`
**Verified Permissions**
Task state resource: `arn:aws:states:::aws-sdk:verifiedpermissions:`[apiAction]``
Exception prefix: `VerifiedPermissions`
**AWS WAF V1**
Task state resource: `arn:aws:states:::aws-sdk:waf:`[apiAction]``
Exception prefix: `Waf`
**AWS WAF V1 Regional**
Task state resource: `arn:aws:states:::aws-sdk:wafregional:`[apiAction]``
Exception prefix: `WafRegional`
**AWS WAF V2**
Task state resource: `arn:aws:states:::aws-sdk:wafv2:`[apiAction]``
Exception prefix: `Wafv2`
**AWS Well-Architected Tool**
Task state resource: `arn:aws:states:::aws-sdk:wellarchitected:`[apiAction]``
Exception prefix: `WellArchitected`
**Amazon WorkDocs**
Task state resource: `arn:aws:states:::aws-sdk:workdocs:`[apiAction]``
Exception prefix: `WorkDocs`
**Amazon WorkMail**
Task state resource: `arn:aws:states:::aws-sdk:workmail:`[apiAction]``
Exception prefix: `WorkMail`
**Amazon WorkMail Message Flow**
Task state resource: `arn:aws:states:::aws-sdk:workmailmessageflow:`[apiAction]``
Exception prefix: `WorkMailMessageFlow`
**Amazon WorkSpaces**
Task state resource: `arn:aws:states:::aws-sdk:workspaces:`[apiAction]``
Exception prefix: `WorkSpaces`
**Amazon WorkSpaces Thin Client**
Task state resource: `arn:aws:states:::aws-sdk:workspacesthinclient:`[apiAction]``
Exception prefix: `WorkSpacesThinClient`
**Amazon WorkSpaces Web**
Task state resource: `arn:aws:states:::aws-sdk:workspacesweb:`[apiAction]``
Exception prefix: `WorkSpacesWeb`
**AWS X-Ray**
Task state resource: `arn:aws:states:::aws-sdk:xray:`[apiAction]``
Exception prefix: `XRay`
**re:Post Private**
Task state resource: `arn:aws:states:::aws-sdk:repostspace:`[apiAction]``
Exception prefix: `Repostspace`
## Deprecated AWS SDK service integrations
The following AWS SDK service integrations are now deprecated:
* AWS Mobile
* Amazon Macie
* AWS IoT RoboRunner
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrating services
Service integration patterns
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.