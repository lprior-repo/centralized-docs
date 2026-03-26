---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#55-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 128
summary: A discovery url that is exposed using kubernetes service 'oidc' in namespace 'oidc-namespace' and discovery information is available at '/.well-known/openid-configuration'. discoveryURL:...
---

A discovery url that is exposed using kubernetes service 'oidc' in namespace 'oidc-namespace'
and discovery information is available at '/.well-known/openid-configuration'.
discoveryURL: "https://oidc.oidc-namespace/.well-known/openid-configuration"
certificateAuthority is used to verify the TLS connection and the hostname on the leaf certificate
must be set to 'oidc.oidc-namespace'.
curl https://oidc.oidc-namespace/.well-known/openid-configuration (.discoveryURL field)
{
issuer: "https://oidc.example.com" (.url field)
}