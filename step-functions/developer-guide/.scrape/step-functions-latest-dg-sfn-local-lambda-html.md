---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sfn-local-lambda.html
title: Tutorial: Testing workflows using Step Functions and AWS SAM CLI Local
word_count: 1121
filtered: true
elements_removed: 0
density_score: 0.85
---

Tutorial: Testing workflows using Step Functions and AWS SAM CLI Local - AWS Step Functions
Tutorial: Testing workflows using Step Functions and AWS SAM CLI Local - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sfn-local-lambda)
[Step 1: Set Up AWS SAM](#install-sam)[Step 2: Test AWS SAM CLI Local](#test-local-lambda)[Step 3: Start AWS SAM CLI Local](#start-local-lambda)[Step 4: Start Step Functions Local](#start-stepfunctions-local)[Step 5: Create a State Machine That References
Your AWS SAM CLI Local Function](#create-local-statemachine)[Step 6: Start an Execution of Your Local State
Machine](#run-local-statemachine)
# Tutorial: Testing workflows using Step Functions and AWS SAM CLI Local
###### Step Functions Local is unsupported
Step Functions Local does **not** provide feature parity and is **unsupported**.
You might consider third party solutions that emulate Step Functions for testing
purposes.
As an alternative to Step Functions Local, you can use the TestState API to unit test your state machine logic before deploying to your AWS account. For more information, see [Testing state machines with TestState API](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html).
With both AWS Step Functions and AWS Lambda running on your local machine, you can test your state
machine and Lambda functions without deploying your code to AWS.
For more information, see the
following
topics:
* [Testing state machines with Step Functions Local (unsupported)](./sfn-local.html)
* [Set Up AWS SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-quick-start.html)
## Step 1: Set Up AWS SAM
AWS Serverless Application Model (AWS SAM) CLI Local requires the AWS Command Line Interface, AWS SAM, and Docker to be installed.
1. [Install the AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html).
###### Note
Before installing the AWS SAM CLI, you need to install the AWS CLI and Docker. See the
[Prerequisites](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) for installing the AWS SAM CLI.
2. Go through the [AWS SAM Quick Start](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-quick-start.html) documentation. Be sure to follow the steps to do the
following:
1. [Initialize the Application](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-quick-start.html#gs-ex1-setup-local-app)
2. [Test the Application Locally](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-quick-start.html#gs-ex1-test-locally)
This creates a `sam-app` directory, and builds an environment that
includes a Python-based Hello World Lambda function.
## Step 2: Test AWS SAM CLI Local
Now that you have installed AWS SAM and created the Hello World Lambda function,
you can
test
the
function.
In the `sam-app` directory, enter the
following
command:
```
`sam local start-api`
```
This launches a local instance of your Lambda
function. You should see
output similar to the following:
```
`2019-01-31 16:40:27 Found credentials in shared credentials file: \~/.aws/credentials
2019-01-31 16:40:27 Mounting HelloWorldFunction at http://127.0.0.1:3000/hello [GET]
2019-01-31 16:40:27 You can now browse to the above endpoints to invoke your functions. You do not need to restart/reload SAM CLI while working on your functions changes will be reflected instantly/automatically. You only need to restart SAM CLI if you update your AWS SAM template
2019-01-31 16:40:27 \* Running on http://127.0.0.1:3000/ (Press CTRL+C to quit)`
```
Open a browser and enter the
following:
```
`http://127.0.0.1:3000/hello`
```
This
will output a response similar to the
following:
```
`{"message": "hello world", "location": "72.21.198.66"}`
```
Enter **CTRL+C** to end the Lambda API.
## Step 3: Start AWS SAM CLI Local
Now that you've tested that the function works, start AWS SAM CLI Local. In the
`sam-app` directory, enter the
following
command:
```
`sam local start-lambda`
```
This starts AWS SAM CLI Local and provides the endpoint to
use, similar to the
following
output:
```
`2019-01-29 15:33:32 Found credentials in shared credentials file: \~/.aws/credentials
2019-01-29 15:33:32 Starting the Local Lambda Service. You can now invoke your Lambda Functions defined in your template through the endpoint.
2019-01-29 15:33:32 \* Running on http://127.0.0.1:3001/ (Press CTRL+C to quit)`
```
### JAR File
If you're using the `.jar` file version of Step Functions Local, start Step Functions
and
specify the Lambda endpoint. In the directory where you extracted the
`.jar` files, enter the
following
command:
```
`java -jar StepFunctionsLocal.jar --lambda-endpoint http://localhost:3001`
```
When Step Functions Local starts, it checks the environment, and then the credentials configured
in your `\~/.aws/credentials` file. By default, it starts using a
fictitious
user ID, and is listed as `region us-east-1`.
```
`2019-01-29 15:38:06.324: Failed to load credentials from environment because Unable to load AWS credentials from environment variables (AWS\_ACCESS\_KEY\_ID (or AWS\_ACCESS\_KEY) and AWS\_SECRET\_KEY (or AWS\_SECRET\_ACCESS\_KEY))
2019-01-29 15:38:06.326: Loaded credentials from profile: default
2019-01-29 15:38:06.326: Starting server on port 8083 with account `account-id`, region us-east-1`
```
### Docker
If you're using the Docker version of Step Functions Local, launch Step Functions with the following
command:
```
`docker run -p 8083:8083 amazon/aws-stepfunctions-local`
```
For information about installing the Docker version of Step Functions, see [Setting Up Step Functions Local (Downloadable Version) in
Docker](./sfn-local.html#sfn-local-docker).
###### Note
You can specify the endpoint through the command line or by setting environment variables
if you launch Step Functions from the `.jar` file. For the Docker version, you must specify
the endpoints and credentials in a text file. See [Setting Configuration Options for Step Functions Local](./sfn-local.html#sfn-local-config-options).
## Step 5: Create a State Machine That References
Your AWS SAM CLI Local Function
After Step Functions Local is running, create a state machine that references the
`HelloWorldFunction`
that you initialized in [Step 1: Set Up AWS SAM](#install-sam).
```
`aws stepfunctions --endpoint http://localhost:8083 create-state-machine --definition "{\\
\\"Comment\\": \\"A Hello World example of the Amazon States Language using an AWS Lambda Local function\\",\\
\\"StartAt\\": \\"HelloWorld\\",\\
\\"States\\": {\\
\\"HelloWorld\\": {\\
\\"Type\\": \\"Task\\",\\
\\"Resource\\": \\"arn:aws:lambda:`region`:`account-id`:function:HelloWorldFunction\\",\\
\\"End\\": true\\
}\\
}\\
}\\" --name "HelloWorld" --role-arn "arn:aws:iam::012345678901:role/DummyRole"`
```
This will create a state machine and provide an Amazon Resource Name (ARN) that you can use to start
an execution.
```
`{
"creationDate": 1548805711.403,
"stateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld"
}`
```
## Step 6: Start an Execution of Your Local State
Machine
Once you have created a state machine, start an
execution.
You'll need to reference the endpoint and state machine ARN when using the following
`aws stepfunctions` command:
```
`aws stepfunctions --endpoint http://localhost:8083 start-execution --state-machine arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld --name test`
```
This starts an execution
named `test`
of your `HelloWorld` state
machine.
```
`{
"startDate": 1548810641.52,
"executionArn": "arn:aws:states:`region`:`account-id`:execution:HelloWorld:test"
}`
```
Now that Step Functions is running locally, you can interact with it using the AWS CLI. For example,
to get information about this execution, use the
following
command:
```
`aws stepfunctions --endpoint http://localhost:8083 describe-execution --execution-arn arn:aws:states:`region`:`account-id`:execution:HelloWorld:test`
```
Calling `describe-execution` for an execution provides more complete details,
similar
to the following output:
```
`{
"status": "SUCCEEDED",
"startDate": 1549056334.073,
"name": "test",
"executionArn": "arn:aws:states:`region`:`account-id`:execution:HelloWorld:test",
"stateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"stopDate": 1549056351.276,
"output": "{\\"statusCode\\": 200, \\"body\\": \\"{\\\\\\"message\\\\\\": \\\\\\"hello world\\\\\\", \\\\\\"location\\\\\\": \\\\\\"72.21.198.64\\\\\\"}\\"}",
"input": "{}"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Step Functions Local (unsupported)
Testing with mocked service integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.