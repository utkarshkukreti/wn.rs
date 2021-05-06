use expect_test::{expect, Expect};

#[test]
fn t() {
    assert!(wn::lmf::from_file("this-file-does-not-exist.xml").is_err());

    let mut root = wn::lmf::from_file("tests/fixtures/wordnet.xml").unwrap();

    assert_eq!(root.lexicons.len(), 1);

    let lexicon = root.lexicons.remove(0);

    assert_eq!(lexicon.id, "ewn");
    assert_eq!(lexicon.label, "English WordNet");
    assert_eq!(lexicon.language, "en");
    assert_eq!(lexicon.email, "english-wordnet@googlegroups.com");
    assert_eq!(
        lexicon.license,
        "https://creativecommons.org/licenses/by/4.0/"
    );
    assert_eq!(lexicon.version, "2020");
    assert_eq!(
        lexicon.url,
        "https://github.com/globalwordnet/english-wordnet"
    );

    assert_eq!(lexicon.lexical_entries.len(), 163087);
    assert_eq!(lexicon.synsets.len(), 120053);

    fn check<T: std::fmt::Debug>(t: T, expect: Expect) {
        expect.assert_debug_eq(&t)
    }

    check(
        &lexicon.lexical_entries[..5],
        expect![[r#"
            [
                LexicalEntry {
                    id: "ewn-fritter_away-v",
                    lemma: Lemma {
                        written_form: "fritter away",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-fritter_away-v-01198337-05",
                            synset: "ewn-01198337-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-patronage-v",
                    lemma: Lemma {
                        written_form: "patronage",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-patronage-v-01187544-01",
                            synset: "ewn-01187544-v",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-patronage-n-01156356-01",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-patronage-v-00910574-03",
                            synset: "ewn-00910574-v",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-patronage-n-01098359-02",
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-patronage-n-08418511-02",
                                },
                            ],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-gluttonise-v",
                    lemma: Lemma {
                        written_form: "gluttonise",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-gluttonise-v-01171347-02",
                            synset: "ewn-01171347-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-lap-v",
                    lemma: Lemma {
                        written_form: "lap",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-lap-v-02698039-01",
                            synset: "ewn-02698039-v",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-lap-n-03647020-01",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-lap-v-01434809-02",
                            synset: "ewn-01434809-v",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-lap-n-00151411-02",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-lap-v-02192644-01",
                            synset: "ewn-02192644-v",
                            relations: [],
                        },
                        Sense {
                            id: "ewn-lap-v-01173263-01",
                            synset: "ewn-01173263-v",
                            relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target: "ewn-lap-n-00151411-02",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-lap-v-00218002-02",
                            synset: "ewn-00218002-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-sate-v",
                    lemma: Lemma {
                        written_form: "sate",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-sate-v-01196254-02",
                            synset: "ewn-01196254-v",
                            relations: [],
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
                    id: "ewn-overclock-v",
                    lemma: Lemma {
                        written_form: "overclock",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-overclock-v-91001651-01",
                            synset: "ewn-91001651-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-photobomb-v",
                    lemma: Lemma {
                        written_form: "photobomb",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-photobomb-v-90007761-01",
                            synset: "ewn-90007761-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-switch_up-v",
                    lemma: Lemma {
                        written_form: "switch up",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-switch_up-v-90020001-01",
                            synset: "ewn-90020001-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-unfriend-v",
                    lemma: Lemma {
                        written_form: "unfriend",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-unfriend-v-90016131-01",
                            synset: "ewn-90016131-v",
                            relations: [],
                        },
                    ],
                },
                LexicalEntry {
                    id: "ewn-untag-v",
                    lemma: Lemma {
                        written_form: "untag",
                        part_of_speech: Verb,
                    },
                    senses: [
                        Sense {
                            id: "ewn-untag-v-90004911-01",
                            synset: "ewn-90004911-v",
                            relations: [],
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
                    id: "ewn-01159300-v",
                    definitions: [
                        "serve oneself to, or consume regularly",
                    ],
                    examples: [
                        "\"Have another bowl of chicken soup!\"",
                        "\"I don\'t take sugar in my coffee\"",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01164607-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01167359-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01168667-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01172332-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01173463-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01174998-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01182162-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01196254-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01197832-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01200618-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01208079-v",
                        },
                        SynsetRelation {
                            rel_type: IsEntailedBy,
                            target: "ewn-01179415-v",
                        },
                        SynsetRelation {
                            rel_type: IsEntailedBy,
                            target: "ewn-01199976-v",
                        },
                        SynsetRelation {
                            rel_type: IsEntailedBy,
                            target: "ewn-01206641-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-01159815-v",
                    definitions: [
                        "use up (resources or materials)",
                    ],
                    examples: [
                        "\"this car consumes a lot of gas\"",
                        "\"We exhausted our savings\"",
                        "\"They run through 20 bottles of wine a week\"",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-02271905-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-00562791-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01160320-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01194178-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01207620-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-02272374-v",
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target: "ewn-02272834-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-02285714-v",
                        },
                        SynsetRelation {
                            rel_type: IsEntailedBy,
                            target: "ewn-01160888-v",
                        },
                        SynsetRelation {
                            rel_type: IsEntailedBy,
                            target: "ewn-01198337-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-01160320-v",
                    definitions: [
                        "deplete of resources",
                    ],
                    examples: [
                        "\"The exercise class drains me of energy\"",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-01159815-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-01160479-v",
                    definitions: [
                        "spend extravagantly",
                    ],
                    examples: [
                        "\"waste not, want not\"",
                    ],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-02271905-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01174572-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01198337-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-01207423-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-02269409-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-02271162-v",
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target: "ewn-02273848-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-01160800-v",
                    definitions: [
                        "use frugally or carefully",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-01160888-v",
                        },
                    ],
                },
            ]
        "#]],
    );

    check(
        &lexicon.synsets[lexicon.synsets.len() - 5..],
        expect![[r#"
            [
                Synset {
                    id: "ewn-91001651-v",
                    definitions: [
                        "To run a processor (CPU), or any electronic logic device, at a speed higher than is recommended by the manufacturer.",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-00230031-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-90007761-v",
                    definitions: [
                        "to jump into someone\'s photo",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-00521099-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-90020001-v",
                    definitions: [
                        "to change something temporarily",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-00380830-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-90016131-v",
                    definitions: [
                        "To remove someone from your list of friends on a social network",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-00173351-v",
                        },
                    ],
                },
                Synset {
                    id: "ewn-90004911-v",
                    definitions: [
                        "to remove a metadata tag from an internet post",
                    ],
                    examples: [],
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target: "ewn-00572138-v",
                        },
                    ],
                },
            ]
        "#]],
    );
}
