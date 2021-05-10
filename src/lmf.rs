pub type S = Box<str>;

pub type V<T> = Box<[T]>;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Root {
    #[serde(rename = "Lexicon")]
    pub lexicons: V<Lexicon>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lexicon {
    pub id: S,
    pub label: S,
    pub language: S,
    pub email: S,
    pub license: S,
    pub version: S,
    pub url: S,
    #[serde(rename = "LexicalEntry")]
    pub lexical_entries: V<LexicalEntry>,
    #[serde(rename = "Synset")]
    pub synsets: V<Synset>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: S,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Sense")]
    pub senses: V<Sense>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: S,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub enum PartOfSpeech {
    #[serde(rename = "a")]
    Adjective,
    #[serde(rename = "s")]
    AdjectiveSatellite,
    #[serde(rename = "r")]
    Adverb,
    #[serde(rename = "n")]
    Noun,
    #[serde(rename = "v")]
    Verb,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Sense {
    pub id: S,
    pub synset: S,
    #[serde(rename = "SenseRelation", default)]
    pub relations: V<SenseRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SenseRelation {
    #[serde(rename = "relType")]
    pub rel_type: SenseRelationType,
    pub target: S,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SenseRelationType {
    Also,
    Antonym,
    Derivation,
    DomainRegion,
    DomainTopic,
    Exemplifies,
    HasDomainRegion,
    HasDomainTopic,
    IsExemplifiedBy,
    Participle,
    Pertainym,
    Similar,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Synset {
    pub id: S,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Definition")]
    pub definitions: V<S>,
    #[serde(rename = "Example", default)]
    pub examples: V<S>,
    #[serde(rename = "SynsetRelation", default)]
    pub relations: V<SynsetRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SynsetRelation {
    #[serde(rename = "relType")]
    pub rel_type: SynsetRelationType,
    pub target: S,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynsetRelationType {
    Also,
    Attribute,
    Causes,
    DomainRegion,
    DomainTopic,
    Entails,
    Exemplifies,
    HasDomainRegion,
    HasDomainTopic,
    HoloMember,
    HoloPart,
    HoloSubstance,
    Hypernym,
    Hyponym,
    InstanceHypernym,
    InstanceHyponym,
    IsCausedBy,
    IsEntailedBy,
    IsExemplifiedBy,
    MeroMember,
    MeroPart,
    MeroSubstance,
    Similar,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("xml parse error: {0}")]
    QuickXml(#[from] quick_xml::de::DeError),
}

pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Root, Error> {
    let file = std::fs::File::open(path)?;
    from_reader(std::io::BufReader::new(file))
}

pub fn from_str(str: &str) -> Result<Root, Error> {
    Ok(quick_xml::de::from_str(str)?)
}

pub fn from_reader(reader: impl std::io::BufRead) -> Result<Root, Error> {
    Ok(quick_xml::de::from_reader(reader)?)
}
