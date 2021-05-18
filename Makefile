WORDNET_COMMIT = 9f666f8a37cde9375224228e63f171c03ca6c9ee

default: tests/fixtures/wordnet.xml

tests/fixtures/wordnet.xml:
	rm -rf tmp
	mkdir -p tmp tests/fixtures
	cd tmp && curl -OL https://github.com/globalwordnet/english-wordnet/archive/${WORDNET_COMMIT}.zip
	cd tmp && unzip ${WORDNET_COMMIT}.zip
	cd tmp && cd english-wordnet-${WORDNET_COMMIT} && python scripts/merge.py
	cd tmp && mv english-wordnet-${WORDNET_COMMIT}/wn.xml ../$@
	rm -rf tmp
