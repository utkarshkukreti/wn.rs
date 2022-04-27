WORDNET_COMMIT = ad2320386b1589dca63c5d45071de5082ba58e40

default: tests/fixtures/wordnet.xml

tests/fixtures/wordnet.xml:
	rm -rf tmp
	mkdir -p tmp tests/fixtures
	cd tmp && curl -OL https://github.com/globalwordnet/english-wordnet/archive/${WORDNET_COMMIT}.zip
	cd tmp && unzip ${WORDNET_COMMIT}.zip
	cd tmp && cd english-wordnet-${WORDNET_COMMIT} && python scripts/merge.py
	cd tmp && mv english-wordnet-${WORDNET_COMMIT}/wn.xml ../$@
	rm -rf tmp
