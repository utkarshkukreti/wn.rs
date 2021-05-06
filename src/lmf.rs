#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Root {
    #[serde(rename = "Lexicon")]
    pub lexicons: Vec<Lexicon>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lexicon {
    pub id: String,
    pub label: String,
    pub language: String,
    pub email: String,
    pub license: String,
    pub version: String,
    pub url: String,
    #[serde(rename = "LexicalEntry")]
    pub lexical_entries: Vec<LexicalEntry>,
    #[serde(rename = "Synset")]
    pub synsets: Vec<Synset>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Sense")]
    pub senses: Vec<Sense>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
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
    pub id: String,
    pub synset: String,
    #[serde(rename = "SenseRelation", default)]
    pub relations: Vec<SenseRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SenseRelation {
    #[serde(rename = "relType")]
    pub rel_type: SenseRelationType,
    pub target: String,
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
    pub id: String,
    #[serde(rename = "Definition")]
    pub definitions: Vec<String>,
    #[serde(rename = "Example", default)]
    pub examples: Vec<String>,
    #[serde(rename = "SynsetRelation", default)]
    pub relations: Vec<SynsetRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SynsetRelation {
    #[serde(rename = "relType")]
    pub rel_type: SynsetRelationType,
    pub target: String,
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
