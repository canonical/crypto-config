#!/bin/bash

set -e
set -u

crypto_config_profile="${1:-default}"

echo 'Clean previous data'
lxc delete -f cc-test 2>/dev/null || true
lxc profile create cc-test 2>/dev/null || true

echo "Set crypto-config profile in cloud-init's user-data.yaml"
sed "s/{crypto_config_profile}/${crypto_config_profile}/" user-data.yaml \
  | lxc profile set cc-test user.user-data -

echo 'Start a container using the cloud-init profile'
lxc launch ubuntu-daily:plucky cc-test -p default -p cc-test

echo 'Wait for the container to have networking'
ipv4=''
while ! [[ "$ipv4" =~ 172* ]]; do
  ipv4="$(lxc list --format json cc-test  | jq -r '.[0].state.network.eth0.addresses[0].address')"
  sleep 1
done

echo 'Wait for the web server to serve requests'
while ! curl -s -k "https://$ipv4" >/dev/null; do sleep 1; done

sslscan "$ipv4"
