---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sfn-local.html
title: sfn local.html
word_count: 1437
filtered: true
elements_removed: 0
density_score: 0.86
---

Testing state machines with Step Functions Local (unsupported) - AWS Step Functions
Testing state machines with Step Functions Local (unsupported) - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sfn-local)
[Setting Up Step Functions Local and Docker](#sfn-local-docker)[Setting Up Step Functions Local - Java Version](#sfn-local-jar)[Configuring Step Functions Local Options](#sfn-local-config-options)[Running Step Functions Local](#sfn-local-computer)
###### Step Functions Local is unsupported
Step Functions Local does **not** provide feature parity and is **unsupported**.
You might consider third party solutions that emulate Step Functions for testing
purposes.
As an alternative to Step Functions Local, you can use the TestState API to unit test your state machine logic before deploying to your AWS account. For more information, see [Testing state machines with TestState API](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html).
With AWS Step Functions Local, a downloadable version of Step Functions, you can test applications with Step Functions
running in your own development environment.
When running Step Functions Local, you can use one of the following ways to invoke service integrations:
* Configuring local endpoints for AWS Lambda and other services.
* Making calls directly to an AWS service from Step Functions Local.
* Mocking the response from service integrations.
AWS Step Functions Local is available as a JAR package or a self-contained Docker image that runs on Microsoft Windows, Linux, macOS, and other platforms that support Java or Docker.
###### Warning
You should only use Step Functions Local for testing and never to process sensitive
information.
###### Topics
* [Setting Up Step Functions Local and Docker](#sfn-local-docker)
* [Setting Up Step Functions Local - Java Version](#sfn-local-jar)
* [Configuring Step Functions Local Options](#sfn-local-config-options)
* [Running Step Functions Local](#sfn-local-computer)
* [Tutorial: Testing using Step Functions and AWS SAM CLI Local](./sfn-local-lambda.html)
* [Testing with mocked service integrations](./sfn-local-test-sm-exec.html)
## Setting Up Step Functions Local (Downloadable Version) in
Docker
The Step Functions Local Docker image enables you to get started with Step Functions Local quickly by
using a Docker image with all the needed dependencies. The Docker image enables you to
include Step Functions Local in your containerized builds and as part of your continuous
integration testing.
To get the Docker image for Step Functions Local, see [https://hub.docker.com/r/amazon/aws-stepfunctions-local](https://hub.docker.com/r/amazon/aws-stepfunctions-local), or enter the following
Docker `pull` command.
```
`docker pull amazon/aws-stepfunctions-local`
```
To start the downloadable version of Step Functions on Docker, run the following Docker
`run` command
```
`docker run -p 8083:8083 amazon/aws-stepfunctions-local`
```
To interact with AWS Lambda or other supported services, you need to configure your
credentials and other configuration options first. For more information, see the
following topics:
* [Setting Configuration Options for Step Functions Local](#sfn-local-config-options)
* [Credentials and configuration for Docker](#docker-credentials)
## Setting Up Step Functions Local (Downloadable Version) - Java Version
The downloadable version of AWS Step Functions is provided as an executable JAR file and as a
Docker image. The Java application runs on Windows, Linux, macOS, and other platforms
that support Java. In addition to Java, you need to install the AWS Command Line Interface (AWS CLI). For
information about installing and configuring the AWS CLI, see the [AWS Command Line Interface User Guide](https://docs.aws.amazon.com/cli/latest/userguide/).
###### To set up and run Step Functions on your computer
1. Download Step Functions using the following links.
|Download Links|Checksum|
|[.tar.gz](https://s3.amazonaws.com/stepfunctionslocal/StepFunctionsLocal.tar.gz)|[.tar.gz.md5](https://s3.amazonaws.com/stepfunctionslocal/StepFunctionsLocal.tar.gz.md5)|
|[.zip](https://s3.amazonaws.com/stepfunctionslocal/StepFunctionsLocal.zip)|[.zip.md5](https://s3.amazonaws.com/stepfunctionslocal/StepFunctionsLocal.zip.md5)|
2. Extract the `.zip` file.
3. Test the download and view version information.
```
`$ java -jar StepFunctionsLocal.jar -v
Step Function Local
Version: 2.0.0
Build: 2024-05-18`
```
4. (Optional) View a listing of available commands.
```
`$ java -jar StepFunctionsLocal.jar -h`
```
5. To start Step Functions on your computer, open a command prompt, navigate to the directory
where you extracted `StepFunctionsLocal.jar`, and enter the following
command.
```
`java -jar StepFunctionsLocal.jar`
```
6. To access Step Functions running locally, use the `--endpoint-url` parameter. For
example, using the AWS CLI, you would specify Step Functions commands as follows:
```
`aws stepfunctions --endpoint-url http://localhost:8083 `command``
```
###### Note
By default, Step Functions Local uses a local test account and credentials, and the AWS
Region is set to US East (N. Virginia). To use Step Functions Local with AWS Lambda, or other
supported services, you must configure your credentials and Region.
If you use Express workflows with Step Functions Local, the execution history will be stored in
a log file. It is not logged to CloudWatch Logs. The log file path will be based on the CloudWatch Logs
log group ARN provided when you create the local state machine. The log file will be
stored in
`/aws/states/log-group-name/`${execution\_arn}`.log`
relative to the location where you are running Step Functions Local. For example, if the
execution ARN is:
```
arn:aws:states:`region`:`account-id`:express:test:example-ExpressLogGroup-wJalrXUtnFEMI
```
the log file will be:
```
aws/states/log-group-name/arn:aws:states:`region`:`account-id`:express:test:example-ExpressLogGroup-wJalrXUtnFEMI.log
```
## Setting Configuration Options for Step Functions Local
When you start AWS Step Functions Local by using the JAR file, you can set configuration options
by using the AWS Command Line Interface (AWS CLI), or by including them in the system environment. For
Docker, you must specify these options in a file that you reference when starting Step Functions
Local.
### Configuration Options
When you configure the Step Functions Local container to use an override endpoint such as Lambda Endpoint and Batch Endpoint, and make calls to that endpoint, Step Functions Local doesn't use the [credentials](#docker-credentials) you specify. Setting these endpoint overrides is optional.
|Option|Command Line|Environment|
|Account|-account, --aws-account|AWS\_ACCOUNT\_ID|
|Region|-region, --aws-region|AWS\_DEFAULT\_REGION|
|Wait Time Scale|-waitTimeScale, --wait-time-scale|WAIT\_TIME\_SCALE|
|Lambda Endpoint|-lambdaEndpoint, --lambda-endpoint|LAMBDA\_ENDPOINT|
|Batch Endpoint|-batchEndpoint, --batch-endpoint|BATCH\_ENDPOINT|
|DynamoDB Endpoint|-dynamoDBEndpoint, --dynamodb-endpoint|DYNAMODB\_ENDPOINT|
|ECS Endpoint |-ecsEndpoint, --ecs-endpoint|ECS\_ENDPOINT|
|Glue Endpoint|-glueEndpoint, --glue-endpoint|GLUE\_ENDPOINT|
|SageMaker Endpoint|-sageMakerEndpoint, --sagemaker-endpoint|SAGE\_MAKER\_ENDPOINT|
|SQS Endpoint|-sqsEndpoint, --sqs-endpoint|SQS\_ENDPOINT|
|SNS Endpoint|-snsEndpoint, --sns-endpoint|SNS\_ENDPOINT|
|Step Functions Endpoint|-stepFunctionsEndpoint, --step-functions-endpoint|STEP\_FUNCTIONS\_ENDPOINT|
### Credentials and configuration for Docker
To configure Step Functions Local for Docker, create the following file:
`aws-stepfunctions-local-credentials.txt`.
This file contains your credentials and other configuration options. The following can
be used as a template when creating the
`aws-stepfunctions-local-credentials.txt` file.
```
`AWS\_DEFAULT\_REGION`=AWS\_REGION\_OF\_YOUR\_AWS\_RESOURCES`
AWS\_ACCESS\_KEY\_ID=`YOUR\_AWS\_ACCESS\_KEY`
AWS\_SECRET\_ACCESS\_KEY=`YOUR\_AWS\_SECRET\_KEY`
WAIT\_TIME\_SCALE=`VALUE`
LAMBDA\_ENDPOINT=`VALUE`
BATCH\_ENDPOINT=`VALUE`
DYNAMODB\_ENDPOINT=`VALUE`
ECS\_ENDPOINT=`VALUE`
GLUE\_ENDPOINT=`VALUE`
SAGE\_MAKER\_ENDPOINT=`VALUE`
SQS\_ENDPOINT=`VALUE`
SNS\_ENDPOINT=`VALUE`
STEP\_FUNCTIONS\_ENDPOINT=`VALUE``
```
Once you have configured your credentials and configuration options in
`aws-stepfunctions-local-credentials.txt`, start Step Functions with the following
command.
```
`docker run -p 8083:8083 --env-file aws-stepfunctions-local-credentials.txt amazon/aws-stepfunctions-local`
```
###### Note
It is recommended to use the special DNS name `host.docker.internal`, which resolves to the internal IP address that the host uses, such as `http://host.docker.internal:8000`. For more information, see Docker documentation for Mac and Windows at [Networking features in Docker Desktop for Mac](https://docs.docker.com/desktop/mac/networking/#use-cases-and-workaround) and [Networking features in Docker Desktop for Windows](https://docs.docker.com/desktop/windows/networking/) respectively.
## Running Step Functions Local on Your Computer
Use the local version of Step Functions to configure, develop and test state machines on your
computer.
### Run a HelloWorld state machine locally
After you run Step Functions locally with the AWS Command Line Interface (AWS CLI), you can start a state machine
execution.
1. Create a state machine from the AWS CLI by escaping out the state machine
definition.
```
`aws stepfunctions --endpoint-url http://localhost:8083 create-state-machine --definition "{\\
\\"Comment\\": \\"A Hello World example of the Amazon States Language using a Pass state\\",\\
\\"StartAt\\": \\"HelloWorld\\",\\
\\"States\\": {\\
\\"HelloWorld\\": {\\
\\"Type\\": \\"Pass\\",\\
\\"End\\": true\\
}\\
}}" --name "HelloWorld" --role-arn "arn:aws:iam::012345678901:role/DummyRole"`
```
###### Note
The `role-arn` is not used for Step Functions Local, but you must include it
with the proper syntax. You can use the Amazon Resource Name (ARN) from
the previous example.
If you successfully create the state machine, Step Functions responds with the creation
date and the state machine ARN.
```
`{
"creationDate": 1548454198.202,
"stateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld"
}`
```
2. Start an execution using the ARN of the state machine you created.
```
`aws stepfunctions --endpoint-url http://localhost:8083 start-execution --state-machine-arn arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld`
```
### Step Functions Local with AWS SAM CLI Local
You can use the local version of Step Functions with a local version of AWS Lambda. To configure
this, you must install and configure AWS SAM.
For information about configuring and running AWS SAM, see the following:
* [Set Up AWS SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-quick-start.html)
* [Start AWS SAM CLI Local](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-local-start-lambda.html)
When Lambda is running on your local system, you can start Step Functions Local. From the
directory where you extracted your Step Functions local JAR files, start Step Functions Local and use
the `--lambda-endpoint` parameter to configure the local Lambda
endpoint.
```
`java -jar StepFunctionsLocal.jar --lambda-endpoint http://127.0.0.1:3001 `command``
```
For more information about running Step Functions Local with AWS Lambda, see [Tutorial: Testing workflows using Step Functions and AWS SAM CLI Local](./sfn-local-lambda.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Testing with TestState
Tutorial: Testing using Step Functions and AWS SAM CLI Local
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.