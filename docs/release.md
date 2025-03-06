# Prepare a release for github and sign it

Generate tarball with

```
  tag=0.7.2
  tarball="../crypto-config_${tag}.orig.tar.gz"
  git tag -m "${tag}" -s "v${tag}"
  git -c tar.tar.gz.command='gzip -cn' archive --format=tar.gz --prefix="crypto-config-${tag}/" -o "${tarball}" "v${tag}"
  gpg --armor --detach-sign "${tarball}"
  gpg --verify "${tarball}.asc"
```

Push the tag.

From the github.com UI, create a release with the .asc file
uploaded
