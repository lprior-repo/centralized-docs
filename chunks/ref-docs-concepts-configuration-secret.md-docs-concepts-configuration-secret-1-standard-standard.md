---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 426
summary: - [Secrets](#secrets)       - [Caution:](#caution)   - [Uses for Secrets](#uses-for-secrets)     - [Use case: dotfiles in a secret volume](#use-case-dotfiles-in-a-secret-volume)       -...
---

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