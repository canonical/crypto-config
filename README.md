# Crypto-config

A configuration management framework to manage the configuration of
cryptography on a system by using system-wide profiles. It is is gradually
being rolled out in Ubuntu.

This repository contains the framework. Profile data is to be stored directly
inside each package. As an exception during early days, this repository may
also contain profile data in order to avoid a chicken-and-egg situation.

# Table of contents

- [Example](#example)
- [Sources of the configuration with `crypto-config`](#sources-of-the-configuration-with-crypto-config)
- [What profiles are](#what-profiles-are)
- [Usage of /usr/bin/crypto-config](#usage-of-usrbincrypto-config)
  - [get](#get)
  - [status](#status)
  - [switch \<profile\>](#switch-profile)
- [Implementation and porting](#implementation-and-porting)
  - [Profile storage and switch](#profile-storage-and-switch)
  - [Inheritance and generation of runtime data](#inheritance-and-generation-of-runtime-data)
  - [Making software read from `crypto-config` profiles](#making-software-read-from-crypto-config-profiles)
- [Specification](#specification)
- [Bug reports](#bug-reports)
- [License](#license)

## Example

Consider two profiles: `default` and `future`. The `future` one disables
TLS < 1.3 and RSA < 4096.

Configure an `nginx` server with TLS and use `sslscan` to check its
configuration; the output will contain the following:

    # sslscan 127.0.0.1
    ...
    TLSv1.2   enabled
    TLSv1.3   enabled

Switch to the `future` profile from `crypto-config:

    crypto-config switch future

Restart `nginx` and use `sslscan` again.

    systemctl restart nginx
    sslscan 127.0.0.1
    ...
    TLSv1.2   disabled
    TLSv1.3   enabled

## Sources of the configuration with `crypto-config`

There are two ways a program such as `nginx` is configured in practice. The
paths below refer to the files used with `crypto-config` specifically.

First, through its own configuration:

    nginx
    ⬇️
    /etc/nginx/nginx.conf
    ⬇️
    /etc/nginx/conf.d/crypto-config.conf
    ⬇️
    /var/lib/crypto-config/profiles/current/nginx.conf.d/*.conf
    ⬇️
    /var/lib/crypto-config/profiles/current/nginx.conf.d/ssl.conf

But some of its features are also impacted by the configuration of its
dependencies:

    nginx
    ⬇️
    /etc/ssl/openssl.cnf
    ⬇️
    /var/lib/crypto-config/profiles/current/openssl.conf.d
    ⬇️
    /var/lib/crypto-config/profiles/current/openssl.conf.d/seclevel.cnf

## What profiles are

Crypto-config profiles are made of drop-in files and configuration fragments.
You can think of a profile as a subset of configuration files on your system,
and choosing a profile as atomically switching these to alternative ones.

Distributions should come up with a list of profiles and the intent for each of
them. Typical examples include `default`, `legacy` and `future`:

- `default` would match what is already being done at the moment,
- `legacy` tweaks `default` to increase compatibility with legacy systems,
- `future` tweaks `default` to increase security and uses settings that are
  expected to become the default ones in the future.

The core goal is to have various software configured in a consistent and
meaningful way. For instance, a profile that disables a protocol for openssl
should disable it for gnutls too. However, it is also understandable that an
algorithm is disabled for HTTPS servers but enabled for SMTP ones because they
live in two different ecosystems.

## Usage of /usr/bin/crypto-config

The crypto-config binary currently accepts three commands: `get`, `status` and
`switch <profile>`'.

### get
Return the current profile in use.

### status
Show the current profile in use. May show more in the future. This is meant for
interactive use rather than for scriptiing.

### switch \<profile\>
Use this profile.

## Implementation and porting

### Profile storage and switch

The profiles are stored in `/var/lib/crypto-config/profiles`.

In this directory there is also a symlink called `current` which points to the
chosen profile. The profile switch is the atomic change of the symlink target.

### Inheritance and generation of runtime data

It is possible to skip the configuration for some software when creating a
profile: it will be inherited from the profile's parent through an inheritance
mechanism.

Packages install profile data under `/usr/share/crypto-config/profiles/`.

After installation, a short (and idempotent) program fills in the skipped data
using based on the inheritance relationship. The result is then stored in
`/var/lib/crypto-config/profiles` and is ready to be used by software.

### Making software read from `crypto-config` profiles

There are dozens, if not hundreds, of different configuration systems. There is
therefore no single way to make software read part of its configuration from
`crypto-config`: it depends on each software.

Luckily, nowadays many pieces of software handle `include`-style directives and
drop-ins.

As an example, for openssl, the following has been added at the end of
`/etc/ssl/openssl.cnf`:

    .include /var/lib/crypto-config/profiles/current/openssl.conf.d

Of course, this will only have an actual effect when there is something in that
directory.

It's more convenient to not fail when the target directory or file does not
exist. For instance, enabling `crypto-config` for nginx uses
`/etc/nginx/conf.d/crypto-config.conf` which contains the following:

    include /var/lib/crypto-config/profiles/current/nginx.conf.d/*.conf;

Nginx refuses to start if it is instructed to `include` a non-existing file for
its configuration. However, if there is a wildcard in the path, it accepts that
there is no matching file.

These small changes to configuration are to be done by package maintainers and
as they should make the best judgement. Similarly, packagers provide
configuration snippets for the various profiles that it makes sense for them to
support directly.

## Specification

The full specification lives on [discourse.ubuntu.com](https://discourse.ubuntu.com/t/spec-crypto-config-a-framework-to-manage-crypto-related-configurations-system-wide/54265/1) and is also [copied in this repository](docs/crypto-config-specification.md).

## Bug reports
Please file bug reports in [Launchpad](https://bugs.launchpad.net/crypto-config).

## License
Crypto-config is licensed under the [GPLv3](COPYING).
