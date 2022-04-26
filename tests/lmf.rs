use expect_test::{expect, Expect};

#[test]
fn t() {
    assert!(wn::lmf::from_file("this-file-does-not-exist.xml").is_err());

    let root = wn::lmf::from_file("tests/fixtures/wordnet.xml").unwrap();

    assert_eq!(root.lexicons.len(), 1);

    let mut lexicon = root.lexicons.into_vec().remove(0);

    assert_eq!(&*lexicon.id, "oewn");
    assert_eq!(&*lexicon.label, "Open English WordNet");
    assert_eq!(&*lexicon.language, "en");
    assert_eq!(&*lexicon.email, "english-wordnet@googlegroups.com");
    assert_eq!(
        &*lexicon.license,
        "https://creativecommons.org/licenses/by/4.0/"
    );
    assert_eq!(&*lexicon.version, "2021");
    assert_eq!(
        &*lexicon.url,
        "https://github.com/globalwordnet/english-wordnet"
    );

    assert_eq!(lexicon.lexical_entries.len(), 163161);
    assert_eq!(lexicon.synsets.len(), 120039);

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
                    id: "oewn--ap-hood-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'hood",
                        part_of_speech: Noun,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-hood__1.14.01..",
                            synset_id: "oewn-08242255-n",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn--ap-s_Gravenhage-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'s Gravenhage",
                        part_of_speech: Noun,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-s_gravenhage__1.15.00..",
                            synset_id: "oewn-08970180-n",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn--ap-tween-r",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'tween",
                        part_of_speech: Adverb,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-tween__4.02.00..",
                            synset_id: "oewn-00252367-r",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn--ap-tween_decks-r",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'tween decks",
                        part_of_speech: Adverb,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-tween_decks__4.02.00..",
                            synset_id: "oewn-00500491-r",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-.22-caliber-a",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: ".22-caliber",
                        part_of_speech: Adjective,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-.22-caliber__3.01.00..",
                            synset_id: "oewn-03157978-a",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "oewn-caliber__1.07.01..",
                                },
                            ],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
            ]
        "#]],
    );

    check(
        &lexicon.lexical_entries[lexicon.lexical_entries.len() - 5..],
        expect![[r#"
            [
                LexicalEntry {
                    id: "oewn-zymolysis-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymolysis",
                        part_of_speech: Noun,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymolysis__1.22.00..",
                            synset_id: "oewn-13596636-n",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-zymolytic__3.01.00..",
                                },
                            ],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymolytic-a",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymolytic",
                        part_of_speech: Adjective,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymolytic__3.01.00..",
                            synset_id: "oewn-03011955-a",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-zymolysis__1.22.00..",
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "oewn-zymosis__1.22.01..",
                                },
                            ],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymosis-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymosis",
                        part_of_speech: Noun,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymosis__1.22.01..",
                            synset_id: "oewn-13596636-n",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-zymotic__3.01.01..",
                                },
                            ],
                            examples: [],
                        },
                        Sense {
                            id: "oewn-zymosis__1.22.00..",
                            synset_id: "oewn-13596429-n",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-zymotic__3.01.00..",
                                },
                            ],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymotic-a",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymotic",
                        part_of_speech: Adjective,
                        pronunciations: [
                            Pronunciation {
                                variety: Some(
                                    "GB",
                                ),
                                notation: None,
                                audio: None,
                                value: "zaɪˈmɒtɪk",
                            },
                        ],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymotic__3.01.01..",
                            synset_id: "oewn-03011955-a",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-zymosis__1.22.01..",
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "oewn-zymosis__1.22.01..",
                                },
                            ],
                            examples: [],
                        },
                        Sense {
                            id: "oewn-zymotic__3.01.00..",
                            synset_id: "oewn-03011849-a",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-zymosis__1.22.00..",
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target: "oewn-zymosis__1.22.00..",
                                },
                            ],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymurgy-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymurgy",
                        part_of_speech: Noun,
                        pronunciations: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymurgy__1.09.00..",
                            synset_id: "oewn-06089949-n",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
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
                    id: "oewn-aardwolf-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "aardwolf",
                        part_of_speech: Noun,
                        pronunciations: [
                            Pronunciation {
                                variety: Some(
                                    "GB",
                                ),
                                notation: None,
                                audio: None,
                                value: "ˈɑːdˌwʊlf",
                            },
                            Pronunciation {
                                variety: Some(
                                    "US",
                                ),
                                notation: None,
                                audio: None,
                                value: "ˈɑɹd.ˌwʊlf",
                            },
                        ],
                    },
                    forms: [
                        Form {
                            written_form: "aardwolves",
                        },
                    ],
                    senses: [
                        Sense {
                            id: "oewn-aardwolf__1.05.00..",
                            synset_id: "oewn-02120828-n",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-abacus-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "abacus",
                        part_of_speech: Noun,
                        pronunciations: [
                            Pronunciation {
                                variety: None,
                                notation: None,
                                audio: None,
                                value: "ˈæbəkəs",
                            },
                        ],
                    },
                    forms: [
                        Form {
                            written_form: "abaci",
                        },
                    ],
                    senses: [
                        Sense {
                            id: "oewn-abacus__1.06.01..",
                            synset_id: "oewn-02668977-n",
                            senses_relations: [],
                            examples: [],
                        },
                        Sense {
                            id: "oewn-abacus__1.06.00..",
                            synset_id: "oewn-02668826-n",
                            senses_relations: [],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-abet-v",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "abet",
                        part_of_speech: Verb,
                        pronunciations: [
                            Pronunciation {
                                variety: None,
                                notation: None,
                                audio: None,
                                value: "əˈbɛt",
                            },
                        ],
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
                            id: "oewn-abet__2.41.00..",
                            synset_id: "oewn-02554908-v",
                            senses_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-abetment__1.10.00..",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-abettal__1.10.00..",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-abettor__1.18.00..",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "oewn-abetter__1.18.00..",
                                },
                            ],
                            examples: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
            ]
        "#]],
    );

    check(
        &lexicon.synsets[..5],
        expect![[r#"
            [
                Synset {
                    id: "oewn-00001740-a",
                    part_of_speech: Adjective,
                    definitions: [
                        "(usually followed by `to') having the necessary means or skill or know-how or authority to do something",
                    ],
                    ili_definition: None,
                    examples: [
                        "able to swim",
                        "she was able to program her computer",
                        "we were at last able to buy a car",
                        "able to get a grant for the project",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Attribute,
                            target: "oewn-05207437-n",
                        },
                        SynsetRelation {
                            rel_type: Attribute,
                            target: "oewn-05624029-n",
                        },
                    ],
                },
                Synset {
                    id: "oewn-00001740-n",
                    part_of_speech: Noun,
                    definitions: [
                        "that which is perceived or known or inferred to have its own distinct existence (living or nonliving)",
                    ],
                    ili_definition: None,
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00001930-n",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00002137-n",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-04431553-n",
                        },
                    ],
                },
                Synset {
                    id: "oewn-00001740-r",
                    part_of_speech: Adverb,
                    definitions: [
                        "without musical accompaniment",
                    ],
                    ili_definition: None,
                    examples: [
                        "they performed a cappella",
                    ],
                    relations: [],
                },
                Synset {
                    id: "oewn-00001740-v",
                    part_of_speech: Verb,
                    definitions: [
                        "draw air into, and expel out of, the lungs",
                    ],
                    ili_definition: None,
                    examples: [
                        "I can breathe better when the air is clean",
                        "The patient is respiring",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Entails,
                            target: "oewn-00005041-v",
                        },
                        SynsetRelation {
                            rel_type: Entails,
                            target: "oewn-00004227-v",
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target: "oewn-00002325-v",
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target: "oewn-00002573-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00002573-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00002724-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00002942-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00003826-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00004032-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00004227-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00005041-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00006697-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00007328-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "oewn-00017024-v",
                        },
                    ],
                },
                Synset {
                    id: "oewn-00001885-r",
                    part_of_speech: Adverb,
                    definitions: [
                        "in the Christian era; used before dates after the supposed year Christ was born",
                    ],
                    ili_definition: None,
                    examples: [
                        "in AD 200",
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
                    id: "oewn-92471097-n",
                    part_of_speech: Noun,
                    definitions: [
                        "an art style in late 16th century Europe characterized by spatial incongruity and excessive elongation of the human figures.",
                    ],
                    ili_definition: Some(
                        "an art style in late 16th century Europe characterized by spatial incongruity and excessive elongation of the human figures.",
                    ),
                    examples: [
                        "Mannerism favors compositional tension and instability rather than the balance and clarity of earlier Renaissance painting.",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-04936599-n",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92471179-n",
                    part_of_speech: Noun,
                    definitions: [
                        "a surface generated by a moving straight line with the result that through every point on the surface a line can be drawn lying wholly in the surface.",
                    ],
                    ili_definition: Some(
                        "a surface generated by a moving straight line with the result that through every point on the surface a line can be drawn lying wholly in the surface.",
                    ),
                    examples: [
                        "In algebraic geometry, ruled surfaces were originally defined as projective surfaces in projective space containing a straight line through any given point.",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-04369112-n",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92471253-n",
                    part_of_speech: Noun,
                    definitions: [
                        "a pentagon in which the angles are all equal, and the sides all equal.",
                    ],
                    ili_definition: Some(
                        "a pentagon in which the angles are all equal, and the sides all equal.",
                    ),
                    examples: [
                        "A regular pentagon has five lines of reflectional symmetry, and rotational symmetry of order 5 (through 72°, 144°, 216° and 288°).",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-13904858-n",
                        },
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-13889754-n",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92767020-n",
                    part_of_speech: Noun,
                    definitions: [
                        "an electrical device used to create artificial light by use of an electric lamp; all light fixtures have a fixture body and a light socket to hold the lamp and allow for its replacement",
                    ],
                    ili_definition: Some(
                        "an electrical device used to create artificial light by use of an electric lamp; all light fixtures have a fixture body and a light socket to hold the lamp and allow for its replacement",
                    ),
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-04270870-n",
                        },
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-03274312-n",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92767095-n",
                    part_of_speech: Noun,
                    definitions: [
                        "a trained person hired to determine the sex of chicken and other hatchlings.",
                    ],
                    ili_definition: Some(
                        "a trained person hired to determine the sex of chicken and other hatchlings.",
                    ),
                    examples: [
                        "Chick sexing is the method of distinguishing the sex of chicken and other hatchlings, usually by a trained person called a chick sexer or chicken sexer.",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "oewn-10451389-n",
                        },
                    ],
                },
            ]
        "#]],
    );
}
