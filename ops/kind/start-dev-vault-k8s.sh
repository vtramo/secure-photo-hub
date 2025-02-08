#!/bin/bash

# Get Service Account Secret Name (vault-auth-secret)
SA_SECRET_NAME=$(kubectl get secrets --output=json \
    | jq -r '.items[].metadata | select(.name|startswith("vault-auth-")).name')

# Service Account JWT Token
SA_JWT_TOKEN=$(kubectl get secret "$SA_SECRET_NAME" \
  --output 'go-template={{ .data.token }}' | base64 --decode)

#{
#  "iss": "kubernetes/serviceaccount",
#  "kubernetes.io/serviceaccount/namespace": "default",
#  "kubernetes.io/serviceaccount/secret.name": "vault-auth-secret",
#  "kubernetes.io/serviceaccount/service-account.name": "vault-auth",
#  "kubernetes.io/serviceaccount/service-account.uid": "decd3b3f-b1ce-4489-aeee-a9ef84bbd46b",
#  "sub": "system:serviceaccount:default:vault-auth"
#}

# PEM encoded CA cert used to talk to Kubernetes API
SA_CA_CRT=$(kubectl config view --raw --minify --flatten --output 'jsonpath={.clusters[].cluster.certificate-authority-data}' | base64 --decode)

# K8S cluster ip
#K8S_PORT=$(kubectl config view --raw --minify --flatten --output 'jsonpath={.clusters[].cluster.server}' | awk -F: '{print $NF}')
K8S_PORT=8001
K8S_HOST="http://host.docker.internal:${K8S_PORT}"

# Start Vault Server
docker rm -f vault
CONTAINER_ID=$(docker run -p 8200:8200 -d \
  -e 'VAULT_DEV_ROOT_TOKEN_ID=dev-only-token' \
  -e 'VAULT_ADDR=http://127.0.0.1:8200' \
  -e 'VAULT_TOKEN=dev-only-token' \
  -e "SA_SECRET_NAME=${SA_SECRET_NAME}" \
  -e "SA_JWT_TOKEN=${SA_JWT_TOKEN}" \
  -e "SA_CA_CRT=${SA_CA_CRT}" \
  -e "K8S_HOST=${K8S_HOST}" \
  --add-host=host.docker.internal:host-gateway \
  --name vault \
  --health-cmd "wget --spider --proxy off http://127.0.0.1:8200/v1/sys/health?standbyok=true" \
  --health-timeout 3s \
  --health-start-period 1s \
  --health-retries 3 \
  --health-interval 1s \
   hashicorp/vault)

CONTAINER_STATUS=$(docker inspect --format "{{json .State.Health.Status }}" "$CONTAINER_ID")
until [[ "$CONTAINER_STATUS" == '"healthy"' ]]
do
  echo "Waiting for container to start..."
  sleep 1
  CONTAINER_STATUS=$(docker inspect --format "{{json .State.Health.Status }}" "$CONTAINER_ID")
done

export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=dev-only-token

vault secrets enable -path=secure-photo-hub kv-v2
vault kv put -mount=secure-photo-hub application-secrets yaml=@application-secrets.yaml
vault kv get -mount=secure-photo-hub application-secrets

vault policy write secure-photo-hub-kv-ro - <<EOF
path "secure-photo-hub/data/application-secrets" {
    capabilities = ["read", "list"]
}
EOF

vault auth enable kubernetes
vault write auth/kubernetes/config \
     token_reviewer_jwt="$SA_JWT_TOKEN" \
     kubernetes_host="$K8S_HOST" \
     kubernetes_ca_cert="$SA_CA_CRT" \
     issuer="https://kubernetes.default.svc.cluster.local"
vault write auth/kubernetes/role/secure-photo-hub-role \
     bound_service_account_names=vault-auth \
     bound_service_account_namespaces=default \
     token_policies=secure-photo-hub-kv-ro \
     ttl=24h

kubectl proxy --address='0.0.0.0' --port=8001 --accept-hosts='^*$'