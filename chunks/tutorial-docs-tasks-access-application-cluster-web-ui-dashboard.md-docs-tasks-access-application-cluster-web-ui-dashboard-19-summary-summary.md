---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#19-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 127
summary: For external Services, you may need to open up one or more ports to do so. Other Services that are only visible from inside the cluster are called internal Services. Irrespective of the Service type,...
---

For external Services, you may need to open up one or more ports to do so.
Other Services that are only visible from inside the cluster are called internal Services.
Irrespective of the Service type, if you choose to create a Service and your container listens
on a port (incoming), you need to specify two ports.
The Service will be created mapping the port (incoming) to the target port seen by the container.
This Service will route to your deployed Pods. Supported protocols are TCP and UDP.
The internal DNS name for this Service will be the value you specified as application name above.
If needed, you can expand the