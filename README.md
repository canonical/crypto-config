# Crypto-config

A configuration management framework designed to cover system-wide cryptography configuration.

Crypto-config lets users chose their system-wide cryptography configuration
using profiles. Coverage is limited so far but will increase over time.

This repository contains the framework. Profile data is to be stored directly
inside each package. As an exception, during early days, this repository also
contains profile data in order to avoid a chicken-and-egg situation.

## Package support

Every package with specific configuration options needs to be modified in order
to ship the relevant configuration files.

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

## Bug reports
Please file bug reports in [Launchpad](https://bugs.launchpad.net/crypto-config).

## License
Crypto-config is licensed under the [GPLv3](COPYING).
