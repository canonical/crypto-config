# Changelog

## [0.7.5] - UNRELEASED

### Fixed

- Correct command-line switch in manpage
- Versions in `CHANGELOG.md`

## [0.7.4] - 2025-03-06

### Updated/added

- Add `doc/lxd-cloud-config` to set up a container which immediately has the
  right profile applied
- Provide an example profile for `exim`

### Removed

- Drop `crypto-config-hijack.sh` which is less needed now that changes have
  made their way in `openssl` and `gnutls`

### Fixed

- Dpkg trigger needs to be set on `/usr/share/crypto-config/profiles`, not
  anywhere in `/etc/`; moreover the trigger must not be made crucial
- `dh_install` dropped `profiles/default/openssl.conf.d` which was empty and
  therefore made `crypto-config` not consider anything openssl
- Bad parsing of the parent profile with `metadata.json`
- Manpage was missing

### Contributors and reviewers

Thanks @waveform and @utkarsh2102

## [0.7.3] - 2025-02-21

_Initial release._

[0.7.5]: https://github.com/canonical/crypto-config/releases/tag/v0.7.5
[0.7.4]: https://github.com/canonical/crypto-config/releases/tag/v0.7.4
[0.7.3]: https://github.com/canonical/crypto-config/releases/tag/v0.7.3
