---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 759
summary: ## Table of Contents  - [Secrets](#secrets)       - [Caution:](#caution)   - [Uses for Secrets](#uses-for-secrets)     - [Use case: dotfiles in a secret volume](#use-case-dotfiles-in-a-secret-volume)...
---

## Table of Contents

- [Secrets](#secrets)
      - [Caution:](#caution)
  - [Uses for Secrets](#uses-for-secrets)
    - [Use case: dotfiles in a secret volume](#use-case-dotfiles-in-a-secret-volume)
      - [Note:](#note)
    - [Use case: Secret visible to one container in a Pod](#use-case-secret-visible-to-one-container-in-a-pod)
    - [Alternatives to Secrets](#alternatives-to-secrets)
  - [Types of Secret](#types-of-secret)
    - [Opaque Secrets](#opaque-secrets)
    - [ServiceAccount token Secrets](#serviceaccount-token-secrets)
      - [Note:](#note)
    - [Docker config Secrets](#docker-config-secrets)
      - [Note:](#note)
      - [Caution:](#caution)
    - [Basic authentication Secret](#basic-authentication-secret)
      - [Note:](#note)
    - [SSH authentication Secrets](#ssh-authentication-secrets)
- [the data is abbreviated in this example](#the-data-is-abbreviated-in-this-example)
      - [Caution:](#caution)
    - [TLS Secrets](#tls-secrets)
- [values are base64 encoded, which obscures them but does NOT provide](#values-are-base64-encoded-which-obscures-them-but-does-not-provide)
- [Replace the following values with your own base64-encoded certificate and key.](#replace-the-following-values-with-your-own-base64-encoded-certificate-and-key)
    - [Bootstrap token Secrets](#bootstrap-token-secrets)
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