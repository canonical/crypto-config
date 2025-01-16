# Crypto-config - a framework for system-wide configuration of cryptography

Cryptography is used pervasively in all computers. It protects data in-transit
or at rest and guarantees its authenticity; it includes many algorithms and
protocols with active research to break and improve them.

Decades of research have resulted in numerous improvements but also in the
discovery or exploitation of major flaws and limitations. The ciphers used in
the 90s are wildly inappropriate today and the ciphers used today were at best
brand new in the 90s.

With all these changes, it is probably no surprise to find systems with very
different configurations, sometimes with outdated cryptography or, on the
opposite, unable to use modern cryptography. This often happens even between
various software of a single machine. Indeed, nothing guarantees that OpenSSL
and GnuTLS are configured the same, or that Apache and MariaDB are either.
There can be as many different configurations as there are users of
cryptography.

## Actual deployment of modern cryptography hampered by minor differences

Several years ago, I started using XMPP and setup a personal server that I
configured with modern cryptography and according to best practices. I ensured
that my clients communicated well with it too and everything seemed well.

Except for one thing:  XMPP is a federation of servers and I soon found out
that I couldn't communicate with the server of a friend. Both of use were using
different software which could use elliptic curves cryptography but weren't
using the same curve. I had to switch to a more lenient configuration on my
machine while working with upstreams to have the two software support several
curves.

This was conceptually a very simple issue: two major server implementations
each offered modern cryptography but not one that could actually be used
reliably. Compound this with how long it takes for updates to propagate and
you're looking at least at several years of incompatibility when such
differences happen.

## crypto-config

The difference above can stem either from code or configuration and it is
unfortunately not an isolated issue but closer to the norm currently.

Compared to Nginx, we see that Apache doesn't offer AES128 for TLS 1.2. We also
see that Postgres only uses P-256. MariaDB doesn't offer CCM mode while mysql
doesn't offer ChaCha20-Poly1305. The list is endless.

These differences are not security issues today but they make hardening,
consistency, compliance, analysis and progress more difficult.

Ubuntu's crypto-config framework tackle these issues by providing consistent
configuration profiles that are easy to select from.
It operates at the level of single machines but once you're using the same
configuration profile everywhere, you can be confident that all your clients
and servers can communicate together in a safe and modern manner.

Every piece of software enrolled in the framework reads configuration dropins
which are managed by crypto-config and are changed atomically upon changing
profile.

While we value consistency, we are also taking into account the different needs
of different services and applications. For instance, a mail server or an HTTP
client are unlikely to only use the most modern cryptography due to
interoperability constraints but a web server where the user base is known
(e.g. mobile-only or internal machines) can move forward more easily.

Crypto-config will also help us continue to improve the security of Ubuntu by
making these configurations explicit and visible for everyone to analyze.

## Moving security forward

## Other systems

We are aware of RedHat' crypto-policies that was created years ago; it helped
start discussing this topic for Ubuntu. After carefully analyzing
crypto-policies, we determined that the initial effort involved in adopting it
was at least as large as the effort to develop crypto-config while also
incurring on-going costs due to large differences between distributions. We
also identified aspects that we deemed too complex for the actual needs and
wanted a simpler design that could also be more easily adopted by other
distributions that may wish to do so.

A major technical difference is that crypto-policies relies on a configuration
generator that needs to be implemented for each and every software being
configured while crypto-config uses static files which are written by package
maintainers. We believe our approach is simpler to integrate in the
distribution, especially with a large number of packages to deal with.

Finally, crypto-policies is good at making a system comply with a rule such as
a given cipher being forbidden. However it doesn't appear to be the best fit
for also moving the cryptography of some components further forward like
crypto-config does.
