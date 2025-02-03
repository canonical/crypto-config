NB: the best way to make the display wider to display diagrams better is Firefox' reading mode; hopefully svgbob will soon render ascii art diagrams below

| Index | FO104 |
| :---: | :---- |
| **Title** | crypto-config \- a framework to manage crypto-related configurations system-wide |
| **[Status](https://docs.google.com/document/d/1lStJjBGW7lyojgBhxGLUNnliUocYWjAZ1VEbbVduX54/edit?usp=sharing)** | Approved |
| **Authors** | [Adrien Nader](mailto:adrien.nader@canonical.com) |
| **[Type](https://docs.google.com/document/d/1lStJjBGW7lyojgBhxGLUNnliUocYWjAZ1VEbbVduX54/edit?usp=sharing)** | Standard |
| **Created** | 2022-11-17 |

| Reviewer | Status |
| ----- | ----- |
| [Andreas Hasenack](mailto:andreas.hasenack@canonical.com) | Approved Sep 19, 2024 |
| [Michael Hudson-Doyle](mailto:michael.hudson@canonical.com) | Approved Aug 16, 2024 |
| [Seth Arnold](mailto:seth.arnold@canonical.com) | Approved Oct 15, 2024 |
| [Tobias Heider](mailto:tobias.heider@canonical.com) | Approved Sep 10, 2024 |

# **Abstract**

Cryptography configuration is unique among types of configuration: it may need to be updated multiple times over the lifetime of a system or a cluster while requiring consistency across all installed software.

We want to offer a way to choose a system-wide crypto configuration among several consistent and supported alternatives.

This specification defines a framework made of tools and policy. The policy affects package building and software behavior. Tools are used for system administration.

This specification puts a greater emphasis on policy rather than tools, as the policy will serve as the foundation for the design and implementation of tools in the future.

## **Aside: crypto-what?**

In this document we use “crypto” to mean basically anything that gnutls or openssl provide. This includes but is not limited to: cryptography primitives, SSL and TLS implementations, and certificate handling.

In addition, we define ‘crypto providers’ which expose crypto APIs and implement protocols and algorithms, and ‘crypto users’ which consume these. A user can also be a provider.

# **Table of contents**

[Abstract](#p-135705-abstract)
-- [Aside: crypto-what?](#p-135705-aside-crypto-what)
[Table of contents](#p-135705-table-of-contents)
[Forewords](#p-135705-forewords)
[Current implementation](#p-135705-current-implementation)
[The issues at stake](#p-135705-the-issues-at-stake)
-- [Lack of consistent configuration: case study on Mantic Minotaur](#p-135705-lack-of-consistent-configuration-case-study-on-mantic-minotaur)
-- [Moving security forward](#p-135705-moving-security-forward)
-- [Crypto-config overview](#p-135705-crypto-config-overview)
-- [Applicability to non-crypto domains](#p-135705-applicability-to-non-crypto-domains)
[State of the art: Fedora’s crypto-policies](#p-135705-state-of-the-art-fedora’s-crypto-policies)
[Rationale for a new policy and tools](#p-135705-rationale-for-a-new-policy-and-tools)
[User stories](#p-135705-user-stories)
-- [Other constraints, motivations and design goals](#p-135705-other-constraints-motivations-and-design-goals)
[Specification](#p-135705-specification)
-- [Constants used in the specification](#p-135705-constants-used-in-the-specification)
-- [In a nutshell](#p-135705-in-a-nutshell)
-- [Overview](#p-135705-overview)
-- [Policy](#p-135705-policy)
-- [Paths](#p-135705-paths)
-- [Profiles design](#p-135705-profiles-design)
-- [Profile inheritance](#p-135705-profile-inheritance)
-- [Packages](#p-135705-packages)
-- [User-facing tool](#p-135705-user-facing-tool)
-- [Configuration sealing](#p-135705-configuration-sealing)
-- [Statically-linked crypto providers](#p-135705-statically-linked-crypto-providers)
[Affected packages](#p-135705-affected-packages)
-- [Crypto providers](#p-135705-crypto-providers)
-- [Crypto users](#p-135705-crypto-users)
[Example: usefulness with krb5](#p-135705-example-usefulness-with-krb5)
[Known Limitations and Future work](#p-135705-known-limitations-and-future-work)
-- [Known Limitations](#p-135705-known-limitations)
-- [Future work](#p-135705-future-work)
[Further Information](#p-135705-further-information)
-- [Annex: bits of security, comparable algorithm strengths, and boiling oceans](#p-135705-annex-bits-of-security-comparable-algorithm-strengths-and-boiling-oceans)
-- [Annex: Possible work split](#p-135705-annex-possible-work-split)
-- [Annex: relationship with distribution upstreams (Debian) and downstreams (Ubuntu derivatives)](#p-135705-annex-relationship-with-distribution-upstreams-(debian)-and-downstreams-(ubuntu-derivatives))
-- [Annex: Example report following package install or removal](#p-135705-annex-example-report-following-package-install-or-removal)
[References](#p-135705-references)

# **Forewords**

This specification is long. It touches many topics and many pieces of software. Not everything can be explained linearly unfortunately and Google docs is not entirely helpful as it is very limited. That being said, I believe the specification is consistent and the puzzle will become a clear picture by the end of the document.

In order to ease reading, I am introducing notes at the beginning of sections. These are not the specification but rather some context around it. They are displayed as follows:

| 💡This is an informative text. Color is copied from the venerable dokuwiki "note" plugin which most recent version is hosted at [https://github.com/lpaulsen93/dokuwiki\_note](https://github.com/lpaulsen93/dokuwiki_note) and is released under the GPLv2. The plugin uses round corners but the best I can do with google docs is a 1-cell table with ugly hard corners. |
| :---- |

It should be noted that this document has been authored in Google Docs. Not all export formats can be used reliably unfortunately: markdown is fine when diagrams and non-docs resources are not used while PDF appears to work for everything but is less convenient to handle. As such, this specification will be exported and published first using the markdown export and a PDF export will also be provided for reference.

# **Current implementation**

There is already a partial implementation of the specification available. It is hosted on [github.com/canonical/crypto-config](http://github.com/canonical/crypto-config).  
It contains the integration with `dpkg` using a `post-inst` trigger. It also handles profile inheritance, albeit with the parent being set using a bare text file rather than JSON because the current implementation is a shell script. This is planned to be solved when re-writing the implementation in Rust, which will also feature a more complete UI.

# **The issues at stake**

NB: A lightning talk on this topic was given at the [Ubuntu Engineering Sprint in Riga in November 2023](https://docs.google.com/presentation/d/1wFuKDPT5woZ4QdjGnTjfqqzNbtyeG2xBgZM732OXXek).

There are several libraries in Ubuntu implementing cryptographic algorithms (ciphers, hashes, …) and protocols (TLS, SSH, …). Hundreds of packages offer crypto-related configurations and these usually take precedence over system-wide library configuration. At least Java, Golang and Rust have their own crypto libraries. This amounts to hundreds if not thousands of configurations to perform. Moreover, several aspects are tricky to get right and version changes can cause breakage.

## **Lack of consistent configuration: case study on Mantic Minotaur**

| Software | Nginx | Apache | Mariadb | Postgres | Mysql | Exim | Rabbitmq |
| :---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- |
| Ciphers | Baseline | Baseline | ~~CAMELLIA ARIA CCM~~ | Baseline | ~~CHACHA20 POLY1305~~ | ~~CCM ARIA CAMELLIA AES-SHA\>128~~ | ~~ARIA CAMELLIA~~ ECDH |
| Key Exchange | Baseline | Baseline | Baseline | P-256 only | P-256 ~~x25519 x448 ffdhe\*~~ | TLS1.3: CCM P-384 P-512 x25519 x448 | TLS1.3: CCM P-256 brainpool sect\* secp\* |
| Bonus |  | Actually: TLS1.2: ~~AES128~~ |  |  |  | TLS 1.0 TLS 1.1 | TLS 1.0 TLS 1.1 Erlang OTP |

At the moment (Mantic Minotaur), there are countless differences between the default cryptographic configuration of packages in the archive, especially for TLS. The results below have been obtained by enabling TLS support and configuring a certificate in nginx, apache, mariadb, postgresql, mysql, exim and rabbitmq. No further step was carried on in order to find what is the closest to a default configuration. It is likely that users run with these configurations, as can be seen through censys.io (internet-wide host scan database) which shows a large number of ubuntu machines running exim with TLS 1.0 and 1.1 enabled.

Compared to nginx' configuration:

- **apache** disables AES128 for TLS 1.2 (but not TLS 1.3 \[ where it's specified as mandatory \])  
- **mariadb** disables CAMELLIA, ARIA, and CCM mode for AES  
- **postgres** only exposes P-256 for key exchange, no matter the TLS version  
- **mysql** disables CHACHA20-POLY1305 for TLS 1.2 (but not TLS 1.3 \[ where it's specified as mandatory \]), it also disables key exchanges that are not P-{256,384,512} (i.e. x25519, x448, ffdhe\*)  
- **exim** disables CAMELLIA, ARIA, CCM8 mode for AES (but not CCM mode which it even enables when doing TLS 1.3), AES256-SHA384 and AES128-SHA256 but not AES{128,256}-SHA; it enables more curves for key exchanges; it also still enables TLS 1.0 and 1.1  
- **rabbitmq** disables ARIA and CAMELLIA and enables ECDH (which uses a lot of CPU), it enables CCM and CCM8 modes with TLS 1.3; with ECDH it enables CBC mode for AES ; it doesn't offer P-521 for key exchange with TLS 1.3 ; it enables many many curves for key exchange with TLS 1.2, including some as small as 163 bits (equivalent security: 81 bits for symmetric encryption) which is widely considered inadequate for more than 10 years ; finally it also enables TLS 1.0 and 1.1.

The topic here is not about which configuration is better but how users can be anything but lost with so many widely different configurations.

## **Moving security forward**

| 💡Using the same cryptography configuration for all software on Ubuntu and for all Ubuntu users actually means sticking to legacy and lenient configurations. We need configuration values that are consistent per-ecosystem rather than across the distribution in order to not tie ourselves to the slowest moving ecosystems (i.e. e-mail). |
| :---- |

How to make our default configurations be on the forefront rather than trailing?

Standards or good practices typically mandate sets of algorithms but not all algorithms from these sets are the same. These sets typically include some algorithms meant for the future, some for current usage and some for compatibility with legacy systems in order to maintain compatibility over time and be able to gradually update systems.

Completely forbidding deprecated algorithms in crypto providers is possible (although this will sometimes involve writing code). Is it enough?  
As explained above, at any given time, we are likely to enable some algorithms for compatibility with legacy. Should they be enabled system-wide by default though? And if they're disabled by default, should enabling them always be system-wide?

I argue that we should aim to disable legacy algorithms and protocols by default and enable them per-application based on every application's environment. TLS 1.3 is understood by 97% of the used web browsers while TLS 1.2 (our current minimum) is understood by 98% of them. The difference is old and non-updated browsers; should we be held behind because of them? On the other hand, mail servers probably still require TLS 1.2 in many cases. As such, we might want something like the following:  
![][image1]

```
 Relative security ^
                   |
+-----------------+| +---+  +---+  +---+
| Modern {g}      || |   |  | N |  | p |
+-----------------+| |   |  | g |  | s |
                   | | E |  | i |  | q |
+-----------------+| | x |  | n |  | l |
| Current {g}     || | i |  | x |  |   |
+-----------------+| | m |  +---+  +---+
┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄+┄+┄┄┄+┄┄┄┄┄┄┄┄┄┄┄┄┄┄
+-----------------+| |   |
| Legacy {o}      || |   |
+-----------------+| +---+
┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄+┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
+-----------------+| Systemwide  minimum
| Deprecated {r}  ||
+-----------------+|

# Legend:
g = {
    fill: #93c47d;
}
o = {
    fill: #f5b26b;
}
r = {
    fill: #e06666;
}
```

## **Crypto-config overview**

| 💡Crypto-config works by introducing an indirection in the path of configuration files. This is as if '/etc' were actually a symlink that you could point to either '/etc-lenient' or '/etc-hardened', except that it only applies to some files that the package maintainer opts into the scheme. The base implementation is 'ln \-sfn'\! There is some architectural work to make everything smooth for both Ubuntu developers and users but it is fairly well-bound and this document should cover everything. The majority of the work is providing meaningful and wise configuration choices to users so that they don't have to face the complexity of configuring everything themselves. In other words, that's typical distribution work. |
| :---- |

Applications have their own configuration but they also use libraries which can also have their own configuration. For instance, Nginx reads "default.conf" but it also uses openssl which can read "openssl.cnf". This typical setup is shown below for several applications. In these pictures, each cell in the left column corresponds to the cells on the same line on the right side.  
![][image2]

```
.-----------------------.  |  .-----------------------.   .-----------------------.   .---------.     .-----------------------.   .---------.     .---------.
|   Cryptography User   |  |  |        Apache2        |   |         Nginx         |   |   ...   |     |         Exim4         |   |   ...   |     |   ...   |
'-----------------------'  |  '-----------+-----------'   '-----------+-----------'   '---------'     '-----------+-----------'   '---------'     '---------'
                           |              +-------------.             +-------------.                             +-------------.                            
                           |              |             |             |             |                             |             |                            
                           |              v             |             v             |                             v             |                            
.-----------------------.  |  .-----------------------. | .-----------------------. | .---------.     .-----------------------. | .---------.     .---------.
|     Configuration     |  |  |       ssl.conf        | | |      default.conf     | | |   ...   |     |"03_exim4...tlsoptions"| | |   ...   |     |   ...   |
'-----------------------'  |  '-----------------------' | '-----------------------' | '---------'     '-----------------------' | '---------'     '---------'
                           |                            |                           |                                           |                            
                           |                            |                           |                                           |                            
                           |                            |                           |                                           |                            
                           |                            |                           |                                           |                            
                           |                            |                           |                                           |                            
                           |                            |                           |                                           |                        
                           |                            |                           |                                           |                        
                           |                            |                           |                                           |                        
                           |                            v                           v                                           v                        
.-----------------------.  |  .-----------------------------------------------------------------.     .-------------------------------------.     .---------.
| Cryptography Provider |  |  |                            OpenSSL                              |     |               GnuTLS                |     |   ...   |
'-----------------------'  |  '-------------------------------+---------------------------------'     '------------------+------------------'     '---------'
                           |                                  |                                                          |                                
                           |                                  |                                                          |                                
                           |                                  |                                                          |                                
                           |                                  v                                                          v                                
.-----------------------.  |  .-----------------------------------------------------------------.     .-------------------------------------.     .---------.
|     Configuration     |  |  |                          openssl.cnf                            |     |           "gnutls/config"           |     |   ...   |
'-----------------------'  |  '-----------------------------------------------------------------'     '-------------------------------------'     '---------'
                           |                                                                                                                                 
                           |                                                                                                                                 
                           |                                                                                                                                 
                           |                                                                                                                                 
                           |                                                                                                                                 
```

With crypto-config, the first idea is to have the following instead: every configuration file can refer to data through the crypto-config framework, therefore making it possible to have a single source of truth for the cryptography configuration.

![][image3]

```
.-----------------------.  |  .-----------------------.   .-----------------------.   .---------.     .-----------------------.   .---------.     .---------.
|   Cryptography User   |  |  |        Apache2        |   |         Nginx         |   |   ...   |     |         Exim4         |   |   ...   |     |   ...   |
'-----------------------'  |  '-----------+-----------'   '-----------+-----------'   '---------'     '-----------+-----------'   '---------'     '---------'
                           |              +-------------.             +-------------.                             +-------------.                            
                           |              |             |             |             |                             |             |                            
                           |              v             |             v             |                             v             |                            
.-----------------------.  |  .-----------------------. | .-----------------------. | .---------.     .-----------------------. | .---------.     .---------.
|                       |  |  |       ssl.conf        | | |     default.conf      | | |   ...   |     |"03_exim4...tlsoptions"| | |   ...   |     |   ...   |
|                       |  |  '-----------+-----------' | '-----------+-----------' | '---------'     '-----------+-----------' | '---------'     '---------'
|     Configuration     |  |              |             |             |             |                             |             |                            
|                       |  |              v             |             v             |                             v             |                            
|                       |  |  .-----------------------------------------------------------------------------------------------------------------------------.
|                       |  |  |                                                       "crypto-config"                                                       |
'-----------------------'  |  '-----------------------------------------------------------------------------------------------------------------------------'
                           |                            |                           |                                           |                        
                           |                            |                           |                                           |                        
                           |                            |                           |                                           |                        
                           |                            v                           v                                           v                        
.-----------------------.  |  .-----------------------------------------------------------------.     .-------------------------------------.     .---------.
| Cryptography Provider |  |  |                            OpenSSL                              |     |               GnuTLS                |     |   ...   |
'-----------------------'  |  '-------------------------------+---------------------------------'     '------------------+------------------'     '---------'
                           |                                  |                                                          |                                
                           |                                  |                                                          |                                
                           |                                  |                                                          |                                
                           |                                  v                                                          v                                
.-----------------------.  |  .-----------------------------------------------------------------.     .-------------------------------------.     .---------.
|                       |  |  |                          openssl.cnf                            |     |           "gnutls/config"           |     |   ...   |
|                       |  |  '-------------------------------+---------------------------------'     '------------------+------------------'     '---------'
|     Configuration     |  |                                  |                                                          |                                   
|                       |  |                                  v                                                          v                                   
|                       |  |  .-----------------------------------------------------------------------------------------------------------------------------.
|                       |  |  |                                                       "crypto-config"                                                       |
'-----------------------'  |  '-----------------------------------------------------------------------------------------------------------------------------'
```

While this is enough to cover the majority of applications, there is still an issue with hard-coded values in applications and applications initializing libraries in a way that doesn't parse the library configuration. We would also like to be able to apply changes system-wide by only touching the cryptography provider.

Due to limitations in cryptographic libraries, these will require patches. Such patches exist in Fedora/RHEL but while the ones for e.g. openssl achieve the desired outcome, they probably cannot be upstreamed without large changes.

Consider the source of configuration values in an application like nginx:

1) defaults in the openssl binaries  
2) openssl configuration  
3) defaults in the nginx binaries  
4) nginx configuration

Today nginx configures the list of enabled ciphers using either a default value built into nginx, or a value which comes from its configuration. There is however no value which means "use what is defined system-wide". A system administrator would have to manually transcribe the system-wide configuration into nginx' configuration. While this is somewhat possible for a single application, it doesn't scale at all. Addressing this situation is the second category of changes for crypto-config.

Both approaches are complementary. Acting at the level of crypto providers ensures no application runs with a forbidden algorithm. Configuring applications directly makes it possible to move some of them forward faster than others: indeed, cryptography for the web has improved much faster than for e-mails which is hindered by backward-compatibility for its federative model.

It is important to keep in mind that in every case, sifting through the Ubuntu archive will be needed in order to ensure packages properly follow this specification. This isn't actually additional work since such [an audit is already needed as shown previously](#lack-of-consistent-configuration:-case-study-on-mantic-minotaur).

## **Applicability to non-crypto domains**

Little of what is described below is limited to cryptography. The only reasons to limit this to cryptography are not technical but practical.

Indeed, the goal is to cover a whole domain with a single configuration value and there doesn’t seem to be another field for which this would make sense or be doable.

In the event such a field is found, the process below would result in a second configuration value that is independent from this one anyway. Indeed, the whole point of this approach is to have users not need to consider combinations of options.

# **State of the art: Fedora’s crypto-policies**

| 💡Here we quickly come back to why adopting crypto-policies from Fedora/RHEL is not useful nor desirable. |
| :---- |

We have analyzed Fedora’s crypto-policies as part of [US021](https://docs.google.com/document/d/1dqgp44bU6gSiyc-nNBB5gbPFElQJZ-yb3Caj-IJ113M). It is too RH-specific to use as-is and not interesting enough to reuse and improve upon.

On the plus side, it seems that it properly lets admins of RH systems experience the upcoming change in defaults in RH by running a simple command. People seem satisfied with a small featureset.

However:

* the implementation is too complicated since it tries to generate configuration for everything from a single configuration file,  
* there has been only one important (public) use case for it so far (deprecating SHA-1),  
* its UI is lacking (there is almost no UI at all actually),  
* it might be impossible to have the host rules apply to snaps.

My criticism of crypto-policies can be summarized as attempting to solve an uncommon use case in a fully generic way and ending up being so complex that the genericity becomes a hindrance for both developers and users. Moreover, I’d wager that most of the development time is spent on fixing it, leaving no time to make it nice to use. Moreover, writing policies seems time-intensive.

Crypto-policies is however very interesting since it is the only software and process that exists in this space today. It therefore constitutes a comparison point. The project also includes a policy for packaging that we can get inspiration from.

# **Rationale for a new policy and tools**

Fedora has been shipping a ‘crypto-policies’ tool for several years. While its goals appealed to us, we have deemed it not appropriate for Ubuntu. This leaves us with a few choices: a) not provide anything, b) find something else that already exists, c) write something ourselves.

Customers have not yet expressed a strong urge to have such a tool but this is a growing demand and some customers (IBM-cloud) are asking for this. There is also an internal demand for it.

There seem to be no existing tools we could use or at least evaluate. This is not very surprising since one of the reasons we rejected Fedora’s crypto-policies is its complex ties to Fedora as it depends on the rest of the configurations in the distribution, on the software versions and even distribution patches (for instance, there is a special case related to RSA key length for openssh).

While it has been packaged and uploaded to Debian, crypto-policies has never been linked to the reset of the system and it has finally been removed in July 2024\. It should be noted that since diverging from Debian is costly, we would prefer something that Debian also adopts or that at least implies a minimal diff. Adoption of policies and tools by Debian does not imply that Ubuntu and Debian will use the same configuration.

This leaves us with writing a tool ourselves. I believe there is a simple path forward that will let us have clearer configuration files that can be easily tweaked by users. I also believe there is a lot of value in bringing this to customers. This is done in baby steps that are also very natural: the risk with each step is minimal and the project can be paused at any time with no need to rollback anything.

# **User stories**

As a sysadmin, I would like to select a configuration policy that applies to all supported services system-wide both for defaults settings and minimum ones.

As a sysadmin, I would like to know which software lowers the crypto settings of its crypto providers.

As a sysadmin, I would like to disable a given algorithm system-wide at a given date (e.g. SHA1 at the end of 2030).

As a sysadmin, I would like to disable a given algorithm system-wide but re-enable it selectively (e.g. a specific machine cannot be upgraded and only supports legacy cryptography).

As a sysadmin or developer, I would like to test a Ubuntu system with some algorithms or protocols disabled in order to assess compatibility (e.g. with upcoming deprecations).

As a developer of software that will run on Ubuntu, I would like to provide users with typical configuration values for them to choose from, reducing documentation burden and support requests.

As an Ubuntu developer, I would like to more easily know which crypto algorithms and configurations are configured and in use.

As a developer of software, I’d like to know which of the myriad of crypto options are recommended and reliable. This ranges from security considerations like “is it secure” to compatibility ones “is it working across all supported Ubuntu releases”.

As an Ubuntu developer, I would like to simplify the task of providing compliant configuration settings by using an integrated system which allows specifying the relevant settings for the various compliance frameworks.

## **Other constraints, motivations and design goals**

In addition to user wishes and needs, there are additional constraints to take into account.

### **Guarantee consistency and don't break systems**

### Centralizing configuration risks breaking every related package at once: upgrading the central crypto-config package can break packages which rely on it. The central package should therefore be simple and developed with care in order to minimize this risk.

### **Handle package upgrades, removals and purges**

Care must be taken not to provide fewer guarantees than the usual configuration handling in Debian.

### **Limit dynamism in configuration**

Dynamism can be useful but we want a safe and deterministic system. It should behave similarly as configuration handling in Debian packages and provide at least the same guarantees, especially those of safety.

### **Don't require lockstep upgrades / transitions**

We don't want to lock everything together. Requiring a transition for every crypto provider and user would be terrible.  
This implies some dynamism in profiles. Indeed, consider package A which knows profiles X and Y; when adding profile Z, if there is no mechanism to automatically populate A's configuration files for Z, package A must be updated. This would apply to every enrolled package, effectively starting a very large transition. In other words, packages built at a given moment should be forward-compatible with profiles introduced later on.  
In order to achieve this kind of compatibility, there must be dynamism after package creation, i.e. when installing. This will involve dpkg triggers as explained in the specification.

### **Avoid incompatibilities with Debian**

As a Debian downstream, deltas are costly and incompatibilities require constant work. Moreover we would like Debian to be able to use this work.  
Provisions should be taken to ensure differences with Debian, whether it is not using this work or it is, do not cause undue burden on Ubuntu developers.

# **Specification**

NB: examples in this section use ‘nginx’ because its configuration format is simple; the same can be done with openssl.

## **Constants used in the specification**

```
DATA_DIR=/usr/share/crypto-config
SYSTEM_PROFILES_DIR=${DATA_DIR}/profiles
STATE_DIR=/var/lib/crypto-config
STATE_PROFILES_DIR=${STATE_DIR}/profiles
SYSCONF_DIR=/etc/crypto-config
SYSCONF_PROFILES_DIR=${SYSCONF_DIR}/profiles
```

## **In a nutshell**

```
SYSTEM_PROFILES="${SYSTEM_PROFILES_DIR}"
CURRENT_PROFILE="${STATE_DIR}/current"

### new files:
regular file: ${SYSTEM_PROFILES}/post-quantum/nginx/ssl-ciphers.conf
regular file: ${SYSTEM_PROFILES}/default/nginx/ssl-ciphers.conf
symlink:      ${CURRENT_PROFILE}    to post-quantum/ or default/

### modified with “include ${CURRENT_PROFILE}/nginx/ssl-ciphers.conf":
/etc/nginx/nginx.conf

### Run
crypto-config switch post-quantum
```

## **Overview**

| 💡Packages install variants of their cryptography configuration and a post-installation dpkg trigger merges them across applications into profiles. Users can use a tool to change the system from one profile to another. There will also be work to ensure all software actually follows the system configuration (proof that all software doesn't work together out of the box: we're still not out of job). |
| :---- |

* All crypto providers are enrolled in the current scheme; this will indirectly enroll their users too  
* Crypto users can also optionally be enrolled in order to tune their behavior more finely  
* Only deal with the organization of configuration files and add tooling to switch between sets of configuration snippets that already exist.  
* Packages’ crypto configurations are extracted into dedicated configuration files.  
* Ubuntu ships sets of Canonical-written configuration files for each supported system-wide configuration  
* The same packages that ship configurations continue to do so and adapt to the new organization  
* Packages install their sets of configuration files under ${SYSTEM\_PROFILES\_DIR}/\<name\_of\_the\_profile\>/\<name\_of\_package\>/  
* A tool is then triggered to fill potential gaps in the profiles through inheritance and installs the result in ${STATE\_DIR}; the tool can also be called from postinst scripts when needed (postinsts run before triggers do). This tool is described in the [Profile inheritance](#profile-inheritance) section.  
* We decide which profiles we want to provide and make mandatory to support by crypto providers; this includes at least one named “default”  
* Every software enrolled in this scheme ships sets of configuration snippets for the default profile and optionally provide snippets for other profiles  
* The configuration snippets are referred to by configuration files through the ${STATE\_DIR}/current path  
* Configuration snippets are static  
* Users can write configuration snippets and create configuration sets but do so without being provided specific support at the moment  
* The configuration choices should be forwarded to snaps (but this is out of scope for the current specification)  
* We create a tool for system administrators to manage the link to the current profile  
* We want to minimize differences from both upstreams and Debian

## **Policy**

| 💡This is mostly the set of rules required for packages to \_actually\_ use system-wide configurations. For instance, if there is a default set of TLS ciphers configured in openssl, applications can change that to almost anything and they very often do. We will therefore work towards making applications use a system-wide value by default (users can still configure them differently). The number of rules is due to how different every software in Ubuntu is. |
| :---- |

* This policy restricts what packages can do in order to make configuring software more manageable by the human users. In essence it defines an interface between users and software crypto configuration files.  
* It seems impossible to be less restrictive than this proposal while actually improving the situation since users are looking for complete or near-complete coverage  
* Packages already in main when the policy comes into effect automatically enter this scheme with a legacy status and will be evaluated over time for actual compliance. As such this policy doesn't put additional constraints on existing packages the day it comes into effect.  
* This policy becomes a requirement for new packages in Ubuntu main starting with the development of 25.04.  
* Crypto providers are libraries such as openssl  
* Crypto users are software that uses these providers; this includes both applications and libraries  
* Crypto users can also be crypto providers (e.g. openssh which implements cryptography but also uses openssl).  
* Crypto users that use hard-coded values to configure their crypto provider must now also accept overrides from their configuration  
* Crypto users must not use API calls to change crypto providers' configuration values that are configured under this scheme at the level of the crypto providers themselves  
* Crypto providers accept configuration through the current framework  
* Canonical-supported snaps should also abide by this policy; details are to be laid out in a dedicated specification  
* There is no hard rule to decide whether a configuration value must be handled through this framework.  
* Configuration values that are handled through this framework are extracted into dedicated files and used through “include” or similar configuration directives.

### **Exceptions**

* Exceptions to the policy can be granted by the Ubuntu Technical Board (this may especially make sense for systems dealing with data at rest, and databases in particular)

### **Reporting and introspection**

| 💡Some APIs of crypto providers can have a large impact on the effective configuration of the system (e.g. by changing the path of the system configuration to /dev/null, or by changing global options). These APIs also have legitimate usages which makes it impossible to outright forbid them. Moreover, static analysis is limited and does not cover software outside of Ubuntu. It will therefore be useful to dynamically query applications to learn if they have used such APIs. |
| :---- |

* Crypto providers expose a symbol or USDT probe  which reports whether a crypto user has used configuration APIs which do not match the policy above.

## **Paths**

| 💡Packages install configuration snippets in /usr. Running software uses configuration data from under /var/lib. A dpkg trigger reads from /usr and populates /var/lib, handling profile inheritance. |
| :---- |

* Files provided by the distribution are installed inside ${SYSTEM\_PROFILES\_DIR}  
* Users use ${SYSCONF\_PROFILES\_DIR} to create profiles; in case of a name conflict, distribution profiles take precedence  
* Packages configurations only refer to ${STATE\_DIR}/current which is a symlink that always points to the current profile  
* Files in ${SYSTEM\_PROFILES\_DIR} and ${SYSCONF\_PROFILES\_DIR} are internal data for the crypto-config tools which uses them to create full profiles in ${STATE\_PROFILES\_DIR} which match the file structure expected by packages  
* Every profile is a collection of dropins for packages; how these dropins are internally organized is up to each package's maintainer  
* Crypto-config handles the collection of dropins for every package as black boxes  
* Besides "default", profiles have no obligation to ship dropins for a given package; these will be filled in according to the rules laid out in [Profiles design](#profiles-design) and [Profile inheritance](#profile-inheritance)  
* After a package operation changes files under ${SYSTEM\_PROFILES\_DIR}, a tool copies profiles from both ${SYSTEM\_PROFILES\_DIR} and ${SYSCONF\_PROFILES\_DIR} to ${STATE\_PROFILES\_DIR}; there the profiles are filled in according to the aforementioned rules  
* All profiles are merged in ${STATE\_DIR} and used from there  
* There is no support for merging profiles across these directories.  
* There is no support for drop-ins (they don't make sense here).  
* There is no dedicated support for masking profiles (e.g. through the creation of symlinks to /dev/null in directories with a higher precedence) since any profile created in a directory with higher precedence masks those of the same names in directories of lower precedence  
* If needed, configuration for the tool itself will go in ${DATA\_DIR}/crypto-config.conf.d or ${SYSCONF\_DIR}/crypto-config.conf.d  
* All profile names are reserved for distribution use except those starting with 'local/' or 'site/' (this is achieved by storing profiles in subdirectories); this may be relaxed in the future once crypto-config has been used more.

### **Without crypto-config**

![][image4]

```
.----------------------.                            .----------------------.
|                      |                            |                      |
|     .deb package     |                            |         User         |
|                      |                            |                      |
'-----------+----------'                            '-----------+----------'
            |                                                   |
            |                                                   |
            |dpkg installs                                      |Edits
            |                                                   |
            v                                                   |
.----------------------.                                        |
|                      |                                        |
|     Application      |                                        |
|                      |                                        |
'-----------+----------'                                        |
            |                                                   |
            |                                                   |
            |Reads                                              |
            |                                                   |
            v                                                   |
.----------------------.                                        |
|                      |                                        |
|        Config        |                                        |
|                      |                                        |
|                      |<---------------------------------------'
|                      |
'----------------------'
```

### **With crypto-config**

![][image5]

```
.----------------------.                            .----------------------.
|                      |                            |                      |
|     .deb package     |                            |         User         |
|                      |                            |                      |
'-----------+----------'                            '-----------+----------'
            |                                                   |
            |dpkg installs                                      |
            +------------------------.                          |Creates
            |                        |                          |
            v                        v                          v
.----------------------.  .----------------------.  .----------------------.
|                      |  |        Files         |  |         Files        |
|     Application      |  |          in          |  |           in         |
|                      |  | "SYSTEM_PROFILES_DIR"|  |"SYSCONF_PROFILES_DIR"|
'-----------+----------'  '-----------+----------'  '-----------+----------'
            |                         |                         |
            |                         |     "crypto-config"     |
            |Reads                    |        generates        |
            |                         `-----------.  .----------'
            v                                     |  |
.----------------------.                          |  |
|                      |                          v  v
|        Config        |                .----------------------.
|  .----------------.  |      Uses      |       Profiles       |
|  |"crypto-config" +--+--------------->|          in          |
|  '----------------'  |                | "STATE_PROFILES_DIR" |
'----------------------'                '----------------------'
```

### **Profiles and dropins collections**

![][image6]

```
+--------------------+ +--------------------+ +--------------------+
| .----------------. | | .----------------. | | .----------------. |
| |GnuTLS  dropins | | | |GnuTLS  dropins | | | |GnuTLS  dropins | |
| '----------------' | | '----------------' | | '----------------' |
| .----------------. | | .----------------. | | .----------------. |
| | Nginx dropins  | | | | Nginx dropins  | | | | Nginx dropins  | |
| '----------------' | | '----------------' | | '----------------' |
| .----------------. | | .----------------. | | .----------------. |
| |OpenSSL dropins | | | |OpenSSL dropins | | | |OpenSSL dropins | |
| '----------------' | | '----------------' | | '----------------' |
| .----------------. | | .----------------. | | .----------------. |
| |      ...       | | | |      ...       | | | |      ...       | |
| '----------------' | | '----------------' | | '----------------' |
+--------------------+ +--------------------+ +--------------------+
 "'Default' Profile"    "'Legacy' Profile"     "'Future' Profile"
```

## **Profiles design**

| 💡Designing profiles is a task that requires dedicated work: the data needs to be stored in a way that can be made sense of afterwards. This is initially constraining but mostly one-time work to enable profile inheritance (which is described in a subsequent section), and proper UI/UX for users. |
| :---- |

* Ubuntu ships a number of profiles through the crypto-config package  
* Profiles contain a file metadata.json with the following values:  
  * pretty-name  
  * summary (a few words, at most a line)  
  * description  
  * maintainer (mostly future-proofing for user customization)  
  * parent (for inheritance across profiles)  
  * condition (a shell command which must succeed for the profile to be visible and usable)  
* DIrectories without metadata are ignored  
* Profile design, layout and metadata is decided and authored at the distribution-level  
* For any profile except the default one, a package either ships no file and no directory, or ships the same list of files as the default profile  
* The list of collections of package dropins in the default profile is the single source of truth for enrolled packages on the system  
* All profiles must inherit directly or indirectly from the default profile. Without inheritance, creating a new profile would require adding support for it in every enrolled package; with it, the dropins collections from the parent are used until more specific one are added to the application package (if wanted)  
* Inheritance happens at the level of dropins collections, not dropins themselves since their set is dynamic and their inheritance would therefore be complex  
* Profiles form a tree through their inheritance relationships  
* Consistency between profiles for an application is mostly achieved by the fact that they are maintained as part of the same package

## **Profile inheritance**

| 💡Profile inheritance creates a profile that uses the same configuration data as another one except for some software. The inheritance is coarse: it occurs at the application-level, not at the level of configuration snippets which would be a much more complicated task, maybe even an undecidable one since that would require parsing every configuration format and making sense of it. If a maintainer (or user) wants to reuse some snippets, it should be done in a traditional way, possibly with symlinks to well-known files. The inheritance is implemented through a program called every time configuration snippets are touched through a dpkg trigger. The process is deterministic and idempotent and can be used for user-customization (this will work but is not officially supported at this time). |
| :---- |

The inheritance process paves the way for user-customized profiles but there are additional aspects to take into account, mostly around conffile handling before officially supporting user-customization.

* Nothing here changes any file directly managed by dpkg; every operation described below happens inside ${STATE\_DIR}.  
* Whenever a package installs or removes collections of dropins, a tool is automatically started at the end of the packages' configuration or de-configuration and guarantees the presence, validity and coverage of every profile in ${STATE\_DIR} based on the installed collections of dropins.  
* When a package does not provide a directory of snippets for a profile, a symlink to the one in the parent profile is created in its place.  
* Existing regular files are not modified.  
* Existing symlinks are replaced by symlinks to the parent's dropins collection.  
* This resolution is done starting from the top of the inheritance tree (the default profile) and towards its leaves, therefore guaranteeing everything resolves eventually  
  * Example: consider profiles and inheritances "default" \-\> "x" \-\> "y" \-\> "z"  
  * Profile "default" covers application "foo"  
  * Profile "x" does not; "foo" will be created as a symlink to the directory from profile "default"  
  * Profile "y" covers it, nothing else will be done  
  * Profile "z" does not; "foo" will be created as a symlink to the directory from profile "y".  
* We can prove the inheritance resolution process always succeeds:  
  * a profile either contains dropins collections for every enrolled packages, or lacks some  
  * by construction the default profile lacks no dropins collection  
  * a profile that lacks some dropins collection and which parent lacks none has all its missing dropins collections  filled in as symlinks to the parent profile and therefore does not lack dropins collections anymore  
  * therefore all profiles that inherit from the default profile lack no dropins collections and the process can be applied recursively to their children  
  * by induction, after the whole process, all profiles that inherit directly or indirectly from the default profile lack no dropins collections.

### **Example profile inheritance tree**

### **![][image7]**

```
                      +------------+
                      |  Default   |
                      +--+-+--+-+--+
                         | |  | |
      .------------------' |  | '------------------.
      |               .----'  '----.               |
      |               |            |               |
      v               v            v               v
+-----------+  +-----------+  +-----------+  +-----------+
|  Legacy   |  |   FIPS    |  |  Future   |  | Customer- |
|           |  |           |  |           |  | specific1 |
+-----------+  +-----+-----+  +-----------+  +-----------+
                     |
                     v
               +-----------+
               | Customer- |
               | specific2 |
               +-----------+
```

### **Example dropins collection re-uses across profiles**

![][image8]

```
    +-----------------------------------------------------------+
    |                                                           |
    |             +------------+  +-----------+  +------------+ |
    | Default     | GnuTLS {g} |  | Nginx {g} |  | OpenSSL {g}| |
 ,->|             +------------+  +-----------+  +------------+ |
 |  |                   ^               ^              ^        |
 |  +-------------------|---------------|--------------|--------+
 |Inherits from         | Reuses        |              | Reuses
 |  +-------------------|---------------|--------------|--------+
 |  |                   |               |              |        |
 `--|             +------------+  +-----------+  +------------+ |
    | Legacy      | GnuTLS {b} |  | Nginx {g} |  | OpenSSL {b}| |
 ,->|             +------------+  +-----------+  +------------+ |
 |  |                   ^               ^              ^        |
 |  +-------------------|---------------|--------------|--------+
 |Inherits from         | Reuses        | Reuses       |
 |  +-------------------|---------------|--------------|--------+
 |  |                   |               |              |        |
 `--|             +------------+  +-----------+  +------------+ |
    | "Customer2" | GnuTLS {b} |  | Nginx {b} |  | OpenSSL {g}| |
    | "-specific" +------------+  +-----------+  +------------+ |
    |                                                           |
    +-----------------------------------------------------------+

Legend:
+------------+
| <pkg>  {b} | Dropins are symlinks to dropins in another profile
+------------+

+------------+
| <pkg>  {g} | Dropins are regular files in the current profile
+------------+

# Legend:
g = {
    fill: #93c47d;
}
o = {
    fill: #f5b26b;
}
r = {
    fill: #e06666;
}
b = {
    fill: #cfe2f3;
}
```

## **Packages**

| 💡Here we go through how crypto-config will be shipped and installed, and its impact on existing packages. |
| :---- |

### **crypto-config**

* New package  
* Installs the metadata of the profiles that are included in Ubuntu  
* Provides a shell script for supporting the installation and configuration of enrolled packages for use by dpkg  
* Provides a tool for managing the configuration (which specification is in the [Tool](#user-facing-tool) section)  
* Registers a dpkg trigger on ${SYSTEM\_PROFILES\_DIR} in order to run the inheritance resolution every time the policies are modified

### **Directly enrolled packages**

* Directly enrolled packages include:  
  * all identified crypto providers (with the goal of identifying all of them)  
  * voluntary crypto users, i.e. ones for which finer control over the crypto configuration is wanted  
* All packages directly enrolled in this scheme depend on the central crypto-config package  
* These packages must ship a configuration that splits apart its crypto aspects either directly or through drop-ins  
* These packages install at least a dropins collection for the default profile

### **Other packages**

* Other packages are unchanged

## **User-facing tool**

* This specification deals with basic tooling only; other tools can be specified elsewhere or developed without specification, by Canonical or by others  
* The tool can show the user the profile currently in use  
* The tool can show the user the list of available profiles  
* The tool can atomically and safely switch the system between policiesrestart  
* After a profile switch, the tool advises the administrator to reboot the system if necessary.  
* Packages should at least Recommends: this tool. Depending on the package and dependencies size, we might prefer Depends: .  
* The tool shall be written in shell script or Rust since it will be installed on most installations of Ubuntu, even minimal and maybe recovery systems  
* The tool implements at least the command-line interface below:

```
help, -h, --help                                display command-line help

status                                          show the currently-used profile

switch PROFILE                                  switch to PROFILE

query profile-condition       PROFILE           value of `condition` in PROFILE's metadata
query profile-current                           profile currently in use
query profile-description     PROFILE           value of `description` in PROFILE's metadata
query profile-list-all                          list all profiles, including disabled ones
query profile-list                              list profiles
query profile-maintainer      PROFILE           value of `maintainer` in PROFILE's metadata
query profile-parent          PROFILE           value of `parent` in PROFILE's metadata
query profile-path            PROFILE           value of `path` in PROFILE's metadata
query profile-summary         PROFILE           value of `summary` in PROFILE's metadata

query application-path        PROFILE PACKAGE   location of the profile for PACKAGE in
                                                PROFILE with symlinks not resolved
query application-realpath    PROFILE PACKAGE   location of the profile for PACKAGE in
                                                PROFILE with symlinks resolved
query application-realprofile PROFILE PACKAGE   profile that holds the application-realpath
                                                for PROFILE and PACKAGE
```

## **Configuration sealing**

| 💡Writing consistent data in configuration files is wonderful but mostly useless if end-applications don't actually use it\! Applications too often don't follow whatever configuration exists for their crypto provider and have their own instead. |
| :---- |

* Every crypto user must initialize its crypto providers' crypto settings to either their system defaults or to a value that is set in one of the crypto user's configuration files  
  * NB: some applications (namely OpenConnect) disable system configuration on purpose and will need patches unless a proper solution is devised with upstream  
* When a configuration change that would lower the security of crypto settings (as in the sense expressed in [Annex: bits of security, comparable algorithm strengths, and boiling oceans](#annex:-bits-of-security,-comparable-algorithm-strengths,-and-boiling-oceans)), crypto providers must make it possible through their configuration, to ignore the requested changes, and either: a) return an error if the already-existing API and code make it possible, b) return success, or c) abort the application.  
* Crypto providers must log calls to APIs that achieve any of the aforementioned.  
* Apparmor profiles must allow access to the relevant configuration files.  
  * NB: starting with Noble Numbat and as a consequence of the work on this topic, the default apparmor configuration allows access to openssl and gnutls; this will be expanded to include relevant directories as this work moves forward.

## **Statically-linked crypto providers**

| 💡This section is spurred by Go and Rust which use static-linking and have their own implementations of crypto providers. The issue is more general however: every statically-linked crypto provider raises the same issue. |
| :---- |

Static linking is used to de-correlate an application from the system's shared libraries. In Ubuntu, configuration is linked to the system's shared libraries. It follows that static linking de-correlates applications from the system's configuration.

Indeed, an application could ship any version of any crypto provider from the past or from the future relative to a Ubuntu version, and the configuration format, values and consequences could be vastly different.

The issue would be somewhat better if configuration files were versioned but this is rarely the case.

* Statically-linked crypto providers are out of scope.

# **Affected packages**

## **Crypto providers**

Crypto providers include at least the following:

* GnuTLS  
* Kerberos  
* libssh  
* NSS  
* OpenJDK  
* OpenSSH  
* OpenSSL  
* StrongSwan  
* cryptopp  
* gnupg  
* libgcrypt  
* libnacl  
* libsodium  
* libtomcrypt  
* mbedtls  
* nettle  
* paramiko  
* postgresql  
* sequoia  
* wireguard  
* wolfssl

There are also providers that are probably not directly affected by this:

* linux  
* openzfs

We should also keep in mind packages that expose some hardware's acceleration features.

## **Crypto users**

There is no fixed list of crypto users to enroll into this scheme, only guidelines.

The following can be taken into account in order to decide if a package (be it in main or in universe) should be effectively enrolled and with which priority.

* Depends on a crypto provider through a dynamic library, an executable or an RPC  
* Would behave differently across profiles if enrolled  
* Would bring value to users by being enrolled

There is no guaranteed criteria indeed: if 'md5sum' were relying on gnutls or openssl, it would be a crypto user but enrolling it wouldn't bring any value to users since its behavior would have to remain forever identical, including across profiles.

The most effective criteria is the Depends and Recommends of a package as with the following command-line filter:  
grep-dctrl \-e 'libssl3|libnss3|libgnutls30|libssh-4|libkrb' \-FDepends \-FRecommends \\  
| grep-dctrl \-v \-e 'universe' \-FSection \-sPackage

and with this command:

(for i in nss gnutls28 openssl; do reverse-depends \-b src:$i \-c main \-l ; done) | sort | uniq

These two approaches yield respectively 256 packages in main and 1305 with universe, and 162 packages in main and 1027 with universe. These two approaches are not directly comparable and both should be used.

It is important to acknowledge that there will be gaps simply due to the number of packages involved and this is why it is important to take the value for users into account.

# **Example: usefulness with krb5**

Until version krb5 1.20-1, the kdc.conf file shipped with Ubuntu included the following:

master\_key\_type \= des3-hmac-sha1

As far as I understand, this was fixed because the krb5\_newrealm outputs a deprecation warning. We would have appreciated that this was found earlier on.

Had this configuration been split into a dedicated file and organized into profiles, it would have been trivial to find it in an automated fashion. Indeed, it would be very easy to automatically extract all files that make up profiles across the archive and analyze them either automatically, or by hand.

Obviously the work leading to splitting this from configuration files would also have been able to surface this issue but this depends on when the splitting is done and what is considered secure at that time.

# **Known Limitations and Future work**

## **Known Limitations**

### **Missing configuration snippets for a software package when using a user-defined profile**

One can imagine situations where the default profile starts using a new configuration snippet, other profiles are not updated to use it.

This is only a small issue for profiles provided by the distribution. It is not actually a new problem: it merely adds new files and paths to manage when updating packages.

This is more an issue for user-designed profiles since they are not updated at the same time. At the moment, user-designed profiles are not supported so we have time to identify issues like this one and address them.

In the future, we can imagine tooling that will identify such issues ahead of time by comparing the set of files in each snippet in order to warn of missing files.

## **Future work**

### **Creating dh\_crypto\_config and lintian checks**

At the moment, the installation of profiles in packages is manual. They will typically be added under debian/conf/ and listed in debian/foo.install. One could imagine a dh\_crypto\_config that would install profiles detected under debian/crypto-config . Similarly, lintian could be used to ensure the profiles in a package are consistent.

It is probably too early to add a lot of automation and it is probably better to first get more real-world experience.

Also, debhelper and lintian are written in perl.

### **Autopkgtest**

Profiles change the behavior not only of the package but also of the system. It will be useful to run autopkgtests under various profiles. Maybe the only requirement will be to run tests in a loop, changing profile and restarting services each time. In any case, actual experience developing packages and profiles will probably be useful first.

Autopkgtests will also be most useful as a way to track upstream changes. Conversely, they will also be most needed when there are more than a few packages effectively enrolled.

# **Further Information**

## **Annex: bits of security, comparable algorithm strengths, and boiling oceans**

The choice of cryptographic algorithms and sizes has long been difficult and there is often no obvious answer when it comes to comparing them. They are also very abstract. Indeed, what does it mean to use AES128? It's the standard symmetric cipher nowadays and no-one is going to be fired for choosing it but why not use AES256 then, and could something else be appropriate too for some given usage scenario and threat model?

A notion that has taken hold is the bits of security: some algorithm is said to provide N bits of security at a given time with the current scientific knowledge (breakthroughs are rare but can change everything). Bits of security is still an abstract notion but can be applied to most algorithms, if not all. This leads to results such as SHA1 provides X bits of security, SHA256 provides Y bits of security and BLAKE2b-512 provides Z bits of security. This doesn't apply across different kinds of cryptographic algorithms but this is not an issue in practice since different usage scenarios require different kinds of such algorithms anyway.

[NIST \- Recommendation for Key Management](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-57pt1r5.pdf) : 5.6 Guidance for Cryptographic Algorithm and Key-Size Selection

It is worth mentioning [Universal security from bits and mips to pools, lakes – and beyond](https://eprint.iacr.org/2013/635.pdf) by Lenstra et al in 2013\. The authors build humorous comparisons between cracking cryptographic algorithms at various bit-lengths and the volume of water that would be boiled with the same amount of energy.

## **Annex: Possible work split**

Below is a rough outline for one of the possible development plans. There are two or three major batches. Tasks will sometimes be done in parallel.

### **Tests**

Tests include tools such as sslscan, ssh-audit or cryptolizer which scan a server in order to retrieve the cryptography algorithms and protocols it supports or offers. Results can be used to build a database of effective configurations that will serve for testsuite and documentation purposes.

Tests are immediately useful as they help prevent regressions and help us know the current behaviors better.

* Benefit: gain confidence in what the tool does  
* Risk: none  
* Cost: average for the infrastructure and low to average for each package but multiplied by the number of packages even though some tests could be shared (e.g. sslscan for every http daemon)  
* Note: don't try perfect coverage as that would require too much time  
* Batch: all batches

### **System administration tool**

Once there are alternative configuration sets, the tool becomes useful

* Benefit: none at that point  
* Risk: none since it is not being used  
* Cost: low because the MVP code is very small simple and the rest is bigger but still simple  
* Batch: 1

### **Crypto configuration split to separate files**

Move the relevant configuration entries to separate files in the proper location and refer to them through ‘include’ directives

* Benefit: immediately more readable  
* Risk: low since no configuration change is expected and we have tests  
* Cost: small for each package but many packages to handle; however there is no requirement to do everything at once  
* Cost: if we can not get Debian on board to follow this might cause quite some additional regular maintenance/merge/update effort due to the delta that we have to carry for as long as we want to support this solution.  
* Batch: all batches  
* Potential order of porting (detailed schedule to be done as part of usual planning tasks):  
1. Crypto providers that aren’t crypto users: openssl, gnutls, …  
2. HTTPS servers (nginx, apache, lighttpd) not in Rust, Go or Java  
3. HTTPS servers but also including some in Rust, Go or Java  
4. Virtualization (encryption of guest management via libvirt, encryption of migration, encryption of things on disk with luks, ...) to be of interest to e.g. disable a particular set of algorithms.  
5. VPN everything (tend to be quite complex)  
6. Everything else

### **Alternative profiles**

Design and provide our first alternatives and ensure everything of interest is covered

* Benefit: possible to begin experimenting  
* Risk: none since nothing is activated  
* Cost: low to average for each package; realistically, only around three such file sets will be created and half a dozen at most  
* Batch: all batches

### **Restrictions through crypto providers**

Restrict which configurations can be used by crypto users by acting only on crypto providers

* Benefit: more certainty that configuration choices are effective system-wide  
* Risk: patching some libraries (e.g. openssl) will be needed  
* Cost: high for packages that require patching (involves analysis, writing the code, ensuring configuration is not broken, ABI isn't changed, and upstreaming the changes)  
* Batch: starting from batch 2

### **Configuration sealing**

This step involves detecting packages that do not use configuration files and patches to stop this behavior

* Benefit: this is a mandatory step to ensure our configuration of the crypto providers is actually in effect  
* Risk: low to average since it seems to not be an existing concern and Fedora already does it without trouble it seem but patching crypto providers might be required  
* Cost: low for each package but there will can be several packages to change and the tools will take some time to write and integrate  
* Batch: starting from batch 3

## **Annex: relationship with distribution upstreams (Debian) and downstreams (Ubuntu derivatives)**

Every modification of a crypto provider or user has a maintenance cost. Most modifications will be small and that cost will correspondingly be low.  
However, a number of the affected packages are certainly without Ubuntu modification currently and are therefore synced. Introducing changes will require merging the packages from Debian, costing time and attention. The large number of changes is a strong incentive to upstream them.

Will Debian generally accept these changes, and what differences will forever remain between the two distributions?

### **Forever differences**

Ubuntu and Debian each have their specificities. It is very unlikely they will have the same configurations for cryptography, partly due to the differences with regard to how decisions are made in both distributions.

Keep in mind that having different profiles does not mean they cannot be stored in the same place however. There is support in crypto-config for hiding profiles based on criteria, which makes it possible to include both Debian and Ubuntu profiles in Debian but only show the relevant ones at runtime, thus easing inclusion in Debian.

### **Upstreaming changes**

The per-package changes rely on dropins which are uncontroversial. The paths they use only make sense when crypto-config is installed. As a consequence, at least the plumbing and minimal UI of crypto-config must be uploaded and maintained in Debian. Since crypto-config doesn't require special treatment or specific changes by itself, this does not cause specific concerns.

Even when users do not install crypto-config, Debian maintainers can benefit from several of the [user stories outlined in this document](#user-stories), in particular to express their intent regarding their package's configuration, especially to expose upcoming changes.

We must keep in mind that some maintainers will refuse to include such changes, possibly on non-technical grounds. Hopefully, crypto-config's lightness, optionality and non-disruptiveness will alleviate concerns. It should be noted that it will be scrutinized too for its inclusion in Ubuntu main ultimately: crypto-config will first be in universe but be used to configure packages from main and this will work without component-mismatches, proving this will not lead to any kind of covert takeover.

## **Annex: Example report following package install or removal**

I made up the file hierarchy: /distro is where the distribution installs its files while /user is where the user can modify files. The profile modification is not supported but this shows it's not a limitation of the script or of the approach but about reconciling them with dpkg.

```
**************************
*   APPLICATION STATUS   *
**************************

installed  =>  nginx
removed    =>  lighttpd

**************************
*    PROFILES STATUS     *
**************************

distro/crypto-config:default:nginx  =>  distro/crypto-config/default/nginx
distro/crypto-config:test1:nginx    =>  distro/crypto-config/default/nginx
distro/crypto-config:test2:nginx    =>  distro/crypto-config/default/nginx
user/crypto-config:test:nginx       =>  distro/crypto-config/default/nginx

**************************
*     PROFILE TREES      *
**************************

distro/crypto-config/test1/nginx     ->  ../default/nginx
distro/crypto-config/test2/nginx     ->  ../test1/nginx
user/crypto-config/test/nginx        ->  /distro/crypto-config/test2/nginx
```

# **References**

[Fedora Packaging Policy for Crypto](https://docs.fedoraproject.org/en-US/packaging-guidelines/CryptoPolicies/)  
[NIST \- Recommendation for Key Management](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-57pt1r5.pdf) and especially section 5.6 Guidance for Cryptographic Algorithm and Key-Size Selection  
[Universal security from bits and mips to pools, lakes – and beyond](https://eprint.iacr.org/2013/635.pdf) by Lenstra et al  
[TLS versions and ciphers used for inbound SMTP connections at Toronto University](https://utcc.utoronto.ca/~cks/space/blog/spam/TLSExternalTypes-2023-04)
