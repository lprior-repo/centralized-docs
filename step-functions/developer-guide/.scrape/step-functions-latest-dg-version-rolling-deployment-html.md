---
url: https://docs.aws.amazon.com/step-functions/latest/dg/version-rolling-deployment.html
title: Perform gradual deployment of state machine versions in Step Functions
word_count: 2135
filtered: true
elements_removed: 0
density_score: 0.79
---

Perform gradual deployment of state machine versions in Step Functions - AWS Step Functions
Perform gradual deployment of state machine versions in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#version-rolling-deployment)
# Perform gradual deployment of state machine versions in Step Functions
A rolling deployment is a deployment strategy that slowly replaces previous versions of an
application with new versions of an application. To perform a rolling deployment of a state
machine
version,
gradually send an increasing amount of execution traffic to the new version. The amount of
traffic and rate of increase are parameters that
you
configure.
You can perform rolling deployment of a version using one of the following options:
* [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/) – Create an alias that points to two versions of the same state machine. For this alias, you configure the routing configuration to shift traffic between the two versions. For more information about using the console to roll out versions, see [Versions](./concepts-state-machine-version.html) and [Aliases](./concepts-state-machine-alias.html).
* **Scripts for AWS CLI and SDK** – Create a shell script using the AWS CLI or the AWS SDK. For more information, see the following sections for using AWS CLI and AWS SDK.
* **AWS CloudFormation templates** – Use the `[AWS::StepFunctions::StateMachineVersion](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html)` and `[AWS::StepFunctions::StateMachineAlias](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html)` resources to publish multiple state machine versions and create an alias to point to one or two of these versions.
The example script in this section shows how you can use the AWS CLI to gradually shift traffic from a previous state machine version to a new state machine version. You can either use this example script or update it according to your requirements.
This script shows a Canary deployment for deploying a new state machine version using an alias. The following steps outline the tasks that the script performs:
1. If the `publish\_revision` parameter is set to true, publish the most recent [revision](./concepts-cd-aliasing-versioning.html#statemachinerev) as the next version of the state machine. This version becomes the new, live version if the deployment succeeds.
If you set the `publish\_revision` parameter to false, the script deploys the last published version of the state machine.
2. Create an alias if it doesn't exist yet. If the alias doesn't exist, point 100 percent of traffic for this alias to the new version, and then exit the script.
3. Update the routing configuration of the alias to shift a small percentage of traffic from the previous version to the new version. You set this canary percentage with the `canary\_percentage` parameter.
4. By default, monitor the configurable CloudWatch alarms every 60 seconds. If any of
these alarms
set
off, rollback the deployment immediately by pointing 100
percent of traffic to the previous version.
After every time interval, in seconds, defined in `alarm\_polling\_interval`, continue monitoring the alarms. Continue monitoring until the time interval defined in `canary\_interval\_seconds` has passed.
5. If no alarms were
set
off during `canary\_interval\_seconds`, shift 100
percent of traffic to the new version.
6. If the new version deploys successfully, delete any versions older than the number specified in the `history\_max` parameter.
```
`#!/bin/bash
# AWS StepFunctions example showing how to create a canary deployment with a
# A canary deployment deploys the new version alongside the old version, while
# routing only a small fraction of the overall traffic to the new version to
# see if there are any errors. Only once the new version has cleared a testing
# For a Blue/Green or All at Once style deployment, you can set the
# canary\_percentage to 100. The script will immediately shift 100% of traffic
# to the new version, but keep on monitoring the alarms (if any) during the
# canary\_interval\_seconds time interval. If any alarms raise during this period,
# the script will automatically rollback to the previous version.
# Step Functions allows you to keep a maximum of 1000 versions in version history
# for a state machine. This script has a version history deletion mechanism at
# the end, where it will delete any versions older than the limit specified.
# For an example that also demonstrates linear (or rolling) deployments, see the following:
# you can safely change the variables in this block to your values
state\_machine\_name="my-state-machine"
alias\_name="alias-1"
region="us-east-1"
# array of cloudwatch alarms to poll during the test period.
# to disable alarm checking, set alarm\_names=()
alarm\_names=("alarm1" "alarm name with a space")
# true to publish the current revision as the next version before deploy.
# false to deploy the latest version from the state machine's version history.
publish\_revision=true
# true to force routing configuration update even if the current routing
# for the alias does not have a 100% routing config.
# false will abandon deploy attempt if current routing config not 100% to a
# Be careful when you combine this flag with publish\_revision - if you just
# rerun the script you might deploy the newly published revision from the
# percentage of traffic to route to the new version during the test period
canary\_percentage=10
# how many seconds the canary deployment lasts before full deploy to 100%
canary\_interval\_seconds=300
# how many versions to keep in history. delete versions prior to this.
# set to 0 to disable old version history deletion.
history\_max=0
# If you don't specify version 2 details, will only create 1 routing entry. In
# this case the routing entry weight must be 100.
#######################################
function update\_routing() {
if [[ $# -eq 2 ]]; then
local routing\_config="[{\\"stateMachineVersionArn\\": \\"$1\\", \\"weight\\":$2}]"
elif [[ $# -eq 4 ]]; then
local routing\_config="[{\\"stateMachineVersionArn\\": \\"$1\\", \\"weight\\":$2}, {\\"stateMachineVersionArn\\": \\"$3\\", \\"weight\\":$4}]"
else
echo "You have to call update\_routing with either 2 or 4 input arguments." &gt;&gt;&amp;&amp;2
exit 1
fi
${aws} update-state-machine-alias --state-machine-alias-arn ${alias\_arn} --routing-configuration "${routing\_config}"
}
# pre-run validation
if [[ (("${#alarm\_names[@]}" -gt 0)) ]]; then
alarm\_exists\_count=$(aws cloudwatch describe-alarms --alarm-names "${alarm\_names[@]}" --alarm-types "CompositeAlarm" "MetricAlarm" --query "length([MetricAlarms, CompositeAlarms][])" --output text)
if [[ (("${#alarm\_names[@]}" -ne "${alarm\_exists\_count}")) ]]; then
echo All of the alarms to monitor do not exist in CloudWatch: $(IFS=,; echo "${alarm\_names[\*]}") &gt;&gt;&amp;&amp;2
echo Only the following alarm names exist in CloudWatch:
aws cloudwatch describe-alarms --alarm-names "${alarm\_names[@]}" --alarm-types "CompositeAlarm" "MetricAlarm" --query "join(', ', [MetricAlarms, CompositeAlarms][].AlarmName)" --output text
exit 1
fi
fi
if [[ (("${history\_max}" -gt 0)) &amp;&amp;&amp;&amp; (("${history\_max}" -lt 2)) ]]; then
echo The minimum value for history\_max is 2. This is the minimum number of older state machine versions to be able to rollback in the future. &gt;&gt;&amp;&amp;2
exit 1
fi
# main block follows
account\_id=$(aws sts get-caller-identity --query Account --output text)
sm\_arn="arn:aws:states:${region}:${account\_id}:stateMachine:${state\_machine\_name}"
# the aws command we'll be invoking a lot throughout.
aws="aws stepfunctions"
# promote the latest revision to the next version
if [[ "${publish\_revision}" = true ]]; then
new\_version=$(${aws} publish-state-machine-version --state-machine-arn=$sm\_arn --query stateMachineVersionArn --output text)
echo Published the current revision of state machine as the next version with arn: ${new\_version}
else
new\_version=$(${aws} list-state-machine-versions --state-machine-arn ${sm\_arn} --max-results 1 --query "stateMachineVersions[0].stateMachineVersionArn" --output text)
echo "Since publish\_revision is false, using the latest version from the state machine's version history: ${new\_version}"
fi
# find the alias if it exists
alias\_arn\_expected="${sm\_arn}:${alias\_name}"
alias\_arn=$(${aws} list-state-machine-aliases --state-machine-arn ${sm\_arn} --query "stateMachineAliases[?stateMachineAliasArn==\\`${alias\_arn\_expected}\\`].stateMachineAliasArn" --output text)
if [[ "${alias\_arn\_expected}" == "${alias\_arn}" ]]; then
echo Found alias ${alias\_arn}
echo Current routing configuration is:
${aws} describe-state-machine-alias --state-machine-alias-arn "${alias\_arn}" --query routingConfiguration
else
echo Alias does not exist. Creating alias ${alias\_arn\_expected} and routing 100% traffic to new version ${new\_version}
${aws} create-state-machine-alias --name "${alias\_name}" --routing-configuration "[{\\"stateMachineVersionArn\\": \\"${new\_version}\\", \\"weight\\":100}]"
echo Done!
exit 0
fi
# find the version to which the alias currently points (the current live version)
old\_version=$(${aws} describe-state-machine-alias --state-machine-alias-arn $alias\_arn --query "routingConfiguration[?weight==\\`100\\`].stateMachineVersionArn" --output text)
if [[ -z "${old\_version}" ]]; then
if [[ "${force}" = true ]]; then
echo Force setting is true. Will force update to routing config for alias to point 100% to new version.
update\_routing "${new\_version}" 100
echo Alias ${alias\_arn} now pointing 100% to ${new\_version}.
echo Done!
exit 0
else
echo Alias ${alias\_arn} does not have a routing config entry with 100% of the traffic. This means there might be a deploy in progress, so not starting another deploy at this time. &gt;&gt;&amp;&amp;2
exit 1
fi
fi
if [[ "${old\_version}" == "${new\_version}" ]]; then
echo The alias already points to this version. No update necessary.
exit 0
fi
echo Switching ${canary\_percentage}% to new version ${new\_version}
(( old\_weight = 100 - ${canary\_percentage} ))
update\_routing "${new\_version}" ${canary\_percentage} "${old\_version}" ${old\_weight}
echo New version receiving ${canary\_percentage}% of traffic.
echo Old version ${old\_version} is still receiving ${old\_weight}%.
if [[ ${#alarm\_names[@]} -eq 0 ]]; then
echo No alarm\_names set. Skipping cloudwatch monitoring.
echo Will sleep for ${canary\_interval\_seconds} seconds before routing 100% to new version.
sleep ${canary\_interval\_seconds}
echo Canary period complete. Switching 100% of traffic to new version...
else
echo Checking if alarms fire for the next ${canary\_interval\_seconds} seconds.
(( total\_wait = canary\_interval\_seconds + $(date +%s) ))
now=$(date +%s)
while [[ ((${now} -lt ${total\_wait})) ]]; do
alarm\_result=$(aws cloudwatch describe-alarms --alarm-names "${alarm\_names[@]}" --state-value ALARM --alarm-types "CompositeAlarm" "MetricAlarm" --query "join(', ', [MetricAlarms, CompositeAlarms][].AlarmName)" --output text)
if [[ ! -z "${alarm\_result}" ]]; then
echo The following alarms are in ALARM state: ${alarm\_result}. Rolling back deploy. &gt;&gt;&amp;&amp;2
update\_routing "${old\_version}" 100
echo Rolled back to ${old\_version}
exit 1
fi
echo Monitoring alarms...no alarms have triggered.
sleep ${alarm\_polling\_interval}
now=$(date +%s)
done
echo No alarms detected during canary period. Switching 100% of traffic to new version...
fi
update\_routing "${new\_version}" 100
echo Version ${new\_version} is now receiving 100% of traffic.
if [[ (("${history\_max}" -eq 0 ))]]; then
echo Version History deletion is disabled. Remember to prune your history, the default limit is 1000 versions.
echo Done!
exit 0
fi
echo Keep the last ${history\_max} versions. Deleting any versions older than that...
# the results are sorted in descending order of the version creation time
version\_history=$(${aws} list-state-machine-versions --state-machine-arn ${sm\_arn} --max-results 1000 --query "join(\\`\\"\\\\n\\"\\`, stateMachineVersions[].stateMachineVersionArn)" --output text)
counter=0
while read line; do
((counter=${counter} + 1))
if [[ (( ${counter} -gt ${history\_max})) ]]; then
echo Deleting old version ${line}
${aws} delete-state-machine-version --state-machine-version-arn ${line}
fi
done &lt;&lt;&lt; "${version\_history}"
echo Done!`
```
The example script at [aws-stepfunctions-examples](https://github.com/aws-samples/aws-stepfunctions-examples/tree/main/gradual-deploy) shows how
to
use the AWS SDK for Python to gradually shift traffic from a previous version to a new
version of a state machine. You can either use this example script or update it
according to your requirements.
The script shows the following deployment strategies:
* **Canary** – Shifts traffic in two increments.
In the first increment, a small percentage of traffic, for example, 10
percent is shifted to the new version. In the second increment, before a
specified time interval in seconds gets over, the remaining traffic is
shifted to the new version. The switch to the new version for the remaining
traffic takes place only if no CloudWatch alarms are
set
off during the specified time interval.
* **Linear or Rolling** – Shifts traffic to the new version in equal increments with an equal number of seconds between each increment.
For example, if you specify the increment percent as `20` with an `--interval` of `600` seconds, this deployment increases traffic by 20 percent every 600 seconds until the new version receives 100 percent of the traffic.
This deployment immediately rolls back the new version if any CloudWatch alarms
are
set
off.
* **All at Once or Blue/Green** – Shifts
100 percent of traffic to the new version immediately. This deployment
monitors the new version and rolls it back automatically to the previous
version if any CloudWatch alarms are
set
off.
The following CloudFormation template example publishes two versions of a state machine named ``MyStateMachine``. It creates an alias named ``PROD``, which points to both these versions, and then deploys the version `2`.
In this example, 10 percent of traffic is shifted to the version `2` every five minutes until this version receives 100 percent of the traffic. This example also shows how you can set CloudWatch alarms. If any of the alarms you set go into the `ALARM` state, the deployment fails and rolls back immediately.
```
`MyStateMachine:
Type: AWS::StepFunctions::StateMachine
Properties:
Type: STANDARD
StateMachineName: MyStateMachine
RoleArn: arn:aws:iam::`account-id`:role/myIamRole
Definition:
StartAt: PassState
States:
PassState:
Type: Pass
Result: Result
End: true
MyStateMachineVersionA:
Type: AWS::StepFunctions::StateMachineVersion
Properties:
Description: Version 1
StateMachineArn: !Ref MyStateMachine
MyStateMachineVersionB:
Type: AWS::StepFunctions::StateMachineVersion
Properties:
Description: Version 2
StateMachineArn: !Ref MyStateMachine
PROD:
Type: AWS::StepFunctions::StateMachineAlias
Properties:
Name: PROD
Description: The PROD state machine alias taking production traffic.
DeploymentPreference:
StateMachineVersionArn: !Ref MyStateMachineVersionB
Type: LINEAR
Percentage: 10
Interval: 5
Alarms:
# A list of alarms that you want to monitor. If any of these alarms trigger, rollback the deployment immediately by pointing 100 percent of traffic to the previous version.
- !Ref CloudWatchAlarm1
- !Ref CloudWatchAlarm2
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deployment example
Handling errors
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.