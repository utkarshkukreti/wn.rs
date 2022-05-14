WORDNET_COMMIT = ad3f7f678fa12e14856f3849b788d49c675c2d77

default: tests/fixtures/wordnet.xml

tests/fixtures/wordnet.xml:
	rm -rf tmp
	mkdir -p tmp tests/fixtures
	cd tmp && curl -OL https://github.com/globalwordnet/english-wordnet/archive/${WORDNET_COMMIT}.zip
	cd tmp && unzip ${WORDNET_COMMIT}.zip
	cd tmp && cd english-wordnet-${WORDNET_COMMIT} && python scripts/from-yaml.py && python scripts/merge.py
	cd tmp && mv english-wordnet-${WORDNET_COMMIT}/wn.xml ../$@
	rm -rf tmp
