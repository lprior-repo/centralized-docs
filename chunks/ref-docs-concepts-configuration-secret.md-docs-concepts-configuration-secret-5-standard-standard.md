---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#5-standard
chunk_level: standard
chunk_type: prose
heading: Uses for Secrets
token_count: 167
summary: ### Use case: Secret visible to one container in a Pod Consider a program that needs to handle HTTP requests, do some complex business logic, and then sign some messages with an HMAC. Because it has...
---

### Use case: Secret visible to one container in a Pod
Consider a program that needs to handle HTTP requests, do some complex business
logic, and then sign some messages with an HMAC. Because it has complex
application logic, there might be an unnoticed remote file reading exploit in
the server, which could expose the private key to an attacker.
This could be divided into two processes in two containers: a frontend container
which handles user interaction and business logic, but which cannot see the
private key; and a signer container that can see the private key, and responds
to simple signing requests from the frontend (for example, over localhost networking).
With this partitioned approach, an attacker now has to trick the application
server into doing something rather arbitrary, which may be harder than getting
it to read a file.