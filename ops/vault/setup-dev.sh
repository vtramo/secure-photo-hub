#!/bin/sh

export VAULT_TOKEN="dev-only-token"
export VAULT_ADDR=http://127.0.0.1:8200

vault secrets enable -path=fast-photo-hub kv-v2

vault kv put -mount=fast-photo-hub keycloak clientId=B clientSecret=A
vault kv get -format=json -mount=fast-photo-hub -field=data keycloak
#{
#  "client-id": "B",
#  "client-secret": "A"
#}

vault policy write fast-photo-hub-keycloak-kv-ro - <<EOF
path "secret/data/fast-photo-hub/keycloak" {
    capabilities = ["read", "list"]
}
EOF
