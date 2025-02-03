# Crypto-config

A configuration management framework designed to cover system-wide cryptography configuration.

Crypto-config lets users chose their system-wide cryptography configuration
using profiles. Coverage is limited so far but will increase over time.

This repository contains the framework. Profile data is to be stored directly
inside each package. As an exception, during early days, this repository also
contains profile data in order to avoid a chicken-and-egg situation.

## Package support

Every package with **specific** configuration options needs to be modified in
order to ship the relevant configuration files.

Modifying libraries can provide coverage for all their dependants at once and
modifying applications enables finer control.

## Profiles

Profiles are sets of configuration files for the packages enrolled in this
scheme.

## Paths

Packages install profiles in `/usr/share/crypto-config/profiles/<profile-name>`.

A postinst script is triggered by dpkg when this directory is modified and
re-creates the hierarchy in `/var/lib/crypto-config` and implements inheritance
between profiles.
The `debian/crypto-config.postinst` closely implements the specification for
profiles except that it does not use a `metadata.json` file to store the
inheritance data (the file will include more metadata over time).

Toplevel package configuration typically uses `include`-like directives to
read the configurations files stored in `/var/lib/crypto-config` although other
means are possible.

## /usr/bin/crypto-config

The crypto-config binary currently accepts three commands: `get`, `status` and
`switch <profile>`'.

### get
Return the current profile in use.

### status
Show the current profile in use. May show more in the future. This is meant for
interactive use rather than for scriptiing.

### switch \<profile\>
Use this profile.

## Specification

The full specification lives on [discourse.ubuntu.com](https://discourse.ubuntu.com/t/spec-crypto-config-a-framework-to-manage-crypto-related-configurations-system-wide/54265/1) and is also [copied in this repository](docs/crypto-config-specification.md).

## Introduction to crypto-config

Crypto-config is an optional framework for managing the configuration of
cryptography in software in a distribution as consistent profiles. It being
optional is important for ease of inclusion as it does not require migrating
everything at once and ensures that systems would not become unusable if
something wrong happened.

### A practical example

Imagine two profiles: `default` and `future`. The future one may disable
TLS < 1.3 and RSA < 4096.

Usage is simply

    crypto-config switch future

Then, restart e.g. `nginx` and use `sslscan` to check its configuration:

    systemctl restart nginx
    sslscan 127.0.0.1 | grep 'TLS.*abled'

This would show:

    TLSv1.0   disabled
    TLSv1.1   disabled
    TLSv1.2   disabled
    TLSv1.3   enabled

### What profiles are

Crypto-config uses so-called profiles which are sets of drop-ins and
configuration fragments, for each software to configure specifically.
Crypto-config doesn't impose constraints on the file layout by itself. If some
software understands drop-ins directories, use that; if it only understands
including a file directly, use that; if it doesn't support any of that, fix it
(this is luckily a very uncommon situation nowadays).

There is no general mandate for profiles. It is expected that distributions
come up with a list of profiles and the intent for each of them. Typical
examples include `default`, `legacy` and `future`:

- `default` would match what is already being done at the moment,
- `legacy` tweaks `default` to increase compatibility with legacy systems,
- `future` tweaks `default` to increase security and uses settings that are
  expected to become the default ones in the future.

A core goal is to have various software configured in a consistent and
meaningful way. For instance, a profile that disables a protocol for openssl
should disable it for gnutls too. However, it is also understandable that an
algorithm is disabled for HTTPS servers but enabled for SMTP ones because they
live in two different ecosystems.

### How profiles are stored and selected

The profiles are stored in `/var/lib/crypto-config/profiles`.

In this directory there is also a symlink called `current` which points to the
chosen profile. The profile switch is the atomic change of the symlink target.

### Inheritance and generation of runtime data

It's possible to skip the configuration for some software when creating a
profile: it will be inherited from the profile's parent through an inheritance
mechanism.

Packages install profile data under `/usr/share/crypto-config/profiles/`.

After installation, a short and idempotent program resolves fills in the
skipped data using based on the inheritance relationship. The result is then
stored in `/var/lib/crypto-config/profiles` and is ready to be used by
software.

### Making software read from `crypto-config` profiles

There are dozens, if not hundreds, of different configuration systems. There is
therefore no single way to make software read part of its configuration from
`crypto-config`: it depends on each software.

Luckily, nowadays many pieces of software handle `include`-style directives and
drop-ins.

As an example, for openssl, the following has been added at the end of
`/etc/ssl/openssl.cnf`:

    .include /var/lib/crypto-config/profiles/current/openssl.conf.d

Of course, this will have an actual effect when there is something in that
directory.

It's more convenient to not fail when the target directory or file does not
exist. For instance, enabling `crypto-config` for nginx uses
`/etc/nginx/conf.d/crypto-config.conf` which contains the following:

    include /var/lib/crypto-config/profiles/current/nginx.conf.d/*.conf;

Nginx refuses to start if it is instructed to `include` a non-existing file for
its configuration. However, if there is a wildcard in the path, it is fine that
there is no matching file. Therefore it is probably better practice to ensure
there is one.

These small changes to configuration are to be done by package maintainers and
as they should make the best judgement. Similarly, packagers provide
configuration snippets for the various profiles that it makes sense for them to
support directly.

### Compared to `alternatives`

While `alternatives` and `crypto-config` use a symlink to enable administrators
to select one element among others, they have been designed for different use
cases and at different times.

All features besides changing the target of a symlink are not used for
crypto-config. This is a good indication that the two have more differences
that commonalities.

I am not aware of any use of `alternatives` for configuration, only for
« determining default commands » (as the manpage of `update-alternatives`
says).

Finally, `crypto-config` stores data under `/var/lib` rather than under `/etc`
because there is data generated (hence it should be under `/var/lib`) and this
also avoids hindering progress on empty `/etc` directories.

## Bug reports
Please file bug reports in [Launchpad](https://bugs.launchpad.net/crypto-config).

## License
Crypto-config is licensed under the [GPLv3](COPYING).
