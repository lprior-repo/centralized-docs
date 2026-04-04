---
doc_id: ref/ru-docs-concepts.md/ru-docs-concepts
chunk_id: ref/ru-docs-concepts.md/ru-docs-concepts#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 996
summary: ### Мастер Kubernetes Мастер Kubernetes отвечает за поддержание желаемого состояния для вашего кластера. Когда вы взаимодействуете с Kubernetes, например, используя интерфейс командной строки...
---

### Мастер Kubernetes
Мастер Kubernetes отвечает за поддержание желаемого состояния для вашего кластера. Когда вы взаимодействуете с Kubernetes, например, используя интерфейс командной строки `kubectl`, вы работаете с мастером Kubernetes вашего кластера.
> Под "мастером" понимается совокупность процессов, которые управляют состоянием кластера. Обычно все эти процессы выполняются на одном узле кластера, и поэтому этот узел называется главным (master). Мастер также может быть реплицирован для доступности и резервирования.
### Узлы Kubernetes
Узлы в кластере - это машины (виртуальные машины, физические серверы и т.д.), на которых работают ваши приложения и облачные рабочие процессы. Мастер Kubernetes контролирует каждый узел; вы редко будете взаимодействовать с узлами напрямую.
## Что дальше
Если вы хотите описать концепт, обратитесь к странице
[Использование шаблонов страниц](/docs/home/contribute/page-templates/)
для получения информации о типе страницы и шаблоне концепции.
##### [Рабочие нагрузки](/ru/docs/concepts/workloads/)
Поймите под, самый маленький развертываемый вычислительный объект в Kubernetes, и абстракции более высокого уровня, которые помогут вам их запускать.
##### [Планирование, приоритизация и вытеснение](/ru/docs/concepts/scheduling-eviction/)
В Kubernetes под планированием понимается поиск подходящих узлов, на которых kubelet сможет запустить Pod'ы. Приоритизация — процесс завершения работы Pod'ов с более низким приоритетом и высвобождения места для Pod'ов с более высоким приоритетом. Вытеснение — это проактивное завершение работы одного или нескольких Pod'ов на узлах с дефицитом ресурсов.
##### [Администрирование кластера](/ru/docs/concepts/cluster-administration/)
Lower-level detail relevant to creating or administering a Kubernetes cluster.
## Обратная связь
Эта страница была полезна?
Да
Нет
Спасибо за обратную связь! Если у вас есть конкретный вопрос об использовании Kubernetes, спросите на
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Создайте issue в репозитории GitHub, если вы хотите
[сообщить о проблеме](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
или
[предложить улучшение](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Изменено March 26, 2024 at 1:34 PM PST: [Rename control plane in Russian (b49f023c29)](https://github.com/kubernetes/website/commit/b49f023c298df65337bb97121e541e5b17458990)
## Related Pages

- [Concepts](fr-docs-concepts.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Service ClusterIP allocation](docs-concepts-services-networking-cluster-ip-allocation.md)
- [Managing Service Accounts](docs-reference-access-authn-authz-service-accounts-admin.md)
- [Taints and Tolerations](docs-concepts-scheduling-eviction-taint-and-toleration.md)