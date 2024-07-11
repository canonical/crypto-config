#!/bin/bash
#
# Copyright (C) 2023-2024 Canonical, Ltd.
# Author: Adrien Nader <adrien.nader@canonical.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; version 3.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

set -e
set -u

CURRENT='/var/lib/crypto-config/profiles/current'

echo 'openssl'
if ! grep -q 'crypto-config' '/etc/ssl/openssl.cnf'; then
  echo ".include ${CURRENT}/openssl.conf.d" >> '/etc/ssl/openssl.cnf'
fi

echo 'gnutls'
# No drop-in support for gnutls
ln -sfn "${CURRENT}/gnutls.conf" '/etc/gnutls/config'

echo 'nginx'
# XXX report issue below
sed -i '/ssl_protocols/ d' '/etc/nginx/nginx.conf'
echo "include ${CURRENT}/nginx.conf.d/*.conf;" > '/etc/nginx/conf.d/crypto-config.conf'

echo 'apt'
declare -a keys
keys=('assert-pubkey-algo')
for k in "${keys[@]}"; do
  ln -sfn "${CURRENT}/apt.conf.d/${k}.conf" "/etc/apt/apt.conf.d/${k}.conf"
done
