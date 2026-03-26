---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 399
summary: - [A bootstrap token Secret usually resides in the kube-system namespace](#a-bootstrap-token-secret-usually-resides-in-the-kube-system-namespace) - [This token ID is used in the...
---

- [A bootstrap token Secret usually resides in the kube-system namespace](#a-bootstrap-token-secret-usually-resides-in-the-kube-system-namespace)
- [This token ID is used in the name](#this-token-id-is-used-in-the-name)
- [This token can be used for authentication](#this-token-can-be-used-for-authentication)
- [and it can be used for signing](#and-it-can-be-used-for-signing)
      - [Note:](#note)
    - [Creating a Secret](#creating-a-secret)
      - [Size limit](#size-limit)
    - [Editing a Secret](#editing-a-secret)
    - [Using a Secret](#using-a-secret)
      - [Optional Secrets](#optional-secrets)
    - [Using Secrets as files from a Pod](#using-secrets-as-files-from-a-pod)
      - [Note:](#note)
    - [Using Secrets as environment variables](#using-secrets-as-environment-variables)
    - [Container image pull Secrets](#container-image-pull-secrets)
      - [Using imagePullSecrets](#using-imagepullsecrets)
        - [Manually specifying an imagePullSecret](#manually-specifying-an-imagepullsecret)
        - [Arranging for imagePullSecrets to be automatically attached](#arranging-for-imagepullsecrets-to-be-automatically-attached)
    - [Using Secrets with static Pods](#using-secrets-with-static-pods)
  - [Immutable Secrets](#immutable-secrets)
      - [Note:](#note)
  - [Information security for Secrets](#information-security-for-secrets)
    - [Configure least-privilege access to Secrets](#configure-least-privilege-access-to-secrets)
      - [Warning:](#warning)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---