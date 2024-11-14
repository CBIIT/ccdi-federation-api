use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 6347743 2.00`**
///
/// This metadata element is defined by the caDSR as "The type of systematic
/// actions performed to select or enrich DNA fragments used in analysis by
/// high-throughput sequencing.". This data element is a subset of the
/// designated CDE as noted
/// [here](https://github.com/CBIIT/ccdi-federation-api/discussions/47#discussioncomment-9863376).
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6347743%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::sample::LibrarySelectionMethod)]
pub enum LibrarySelectionMethod {
    /// `ChIP`
    ///
    /// * **VM Long Name**: Chromatin Immunoprecipitation
    /// * **VM Public ID**: 6287017
    /// * **Concept Code**: C106048
    /// * **Begin Date**:   05/15/2018
    ///
    /// A technique that utilizes immunoprecipitation to find protein-DNA
    /// interactions in a biological sample. First, the sample is treated
    /// causing the formation of protein-chromatin crosslinks. Then, the sample
    /// is sheared to separate individual protein-DNA complexes. Finally, an
    /// antibody directed against a protein of interest is applied and the
    /// protein-DNA complexes are immunoprecipitated.
    #[serde(rename = "ChIP")]
    Chip,

    /// `CF-M`
    ///
    /// * **VM Long Name**: Cot-filtered moderately repetitive
    /// * **VM Public ID**: 6287018
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Cot-filtered moderately repetitive genomic DNA
    #[serde(rename = "CF-M")]
    CfM,

    /// `CF-S`
    ///
    /// * **VM Long Name**: Cot-filtered single
    /// * **VM Public ID**: 6287019
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Cot-filtered single/low-copy genomic DNA
    #[serde(rename = "CF-S")]
    CfS,

    /// `MF`
    ///
    /// * **VM Long Name**: Methyl Filtrated
    /// * **VM Public ID**: 6287020
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Methyl Filtrated
    #[serde(rename = "MF")]
    Mf,

    /// `HMPR`
    ///
    /// * **VM Long Name**: HMPR
    /// * **VM Public ID**: 6287021
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Hypo-methylated partial restriction digest
    #[serde(rename = "HMPR")]
    Hmpr,

    /// `RT-PCR`
    ///
    /// * **VM Long Name**: RT-PCR
    /// * **VM Public ID**: 3121570
    /// * **Concept Code**: C18136
    /// * **Begin Date**:   01/19/2018
    ///
    /// RT-PCR is short for reverse transcriptase-polymerase chain reaction. It
    /// is a technique in which an RNA strand is first transcribed into a DNA
    /// complement and then subjected to PCR amplification. Transcribing an RNA
    /// strand into a DNA complement is termed reverse transcription and is done
    /// by the enzyme reverse transcriptase. (from Wikipedia)
    #[serde(rename = "RT-PCR")]
    RtPcr,

    /// `Random PCR`
    ///
    /// * **VM Long Name**: Random PCR
    /// * **VM Public ID**: 6287022
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Source material was selected by randomly generated primers
    #[serde(rename = "Random PCR")]
    RandomPcr,

    /// `PCR`
    ///
    /// * **VM Long Name**: Polymerase Chain Reaction
    /// * **VM Public ID**: 3234683
    /// * **Concept Code**: C17003
    /// * **Begin Date**:   12/01/2023
    ///
    /// A method for amplifying a DNA base sequence using multiple rounds of
    /// heat denaturation of the DNA and annealing of oligonucleotide primers
    /// complementary to flanking regions in the presence of a heat-stable
    /// polymerase.  This results in duplication of the targeted DNA region.
    /// Newly synthesized DNA strands can subsequently serve as additional
    /// templates for the same primer sequences, so that successive rounds of
    /// primer annealing, strand elongation, and dissociation produce rapid and
    /// highly specific amplification of the desired sequence. PCR also can be
    /// used to detect the existence of the defined sequence in a DNA sample.
    #[serde(rename = "PCR")]
    Pcr,

    /// `Random`
    ///
    /// * **VM Long Name**: Random
    /// * **VM Public ID**: 6287023
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Random selection by shearing or other method
    #[serde(rename = "Random")]
    Random,

    /// `Inverse rRNA`
    ///
    /// * **VM Long Name**: Inverse rRNA
    /// * **VM Public ID**: 6287897
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Depletion of ribosomal RNA by oligo hybridization.
    #[serde(rename = "Inverse rRNA")]
    InverseRrna,

    /// `cDNA random priming`
    ///
    /// * **VM Long Name**: cDNA random priming
    /// * **VM Public ID**: 6287898
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Oligodeoxyribonucleotides (mostly hexamers) used to prepare labeled DNA
    /// probes.
    #[serde(rename = "cDNA random priming")]
    CdnaRandomPriming,

    /// `cDNA oligo_dT`
    ///
    /// * **VM Long Name**: cDNA_oligo_dT
    /// * **VM Public ID**: 6287899
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Primers consist of a stretch of 12 18 deoxythymidines that anneal to
    /// poly(A) tails of eukaryotic mRNA.
    #[serde(rename = "cDNA oligo_dT")]
    CdnaOligoDt,

    /// `Padlock probes capture method`
    ///
    /// * **VM Long Name**: Padlock probes capture method
    /// * **VM Public ID**: 6287900
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Circularized oligonucleotide probes
    #[serde(rename = "Padlock probes capture method")]
    PadlockProbesCaptureMethod,

    /// `Size fractionation`
    ///
    /// * **VM Long Name**: Size fractionation
    /// * **VM Public ID**: 6287901
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Physical selection of size appropriate targets
    #[serde(rename = "Size fractionation")]
    SizeFractionation,

    /// `RACE`
    ///
    /// * **VM Long Name**: Rapid Amplification of cDNA Ends
    /// * **VM Public ID**: 6287902
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Rapid amplification of cDNA ends.
    #[serde(rename = "RACE")]
    Race,

    /// `CAGE`
    ///
    /// * **VM Long Name**: Cap-analysis gene expression
    /// * **VM Public ID**: 6287903
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Cap-analysis gene expression
    #[serde(rename = "CAGE")]
    Cage,

    /// `MBD2 protein methyl-CpG binding domain`
    ///
    /// * **VM Long Name**: MBD2 protein methyl-CpG binding domain
    /// * **VM Public ID**: 6287904
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Enrichment by methyl-CpG binding domain.
    #[serde(rename = "MBD2 protein methyl-CpG binding domain")]
    Mbd2ProteinMethylCpgBindingDomain,

    /// `5-methylcytidine antibody`
    ///
    /// * **VM Long Name**: 5-methylcytidine antibody
    /// * **VM Public ID**: 6287905
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Selection of methylated DNA fragments using an antibody raised against
    /// 5-methylcytosine or 5-methylcytidine (m5C).
    #[serde(rename = "5-methylcytidine antibody")]
    FiveMethylcytidineAntibody,

    /// `Restriction Digest`
    ///
    /// * **VM Long Name**: Multiple Complete Digest Restriction Fragment
    ///   Mapping
    /// * **VM Public ID**: 6287907
    /// * **Concept Code**: C116160
    /// * **Begin Date**:   05/15/2018
    ///
    /// A DNA mapping technique based on complete restriction enzyme digestion
    /// of a set of overlapping clones that provides highly redundant coverage
    /// of the mapping target.
    #[serde(rename = "Restriction Digest")]
    RestrictionDigest,

    /// `Reduced Representation`
    ///
    /// * **VM Long Name**: Reduced Representation
    /// * **VM Public ID**: 6287908
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Reproducible genomic subsets, often generated by restriction fragment
    /// size selection, containing a manageable number of loci to facilitate
    /// re-sampling.
    #[serde(rename = "Reduced Representation")]
    ReducedRepresentation,

    /// `Hybrid Selection`
    ///
    /// * **VM Long Name**: Hybrid Selection
    /// * **VM Public ID**: 6287909
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Selection by hybridization in array or solution.
    #[serde(rename = "Hybrid Selection")]
    HybridSelection,

    /// `DNAse`
    ///
    /// * **VM Long Name**: MNase Sequencing
    /// * **VM Public ID**: 6287911
    /// * **Concept Code**: C106056
    /// * **Begin Date**:   05/15/2018
    ///
    /// A molecular genetic technique where genome-wide sequencing is performed
    /// on chromosomal DNA that is resistant to treatment with micrococcal
    /// nuclease. This technique identifies nucleosomal DNA sequences.
    #[serde(rename = "DNAse")]
    Dnase,

    /// `Repeat fractionation`
    ///
    /// * **VM Long Name**: Repeat fractionation
    /// * **VM Public ID**: 6287912
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Selection for less repetitive (and more gene rich) sequence through Cot
    /// filtration (CF) or other fractionation techniques based on DNA kinetics.
    #[serde(rename = "Repeat fractionation")]
    RepeatFractionation,

    /// `Unspecified`
    ///
    /// * **VM Long Name**: Unspecified Sequencing Library Method
    /// * **VM Public ID**: 6287913
    /// * **Concept Code**: C71460
    /// * **Begin Date**:   05/15/2018
    ///
    /// Not stated explicitly or in detail.: A collection of double stranded DNA
    /// fragments flanked by oligonucleotide sequence adapters to enable their
    /// analysis by high-throughput sequencing.: A means, manner of procedure,
    /// or systematic course of actions that have to be performed in order to
    /// accomplish a particular goal.
    #[serde(rename = "Unspecified")]
    Unspecified,

    /// `MSLL`
    ///
    /// * **VM Long Name**: Methylation spanning linking library
    /// * **VM Public ID**: 6287868
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Methylation Spanning Linking Library
    #[serde(rename = "MSLL")]
    Msll,

    /// `MDA`
    ///
    /// * **VM Long Name**: MDA
    /// * **VM Public ID**: 6287869
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Multiple displacement amplification
    #[serde(rename = "MDA")]
    Mda,

    /// `CF-T`
    ///
    /// * **VM Long Name**: Cot-filtered theoretical single-copy
    /// * **VM Public ID**: 6287870
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Cot-filtered theoretical single-copy genomic DNA
    #[serde(rename = "CF-T")]
    CfT,

    /// `CF-H`
    ///
    /// * **VM Long Name**: Cot-filtered highly repetitive
    /// * **VM Public ID**: 6287871
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Cot-filtered highly repetitive genomic DNA
    #[serde(rename = "CF-H")]
    CfH,

    /// `cDNA`
    ///
    /// * **VM Long Name**: cDNA
    /// * **VM Public ID**: 2581950
    /// * **Concept Code**: C324
    /// * **Begin Date**:   11/15/2006
    ///
    /// Single-stranded complementary DNA synthesized from an RNA template by
    /// the action of RNA-dependent DNA polymerase. cDNA (i.e., complementary
    /// DNA, not circular DNA, not C-DNA) is used in a variety of molecular
    /// cloning experiments as well as serving as a specific hybridization
    /// probe.
    #[serde(rename = "cDNA")]
    Cdna,

    /// `Other`
    ///
    /// * **VM Long Name**: Other Sequencing Library Method
    /// * **VM Public ID**: 6287874
    /// * **Concept Code**: C71460
    /// * **Begin Date**:   05/15/2018
    ///
    /// Different than the one(s) previously specified or mentioned.: A
    /// collection of double stranded DNA fragments flanked by oligonucleotide
    /// sequence adapters to enable their analysis by high-throughput
    /// sequencing.: A means, manner of procedure, or systematic course of
    /// actions that have to be performed in order to accomplish a particular
    /// goal.
    #[serde(rename = "Other")]
    Other,

    /// `miRNA Size Fractionation`
    ///
    /// * **VM Long Name**: miRNA Size Fractionated Genomic Library
    /// * **VM Public ID**: 7537067
    /// * **Concept Code**: C163991
    /// * **Begin Date**:   12/22/2020
    ///
    /// A RNA library that has been fractionated by size exclusion methods to
    /// enrich for microRNAs.
    #[serde(rename = "miRNA Size Fractionation")]
    MirnaSizeFractionation,

    /// `rRNA Depletion`
    ///
    /// * **VM Long Name**: rRNA Depleted Genomic Library
    /// * **VM Public ID**: 7537069
    /// * **Concept Code**: C163990
    /// * **Begin Date**:   12/22/2020
    ///
    /// A genomic library where the sample has been subjected to a ribosomal RNA
    /// depletion step before it is amplified.
    #[serde(rename = "rRNA Depletion")]
    RrnaDepletion,

    /// `Affinity Enrichment`
    ///
    /// * **VM Long Name**: Affinity Enrichment
    /// * **VM Public ID**: 7601843
    /// * **Concept Code**: C154307
    /// * **Begin Date**:   03/16/2021
    ///
    /// The strength of noncovalent chemical binding between two substances as
    /// measured by the dissociation constant of the complex.: Any of various
    /// techniques designed to select or increase a target item in a mixed
    /// sample.
    #[serde(rename = "Affinity Enrichment")]
    AffinityEnrichment,

    /// `Not applicable`
    ///
    /// * **VM Long Name**: Not Applicable
    /// * **VM Public ID**: 5682946
    /// * **Concept Code**: C48660
    /// * **Begin Date**:   08/30/2024
    ///
    /// Determination of a value is not relevant in the current context.
    #[serde(rename = "Not applicable")]
    NotApplicable,

    /// `MNase`
    ///
    /// * **VM Long Name**: MNase Sequencing
    /// * **VM Public ID**: 6287911
    /// * **Concept Code**: C106056
    /// * **Begin Date**:   05/15/2018
    ///
    /// A molecular genetic technique where genome-wide sequencing is performed
    /// on chromosomal DNA that is resistant to treatment with micrococcal
    /// nuclease. This technique identifies nucleosomal DNA sequences.
    #[serde(rename = "MNase")]
    Mnase,

    /// `Poly-A Enriched Genomic Library`
    ///
    /// * **VM Long Name**: Poly-A Enriched Genomic Library
    /// * **VM Public ID**: 14901531
    /// * **Concept Code**: C163988
    /// * **Begin Date**:   04/18/2024
    ///
    /// A genomic library where the sample has been subjected to enrichment via
    /// binding to immobilized poly-T oligonucleotides, which will
    /// preferentially isolate polyadenylated (poly-A) RNAs.
    #[serde(rename = "Poly-A Enriched Genomic Library")]
    PolyAEnrichedGenomicLibrary,
}

impl CDE for LibrarySelectionMethod {}

impl std::fmt::Display for LibrarySelectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibrarySelectionMethod::Chip => write!(f, "ChIP"),
            LibrarySelectionMethod::CfM => write!(f, "CF-M"),
            LibrarySelectionMethod::CfS => write!(f, "CF-S"),
            LibrarySelectionMethod::Mf => write!(f, "MF"),
            LibrarySelectionMethod::Hmpr => write!(f, "HMPR"),
            LibrarySelectionMethod::RtPcr => write!(f, "RT-PCR"),
            LibrarySelectionMethod::RandomPcr => write!(f, "Random PCR"),
            LibrarySelectionMethod::Pcr => write!(f, "PCR"),
            LibrarySelectionMethod::Random => write!(f, "Random"),
            LibrarySelectionMethod::InverseRrna => write!(f, "Inverse rRNA"),
            LibrarySelectionMethod::CdnaRandomPriming => write!(f, "cDNA random priming"),
            LibrarySelectionMethod::CdnaOligoDt => write!(f, "cDNA oligo_dT"),
            LibrarySelectionMethod::PadlockProbesCaptureMethod => {
                write!(f, "Padlock probes capture method")
            }
            LibrarySelectionMethod::SizeFractionation => write!(f, "Size fractionation"),
            LibrarySelectionMethod::Race => write!(f, "RACE"),
            LibrarySelectionMethod::Cage => write!(f, "CAGE"),
            LibrarySelectionMethod::Mbd2ProteinMethylCpgBindingDomain => {
                write!(f, "MBD2 protein methyl-CpG binding domain")
            }
            LibrarySelectionMethod::FiveMethylcytidineAntibody => {
                write!(f, "5-methylcytidine antibody")
            }
            LibrarySelectionMethod::RestrictionDigest => write!(f, "Restriction Digest"),
            LibrarySelectionMethod::ReducedRepresentation => write!(f, "Reduced Representation"),
            LibrarySelectionMethod::HybridSelection => write!(f, "Hybrid Selection"),
            LibrarySelectionMethod::Dnase => write!(f, "DNAse"),
            LibrarySelectionMethod::RepeatFractionation => write!(f, "Repeat fractionation"),
            LibrarySelectionMethod::Unspecified => write!(f, "Unspecified"),
            LibrarySelectionMethod::Msll => write!(f, "MSLL"),
            LibrarySelectionMethod::Mda => write!(f, "MDA"),
            LibrarySelectionMethod::CfT => write!(f, "CF-T"),
            LibrarySelectionMethod::CfH => write!(f, "CF-H"),
            LibrarySelectionMethod::Cdna => write!(f, "cDNA"),
            LibrarySelectionMethod::Other => write!(f, "Other"),
            LibrarySelectionMethod::MirnaSizeFractionation => write!(f, "miRNA Size Fractionation"),
            LibrarySelectionMethod::RrnaDepletion => write!(f, "rRNA Depletion"),
            LibrarySelectionMethod::AffinityEnrichment => write!(f, "Affinity Enrichment"),
            LibrarySelectionMethod::NotApplicable => write!(f, "Not applicable"),
            LibrarySelectionMethod::Mnase => write!(f, "MNase"),
            LibrarySelectionMethod::PolyAEnrichedGenomicLibrary => {
                write!(f, "Poly-A Enriched Genomic Library")
            }
        }
    }
}

impl Distribution<LibrarySelectionMethod> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LibrarySelectionMethod {
        match rng.gen_range(0..=35) {
            0 => LibrarySelectionMethod::Chip,
            1 => LibrarySelectionMethod::CfM,
            2 => LibrarySelectionMethod::CfS,
            3 => LibrarySelectionMethod::Mf,
            4 => LibrarySelectionMethod::Hmpr,
            5 => LibrarySelectionMethod::RtPcr,
            6 => LibrarySelectionMethod::RandomPcr,
            7 => LibrarySelectionMethod::Pcr,
            8 => LibrarySelectionMethod::Random,
            9 => LibrarySelectionMethod::InverseRrna,
            10 => LibrarySelectionMethod::CdnaRandomPriming,
            11 => LibrarySelectionMethod::CdnaOligoDt,
            12 => LibrarySelectionMethod::PadlockProbesCaptureMethod,
            13 => LibrarySelectionMethod::SizeFractionation,
            14 => LibrarySelectionMethod::Race,
            15 => LibrarySelectionMethod::Cage,
            16 => LibrarySelectionMethod::Mbd2ProteinMethylCpgBindingDomain,
            17 => LibrarySelectionMethod::FiveMethylcytidineAntibody,
            18 => LibrarySelectionMethod::RestrictionDigest,
            19 => LibrarySelectionMethod::ReducedRepresentation,
            20 => LibrarySelectionMethod::HybridSelection,
            21 => LibrarySelectionMethod::Dnase,
            22 => LibrarySelectionMethod::RepeatFractionation,
            23 => LibrarySelectionMethod::Unspecified,
            24 => LibrarySelectionMethod::Msll,
            25 => LibrarySelectionMethod::Mda,
            26 => LibrarySelectionMethod::CfT,
            27 => LibrarySelectionMethod::CfH,
            28 => LibrarySelectionMethod::Cdna,
            29 => LibrarySelectionMethod::Other,
            30 => LibrarySelectionMethod::MirnaSizeFractionation,
            31 => LibrarySelectionMethod::RrnaDepletion,
            32 => LibrarySelectionMethod::AffinityEnrichment,
            33 => LibrarySelectionMethod::NotApplicable,
            34 => LibrarySelectionMethod::Mnase,
            _ => LibrarySelectionMethod::PolyAEnrichedGenomicLibrary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(LibrarySelectionMethod::Chip.to_string(), "ChIP");
        assert_eq!(LibrarySelectionMethod::CfM.to_string(), "CF-M");
        assert_eq!(LibrarySelectionMethod::CfS.to_string(), "CF-S");
        assert_eq!(LibrarySelectionMethod::Mf.to_string(), "MF");
        assert_eq!(LibrarySelectionMethod::Hmpr.to_string(), "HMPR");
        assert_eq!(LibrarySelectionMethod::RtPcr.to_string(), "RT-PCR");
        assert_eq!(LibrarySelectionMethod::RandomPcr.to_string(), "Random PCR");
        assert_eq!(LibrarySelectionMethod::Pcr.to_string(), "PCR");
        assert_eq!(LibrarySelectionMethod::Random.to_string(), "Random");
        assert_eq!(
            LibrarySelectionMethod::InverseRrna.to_string(),
            "Inverse rRNA"
        );
        assert_eq!(
            LibrarySelectionMethod::CdnaRandomPriming.to_string(),
            "cDNA random priming"
        );
        assert_eq!(
            LibrarySelectionMethod::CdnaOligoDt.to_string(),
            "cDNA oligo_dT"
        );
        assert_eq!(
            LibrarySelectionMethod::PadlockProbesCaptureMethod.to_string(),
            "Padlock probes capture method"
        );
        assert_eq!(
            LibrarySelectionMethod::SizeFractionation.to_string(),
            "Size fractionation"
        );
        assert_eq!(LibrarySelectionMethod::Race.to_string(), "RACE");
        assert_eq!(LibrarySelectionMethod::Cage.to_string(), "CAGE");
        assert_eq!(
            LibrarySelectionMethod::Mbd2ProteinMethylCpgBindingDomain.to_string(),
            "MBD2 protein methyl-CpG binding domain"
        );
        assert_eq!(
            LibrarySelectionMethod::FiveMethylcytidineAntibody.to_string(),
            "5-methylcytidine antibody"
        );
        assert_eq!(
            LibrarySelectionMethod::RestrictionDigest.to_string(),
            "Restriction Digest"
        );
        assert_eq!(
            LibrarySelectionMethod::ReducedRepresentation.to_string(),
            "Reduced Representation"
        );
        assert_eq!(
            LibrarySelectionMethod::HybridSelection.to_string(),
            "Hybrid Selection"
        );
        assert_eq!(LibrarySelectionMethod::Dnase.to_string(), "DNAse");
        assert_eq!(
            LibrarySelectionMethod::RepeatFractionation.to_string(),
            "Repeat fractionation"
        );
        assert_eq!(
            LibrarySelectionMethod::Unspecified.to_string(),
            "Unspecified"
        );
        assert_eq!(LibrarySelectionMethod::Msll.to_string(), "MSLL");
        assert_eq!(LibrarySelectionMethod::Mda.to_string(), "MDA");
        assert_eq!(LibrarySelectionMethod::CfT.to_string(), "CF-T");
        assert_eq!(LibrarySelectionMethod::CfH.to_string(), "CF-H");
        assert_eq!(LibrarySelectionMethod::Cdna.to_string(), "cDNA");
        assert_eq!(LibrarySelectionMethod::Other.to_string(), "Other");
        assert_eq!(
            LibrarySelectionMethod::MirnaSizeFractionation.to_string(),
            "miRNA Size Fractionation"
        );
        assert_eq!(
            LibrarySelectionMethod::RrnaDepletion.to_string(),
            "rRNA Depletion"
        );
        assert_eq!(
            LibrarySelectionMethod::AffinityEnrichment.to_string(),
            "Affinity Enrichment"
        );
        assert_eq!(
            LibrarySelectionMethod::NotApplicable.to_string(),
            "Not applicable"
        );
        assert_eq!(LibrarySelectionMethod::Mnase.to_string(), "MNase");
        assert_eq!(
            LibrarySelectionMethod::PolyAEnrichedGenomicLibrary.to_string(),
            "Poly-A Enriched Genomic Library"
        );
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Chip).unwrap(),
            "\"ChIP\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::CfM).unwrap(),
            "\"CF-M\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::CfS).unwrap(),
            "\"CF-S\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Mf).unwrap(),
            "\"MF\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Hmpr).unwrap(),
            "\"HMPR\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::RtPcr).unwrap(),
            "\"RT-PCR\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::RandomPcr).unwrap(),
            "\"Random PCR\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Pcr).unwrap(),
            "\"PCR\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Random).unwrap(),
            "\"Random\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::InverseRrna).unwrap(),
            "\"Inverse rRNA\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::CdnaRandomPriming).unwrap(),
            "\"cDNA random priming\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::CdnaOligoDt).unwrap(),
            "\"cDNA oligo_dT\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::PadlockProbesCaptureMethod).unwrap(),
            "\"Padlock probes capture method\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::SizeFractionation).unwrap(),
            "\"Size fractionation\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Race).unwrap(),
            "\"RACE\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Cage).unwrap(),
            "\"CAGE\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Mbd2ProteinMethylCpgBindingDomain)
                .unwrap(),
            "\"MBD2 protein methyl-CpG binding domain\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::FiveMethylcytidineAntibody).unwrap(),
            "\"5-methylcytidine antibody\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::RestrictionDigest).unwrap(),
            "\"Restriction Digest\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::ReducedRepresentation).unwrap(),
            "\"Reduced Representation\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::HybridSelection).unwrap(),
            "\"Hybrid Selection\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Dnase).unwrap(),
            "\"DNAse\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::RepeatFractionation).unwrap(),
            "\"Repeat fractionation\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Unspecified).unwrap(),
            "\"Unspecified\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Msll).unwrap(),
            "\"MSLL\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Mda).unwrap(),
            "\"MDA\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::CfT).unwrap(),
            "\"CF-T\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::CfH).unwrap(),
            "\"CF-H\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Cdna).unwrap(),
            "\"cDNA\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Other).unwrap(),
            "\"Other\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::MirnaSizeFractionation).unwrap(),
            "\"miRNA Size Fractionation\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::RrnaDepletion).unwrap(),
            "\"rRNA Depletion\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::AffinityEnrichment).unwrap(),
            "\"Affinity Enrichment\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::NotApplicable).unwrap(),
            "\"Not applicable\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Mnase).unwrap(),
            "\"MNase\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::PolyAEnrichedGenomicLibrary).unwrap(),
            "\"Poly-A Enriched Genomic Library\""
        );
    }
}
