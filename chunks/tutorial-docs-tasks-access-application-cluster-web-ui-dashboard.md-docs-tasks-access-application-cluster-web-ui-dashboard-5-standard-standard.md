---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#5-standard
chunk_level: standard
chunk_type: prose
heading: Deploying containerized applications
token_count: 279
summary: #### Note: For external Services, you may need to open up one or more ports to do so. Other Services that are only visible from inside the cluster are called internal Services. Irrespective of the...
---

#### Note:
For external Services, you may need to open up one or more ports to do so.
Other Services that are only visible from inside the cluster are called internal Services.
Irrespective of the Service type, if you choose to create a Service and your container listens
on a port (incoming), you need to specify two ports.
The Service will be created mapping the port (incoming) to the target port seen by the container.
This Service will route to your deployed Pods. Supported protocols are TCP and UDP.
The internal DNS name for this Service will be the value you specified as application name above.
If needed, you can expand the **Advanced options** section where you can specify more settings:
* **Description**: The text you enter here will be added as an
[annotation](/docs/concepts/overview/working-with-objects/annotations/)
to the Deployment and displayed in the application's details.
* **Labels**: Default [labels](/docs/concepts/overview/working-with-objects/labels/) to be used
for your application are application name and version.
You can specify additional labels to be applied to the Deployment, Service (if any), and Pods,
such as release, environment, tier, partition, and release track.
Example:
```
`release=1.0
tier=frontend
environment=pod
track=stable
`
```