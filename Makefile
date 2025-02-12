all: manpages

%.1: %.1.adoc
	asciidoctor -b manpage "$?"

manpages: man/crypto-config.1

clean:
	rm -f man/crypto-config.1
