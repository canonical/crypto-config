cat << EOF

**************************
*    PROFILES STATUS     *
**************************

EOF

for profile_dir in "${DESTDIR}${DISTRO_PROFILES}" "${DESTDIR}${USER_PROFILES}"; do

  for profile in $(dirs_in "${profile_dir}"); do

    profile="$(basename "${profile}")"

    for app in ${apps}; do

      if is_removed "${app}"; then
        continue
      fi

      link="${profile_dir}/${profile}/${app}"

      if target="$(realpath --relative-to="${DESTDIR}" "${link}" 2>/dev/null)"; then

        target_canonical="$(realpath --canonicalize-existing --relative-to="${DESTDIR}" "${link}")"

        if [ "${target}" = "${target_canonical}" ]; then
          echo "${profile_dir/${DESTDIR}}:${profile}:${app} => ${target}"
        else
          echo "${profile_dir/${DESTDIR}}:${profile}:${app} => ${target} => ${target_canonical}"
        fi

      else

        echo ${profile_dir}:"${profile}:${app} absent"

      fi

    done

  done

done | sort | uniq | column -t

cat << EOF

**************************
*     PROFILE TREES      *
**************************

EOF

(
if [ -n "${DESTDIR}" ]; then
  cd "${DESTDIR}"
fi
  find "${DISTRO_PROFILES}" "${USER_PROFILES}" \( -type f -name parent \) -o  \( -type l -o -type f \) -exec ls --color -oh {} +
)

