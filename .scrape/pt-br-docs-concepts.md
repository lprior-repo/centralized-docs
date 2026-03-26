---
url: https://kubernetes.io/pt-br/docs/concepts/
title: Conceitos
word_count: 355
filtered: true
elements_removed: 0
density_score: 0.86
---

## Table of Contents

- [Conceitos](#conceitos)
        - [[Visão Geral](/pt-br/docs/concepts/overview/)](#visão-geralpt-brdocsconceptsoverview)
        - [[Arquitetura do Cluster](/pt-br/docs/concepts/architecture/)](#arquitetura-do-clusterpt-brdocsconceptsarchitecture)
        - [[Contêineres](/pt-br/docs/concepts/containers/)](#contêinerespt-brdocsconceptscontainers)
        - [[Serviços, balanceamento de carga e conectividade](/pt-br/docs/concepts/services-networking/)](#serviços-balanceamento-de-carga-e-conectividadept-brdocsconceptsservices-networking)
        - [[Armazenamento](/pt-br/docs/concepts/storage/)](#armazenamentopt-brdocsconceptsstorage)
        - [[Segurança](/pt-br/docs/concepts/security/)](#segurançapt-brdocsconceptssecurity)
        - [[Políticas](/pt-br/docs/concepts/policy/)](#políticaspt-brdocsconceptspolicy)
        - [[Escalonamento, preempção e remoção](/pt-br/docs/concepts/scheduling-eviction/)](#escalonamento-preempção-e-remoçãopt-brdocsconceptsscheduling-eviction)
        - [[Administração de Cluster](/pt-br/docs/concepts/cluster-administration/)](#administração-de-clusterpt-brdocsconceptscluster-administration)
        - [[Windows no Kubernetes](/pt-br/docs/concepts/windows/)](#windows-no-kubernetespt-brdocsconceptswindows)
        - [[Estendendo o Kubernetes](/pt-br/docs/concepts/extend-kubernetes/)](#estendendo-o-kubernetespt-brdocsconceptsextend-kubernetes)
  - [Comentários](#comentários)

---

# Conceitos
A seção de Conceitos irá te ajudar a aprender mais sobre as partes do ecossistema Kubernetes e as abstrações que o Kubernetes usa para representar seu [cluster](/pt-br/docs/reference/glossary/?all=true#term-cluster).
Ela irá lhe ajudar a obter um entendimento mais profundo sobre como o Kubernetes funciona.
##### [Visão Geral](/pt-br/docs/concepts/overview/)
Kubernetes é um plataforma de código aberto, portável e extensiva para o gerenciamento de cargas de trabalho e serviços distribuídos em contêineres, que facilita tanto a configuração declarativa quanto a automação. Ele possui um ecossistema grande, e de rápido crescimento. Serviços, suporte, e ferramentas para Kubernetes estão amplamente disponíveis.
##### [Arquitetura do Cluster](/pt-br/docs/concepts/architecture/)
Os conceitos arquiteturais por trás do Kubernetes.
##### [Contêineres](/pt-br/docs/concepts/containers/)
Tecnologia para empacotar aplicações com suas dependências em tempo de execução
##### [Serviços, balanceamento de carga e conectividade](/pt-br/docs/concepts/services-networking/)
Conceitos e recursos por trás da conectividade no Kubernetes.
##### [Armazenamento](/pt-br/docs/concepts/storage/)
Formas de fornecer armazenamento temporário e de longa duração a Pods em seu cluster.
##### [Segurança](/pt-br/docs/concepts/security/)
Conceitos para manter suas cargas de trabalho cloud native seguras.
##### [Políticas](/pt-br/docs/concepts/policy/)
Políticas que você pode configurar e que afetam grupos de recursos.
##### [Escalonamento, preempção e remoção](/pt-br/docs/concepts/scheduling-eviction/)
No Kubernetes, escalonamento refere-se à certeza de que os Pods correspondam aos nós para que o kubelet possa executá-los. Preempção é o processo de finalizar Pods com menor prioridade, para que os Pods com maior prioridade possam ser escalonados nos nós. Remoção é o processo de finalização proativa de um ou mais Pods em nós com poucos recursos.
##### [Administração de Cluster](/pt-br/docs/concepts/cluster-administration/)
Detalhes de baixo nível relevantes para criar ou administrar um cluster Kubernetes.
##### [Windows no Kubernetes](/pt-br/docs/concepts/windows/)
O Kubernetes oferece suporte a nós que executam Microsoft Windows.
##### [Estendendo o Kubernetes](/pt-br/docs/concepts/extend-kubernetes/)
Diferentes maneiras de mudar o comportamento do seu cluster do Kubernetes.
## Comentários
Esta página foi útil?
Sim
Não
Obrigado pelo feedback. Se você tiver uma pergunta específica sobre como utilizar o Kubernetes, faça em
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Abra um bug no repositório do GitHub se você deseja
[relatar um problema](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
ou
[sugerir uma melhoria](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Última modificação April 19, 2021 at 9:10 PM PST: [Move portuguese content to pt-br to correct shortcode problems and add redirection (#27413) (ae197690de)](https://github.com/kubernetes/website/commit/ae197690de7fee20f4e5a506da76a4605e105e83)
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)
