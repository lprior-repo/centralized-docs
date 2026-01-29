---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-python-runtime.html
title: Python runtime for Lambda Managed Instances
word_count: 781
filtered: true
elements_removed: 0
density_score: 0.89
---

Python runtime for Lambda Managed Instances - AWS Lambda
Python runtime for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-python-runtime)
[Concurrency configuration](#lambda-managed-instances-python-concurrency-config)[Building functions for multi-concurrency](#lambda-managed-instances-python-building)[Shared /tmp directory](#lambda-managed-instances-python-shared-tmp)[Logging](#lambda-managed-instances-python-logging)[Request context](#lambda-managed-instances-python-request-context)[Initialization and shutdown](#lambda-managed-instances-python-init-shutdown)[Dependency versions](#lambda-managed-instances-python-dependencies)[Powertools for AWS Lambda (Python)](#lambda-managed-instances-python-powertools)[Next steps](#lambda-managed-instances-python-next-steps)
# Python runtime for Lambda Managed Instances
The Lambda runtime uses multiple Python processes to handle concurrent requests. Each concurrent request runs in a separate process with its own memory space and initialization. Each process handles one request at a time, synchronously. Processes don't share memory directly, so global variables, module-level caches, and singleton objects are isolated between concurrent requests.
## Concurrency configuration
The maximum number of concurrent requests which Lambda sends to each execution environment is controlled by the `PerExecutionEnvironmentMaxConcurrency` setting in the function configuration. This is an optional setting, and the default value varies depending on the runtime. For Python runtimes, the default is 16 concurrent requests per vCPU, or you can configure your own value. This value also determines the number of processes used by the Python runtime. Lambda automatically adjusts the number of concurrent requests up to the configured maximum based on the capacity of each execution environment to absorb those requests.
###### Important
Using process-based concurrency means each runtime worker process performs its own initialization. Total memory usage equals the per-process memory multiplied by the number of concurrent processes. If you are loading large libraries or data sets and have high concurrency, you will have a large memory footprint. According to your workload, you may need to tune your CPU-to-memory ratio or use a lower concurrency setting to avoid exceeding the available memory. You can use the `MemoryUtilization` metric in CloudWatch to track memory consumption.
## Building functions for multi-concurrency
Due to the process-based multi-concurrency model, Lambda Managed Instances functions using Python runtimes do not access in-memory resources concurrently from multiple invokes. You do not need to apply coding practices for in-memory concurrency safety.
## Shared /tmp directory
The `/tmp` directory is shared across all concurrent requests in the execution environment. Concurrent writes to the same file can cause data corruption, for example if another process overwrites the file. To address this, either implement file locking for shared files or use unique file names per process or per request to avoid conflicts. Remember to clean up unneeded files to avoid exhausting the available space.
## Logging
Log interleaving (log entries from different requests being interleaved in logs) is normal in multi-concurrent systems.
Functions using Lambda Managed Instances always use the structured JSON log format introduced with [advanced logging controls](./monitoring-logs.html#monitoring-cloudwatchlogs-advanced). This format includes the `requestId`, allowing log entries to be correlated to a single request. When you use the `logging` module from the Python standard library in Lambda, the `requestId` is automatically included in each log entry. For further information, see [Using Lambda advanced logging controls with Python](https://docs.aws.amazon.com/lambda/latest/dg/python-logging.html#python-logging-advanced).
## Request context
Use `context.aws\_request\_id` to access to the request ID for the current request.
With Python runtimes, you can use the `\_X\_AMZN\_TRACE\_ID` environment variable to access the X-Ray trace ID with Lambda Managed Instances. The X-Ray trace ID is propagated automatically when using the AWS SDK.
Use `context.get\_remaining\_time\_in\_millis()` to detect timeouts. See [Error handling and recovery](./lambda-managed-instances-execution-environment.html#lambda-managed-instances-error-handling) for more information.
## Initialization and shutdown
Function initialization occurs once per process. You may see repeat log entries if your function emits logs during initialization.
For Lambda functions with extensions, the execution environment emits a SIGTERM signal during shut down. This signal is used by extensions to trigger clean up tasks, such as flushing buffers. You can subscribe to SIGTERM events to trigger function clean-up tasks, such as closing database connections. To learn more about the execution environment lifecycle, see [Understanding the Lambda execution environment lifecycle](./lambda-runtime-environment.html).
## Dependency versions
Lambda Managed Instances requires the following minimum package versions:
* Powertools for AWS Lambda (Python): version 3.23.0 or later
## Powertools for AWS Lambda (Python)
Powertools for AWS Lambda (Python) is compatible with Lambda Managed Instances and provides utilities for logging, tracing, metrics, and more. For more information, see [Powertools for AWS Lambda (Python)](https://github.com/aws-powertools/powertools-lambda-python).
## Next steps
* Review [Java runtime for Lambda Managed Instances](./lambda-managed-instances-java-runtime.html)
* Review [Node.js runtime for Lambda Managed Instances](./lambda-managed-instances-nodejs-runtime.html)
* Review [.NET runtime for Lambda Managed Instances](./lambda-managed-instances-dotnet-runtime.html)
* Learn about [scaling Lambda Managed Instances](./lambda-managed-instances-scaling.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Node.js runtime
.NET runtime
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.