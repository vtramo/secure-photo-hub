#!/bin/sh

# This command starts Keycloak exposed on the local port 8080 and creates
# an initial admin user with the username admin and password admin.
docker run \
  -p 8080:8080 \
  -e KC_BOOTSTRAP_ADMIN_USERNAME=admin \
  -e KC_BOOTSTRAP_ADMIN_PASSWORD=admin \
  -v /home/vincenzo/projects/rust-oauth2-openid-keycloak/keycloak/policies/jar/only-resource-owner-policy.jar:/opt/keycloak/providers/only-resource-owner-policy.jar \
  -v /home/vincenzo/projects/rust-oauth2-openid-keycloak/keycloak/import/secure-photo-hub-realm.json:/opt/keycloak/data/import/secure-photo-hub-realm.json \
  -it quay.io/keycloak/keycloak:26.0.5 start-dev --import-realm --verbose

