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
    #[serde(rename = "SyntacticBehaviour", default)]
    pub syntactic_behaviours: V<SyntacticBehaviour>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: S,
    pub status: Option<S>,
    pub note: Option<S>,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Form", default)]
    pub forms: V<Form>,
    #[serde(rename = "Sense")]
    pub senses: V<Sense>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub syntactic_behaviours: V<SyntacticBehaviour>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: S,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: V<Pronunciation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Form {
    #[serde(rename = "writtenForm")]
    pub written_form: S,
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
    #[serde(rename = "c")]
    Conjunction,
    #[serde(rename = "p")]
    Adposition,
    #[serde(rename = "x")]
    Other,
    #[serde(rename = "u")]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Pronunciation {
    pub variety: Option<S>,
    pub notation: Option<S>,
    pub audio: Option<S>,
    #[serde(rename = "$value")]
    pub value: S,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Sense {
    pub id: S,
    #[serde(rename = "synset")]
    pub synset_id: S,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: V<SenseRelation>,
    #[serde(rename = "Example", default)]
    pub examples: V<S>,
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
    Other,
    Participle,
    Pertainym,
    Similar,
    // =========
    SimpleAspectIp,
    SecondaryAspectIp,
    SimpleAspectPi,
    SecondaryAspectPi,
    Feminine,
    HasFeminine,
    Masculine,
    HasMasculine,
    Young,
    HasYoung,
    Diminutive,
    HasDiminutive,
    Augmentative,
    HasAugmentative,
    AntoGradable,
    AntoSimple,
    AntoConverse,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Synset {
    pub id: S,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Definition")]
    pub definitions: V<S>,
    #[serde(rename = "ILIDefinition")]
    pub ili_definition: Option<S>,
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
    // =========
    Agent,
    Antonym,
    BeInState,
    ClassifiedBy,
    Classifies,
    CoAgentInstrument,
    CoAgentPatient,
    CoAgentResult,
    CoInstrumentAgent,
    CoInstrumentPatient,
    CoInstrumentResult,
    CoPatientAgent,
    CoPatientInstrument,
    CoResultAgent,
    CoResultInstrument,
    CoRole,
    Direction,
    EqSynonym,
    HoloLocation,
    HoloPortion,
    Holonym,
    InManner,
    Instrument,
    Involved,
    InvolvedAgent,
    InvolvedDirection,
    InvolvedInstrument,
    InvolvedLocation,
    InvolvedPatient,
    InvolvedResult,
    InvolvedSourceDirection,
    InvolvedTargetDirection,
    Location,
    MannerOf,
    MeroLocation,
    MeroPortion,
    Meronym,
    Other,
    Patient,
    RestrictedBy,
    Restricts,
    Result,
    Role,
    SourceDirection,
    StateOf,
    TargetDirection,
    Subevent,
    IsSubeventOf,
    Feminine,
    HasFeminine,
    Masculine,
    HasMasculine,
    Young,
    HasYoung,
    Diminutive,
    HasDiminutive,
    Augmentative,
    HasAugmentative,
    AntoGradable,
    AntoSimple,
    AntoConverse,
    IrSynonym,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SyntacticBehaviour {
    #[serde(rename = "subcategorizationFrame")]
    pub subcategorization_frame: S,
    #[serde(default)]
    pub senses: V<S>,
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
