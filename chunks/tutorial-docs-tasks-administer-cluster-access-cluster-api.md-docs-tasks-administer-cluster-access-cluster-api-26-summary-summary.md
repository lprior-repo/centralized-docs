---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#26-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 117
summary: `package main import ( \"context\" \"fmt\" \"k8s.io/apimachinery/pkg/apis/meta/v1\" \"k8s.io/client-go/kubernetes\" \"k8s.io/client-go/tools/clientcmd\" ) func main() { // uses the current context in...
---

`package main
import (
"context"
"fmt"
"k8s.io/apimachinery/pkg/apis/meta/v1"
"k8s.io/client-go/kubernetes"
"k8s.io/client-go/tools/clientcmd"
)
func main() {
// uses the current context in kubeconfig
// path-to-kubeconfig -- for example, /root/.kube/config
config, \_ := clientcmd.BuildConfigFromFlags("", "&lt;path-to-kubeconfig&gt;")
// creates the clientset
clientset, \_ := kubernetes.NewForConfig(config)
// access the API to list pods