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

# It seems that exim listens on interfaces which are identified by their IP
# rather than by their name...
lxc exec cc-test -- sh -c 'sed -i "/dc_local_interfaces/ s/;/; $(ip -4 --json a s dev eth0  | jq -r ".[0].addr_info[0].local");/" /etc/exim4/update-exim4.conf.conf'
lxc exec cc-test systemctl restart exim4
sslscan --starttls-smtp --sleep=60 "$ipv4"
