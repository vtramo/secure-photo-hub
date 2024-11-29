#!/bin/sh

export VAULT_TOKEN="dev-only-token"
export VAULT_ADDR=http://127.0.0.1:8200

vault secrets enable -path=secure-photo-hub kv-v2

vault kv put -mount=secure-photo-hub keycloak clientId=B clientSecret=A
vault kv get -format=json -mount=secure-photo-hub -field=data keycloak
#{
#  "client-id": "B",
#  "client-secret": "A"
#}

vault policy write secure-photo-hub-keycloak-kv-ro - <<EOF
path "secret/data/secure-photo-hub/keycloak" {
    capabilities = ["read", "list"]
}
EOF
