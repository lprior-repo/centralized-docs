---
url: https://docs.aws.amazon.com/lambda/latest/dg/powershell-package.html
title: Deploy PowerShell Lambda functions with .zip file archives
word_count: 606
filtered: true
elements_removed: 0
density_score: 0.85
---

Deploy PowerShell Lambda functions with .zip file archives - AWS Lambda
Deploy PowerShell Lambda functions with .zip file archives - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#powershell-package)
[Creating a Lambda function](#powershell-package-create)
# Deploy PowerShell Lambda functions with .zip file archives
A deployment package for the PowerShell runtime contains your PowerShell script,
PowerShell modules that are required for your PowerShell script, and the assemblies needed to host PowerShell Core.
## Creating the Lambda function
To get started writing and invoking a PowerShell script with Lambda, you can use the
`New-AWSPowerShellLambda` cmdlet to create a starter script based on a template. You can use the
`Publish-AWSPowerShellLambda` cmdlet to deploy your script to Lambda. Then you can test your script
either through the command line or the Lambda console.
To create a new PowerShell script, upload it, and test it, do the following:
1. To view the list of available templates, run the following command:
```
`PS C:\\&gt;&gt; Get-AWSPowerShellLambdaTemplate
Template Description
-------- -----------
Basic Bare bones script
CodeCommitTrigger Script to process AWS CodeCommit Triggers
...`
```
2. To create a sample script based on the `Basic` template, run the following command:
```
`New-AWSPowerShellLambda -ScriptName MyFirstPSScript -Template Basic`
```
A new file named `MyFirstPSScript.ps1` is created in a new subdirectory of the current directory.
The name of the directory is based on the `-ScriptName` parameter. You can use the
`-Directory` parameter to choose an alternative directory.
You can see that the new file has the following contents:
```
`# PowerShell script file to run as a Lambda function
# When executing in Lambda the following variables are predefined.
# $LambdaInput - A PSObject that contains the Lambda function input data.
# $LambdaContext - An Amazon.Lambda.Core.ILambdaContext object that contains information about the currently running Lambda environment.
# The last item in the PowerShell pipeline is returned as the result of the Lambda function.
# To include PowerShell modules with your Lambda function, like the AWSPowerShell.NetCore module, add a "#Requires" statement
# Write-Host (ConvertTo-Json -InputObject $LambdaInput -Compress -Depth 5)`
```
3. To see how log messages from your PowerShell script are sent to Amazon CloudWatch Logs, uncomment the
`Write-Host` line of the sample script.
To demonstrate how you can return data back from your Lambda functions, add a new line at the end of the
script with `$PSVersionTable`. This adds the `$PSVersionTable` to the PowerShell pipeline.
After the PowerShell script is complete, the last object in the PowerShell pipeline is the return data for the
Lambda function. `$PSVersionTable` is a PowerShell global variable that also provides information
about the running environment.
After making these changes, the last two lines of the sample script look like this:
```
`Write-Host (ConvertTo-Json -InputObject $LambdaInput -Compress -Depth 5)
$PSVersionTable`
```
4. After editing the `MyFirstPSScript.ps1` file, change the directory to the script's location. Then
run the following command to publish the script to Lambda:
```
`Publish-AWSPowerShellLambda -ScriptPath .\\MyFirstPSScript.ps1 -Name MyFirstPSScript -Region us-east-2`
```
Note that the `-Name` parameter specifies the Lambda function name, which appears in the Lambda
console. You can use this function to invoke your script manually.
5. Invoke your function using the AWS Command Line Interface (AWS CLI) `invoke` command.
```
`&gt; aws lambda invoke --function-name MyFirstPSScript out`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Development Environment
Handler
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.