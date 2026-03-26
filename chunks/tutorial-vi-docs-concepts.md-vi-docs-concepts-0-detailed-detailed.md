---
doc_id: tutorial/vi-docs-concepts.md/vi-docs-concepts
chunk_id: tutorial/vi-docs-concepts.md/vi-docs-concepts#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 1015
summary: ## Table of Contents  - [Các khái niệm](#các-khái-niệm)         - [[Service](/vi/docs/concepts/services-networking/service/)](#servicevidocsconceptsservices-networkingservice)         - [[IPv4/IPv6...
---

## Table of Contents

- [Các khái niệm](#các-khái-niệm)
        - [[Service](/vi/docs/concepts/services-networking/service/)](#servicevidocsconceptsservices-networkingservice)
        - [[IPv4/IPv6 dual-stack](/vi/docs/concepts/services-networking/dual-stack/)](#ipv4ipv6-dual-stackvidocsconceptsservices-networkingdual-stack)
        - [[Mở rộng Kubernetes](/vi/docs/concepts/extend-kubernetes/)](#mở-rộng-kubernetesvidocsconceptsextend-kubernetes)
        - [[ReplicaSet](/vi/docs/concepts/workloads/controllers/replicaset/)](#replicasetvidocsconceptsworkloadscontrollersreplicaset)
  - [Phản hồi](#phản-hồi)

---

# Các khái niệm
Phần Khái niệm giúp bạn tìm hiểu về các bộ phận của hệ thống Kubernetes và các khái niệm mà Kubernetes sử dụng để biểu diễn [cụm cluster](/vi/docs/reference/glossary/?all=true#term-cluster) của bạn, đồng thời giúp bạn hiểu sâu hơn về cách thức hoạt động của Kubernetes.
##### [Service](/vi/docs/concepts/services-networking/service/)
Hiển thị một ứng dụng đang chạy trong cluster của bạn dưới cùng một endpoint hướng ra bên ngoài, ngay cả khi workload được chia thành nhiều backend.
##### [IPv4/IPv6 dual-stack](/vi/docs/concepts/services-networking/dual-stack/)
Kubernetes cho phép bạn cấu hình mạng IPv4 đơn, mạng IPv6 đơn, hoặc mạng kép với cả hai họ mạng hoạt động. Trang này giải thích cách thức hoạt động.
##### [Mở rộng Kubernetes](/vi/docs/concepts/extend-kubernetes/)
Các cách khác nhau để thay đổi hành vi của cluster Kubernetes của bạn.
##### [ReplicaSet](/vi/docs/concepts/workloads/controllers/replicaset/)
Mục đích của ReplicaSet là duy trì một tập hợp ổn định các Pod bản sao đang chạy tại bất kỳ thời điểm nào. Thông thường, bạn định nghĩa một Deployment và để Deployment đó quản lý các ReplicaSet tự động.
## Phản hồi
Trang này có hữu ích không?
Có
Không
Cảm ơn bạn đã phản hồi. Nếu bạn có một câu hỏi cụ thể và có thể trả lời về cách sử dụng Kubernetes, hãy đặt nó trên
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Mở một vấn đề trong [Kho GitHub](https://www.github.com/kubernetes/website/) nếu bạn muốn
[báo cáo một vấn đề](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
hoặc
[đề xuất một cải tiến](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified March 27, 2025 at 6:42 PM PST: [Reconstruct vietnamese docs (adeef47157)](https://github.com/kubernetes/website/commit/adeef471571b9ab53c33f722d80baaf9bc7de369)
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)