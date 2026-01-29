---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-testing.html
title: Testing Lambda durable functions
word_count: 1266
filtered: true
elements_removed: 0
density_score: 0.87
---

Testing Lambda durable functions - AWS Lambda
Testing Lambda durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-testing)
[Local testing](#durable-local-testing)[Cloud testing](#durable-cloud-testing)[What to test](#durable-testing-patterns)[Testing strategy](#durable-testing-strategy)[Debugging failures](#durable-testing-debugging)
# Testing Lambda durable functions
AWS provides dedicated testing SDKs for durable functions that let you run and inspect executions both locally and in the cloud. Install the testing SDK for your language:
TypeScript
```
`
npm install --save-dev @aws/aws-durable-execution-sdk-js-testing
`
```
For complete documentation and examples, see the [TypeScript testing SDK](https://github.com/aws/aws-durable-execution-sdk-js/tree/development/packages/aws-durable-execution-sdk-js-testing) on GitHub.
Python
```
`
pip install aws-durable-execution-sdk-python-testing
`
```
For complete documentation and examples, see the [Python testing SDK](https://github.com/aws/aws-durable-execution-sdk-python-testing) on GitHub.
The testing SDK provides two testing modes: local testing for fast unit tests, and cloud testing for integration tests against deployed functions.
## Local testing
Local testing runs your durable functions in your development environment without requiring deployed resources. The test runner runs your function code directly and captures all operations for inspection.
Use local testing for unit tests, test-driven development, and CI/CD pipelines. Tests run locally without network latency or additional costs.
**Example test:**
TypeScript
```
`
import { withDurableExecution } from '@aws/aws-durable-execution-sdk-js';
import { DurableFunctionTestRunner } from '@aws/aws-durable-execution-sdk-js-testing';
const handler = withDurableExecution(async (event, context) =&gt; {
const result = await context.step('calculate', async () =&gt; {
return event.a + event.b;
});
return result;
});
test('addition works correctly', async () =&gt; {
const runner = new DurableFunctionTestRunner({ handler });
const result = await runner.run({ a: 5, b: 3 });
expect(result.status).toBe('SUCCEEDED');
expect(result.result).toBe(8);
const step = result.getStep('calculate');
expect(step.result).toBe(8);
});
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import durable\_execution, DurableContext
from aws\_durable\_execution\_sdk\_python\_testing import DurableFunctionTestRunner
from aws\_durable\_execution\_sdk\_python.execution import InvocationStatus
@durable\_execution
def handler(event: dict, context: DurableContext) -&gt;&gt; int:
result = context.step(lambda \_: event["a"] + event["b"], name="calculate")
return result
def test\_addition():
runner = DurableFunctionTestRunner(handler=handler)
with runner:
result = runner.run(input={"a": 5, "b": 3}, timeout=10)
assert result.status is InvocationStatus.SUCCEEDED
assert result.result == 8
step = result.get\_step("calculate")
assert step.result == 8
`
```
The test runner captures execution state including the final result, individual step results, wait operations, callbacks, and any errors. You can inspect operations by name or iterate through all operations to verify execution behavior.
### Execution stores
The testing SDK uses execution stores to persist test execution data. By default, tests use an in-memory store that's fast and requires no cleanup. For debugging or analyzing execution history, you can use a filesystem store that saves executions as JSON files.
**In-memory store (default):**
The in-memory store keeps execution data in memory during test runs. Data is lost when tests complete, making it ideal for standard unit tests and CI/CD pipelines where you don't need to inspect executions after tests finish.
**Filesystem store:**
The filesystem store persists execution data to disk as JSON files. Each execution is saved in a separate file, making it easy to inspect execution history after tests complete. Use the filesystem store when debugging complex test failures or analyzing execution patterns over time.
Configure the store using environment variables:
```
`
# Run tests
pytest tests/
`
```
Execution files are stored with sanitized names and contain the complete execution state including operations, checkpoints, and results. The filesystem store automatically creates the storage directory if it doesn't exist.
## Cloud testing
Cloud testing invokes deployed durable functions in AWS and retrieves their execution history using the Lambda API. Use cloud testing to verify behavior in production-like environments with real AWS services and configurations.
Cloud testing requires a deployed function and AWS credentials with permissions to invoke functions and read execution history:
```
`
{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"lambda:InvokeFunction",
"lambda:GetDurableExecution",
"lambda:GetDurableExecutionHistory"
],
"Resource": "arn:aws:lambda:region:account-id:function:function-name"
}
]
}
`
```
**Example cloud test:**
TypeScript
```
`
import { DurableFunctionCloudTestRunner } from '@aws/aws-durable-execution-sdk-js-testing';
test('deployed function processes orders', async () =&gt; {
const runner = new DurableFunctionCloudTestRunner({
functionName: 'order-processor',
region: 'us-east-1'
});
const result = await runner.run({ orderId: 'order-123' });
expect(result.status).toBe('SUCCEEDED');
expect(result.result.status).toBe('completed');
});
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python\_testing import (
DurableFunctionCloudTestRunner,
DurableFunctionCloudTestRunnerConfig
)
def test\_deployed\_function():
config = DurableFunctionCloudTestRunnerConfig(
function\_name="order-processor",
region="us-east-1"
)
runner = DurableFunctionCloudTestRunner(config=config)
result = runner.run(input={"orderId": "order-123"})
assert result.status is InvocationStatus.SUCCEEDED
assert result.result["status"] == "completed"
`
```
Cloud tests invoke the actual deployed function and retrieve execution history from AWS. This lets you verify integration with other AWS services, validate performance characteristics, and test with production-like data and configurations.
## What to test
Test durable functions by verifying execution outcomes, operation behavior, and error handling. Focus on business logic correctness rather than implementation details.
**Verify execution results:** Check that functions return the expected values for given inputs. Test both successful executions and error cases to ensure functions handle invalid input appropriately.
**Inspect operation execution:** Verify that steps, waits, and callbacks execute as expected. Check step results to ensure intermediate operations produce correct values. Validate that wait operations are configured with appropriate timeouts and that callbacks are created with correct settings.
**Test error handling:** Verify functions fail correctly with descriptive error messages when given invalid input. Test retry behavior by simulating transient failures and confirming operations retry appropriately. Check that permanent failures don't trigger unnecessary retries.
**Validate workflows:** For multi-step workflows, verify operations execute in the correct order. Test conditional branching to ensure different execution paths work correctly. Validate parallel operations execute concurrently and produce expected results.
The SDK documentation repositories contain extensive examples of testing patterns including multi-step workflows, error scenarios, timeout handling, and polling patterns.
## Testing strategy
Use local testing for unit tests during development and in CI/CD pipelines. Local tests run fast, don't require AWS credentials, and provide immediate feedback on code changes. Write local tests to verify business logic, error handling, and operation behavior.
Use cloud testing for integration tests before deploying to production. Cloud tests verify behavior with real AWS services and configurations, validate performance characteristics, and test end-to-end workflows. Run cloud tests in staging environments to catch integration issues before they reach production.
Mock external dependencies in local tests to isolate function logic and keep tests fast. Use cloud tests to verify actual integration with external services like databases, APIs, and other AWS services.
Write focused tests that verify one specific behavior. Use descriptive test names that explain what's being tested. Group related tests together and use test fixtures for common setup code. Keep tests simple and avoid complex test logic that's hard to understand.
## Debugging failures
When tests fail, inspect the execution result to understand what went wrong. Check the execution status to see if the function succeeded, failed, or timed out. Read error messages to understand the failure cause.
Inspect individual operation results to find where behavior diverged from expectations. Check step results to see what values were produced. Verify operation ordering to confirm operations executed in the expected sequence. Count operations to ensure the right number of steps, waits, and callbacks were created.
Common issues include non-deterministic code that produces different results on replay, shared state through global variables that breaks during replay, and missing operations due to conditional logic errors. Use standard debuggers and logging to step through function code and track execution flow.
For cloud tests, inspect execution history in CloudWatch Logs to see detailed operation logs. Use tracing to track execution flow across services and identify bottlenecks.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Idempotency
Monitoring durable functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.