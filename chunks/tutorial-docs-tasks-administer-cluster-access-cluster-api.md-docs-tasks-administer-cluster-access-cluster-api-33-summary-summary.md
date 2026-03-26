---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#33-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 119
summary: `package io.kubernetes.client.examples; import io.kubernetes.client.ApiClient; import io.kubernetes.client.ApiException; import io.kubernetes.client.Configuration; import...
---

`package io.kubernetes.client.examples;
import io.kubernetes.client.ApiClient;
import io.kubernetes.client.ApiException;
import io.kubernetes.client.Configuration;
import io.kubernetes.client.apis.CoreV1Api;
import io.kubernetes.client.models.V1Pod;
import io.kubernetes.client.models.V1PodList;
import io.kubernetes.client.util.ClientBuilder;
import io.kubernetes.client.util.KubeConfig;
import java.io.FileReader;
import java.io.IOException;
/\*\*
\* A simple example of how to use the Java API from an application outside a kubernetes cluster
\*