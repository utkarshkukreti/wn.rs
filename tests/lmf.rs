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
}
