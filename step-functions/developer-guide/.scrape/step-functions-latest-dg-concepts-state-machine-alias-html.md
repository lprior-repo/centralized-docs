---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html
title: State machine aliases in Step Functions workflows
word_count: 1011
filtered: true
elements_removed: 0
density_score: 0.78
---

State machine aliases in Step Functions workflows - AWS Step Functions
State machine aliases in Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-state-machine-alias)
[Creating a state machine alias
(Console)](#procedure-create-aliases)[Managing aliases with
APIs](#manage-aliases-with-api)[Alias routing configuration](#alias-routing-config)[Running a state machine using an alias
(Console)](#procedure-run-exec-with-alias)
# State machine aliases in Step Functions workflows
An *alias* is a pointer for up to two versions of the same state
machine. You can create multiple aliases for your state machines. Each alias has a unique Amazon Resource Name (ARN). The alias
ARN is a combination of the state machine's ARN and the alias name, separated by a colon
(:).
The following example shows the format of a state machine alias ARN.
```
`arn:`partition`:states:`region`:`account-id`:stateMachine:`myStateMachine`:`aliasName``
```
You
can use an alias to
[route traffic](#alias-routing-config) between one
of the two state machine versions. You can also create an alias that points to a single
version. Aliases can only point to state machine versions. You can't use an alias to point
to another alias. You can also update an alias to point to a different version of the state
machine.
![Diagram showing an alias sending 80 percent of requests to v1, and 20 percent to v2.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/aliases-concept.png)
###### Contents
* [Creating a state machine alias
(Console)](#procedure-create-aliases)
* [Managing aliases with
APIs](#manage-aliases-with-api)
* [Alias routing configuration](#alias-routing-config)
* [Running a state machine using an alias
(Console)](#procedure-run-exec-with-alias)
## Creating a state machine alias
(Console)
You can create up to 100 aliases for each state machine by using the Step Functions console or
by invoking the [CreateStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachineAlias.html) API action. To request an increase
to this soft limit, use the **Support Center** page in the
[AWS Management Console](https://docs.aws.amazon.com/servicequotas/latest/userguide/request-quota-increase.html). Delete unused aliases from the console or by invoking the [DeleteStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachineAlias.html) API action.
###### To create a state machine alias
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), and then choose an existing state machine.
2. On the **State machine detail** page, choose the
**Aliases** tab.
3. Choose **Create new alias**.
4. On the **Create alias** page, do the following:
1. Enter an **Alias name**.
2. (Optional) Enter a **Description** for the
alias.
3. To configure routing on the alias, see [Alias routing configuration](#alias-routing-config).
4. Choose **Create alias**.
## Managing aliases with Step Functions API
operations
Step Functions provides the following API operations that you can use to create and manage
state machine aliases or get information about the aliases:
* [CreateStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachineAlias.html) – Creates an alias
for a state machine.
* [DescribeStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachineAlias.html) – Returns details
about a state machine alias.
* [ListStateMachineAliases](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachineAliases.html) – Lists aliases for
the specified state machine ARN.
* [UpdateStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachineAlias.html) – Updates the
configuration of an existing state machine alias by modifying its `description` or `routingConfiguration`.
* [DeleteStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachineAlias.html) – Deletes a state
machine alias.
To create an alias named ``PROD`` that points to
version 1 of a state machine named
``myStateMachine`` using the AWS Command Line Interface, use the
`create-state-machine-alias` command.
```
`aws stepfunctions create-state-machine-alias --name PROD --routing-configuration "[{\\"stateMachineVersionArn\\":\\"arn:aws:states:`region`:`account-id`:stateMachine:`myStateMachine`:`1`\\",\\"weight\\":100}]"`
```
## Alias routing configuration
You can use an alias to route execution traffic between two versions of a state
machine. For example, say you want to launch a new version of your state machine. You
can reduce the risks involved in deploying the new version by configuring routing on an
alias. By configuring routing, you can send most of your traffic to an earlier, tested
version of your state machine. The new version can then receive a smaller percentage,
until you can
confirm
that it's safe to roll forward the new version.
To define routing configuration, make sure
that
you publish both state machine versions
that
your alias points to. When you start an execution from an alias, Step Functions
randomly chooses the state machine version to run from the versions specified in the
routing configuration. It bases this choice on the traffic percentage that you assign to
each version in the alias routing configuration.
###### To configure routing configuration on an alias
* On the **Create alias** page, under **Routing
configuration**, do the following:
1. For **Version**, choose the first state machine
version that the alias points to.
2. Select
the **Split traffic between two versions** check box.
###### Tip
To point to a single version, clear the **Split traffic
between two versions** check box.
3. For **Version**, choose the second version that the
alias must point to.
4. In the **Traffic percentage** fields, specify the
percentage of traffic to route to each version. For example, enter
`60` and `40` to route 60
percent
of the execution traffic to the first version and 40 percent traffic to
the second version.
The combined traffic percentages must equal to 100 percent.
## Running a state machine using an alias
(Console)
You can start state machine executions with an alias from either the console or by
invoking the [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) API action with the alias' ARN. Step Functions then
runs the version specified by the alias. By default, if you don't specify a version or
alias when you start a state machine execution, Step Functions uses the most recent
revision.
###### To start a state machine execution using an alias
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), then choose an existing state machine that you've created
an alias for. For information about creating an alias, see [Creating a state machine alias
(Console)](#procedure-create-aliases).
2. On the **State machine detail** page, choose the
**Aliases** tab.
3. In the **Aliases** section, do the following:
1. Select
the alias that you want to start the execution with.
2. Choose **Start execution**.
3. (Optional) In the **Start execution** dialog box, enter a
name for the execution.
4. If required, enter the execution input, and then choose **Start
execution**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Versions
Versions and alias authorization
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.