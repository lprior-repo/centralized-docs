---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 470
summary: ## Table of Contents  - [Validating Admission Policy](#validating-admission-policy)   - [What is Validating Admission Policy?](#what-is-validating-admission-policy)   - [What Resources Make a...
---

## Table of Contents

- [Validating Admission Policy](#validating-admission-policy)
  - [What is Validating Admission Policy?](#what-is-validating-admission-policy)
  - [What Resources Make a Policy](#what-resources-make-a-policy)
  - [Getting Started with Validating Admission Policy](#getting-started-with-validating-admission-policy)
    - [Creating a ValidatingAdmissionPolicy](#creating-a-validatingadmissionpolicy)
      - [Note:](#note)
      - [Validation actions](#validation-actions)
    - [Parameter resources](#parameter-resources)
      - [Optional parameters](#optional-parameters)
      - [Parameter selector](#parameter-selector)
      - [Authorization checks](#authorization-checks)
      - [`paramRef`](#paramref)
      - [Note:](#note)
      - [Handling Missing Parameters with `parameterNotFoundAction`](#handling-missing-parameters-with-parameternotfoundaction)
    - [Failure Policy](#failure-policy)
    - [Validation Expression](#validation-expression)
    - [Matching requests: `matchConditions`](#matching-requests-matchconditions)
- [other fields](#other-fields)
    - [Message expression](#message-expression)
    - [Type checking](#type-checking)
- [Except for "exempt" deployments, or any containers that do not belong to the "example.com" organization (e.g. common sidecars).](#except-for-exempt-deployments-or-any-containers-that-do-not-belong-to-the-examplecom-organization-eg-common-sidecars)
- [For example, if the namespace has a label of {"environment": "staging"}, all container images must be either staging.example.com/\*](#for-example-if-the-namespace-has-a-label-of-environment-staging-all-container-images-must-be-either-stagingexamplecom)
- [or do not contain "example.com" at all, unless the deployment has {"exempt": "true"} label.](#or-do-not-contain-examplecom-at-all-unless-the-deployment-has-exempt-true-label)
  - [API kinds exempt from admission validation](#api-kinds-exempt-from-admission-validation)
  - [Feedback](#feedback)

---