---
doc_id: tutorial/vi-docs-concepts.md/vi-docs-concepts
chunk_id: tutorial/vi-docs-concepts.md/vi-docs-concepts#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 432
summary: # Các khái niệm Phần Khái niệm giúp bạn tìm hiểu về các bộ phận của hệ thống Kubernetes và các khái niệm mà Kubernetes sử dụng để biểu diễn [cụm...
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