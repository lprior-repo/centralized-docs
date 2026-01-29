---
url: https://docs.aws.amazon.com/lambda/latest/dg/governance-cloudformation-guard.html
title: Proactive controls for Lambda with AWS CloudFormation Guard
word_count: 824
filtered: true
elements_removed: 0
density_score: 0.82
---

Proactive controls for Lambda with AWS CloudFormation Guard - AWS Lambda
Proactive controls for Lambda with AWS CloudFormation Guard - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#governance-cloudformation-guard)
[Caveats](#governance-cloudformation-guard-considerations)
# Proactive controls for Lambda with AWS CloudFormation Guard
[AWS CloudFormation Guard](https://docs.aws.amazon.com/cfn-guard/latest/ug/what-is-guard.html) is an open-source, general-purpose, policy-as-code evaluation tool.
This can be used for preventative governance and compliance by validating Infrastructure as Code
(IaC) templates and service compositions against policy rules. These rules can be customized based
on your team or organizational requirements. For Lambda functions, the Guard rules can be used to control resource creation and configuration updates by
defining the required property settings needed while creating or updating a Lambda function.
Compliance administrators define the list of controls and governance policies that are required
for deploying and updating Lambda functions. Platform administrators implement the controls in
CI/CD pipelines, as pre-commit validation webhooks with code repositories, and provide developers
with command line tools for validating templates and code on local workstations. Developers author
code, validate templates with command line tools, and then commit code to repositories, which are
then automatically validated via the CI/CD pipelines prior to deployment into an AWS environment.
Guard allows you to [write your
rules](https://docs.aws.amazon.com/cfn-guard/latest/ug/writing-rules.html) and implement your controls with a domain-specific language as follows.
![Guard rules include resource type, property name, operator, expression value, and optional comment](https://docs.aws.amazon.com/images/lambda/latest/dg/images/governance-cloudformation-guard.png)
For example, suppose you want to ensure that developers choose only the latest runtimes. You
could specify two different policies, one to identify [runtimes](./lambda-runtimes.html)
that are already deprecated and another to identify runtimes that are to be deprecated soon. To do
this, you might write the following `etc/rules.guard` file:
```
`let lambda\_functions = Resources.\*[
Type == "AWS::Lambda::Function"
]
rule lambda\_already\_deprecated\_runtime when %lambda\_functions !empty {
%lambda\_functions {
Properties {
when Runtime exists {
Runtime !in ["dotnetcore3.1", "nodejs12.x", "python3.6", "python2.7", "dotnet5.0", "dotnetcore2.1", "ruby2.5", "nodejs10.x", "nodejs8.10", "nodejs4.3", "nodejs6.10", "dotnetcore1.0", "dotnetcore2.0", "nodejs4.3-edge", "nodejs"] &lt;&lt;&lt;&lt;Lambda function is using a deprecated runtime.&gt;&gt;&gt;&gt;
}
}
}
}
rule lambda\_soon\_to\_be\_deprecated\_runtime when %lambda\_functions !empty {
%lambda\_functions {
Properties {
when Runtime exists {
Runtime !in ["nodejs16.x", "nodejs14.x", "python3.7", "java8", "dotnet7", "go1.x", "ruby2.7", "provided"] &lt;&lt;Lambda function is using a runtime that is targeted for deprecation.&gt;&gt;
}
}
}
}`
```
Now suppose you write the following `iac/lambda.yaml` CloudFormation template
that defines a Lambda function:
```
`
Fn:
Type: AWS::Lambda::Function
Properties:
Runtime: python3.7
CodeUri: src
Handler: fn.handler
Role: !GetAtt FnRole.Arn
Layers:
- arn:aws:lambda:us-east-1:111122223333:layer:LambdaInsightsExtension:35`
```
After [installing](https://docs.aws.amazon.com/cfn-guard/latest/ug/setting-up.html)
the Guard utility, validate your template:
```
`cfn-guard validate --rules etc/rules.guard --data iac/lambda.yaml`
```
The output looks like this:
```
`lambda.yaml Status = FAIL
FAILED rules
rules.guard/lambda\_soon\_to\_be\_deprecated\_runtime
---
Evaluating data lambda.yaml against rules rules.guard
Number of non-compliant resources 1
Resource = Fn {
Type = AWS::Lambda::Function
Rule = lambda\_soon\_to\_be\_deprecated\_runtime {
ALL {
Check = Runtime not IN ["nodejs16.x","nodejs14.x","python3.7","java8","dotnet7","go1.x","ruby2.7","provided"] {
ComparisonError {
Message = Lambda function is using a runtime that is targeted for deprecation.
Error = Check was not compliant as property [/Resources/Fn/Properties/Runtime[L:88,C:15]] was not present in [(resolved, Path=[L:0,C:0] Value=["nodejs16.x","nodejs14.x","python3.7","java8","dotnet7","go1.x","ruby2.7","provided"])]
}
PropertyPath = /Resources/Fn/Properties/Runtime[L:88,C:15]
Operator = NOT IN
Value = "python3.7"
ComparedWith = [["nodejs16.x","nodejs14.x","python3.7","java8","dotnet7","go1.x","ruby2.7","provided"]]
Code:
86. Fn:
87. Type: AWS::Lambda::Function
88. Properties:
89. Runtime: python3.7
90. CodeUri: src
91. Handler: fn.handler
}
}
}
}`
```
Guard allows your developers to see from their local developer workstations that
they need to update the template to use a runtime that is allowed by the organization. This
happens prior to committing to a code repository and subsequently failing checks within a CI/CD
pipeline. As a result, your developers get this feedback on how to develop compliant templates and
shift their time to writing code that delivers business value. This control can be applied on
the local developer workstation, in a pre-commit validation webhook, and/or in the CI/CD pipeline
prior to deployment.
## Caveats
If you're using AWS Serverless Application Model (AWS SAM) templates to define Lambda functions, be aware that
you need to update the Guard rule to search for the `AWS::Serverless::Function` resource type as follows.
```
`let lambda\_functions = Resources.\*[
Type == "AWS::Serverless::Function"
]`
```
Guard also expects the properties to be included within the resource definition.
Meanwhile, AWS SAM templates allow for properties to be specified in a separate
[Globals](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-specification-template-anatomy-globals.html)
section. Properties that are defined in the Globals section are not validated with your
Guard rules.
As outlined in the Guard troubleshooting [documentation](https://docs.aws.amazon.com/cfn-guard/latest/ug/troubleshooting.html),
be aware that Guard doesn't support short-form intrinsics like `!GetAtt` or `!Sub` and instead requires using the expanded
forms: `Fn::GetAtt` and `Fn::Sub`. (The [earlier example](#guard-iac-yaml) doesn't evaluate the Role property, so the short-form intrinsic was used for simplicity.)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Governance
Proactive controls with AWS Config
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.