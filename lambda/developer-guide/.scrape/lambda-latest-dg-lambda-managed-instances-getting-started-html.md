---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-getting-started.html
title: Create a simple Python handler
word_count: 1374
filtered: true
elements_removed: 0
density_score: 0.89
---

Getting started with Lambda Managed Instances - AWS Lambda
Getting started with Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-getting-started)
[Creating a Lambda Managed Instance function (console)](#lambda-managed-instances-getting-started-console)[Creating a Lambda Managed Instance function (AWS CLI)](#lambda-managed-instances-getting-started-cli)
## Creating a Lambda Managed Instance function (console)
You can use the Lambda console to create a Managed Instance function that runs on Amazon EC2 instances managed by a capacity provider.
**Important:** Before creating a Managed Instance function, you must first create a capacity provider. These functions require a capacity provider to define the Amazon EC2 infrastructure that will run your functions.
**To create a Lambda Managed Instance function (console)**
1. Open the Lambda console.
2. Choose **Capacity providers** from the left navigation pane.
3. Choose **Create capacity provider**.
4. In the **Capacity provider settings** section, enter a name for your capacity provider.
5. Select VPC and permissions for your capacity provider. You can either use an existing or create a new one. For information about creating the required operator role, see [Lambda Operator role for Lambda Managed Instances](./lambda-managed-instances-operator-role.html).
6. Expand **Advanced settings**.
7. Define your **Instance requirements** by choosing the processor architecture and instance types.
8. Under **Auto scaling**, specify the maximum number of EC2 vCPUs for your capacity provider. You can also choose **Manual instance scaling mode** to set your own scaling value for precise control.
9. Choose **Create capacity provider** to create a new one.
10. Next, choose **Create function**.
11. Select **Author from scratch**.
12. In the **Basic information** pane, provide a **Function name**.
13. For **Runtime**, choose any of the supported Runtimes.
14. Choose the **Architecture** for your function (same as the one you selected for capacity provider). By default, **x86\_64**.
15. Under **Permissions**, ensure you have permission for the chosen **Execution role**. Otherwise, you can create a new role.
16. Under **Additional configurations**, pick the **Compute type** as **Lambda Managed Instances**.
17. Capacity provider ARN of the capacity provider you created in the previous steps should be pre-selected.
18. Choose **Memory size** and **Execution environment memory (GiB) per vCPU ratio**.
19. Choose **Create function**.
Your Lambda Managed Instance function is created and will provision capacity on your specified capacity provider. Function creation typically takes several minutes. Once complete, you can edit your function code and run your first test.
### Prerequisites
Before you begin, make sure you have the following:
* **AWS CLI** – Install and configure the AWS CLI. For more information, see [Installing or updating the latest version of the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
* **IAM permissions** – Your IAM user or role must have permissions to create Lambda functions, capacity providers, and pass IAM roles. Note that you'll also need `iam:CreateServiceLinkedRole` if it's the first time creating a capacity provider in the account or if the Service Linked Role (SLR) was deleted.
### Step 1: Create the required IAM roles
Lambda Managed Instances require two IAM roles: an execution role for your function and an operator role for the capacity provider. The operator role allows Lambda to launch, terminate, and monitor Amazon EC2 instances on your behalf. The function execution role grants the function permissions to access other AWS services and resources.
**To create the Lambda execution role**
1. Create a trust policy document that allows Lambda to assume the role:
```
`cat &gt; lambda-trust-policy.json &lt;&lt; 'EOF'
{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "lambda.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}
EOF`
```
2. Create the execution role:
```
`aws iam create-role \\
--role-name MyLambdaExecutionRole \\
--assume-role-policy-document file://lambda-trust-policy.json`
```
3. Attach the basic execution policy:
```
`aws iam attach-role-policy \\
--role-name MyLambdaExecutionRole \\
--policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole`
```
**To create the capacity provider operator role**
1. Create a trust policy document that allows Lambda to assume the operator role:
```
`cat &gt; operator-trust-policy.json &lt;&lt; 'EOF'
{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "lambda.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}
EOF`
```
2. Create the operator role:
```
`aws iam create-role \\
--role-name MyCapacityProviderOperatorRole \\
--assume-role-policy-document file://operator-trust-policy.json`
```
3. Attach the required EC2 permissions policy:
```
`aws iam attach-role-policy \\
--role-name MyCapacityProviderOperatorRole \\
--policy-arn arn:aws:iam::aws:policy/AWSLambdaManagedEC2ResourceOperator`
```
### Step 2: Set up VPC resources
Lambda Managed Instances run in your VPC and require a subnet and security group.
**To create VPC resources**
1. Create a VPC:
```
`VPC\_ID=$(aws ec2 create-vpc \\
--cidr-block 10.0.0.0/16 \\
--query 'Vpc.VpcId' \\
--output text)`
```
2. Create a subnet:
```
`SUBNET\_ID=$(aws ec2 create-subnet \\
--vpc-id $VPC\_ID \\
--cidr-block 10.0.1.0/24 \\
--query 'Subnet.SubnetId' \\
--output text)`
```
3. Create a security group:
```
`SECURITY\_GROUP\_ID=$(aws ec2 create-security-group \\
--group-name my-capacity-provider-sg \\
--description "Security group for Lambda Managed Instances" \\
--vpc-id $VPC\_ID \\
--query 'GroupId' \\
--output text)`
```
**Note:** Your Lambda Managed Instances functions require VPC configuration to access resources outside the VPC and to transmit telemetry data to CloudWatch Logs and X-Ray. For configuration details, see [Networking for Lambda Managed Instances](./lambda-managed-instances-networking.html).
### Step 3: Create a capacity provider
A capacity provider manages the EC2 instances that run your Lambda functions.
**To create a capacity provider**
```
`ACCOUNT\_ID=$(aws sts get-caller-identity --query Account --output text)
aws lambda create-capacity-provider \\
--capacity-provider-name my-capacity-provider \\
--vpc-config SubnetIds=[$SUBNET\_ID],SecurityGroupIds=[$SECURITY\_GROUP\_ID] \\
--permissions-config CapacityProviderOperatorRoleArn=arn:aws:iam::${ACCOUNT\_ID}:role/MyCapacityProviderOperatorRole \\
--instance-requirements Architectures=[x86\_64] \\
--capacity-provider-scaling-config MaxVCpuCount=30`
```
This command creates a capacity provider with the following configuration:
* **VPC configuration** – Specifies the subnet and security group for the EC2 instances
* **Permissions** – Defines the IAM role that Lambda uses to manage EC2 instances
* **Instance requirements** – Specifies the x86\_64 architecture
* **Scaling configuration** – Sets a maximum of 30 vCPUs for the capacity provider
### Step 4: Create a Lambda function with inline code
**To create a function with inline code**
1. First, create a simple Python function and package it inline:
```
`# Create a temporary directory for the function code
mkdir -p /tmp/my-lambda-function
cd /tmp/my-lambda-function
# Create a simple Python handler
cat &gt;&gt; lambda\_function.py &lt;&lt;&lt;&lt; 'EOF'
import json
def lambda\_handler(event, context):
return {
'statusCode': 200,
'body': json.dumps({
'message': 'Hello from Lambda Managed Instances!',
'event': event
})
}
EOF
# Create a ZIP file
zip function.zip lambda\_function.py`
```
2. Create the Lambda function using the inline ZIP file:
```
`ACCOUNT\_ID=$(aws sts get-caller-identity --query Account --output text)
REGION=$(aws configure get region)
aws lambda create-function \\
--function-name my-managed-instance-function \\
--package-type Zip \\
--runtime python3.13 \\
--handler lambda\_function.lambda\_handler \\
--zip-file fileb:///tmp/my-lambda-function/function.zip \\
--role arn:aws:iam::${ACCOUNT\_ID}:role/MyLambdaExecutionRole \\
--architectures x86\_64 \\
--memory-size 2048 \\
--ephemeral-storage Size=512 \\
--capacity-provider-config LambdaManagedInstancesCapacityProviderConfig={CapacityProviderArn=arn:aws:lambda:${REGION}:${ACCOUNT\_ID}:capacity-provider:my-capacity-provider}`
```
The function is created with:
* **Runtime** – Python 3.13
* **Handler** – The `lambda\_handler` function in `lambda\_function.py`
* **Memory** – 2048 MB
* **Ephemeral storage** – 512 MB
* **Capacity provider** – Links to the capacity provider you created
### Step 5: Publish a function version
To run your function on Lambda Managed Instances, you must publish a version.
**To publish a function version**
```
`aws lambda publish-version \\
--function-name my-managed-instance-function`
```
This command publishes version 1 of your function and deploys it to the capacity provider.
### Step 6: Invoke your function
After publishing, you can invoke your function.
**To invoke your function**
```
`aws lambda invoke \\
--function-name my-managed-instance-function:1 \\
--payload '{"name": "World"}' \\
response.json
# View the response
cat response.json`
```
The function runs on the EC2 instances managed by your capacity provider and returns a response.
### Clean up
To avoid incurring charges, delete the resources you created:
1. Delete the function:
```
`aws lambda delete-function --function-name my-managed-instance-function`
```
2. Delete the capacity provider:
```
`aws lambda delete-capacity-provider --capacity-provider-name my-capacity-provider`
```
3. Delete the VPC resources:
```
`aws ec2 delete-security-group --group-id $SECURITY\_GROUP\_ID
aws ec2 delete-subnet --subnet-id $SUBNET\_ID
aws ec2 delete-vpc --vpc-id $VPC\_ID`
```
4. Delete the IAM roles:
```
`aws iam detach-role-policy \\
--role-name MyLambdaExecutionRole \\
--policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
aws iam detach-role-policy \\
--role-name MyCapacityProviderOperatorRole \\
--policy-arn arn:aws:iam::aws:policy/AWSLambdaManagedEC2ResourceOperator
aws iam delete-role --role-name MyLambdaExecutionRole
aws iam delete-role --role-name MyCapacityProviderOperatorRole`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda Managed Instances
Core concepts
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.