#!/bin/bash

set -e
set -u

CURRENT='/var/lib/crypto-config/profiles/current'

echo 'openssl'
if ! grep -q 'crypto-config' '/etc/ssl/openssl.cnf'; then
  echo ".include ${CURRENT}/openssl.conf.d" >> '/etc/ssl/openssl.cnf'
fi

echo 'gnutls'
# No dropin support for gnutls
ln -sfn "${CURRENT}/gnutls.conf" '/etc/gnutls/config'

echo 'nginx'
sed -i '/ssl_protocols/ d' '/etc/nginx/nginx.conf'
echo "include ${CURRENT}/nginx.conf.d/*.conf;" > '/etc/nginx/conf.d/crypto-config.conf'
