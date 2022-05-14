pub type BoxStr = Box<str>;

pub type BoxSlice<T> = Box<[T]>;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Root {
    #[serde(rename = "Lexicon")]
    pub lexicons: BoxSlice<Lexicon>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lexicon {
    pub id: BoxStr,
    pub label: BoxStr,
    pub language: BoxStr,
    pub email: BoxStr,
    pub license: BoxStr,
    pub version: BoxStr,
    pub url: BoxStr,
    #[serde(rename = "LexicalEntry")]
    pub lexical_entries: BoxSlice<LexicalEntry>,
    #[serde(rename = "Synset")]
    pub synsets: BoxSlice<Synset>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub syntactic_behaviours: BoxSlice<SyntacticBehaviour>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: BoxStr,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Form", default)]
    pub forms: BoxSlice<Form>,
    #[serde(rename = "Sense")]
    pub senses: BoxSlice<Sense>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub syntactic_behaviours: BoxSlice<SyntacticBehaviour>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: BoxStr,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: BoxSlice<Pronunciation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Form {
    #[serde(rename = "writtenForm")]
    pub written_form: BoxStr,
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
    pub variety: Option<BoxStr>,
    pub notation: Option<BoxStr>,
    pub audio: Option<BoxStr>,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Sense {
    pub id: BoxStr,
    #[serde(rename = "synset")]
    pub synset_id: BoxStr,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: BoxSlice<SenseRelation>,
    #[serde(rename = "Example", default)]
    pub examples: BoxSlice<BoxStr>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SenseRelation {
    #[serde(rename = "relType")]
    pub rel_type: SenseRelationType,
    pub target: BoxStr,
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
    pub id: BoxStr,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Definition")]
    pub definitions: BoxSlice<BoxStr>,
    #[serde(rename = "ILIDefinition")]
    pub ili_definition: Option<BoxStr>,
    #[serde(rename = "Example", default)]
    pub examples: BoxSlice<BoxStr>,
    #[serde(rename = "SynsetRelation", default)]
    pub relations: BoxSlice<SynsetRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SynsetRelation {
    #[serde(rename = "relType")]
    pub rel_type: SynsetRelationType,
    pub target: BoxStr,
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
    pub subcategorization_frame: BoxStr,
    #[serde(default)]
    pub senses: BoxSlice<BoxStr>,
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
