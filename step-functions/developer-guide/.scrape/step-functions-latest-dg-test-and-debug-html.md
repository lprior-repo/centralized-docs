---
url: https://docs.aws.amazon.com/step-functions/latest/dg/test-and-debug.html
title: Testing and debugging Step Functions state machines
word_count: 388
filtered: true
elements_removed: 0
density_score: 0.87
---

Testing and debugging Step Functions state machines - AWS Step Functions
Testing and debugging Step Functions state machines - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#test-and-debug)
[Test with Test State](#test-with-teststate)[Data flow simulator (unsupported)](#use-data-flow-simulator)[Step Functions Local (unsupported)](#sfn-local-summary)
# Testing and debugging Step Functions state machines
Step Functions provides the following ways to test and debug state machines:
## Test with Test State in console and API
In the Step Functions console, you can test an individual state with **Test State**. You provide the state definition and inputs in the console, then Step Functions runs the state and shows the outputs, all without creating a state machine.
Or, you can use the [TestState](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TestState.html) API to test an individual state. You provide the definition of a single state, and the API will execute the state and report results, also without creating an actual state machine.
See [Testing with TestState](./test-state-isolation.html)
through the [TestState API](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TestState.html) to
test your states.
## Data flow simulator (unsupported)
Data flow simulator is a console tool that was built to test JSONPath syntax. The data
flow simulator is **unsupported**.
See [Testing with TestState](./test-state-isolation.html)
through the [TestState API](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TestState.html) to
test your states.
## Step Functions Local (unsupported)
With AWS Step Functions Local, a downloadable version of Step Functions, you can test applications with
Step Functions running in your own development environment.
Step Functions Local does **not** provide feature parity. For
example, there is no support for optimized service integrations, cross-account access, or distributed map.
###### Step Functions Local is unsupported
Step Functions Local does **not** provide feature parity and is **unsupported**.
You might consider third party solutions that emulate Step Functions for testing
purposes.
As an alternative to Step Functions Local, you can use the TestState API to unit test your state machine logic before deploying to your AWS account. For more information, see [Testing state machines with TestState API](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Events using User Notifications
Testing with TestState
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.