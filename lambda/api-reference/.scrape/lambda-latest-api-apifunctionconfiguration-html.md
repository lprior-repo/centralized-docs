---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionConfiguration.html
title: API FunctionConfiguration.html
word_count: 1342
filtered: true
elements_removed: 0
density_score: 0.85
---

FunctionConfiguration - AWS Lambda
FunctionConfiguration - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionConfiguration)
[Contents](#API_FunctionConfiguration_Contents)[See Also](#API_FunctionConfiguration_SeeAlso)
## Contents
**
Architectures
**
The instruction set architecture that the function supports. Architecture is a string array with one of the
valid values. The default architecture value is `x86\_64`.
Type: Array of strings
Array Members: Fixed number of 1 item.
Valid Values: `x86\_64 | arm64`
Required: No
**
CapacityProviderConfig
**
Configuration for the capacity provider that manages compute resources for Lambda functions.
Type: [CapacityProviderConfig](./API_CapacityProviderConfig.html) object
Required: No
**
CodeSha256
**
The SHA256 hash of the function's deployment package.
Type: String
Required: No
**
CodeSize
**
The size of the function's deployment package, in bytes.
Type: Long
Required: No
**
ConfigSha256
**
The SHA256 hash of the function configuration.
Type: String
Required: No
**
DeadLetterConfig
**
The function's dead letter queue.
Type: [DeadLetterConfig](./API_DeadLetterConfig.html) object
Required: No
**
Description
**
The function's description.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
**
DurableConfig
**
The function's durable execution configuration settings, if the function is configured for durability.
Type: [DurableConfig](./API_DurableConfig.html) object
Required: No
**
Environment
**
The function's [environment variables](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html). Omitted from AWS CloudTrail logs.
Type: [EnvironmentResponse](./API_EnvironmentResponse.html) object
Required: No
**
EphemeralStorage
**
The size of the function's `/tmp` directory in MB. The default value is 512, but can be any whole
number between 512 and 10,240 MB. For more information, see [Configuring ephemeral storage (console)](https://docs.aws.amazon.com/lambda/latest/dg/configuration-function-common.html#configuration-ephemeral-storage).
Type: [EphemeralStorage](./API_EphemeralStorage.html) object
Required: No
**
FileSystemConfigs
**
Connection settings for an [Amazon EFS file system](https://docs.aws.amazon.com/lambda/latest/dg/configuration-filesystem.html).
Type: Array of [FileSystemConfig](./API_FileSystemConfig.html) objects
Array Members: Minimum number of 0 items. Maximum number of 1 item.
Required: No
**
FunctionArn
**
The function's Amazon Resource Name (ARN).
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_\\.]+(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: No
**
FunctionName
**
The name of the function.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `(arn:(aws[a-zA-Z-]\*)?:lambda:)?((eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:)?(\\d{12}:)?(function:)?([a-zA-Z0-9-\_\\.]+)(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: No
**
Handler
**
The function that Lambda calls to begin running your function.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 128.
Pattern: `[^\\s]+`
Required: No
**
ImageConfigResponse
**
The function's image configuration values.
Type: [ImageConfigResponse](./API_ImageConfigResponse.html) object
Required: No
**
KMSKeyArn
**
The ARN of the AWS Key Management Service (AWS KMS) customer managed key that's used to encrypt the following resources:
* The function's [environment variables](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html#configuration-envvars-encryption).
* The function's [Lambda SnapStart](https://docs.aws.amazon.com/lambda/latest/dg/snapstart-security.html) snapshots.
* When used with `SourceKMSKeyArn`, the unzipped version of the .zip deployment package that's used for function invocations. For more information, see [
Specifying a customer managed key for Lambda](https://docs.aws.amazon.com/lambda/latest/dg/encrypt-zip-package.html#enable-zip-custom-encryption).
* The optimized version of the container image that's used for function invocations. Note that this is not the same key that's used to protect your container image in the Amazon Elastic Container Registry (Amazon ECR). For more information, see [Function lifecycle](https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#images-lifecycle).
If you don't provide a customer managed key, Lambda uses an [AWS owned key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk) or an [AWS managed key](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk).
Type: String
Pattern: `(arn:(aws[a-zA-Z-]\*)?:[a-z0-9-.]+:.\*)|()`
Required: No
**
LastModified
**
The date and time that the function was last updated, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: String
Required: No
**
LastUpdateStatus
**
The status of the last update that was performed on the function. This is first set to `Successful`
after function creation completes.
Type: String
Valid Values: `Successful | Failed | InProgress`
Required: No
**
LastUpdateStatusReason
**
The reason for the last update that was performed on the function.
Type: String
Required: No
**
LastUpdateStatusReasonCode
**
The reason code for the last update that was performed on the function.
Type: String
Valid Values: `EniLimitExceeded | InsufficientRolePermissions | InvalidConfiguration | InternalError | SubnetOutOfIPAddresses | InvalidSubnet | InvalidSecurityGroup | ImageDeleted | ImageAccessDenied | InvalidImage | KMSKeyAccessDenied | KMSKeyNotFound | InvalidStateKMSKey | DisabledKMSKey | EFSIOError | EFSMountConnectivityError | EFSMountFailure | EFSMountTimeout | InvalidRuntime | InvalidZipFileException | FunctionError | VcpuLimitExceeded | CapacityProviderScalingLimitExceeded | InsufficientCapacity | EC2RequestLimitExceeded | FunctionError.InitTimeout | FunctionError.RuntimeInitError | FunctionError.ExtensionInitError | FunctionError.InvalidEntryPoint | FunctionError.InvalidWorkingDirectory | FunctionError.PermissionDenied | FunctionError.TooManyExtensions | FunctionError.InitResourceExhausted`
Required: No
**
Layers
**
The function's [layers](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
Type: Array of [Layer](./API_Layer.html) objects
Required: No
**
LoggingConfig
**
The function's Amazon CloudWatch Logs configuration settings.
Type: [LoggingConfig](./API_LoggingConfig.html) object
Required: No
**
MasterArn
**
For Lambda@Edge functions, the ARN of the main function.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
Required: No
**
MemorySize
**
The amount of memory available to the function at runtime.
Type: Integer
Valid Range: Minimum value of 128. Maximum value of 32768.
Required: No
**
PackageType
**
The type of deployment package. Set to `Image` for container image and set `Zip` for .zip file archive.
Type: String
Valid Values: `Zip | Image`
Required: No
**
RevisionId
**
The latest updated revision of the function or alias.
Type: String
Required: No
**
Role
**
The function's execution role.
Type: String
Pattern: `arn:(aws[a-zA-Z-]\*)?:iam::\\d{12}:role/?[a-zA-Z\_0-9+=,.@\\-\_/]+`
Required: No
**
Runtime
**
The identifier of the function's [
runtime](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html). Runtime is required if the deployment package is a .zip file archive. Specifying a runtime results in
an error if you're deploying a function using a container image.
The following list includes deprecated runtimes. Lambda blocks creating new functions and updating existing
functions shortly after each runtime is deprecated. For more information, see
[Runtime use after deprecation](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels).
For a list of all currently supported runtimes, see
[Supported runtimes](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported).
Type: String
Valid Values: `nodejs | nodejs4.3 | nodejs6.10 | nodejs8.9 | nodejs8.10 | nodejs8.x | nodejs10.x | nodejs12.x | nodejs14.x | nodejs16.x | nodejs18.x | nodejs20.x | nodejs22.x | nodejs24.x | java8 | java8.al2 | java11 | java17 | java21 | java25 | python2.7 | python3.4 | python3.6 | python3.7 | python3.8 | python3.9 | python3.10 | python3.11 | python3.12 | python3.13 | python3.14 | dotnetcore1.0 | dotnetcore2.0 | dotnetcore2.1 | dotnetcore3.1 | dotnet6 | dotnet8 | dotnet10 | nodejs4.3-edge | python2.7-greengrass | byol | go1.9 | go1.x | ruby2.5 | ruby2.6 | ruby2.7 | ruby3.2 | ruby3.3 | ruby3.4 | provided | provided.al2 | provided.al2023 | nasa | nodejs26.x | ruby3.5 | python3.15`
Required: No
**
RuntimeVersionConfig
**
The ARN of the runtime and any errors that occured.
Type: [RuntimeVersionConfig](./API_RuntimeVersionConfig.html) object
Required: No
**
SigningJobArn
**
The ARN of the signing job.
Type: String
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No
**
SigningProfileVersionArn
**
The ARN of the signing profile version.
Type: String
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No
**
SnapStart
**
Set `ApplyOn` to `PublishedVersions` to create a snapshot of the initialized execution
environment when you publish a function version. For more information, see [Improving startup performance with Lambda SnapStart](https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html).
Type: [SnapStartResponse](./API_SnapStartResponse.html) object
Required: No
**
State
**
The current state of the function. When the state is `Inactive`, you can reactivate the function by
invoking it.
Type: String
Valid Values: `Pending | Active | Inactive | Failed | Deactivating | Deactivated | ActiveNonInvocable | Deleting`
Required: No
**
StateReason
**
The reason for the function's current state.
Type: String
Required: No
**
StateReasonCode
**
The reason code for the function's current state. When the code is `Creating`, you can't invoke or
modify the function.
Type: String
Valid Values: `Idle | Creating | Restoring | EniLimitExceeded | InsufficientRolePermissions | InvalidConfiguration | InternalError | SubnetOutOfIPAddresses | InvalidSubnet | InvalidSecurityGroup | ImageDeleted | ImageAccessDenied | InvalidImage | KMSKeyAccessDenied | KMSKeyNotFound | InvalidStateKMSKey | DisabledKMSKey | EFSIOError | EFSMountConnectivityError | EFSMountFailure | EFSMountTimeout | InvalidRuntime | InvalidZipFileException | FunctionError | DrainingDurableExecutions | VcpuLimitExceeded | CapacityProviderScalingLimitExceeded | InsufficientCapacity | EC2RequestLimitExceeded | FunctionError.InitTimeout | FunctionError.RuntimeInitError | FunctionError.ExtensionInitError | FunctionError.InvalidEntryPoint | FunctionError.InvalidWorkingDirectory | FunctionError.PermissionDenied | FunctionError.TooManyExtensions | FunctionError.InitResourceExhausted`
Required: No
**
TenancyConfig
**
The function's tenant isolation configuration settings. Determines whether the Lambda function
runs on a shared or dedicated infrastructure per unique tenant.
Type: [TenancyConfig](./API_TenancyConfig.html) object
Required: No
**
Timeout
**
The amount of time in seconds that Lambda allows a function to run before stopping it.
Type: Integer
Valid Range: Minimum value of 1.
Required: No
**
TracingConfig
**
The function's AWS X-Ray tracing configuration.
Type: [TracingConfigResponse](./API_TracingConfigResponse.html) object
Required: No
**
Version
**
The version of the Lambda function.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Pattern: `(\\$LATEST|[0-9]+)`
Required: No
**
VpcConfig
**
The function's networking configuration.
Type: [VpcConfigResponse](./API_VpcConfigResponse.html) object
Required: No