use expect_test::{expect, Expect};

#[test]
fn t() {
    assert!(wn::lmf::from_file("this-file-does-not-exist.xml").is_err());

    let root = wn::lmf::from_file("tests/fixtures/wordnet.xml").unwrap();

    assert_eq!(root.lexicons.len(), 1);

    let mut lexicon = root.lexicons.into_vec().remove(0);

    assert_eq!(&*lexicon.id, "ewn");
    assert_eq!(&*lexicon.label, "English WordNet");
    assert_eq!(&*lexicon.language, "en");
    assert_eq!(&*lexicon.email, "english-wordnet@googlegroups.com");
    assert_eq!(
        &*lexicon.license,
        "https://creativecommons.org/licenses/by/4.0/"
    );
    assert_eq!(&*lexicon.version, "2020");
    assert_eq!(
        &*lexicon.url,
        "https://github.com/globalwordnet/english-wordnet"
    );

    assert_eq!(lexicon.lexical_entries.len(), 163087);
    assert_eq!(lexicon.synsets.len(), 120053);

    fn check<T: std::fmt::Debug>(t: T, expect: Expect) {
        expect.assert_debug_eq(&t)
    }

    lexicon.lexical_entries.sort_by(|a, b| a.id.cmp(&b.id));
    lexicon.synsets.sort_by(|a, b| a.id.cmp(&b.id));

    check(
        &lexicon.lexical_entries[..5],
        expect![[r#"
            [
                LexicalEntry {
                    id: "ewn--ap-hood-n",
                    lemma: Lemma {
                        written_form: "\'hood",
                        part_of_speech: Noun,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn--ap-hood-n-08242255-02",
                            synset: "ewn-08242255-n",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn--ap-s_Gravenhage-n",
                    lemma: Lemma {
                        written_form: "\'s Gravenhage",
                        part_of_speech: Noun,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn--ap-s_Gravenhage-n-08970180-02",
                            synset: "ewn-08970180-n",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn--ap-tween-r",
                    lemma: Lemma {
                        written_form: "\'tween",
                        part_of_speech: Adverb,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn--ap-tween-r-00252367-02",
                            synset: "ewn-00252367-r",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn--ap-tween_decks-r",
                    lemma: Lemma {
                        written_form: "\'tween decks",
                        part_of_speech: Adverb,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn--ap-tween_decks-r-00500491-02",
                            synset: "ewn-00500491-r",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-.22-caliber-a",
                    lemma: Lemma {
                        written_form: ".22-caliber",
                        part_of_speech: Adjective,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn-.22-caliber-a-03157978-02",
                            synset: "ewn-03157978-a",
                            relations: [
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "ewn-caliber-n-05110583-03",
                                },
                            ],
                        },
                    ],
                },
            ]
        "#]],
    );

    check(
        &lexicon.lexical_entries[lexicon.lexical_entries.len() - 5..],
        expect![[r#"
            [
                LexicalEntry {
                    id: "ewn-zymolysis-n",
                    lemma: Lemma {
                        written_form: "zymolysis",
                        part_of_speech: Noun,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn-zymolysis-n-13596636-02",
                            synset: "ewn-13596636-n",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-zymolytic-a-03011955-02",
                                },
                            ],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-zymolytic-a",
                    lemma: Lemma {
                        written_form: "zymolytic",
                        part_of_speech: Adjective,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn-zymolytic-a-03011955-02",
                            synset: "ewn-03011955-a",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-zymolysis-n-13596636-02",
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "ewn-zymosis-n-13596636-01",
                                },
                            ],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-zymosis-n",
                    lemma: Lemma {
                        written_form: "zymosis",
                        part_of_speech: Noun,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn-zymosis-n-13596636-01",
                            synset: "ewn-13596636-n",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-zymotic-a-03011955-01",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-zymosis-n-13596429-01",
                            synset: "ewn-13596429-n",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-zymotic-a-03011849-01",
                                },
                            ],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-zymotic-a",
                    lemma: Lemma {
                        written_form: "zymotic",
                        part_of_speech: Adjective,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn-zymotic-a-03011955-01",
                            synset: "ewn-03011955-a",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-zymosis-n-13596636-01",
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "ewn-zymosis-n-13596636-01",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-zymotic-a-03011849-01",
                            synset: "ewn-03011849-a",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-zymosis-n-13596429-01",
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "ewn-zymosis-n-13596429-01",
                                },
                            ],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-zymurgy-n",
                    lemma: Lemma {
                        written_form: "zymurgy",
                        part_of_speech: Noun,
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "ewn-zymurgy-n-06089949-02",
                            synset: "ewn-06089949-n",
                            relations: [],
                        },
                    ],
                },
            ]
        "#]],
    );

    check(
        lexicon
            .lexical_entries
            .iter()
            .filter(|lexical_entry| lexical_entry.forms.len() > 0)
            .take(3)
            .collect::<Vec<_>>(),
        expect![[r#"
            [
                LexicalEntry {
                    id: "ewn-aardwolf-n",
                    lemma: Lemma {
                        written_form: "aardwolf",
                        part_of_speech: Noun,
                    },
                    forms: [
                        Form {
                            written_form: "aardwolves",
                        },
                    ],
                    senses: [
                        Sense {
                            id: "ewn-aardwolf-n-02120828-01",
                            synset: "ewn-02120828-n",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-abacus-n",
                    lemma: Lemma {
                        written_form: "abacus",
                        part_of_speech: Noun,
                    },
                    forms: [
                        Form {
                            written_form: "abaci",
                        },
                    ],
                    senses: [
                        Sense {
                            id: "ewn-abacus-n-02668977-01",
                            synset: "ewn-02668977-n",
                            relations: [],
                        },
                        Sense {
                            id: "ewn-abacus-n-02668826-01",
                            synset: "ewn-02668826-n",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-abet-v",
                    lemma: Lemma {
                        written_form: "abet",
                        part_of_speech: Verb,
                    },
                    forms: [
                        Form {
                            written_form: "abetted",
                        },
                        Form {
                            written_form: "abetting",
                        },
                    ],
                    senses: [
                        Sense {
                            id: "ewn-abet-v-02554908-01",
                            synset: "ewn-02554908-v",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-abetment-n-07266287-01",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-abettal-n-07266287-02",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-abettor-n-09775380-01",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-abetter-n-09775380-02",
                                },
                            ],
                        },
                    ],
                },
            ]
        "#]],
    );

    check(
        &lexicon.synsets[..5],
        expect![[r#"
            [
                Synset {
                    id: "ewn-00001740-a",
                    part_of_speech: Adjective,
                    definitions: [
                        "(usually followed by `to\') having the necessary means or skill or know-how or authority to do something",
                    ],
                    examples: [
                        "\"able to swim\"",
                        "\"she was able to program her computer\"",
                        "\"we were at last able to buy a car\"",
                        "\"able to get a grant for the project\"",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Attribute,
                            target: "ewn-05207437-n",
                        },
                        SynsetRelation {
                            rel_type: Attribute,
                            target: "ewn-05624029-n",
                        },
                    ],
                },
                Synset {
                    id: "ewn-00001740-n",
                    part_of_speech: Noun,
                    definitions: [
                        "that which is perceived or known or inferred to have its own distinct existence (living or nonliving)",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00001930-n",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00002137-n",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-04431553-n",
                        },
                    ],
                },
                Synset {
                    id: "ewn-00001740-r",
                    part_of_speech: Adverb,
                    definitions: [
                        "without musical accompaniment",
                    ],
                    examples: [
                        "\"they performed a cappella\"",
                    ],
                    relations: [],
                },
                Synset {
                    id: "ewn-00001740-v",
                    part_of_speech: Verb,
                    definitions: [
                        "draw air into, and expel out of, the lungs",
                    ],
                    examples: [
                        "\"I can breathe better when the air is clean\"",
                        "\"The patient is respiring\"",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Entails,
                            target: "ewn-00005041-v",
                        },
                        SynsetRelation {
                            rel_type: Entails,
                            target: "ewn-00004227-v",
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target: "ewn-00002325-v",
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target: "ewn-00002573-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00002573-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00002724-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00002942-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00003826-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00004032-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00004227-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00005041-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00006697-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00007328-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00017024-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-00001885-r",
                    part_of_speech: Adverb,
                    definitions: [
                        "in the Christian era; used before dates after the supposed year Christ was born",
                    ],
                    examples: [
                        "\"in AD 200\"",
                    ],
                    relations: [],
                },
            ]
        "#]],
    );

    check(
        &lexicon.synsets[lexicon.synsets.len() - 5..],
        expect![[r#"
            [
                Synset {
                    id: "ewn-92471097-n",
                    part_of_speech: Noun,
                    definitions: [
                        "an art style in late 16th century Europe characterized by spatial incongruity and excessive elongation of the human figures.",
                    ],
                    examples: [
                        "Mannerism favors compositional tension and instability rather than the balance and clarity of earlier Renaissance painting.",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-04936599-n",
                        },
                    ],
                },
                Synset {
                    id: "ewn-92471179-n",
                    part_of_speech: Noun,
                    definitions: [
                        "a surface generated by a moving straight line with the result that through every point on the surface a line can be drawn lying wholly in the surface.",
                    ],
                    examples: [
                        "In algebraic geometry, ruled surfaces were originally defined as projective surfaces in projective space containing a straight line through any given point.",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-04369112-n",
                        },
                    ],
                },
                Synset {
                    id: "ewn-92471253-n",
                    part_of_speech: Noun,
                    definitions: [
                        "a pentagon in which the angles are all equal, and the sides all equal.",
                    ],
                    examples: [
                        "A regular pentagon has five lines of reflectional symmetry, and rotational symmetry of order 5 (through 72°, 144°, 216° and 288°).",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-13904858-n",
                        },
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-13889754-n",
                        },
                    ],
                },
                Synset {
                    id: "ewn-92767020-n",
                    part_of_speech: Noun,
                    definitions: [
                        "an electrical device used to create artificial light by use of an electric lamp; all light fixtures have a fixture body and a light socket to hold the lamp and allow for its replacement",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-04270870-n",
                        },
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-03274312-n",
                        },
                    ],
                },
                Synset {
                    id: "ewn-92767095-n",
                    part_of_speech: Noun,
                    definitions: [
                        "a trained person hired to determine the sex of chicken and other hatchlings.",
                    ],
                    examples: [
                        "Chick sexing is the method of distinguishing the sex of chicken and other hatchlings, usually by a trained person called a chick sexer or chicken sexer.",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-10451389-n",
                        },
                    ],
                },
            ]
        "#]],
    );
}
