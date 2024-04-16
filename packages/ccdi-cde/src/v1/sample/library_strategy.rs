use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 6273393 v1.00`**
///
/// This metadata element is defined by the caDSR as "The overall strategy for
/// the collection of double stranded DNA fragments flanked by oligonucleotide
/// sequence adapters to enable their analysis by high-throughput sequencing.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6273393%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::LibraryStrategy)]
pub enum LibraryStrategy {
    /// `AMPLICON`
    ///
    /// * **VM Long Name**: AMPLICON
    /// * **VM Public ID**: 6273354
    /// * **Concept Code**: C204813
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method for targeted DNA sequencing that uses oligonucleotide primers
    /// to amplify regions of interest, followed by next-generation sequencing
    /// (NGS) for deep coverage of the region.
    #[serde(rename = "AMPLICON")]
    Amplicon,

    /// `ATAC-Seq`
    ///
    /// * **VM Long Name**: ATAC-seq
    /// * **VM Public ID**: 6273353
    /// * **Concept Code**: C156056
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique that isolates and sequences chromosomal
    /// regions that are rich in open chromatin. First, nuclei are harvested
    /// from a cellular sample. Then a hyperactive Tn5 transposase is added to
    /// the nuclei where it excises non-nucleosomal DNA strands and ligates co-
    /// administered high-throughput sequencing adapters (tagmentation). The
    /// tagged DNA fragments are isolated, amplified by PCR and sequenced. The
    /// number of reads for specific region of DNA correlate with increased
    /// chromatin accessibility and this method can identify regions of
    /// transcription factor and nucleosome binding.
    #[serde(rename = "ATAC-Seq")]
    AtacSeq,

    /// `Bisulfite-Seq`
    ///
    /// * **VM Long Name**: Bisulfite-Seq
    /// * **VM Public ID**: 6273352
    /// * **Concept Code**: C106054
    /// * **Begin Date**:   05/11/2018
    ///
    /// A DNA sequencing technique that can differentiate cytosine from
    /// 5-methylcytosine in a DNA sample. First, a denatured DNA sample is
    /// treated with bisulfite which converts non-methylated cytosine to uracil.
    /// Next, the sample is amplified using a PCR method that does not
    /// discriminate between non-methylated and methylated sequences. The
    /// amplified DNA is subjected to nucleotide sequencing. The resulting
    /// sequence is compared to an identical control sample of DNA that was not
    /// treated with bisulfite. Unmethylated cytosines will be displayed as
    /// cytosines in the control sample and as thymines in the bisulfite-treated
    /// sample.
    #[serde(rename = "Bisulfite-Seq")]
    BisulfiteSeq,

    /// `ChIA-PET`
    ///
    /// * **VM Long Name**: ChIA-PET
    /// * **VM Public ID**: 6273351
    /// * **Concept Code**: C172845
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique that combines chromatin
    /// immunoprecipitation (ChIP) with paired end tagged (PET) DNA sequencing
    /// to identify the nucleotide sequences for the binding sites occupied by
    /// DNA-associated proteins in a sample.
    #[serde(rename = "ChIA-PET")]
    ChiaPet,

    /// `ChIP-Seq`
    ///
    /// * **VM Long Name**: ChIP-Seq
    /// * **VM Public ID**: 6273350
    /// * **Concept Code**: C106049
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique that combines chromatin
    /// immunoprecipitation (ChIP) with massively parallel DNA sequencing to map
    /// the binding sites of DNA-associated proteins in a sample of cells.
    /// First, crosslinked protein-DNA complexes are isolated using ChIP. Next,
    /// the crosslinks are broken, the proteins are removed and the purified DNA
    /// is modified with adaptor oligonucleotides to facilitate massively
    /// parallel DNA sequencing. Following sequencing, the DNA sequences that
    /// are obtained can be mapped to their genomic locations.
    #[serde(rename = "ChIP-Seq")]
    ChipSeq,

    /// `CLONE`
    ///
    /// * **VM Long Name**: CLONE
    /// * **VM Public ID**: 6273348
    /// * **Concept Code**: C204814
    /// * **Begin Date**:   05/11/2018
    ///
    /// DNA sequencing where a target is cloned into a vector, physically
    /// mapped, and then shotgun sequenced.
    #[serde(rename = "CLONE")]
    Clone,

    /// `CLONEEND`
    ///
    /// * **VM Long Name**: CLONEEND
    /// * **VM Public ID**: 6273347
    /// * **Concept Code**: C204815
    /// * **Begin Date**:   05/11/2018
    ///
    /// A DNA sequencing strategy where a single read is initiated from one or
    /// both ends of a cloned DNA fragment.
    #[serde(rename = "CLONEEND")]
    Cloneend,

    /// `CTS`
    ///
    /// * **VM Long Name**: CTS
    /// * **VM Public ID**: 6273361
    /// * **Concept Code**: C204816
    /// * **Begin Date**:   05/11/2018
    ///
    /// A DNA sequencing strategy where a sequencing primer is chosen from a
    /// sequenced region in order to extend the sequenced region.
    #[serde(rename = "CTS")]
    Cts,

    /// `DNA-Seq`
    ///
    /// * **VM Long Name**: DNA Sequencing
    /// * **VM Public ID**: 6424615
    /// * **Concept Code**: C153598
    /// * **Begin Date**:   03/25/2024
    ///
    /// The determination of the sequence of purines and pyrimidines in
    /// deoxyribonucleic acid (DNA).
    #[serde(rename = "DNA-Seq")]
    DnaSeq,

    /// `DNase-Hypersensitivity`
    ///
    /// * **VM Long Name**: DNase-Hypersensitivity
    /// * **VM Public ID**: 6273360
    /// * **Concept Code**: C106052
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique where genome-wide sequencing is performed
    /// on DNA regions that are super sensitive to cleavage by DNase I to
    /// identify putative DNA regulatory sequences.
    #[serde(rename = "DNase-Hypersensitivity")]
    DnaseHypersensitivity,

    /// `EST`
    ///
    /// * **VM Long Name**: EST
    /// * **VM Public ID**: 6273359
    /// * **Concept Code**: C16208
    /// * **Begin Date**:   05/11/2018
    ///
    /// A random cDNA library comprised of 200-800 bp random pooled mRNA clones
    /// isolated from a specific organism, specific tissue and/or a given stage
    /// of development. Clones are randomly selected for single-pass sequencing
    /// reads, the raw sequence reads are processed to remove low-quality
    /// sequence information and contaminating vector sequence and the resulting
    /// higher-quality sequences can be aggregated and aligned with others to
    /// assemble complete genetic or genomic sequences.
    #[serde(rename = "EST")]
    Est,

    /// `FAIRE-seq`
    ///
    /// * **VM Long Name**: FAIRE-Seq
    /// * **VM Public ID**: 6273358
    /// * **Concept Code**: C106051
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique that depletes a biological sample of
    /// nucleosomal DNA and then subjects the non-nucleosome-associated DNA to
    /// next-generation sequencing. Since nucleosome disruption of chromatin is
    /// indicative of active sites of DNA transcription, this technique can
    /// isolate DNA sequences that are involved in transcriptional regulation.
    /// First, a sample is treated with formaldehyde to form DNA-protein
    /// crosslinks, followed by sample lysis and sonication. The processed
    /// sample is subjected to phenol/chloroform extraction and the DNA in the
    /// aqueous phase is analyzed using next-generation sequencing techniques.
    #[serde(rename = "FAIRE-seq")]
    FaireSeq,

    /// `FINISHING`
    ///
    /// * **VM Long Name**: FINISHING
    /// * **VM Public ID**: 6273356
    /// * **Concept Code**: C204818
    /// * **Begin Date**:   05/11/2018
    ///
    /// Targeted sequencing that uses primers chosen from a sequenced region to
    /// cover unsequenced strands or regions.
    #[serde(rename = "FINISHING")]
    Finishing,

    /// `FL-cDNA`
    ///
    /// * **VM Long Name**: FL-cDNA
    /// * **VM Public ID**: 6273392
    /// * **Concept Code**: C204817
    /// * **Begin Date**:   05/11/2018
    ///
    /// Sequencing of cDNA to obtain a continuous full-length sequence.
    #[serde(rename = "FL-cDNA")]
    FlCdna,

    /// `Hi-C`
    ///
    /// * **VM Long Name**: Hi-C
    /// * **VM Public ID**: 6273391
    /// * **Concept Code**: C204819
    /// * **Begin Date**:   05/11/2018
    ///
    /// A DNA sequencing strategy designed to detect genome-wide chromatin
    /// interactions in the nucleus. The method is based on Chromosome
    /// Conformation Capture, in which chromatin is crosslinked with
    /// formaldehyde, then digested, and re-ligated in such a way that only DNA
    /// fragments that are covalently linked together form ligation products. In
    /// Hi-C, a biotin-labeled nucleotide is incorporated at the ligation
    /// junction, enabling selective purification of chimeric DNA ligation
    /// junctions followed by deep sequencing.
    #[serde(rename = "Hi-C")]
    HiC,

    /// `MBD-Seq`
    ///
    /// * **VM Long Name**: MBD-Seq
    /// * **VM Public ID**: 6273390
    /// * **Concept Code**: C204820
    /// * **Begin Date**:   05/11/2018
    ///
    /// A strategy to obtain DNA sequences from (all) methylated sites. Genomic
    /// DNA is randomly fragmented with ultrasonication and then methylated
    /// fragments are captured by a protein with high affinity for double-
    /// stranded DNA harboring methylated CpGs; non-methylated DNA fragments are
    /// washed away. The methylation-enriched fraction is then barcode tagged
    /// and sequenced.
    #[serde(rename = "MBD-Seq")]
    MbdSeq,

    /// `MeDIP-Seq`
    ///
    /// * **VM Long Name**: MeDIP-Seq
    /// * **VM Public ID**: 6273389
    /// * **Concept Code**: C204821
    /// * **Begin Date**:   05/11/2018
    ///
    /// A strategy to obtain DNA sequences from methylated sites. Genomic DNA is
    /// randomly sheared by sonication and immunoprecipitated with a monoclonal
    /// antibody that specifically recognizes 5-methylcytidine. The DNA
    /// fragments are then isolated and sequenced.
    #[serde(rename = "MeDIP-Seq")]
    MedipSeq,

    /// `miRNA-Seq`
    ///
    /// * **VM Long Name**: miRNA-Seq
    /// * **VM Public ID**: 6273388
    /// * **Concept Code**: C156057
    /// * **Begin Date**:   05/11/2018
    ///
    /// A next-generation or massively parallel high-throughput DNA sequencing-
    /// based procedure that can identify and quantify the microRNA sequences
    /// present in a biological sample.
    #[serde(rename = "miRNA-Seq")]
    MirnaSeq,

    /// `MNase-Seq`
    ///
    /// * **VM Long Name**: MNase-Seq
    /// * **VM Public ID**: 6273387
    /// * **Concept Code**: C106056
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique where genome-wide sequencing is performed
    /// on chromosomal DNA that is resistant to treatment with micrococcal
    /// nuclease. This technique identifies nucleosomal DNA sequences.
    #[serde(rename = "MNase-Seq")]
    MnaseSeq,

    /// `MRE-Seq`
    ///
    /// * **VM Long Name**: MRE-Seq
    /// * **VM Public ID**: 6273386
    /// * **Concept Code**: C204822
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method to study DNA methylation. Genomic DNA is separately digested
    /// with different methylation sensitive restriction enzymes and the
    /// fragments are used to generate a library. Deep sequencing of the library
    /// allows for accurate detection of methylation sites in the genome.
    #[serde(rename = "MRE-Seq")]
    MreSeq,

    /// `ncRNA-Seq`
    ///
    /// * **VM Long Name**: ncRNA-Seq
    /// * **VM Public ID**: 6273385
    /// * **Concept Code**: C172858
    /// * **Begin Date**:   05/11/2018
    ///
    /// A molecular genetic technique that can determine the RNA sequences for
    /// all or part of the population of small and large non-protein coding RNA
    /// transcripts in a sample.
    #[serde(rename = "ncRNA-Seq")]
    NcrnaSeq,

    /// `Other`
    ///
    /// * **VM Long Name**: Other Library strategy
    /// * **VM Public ID**: 6273371
    /// * **Concept Code**: C17649
    /// * **Begin Date**:   05/11/2018
    ///
    /// Different than the one(s) previously specified or mentioned.
    #[serde(rename = "Other")]
    Other,

    /// `POOLCLONE`
    ///
    /// * **VM Long Name**: POOLCLONE
    /// * **VM Public ID**: 6273384
    /// * **Concept Code**: C204823
    /// * **Begin Date**:   05/11/2018
    ///
    /// Shotgun sequencing of pooled DNA clones.
    #[serde(rename = "POOLCLONE")]
    Poolclone,

    /// `RAD-Seq`
    ///
    /// * **VM Long Name**: RAD-Seq
    /// * **VM Public ID**: 6273383
    /// * **Concept Code**: C204824
    /// * **Begin Date**:   05/11/2018
    ///
    /// A type of genome-wide sampling sequencing that reduces the complexity of
    /// the genome by subsampling only at specific sites defined by restriction
    /// enzymes. It is able to identify, verify, and score markers
    /// simultaneously and to identify which markers derive from each site.
    #[serde(rename = "RAD-Seq")]
    RadSeq,

    /// `RIP-Seq`
    ///
    /// * **VM Long Name**: RIP-Seq
    /// * **VM Public ID**: 6273382
    /// * **Concept Code**: C204825
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method to recover and sequence interaction sites between RNA and
    /// specific ribosomal binding proteins. RNA-protein complexes are
    /// immunoprecipitated with antibodies targeted to the protein of interest.
    /// After RNase digestion, the RNA that was protected by the protein binding
    /// is extracted, reverse-transcribed to cDNA, and sequenced.
    #[serde(rename = "RIP-Seq")]
    RipSeq,

    /// `RNA-Seq`
    ///
    /// * **VM Long Name**: RNA-Seq
    /// * **VM Public ID**: 6273381
    /// * **Concept Code**: C124261
    /// * **Begin Date**:   05/11/2018
    ///
    /// A procedure that can determine the nucleotide sequence for all of the
    /// RNA transcripts in an individual.
    #[serde(rename = "RNA-Seq")]
    RnaSeq,

    /// `SELEX`
    ///
    /// * **VM Long Name**: SELEX
    /// * **VM Public ID**: 6273380
    /// * **Concept Code**: C204826
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method for isolating single-stranded DNAs or RNAs (aptamers) with
    /// high-affinity to a target protein from a large library with random
    /// sequences. The target protein is expressed as a fusion with
    /// streptavidin-binding peptide and is the mixed with a pooled library of
    /// DNA or RNA ligands containing a 14bp randomized region (14N), and a 5 bp
    /// barcode that uniquely identifies the individual SELEX sample. Partially
    /// nested primers are used in successive SELEX rounds.
    #[serde(rename = "SELEX")]
    Selex,

    /// `snATAC-Seq`
    ///
    /// * **VM Long Name**: Single Nucleus ATAC-Seq
    /// * **VM Public ID**: 13260568
    /// * **Concept Code**: C198496
    /// * **Begin Date**:   03/28/2023
    ///
    /// A molecular genetic technique where DNA is harvested from a single cell
    /// nucleus (sn) samples and amplified to create a genomic library. Then the
    /// library is subjected to ATAC-seq, which isolates and sequences regions
    /// rich in open chromatin.
    #[serde(rename = "snATAC-Seq")]
    SnatacSeq,

    /// `ssRNA-seq`
    ///
    /// * **VM Long Name**: ssRNA-seq
    /// * **VM Public ID**: 6273379
    /// * **Concept Code**: C172859
    /// * **Begin Date**:   05/11/2018
    ///
    /// Bidirectional sequencing to determine the nucleotide sequence of the
    /// complementary strand and/or the transcriptional strand of a transcribed
    /// RNA.
    #[serde(rename = "ssRNA-seq")]
    SsrnaSeq,

    /// `Synthetic-Long-Read`
    ///
    /// * **VM Long Name**: Synthetic-Long-Read
    /// * **VM Public ID**: 6273378
    /// * **Concept Code**: C204827
    /// * **Begin Date**:   05/11/2018
    ///
    /// A sequence analysis method that uses sample processing and conventional
    /// sequencing to computationally reconstruct long reads from shorter
    /// sequencing reads.
    #[serde(rename = "Synthetic-Long-Read")]
    SyntheticLongRead,

    /// `Targeted-Capture`
    ///
    /// * **VM Long Name**: Targeted-Capture
    /// * **VM Public ID**: 6273377
    /// * **Concept Code**: C204828
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method to enrich and sequence genomic regions of interest. Sequences
    /// are selected by oligonucleotides that are used to pull-down
    /// complementary DNA through hybridization.
    #[serde(rename = "Targeted-Capture")]
    TargetedCapture,

    /// `Tethered Chromatin Conformation Capture`
    ///
    /// * **VM Long Name**: Tethered Chromatin Conformation Capture
    /// * **VM Public ID**: 6273376
    /// * **Concept Code**: C204829
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method for genome-wide mapping of chromatin interactions. It is
    /// similar to Hi-C sequencing except that the ligations are performed on a
    /// solid substrate rather than in solution. This enhances the signal-to-
    /// noise ratio, thereby facilitating a detailed analysis of interactions
    /// within and between chromosomes.
    #[serde(rename = "Tethered Chromatin Conformation Capture")]
    TetheredChromatinConformationCapture,

    /// `Tn-Seq`
    ///
    /// * **VM Long Name**: Tn-Seq
    /// * **VM Public ID**: 6273375
    /// * **Concept Code**: C204830
    /// * **Begin Date**:   05/11/2018
    ///
    /// A method for accurately determining quantitative genetic interactions on
    /// a genome-wide scale in microorganisms. It is based on the assembly of a
    /// saturated Mariner transposon insertion library. After library selection,
    /// changes in frequency of each insertion mutant are determined by
    /// sequencing the flanking regions en masse. Insertion site identification
    /// can be used to link DNA mutations to phenotypic variants on a genome-
    /// wide scale.
    #[serde(rename = "Tn-Seq")]
    TnSeq,

    /// `WCS`
    ///
    /// * **VM Long Name**: WCS
    /// * **VM Public ID**: 6273374
    /// * **Concept Code**: C204831
    /// * **Begin Date**:   05/11/2018
    ///
    /// A DNA sequencing method, which involves random sequencing of clones
    /// derived from a whole chromosome or other genomic replicon. The sequences
    /// can be compared and aligned computationally to assemble the entire
    /// chromosome or replicon sequence.
    #[serde(rename = "WCS")]
    Wcs,

    /// `WGA`
    ///
    /// * **VM Long Name**: WGA
    /// * **VM Public ID**: 6273373
    /// * **Concept Code**: C19590
    /// * **Begin Date**:   05/11/2018
    ///
    /// Any technique designed to amplify a limited genomic DNA sample so as to
    /// generate a new sample that is indistinguishable from the original but
    /// with a higher DNA concentration.
    #[serde(rename = "WGA")]
    Wga,

    /// `WGS`
    ///
    /// * **VM Long Name**: Whole Genome Sequencing
    /// * **VM Public ID**: 3463244
    /// * **Concept Code**: C101294
    /// * **Begin Date**:   05/11/2018
    ///
    /// A procedure that can determine the DNA sequence for nearly the entire
    /// genome of an individual.
    #[serde(rename = "WGS")]
    Wgs,

    /// `WXS`
    ///
    /// * **VM Long Name**: WXS
    /// * **VM Public ID**: 6273372
    /// * **Concept Code**: C101295
    /// * **Begin Date**:   05/11/2018
    ///
    /// A procedure that can determine the DNA sequence for all of the exons in
    /// an individual.
    #[serde(rename = "WXS")]
    Wxs,
}

impl CDE for LibraryStrategy {}

impl std::fmt::Display for LibraryStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibraryStrategy::Amplicon => write!(f, "AMPLICON"),
            LibraryStrategy::AtacSeq => write!(f, "ATAC-Seq"),
            LibraryStrategy::BisulfiteSeq => write!(f, "Bisulfite-Seq"),
            LibraryStrategy::ChiaPet => write!(f, "ChIA-PET"),
            LibraryStrategy::ChipSeq => write!(f, "ChIP-Seq"),
            LibraryStrategy::Clone => write!(f, "CLONE"),
            LibraryStrategy::Cloneend => write!(f, "CLONEEND"),
            LibraryStrategy::Cts => write!(f, "CTS"),
            LibraryStrategy::DnaSeq => write!(f, "DNA-Seq"),
            LibraryStrategy::DnaseHypersensitivity => write!(f, "DNase-Hypersensitivity"),
            LibraryStrategy::Est => write!(f, "EST"),
            LibraryStrategy::FaireSeq => write!(f, "FAIRE-seq"),
            LibraryStrategy::Finishing => write!(f, "FINISHING"),
            LibraryStrategy::FlCdna => write!(f, "FL-cDNA"),
            LibraryStrategy::HiC => write!(f, "Hi-C"),
            LibraryStrategy::MbdSeq => write!(f, "MBD-Seq"),
            LibraryStrategy::MedipSeq => write!(f, "MeDIP-Seq"),
            LibraryStrategy::MirnaSeq => write!(f, "miRNA-Seq"),
            LibraryStrategy::MnaseSeq => write!(f, "MNase-Seq"),
            LibraryStrategy::MreSeq => write!(f, "MRE-Seq"),
            LibraryStrategy::NcrnaSeq => write!(f, "ncRNA-Seq"),
            LibraryStrategy::Other => write!(f, "Other"),
            LibraryStrategy::Poolclone => write!(f, "POOLCLONE"),
            LibraryStrategy::RadSeq => write!(f, "RAD-Seq"),
            LibraryStrategy::RipSeq => write!(f, "RIP-Seq"),
            LibraryStrategy::RnaSeq => write!(f, "RNA-Seq"),
            LibraryStrategy::Selex => write!(f, "SELEX"),
            LibraryStrategy::SnatacSeq => write!(f, "snATAC-Seq"),
            LibraryStrategy::SsrnaSeq => write!(f, "ssRNA-seq"),
            LibraryStrategy::SyntheticLongRead => write!(f, "Synthetic-Long-Read"),
            LibraryStrategy::TargetedCapture => write!(f, "Targeted-Capture"),
            LibraryStrategy::TetheredChromatinConformationCapture => {
                write!(f, "Tethered Chromatin Conformation Capture")
            }
            LibraryStrategy::TnSeq => write!(f, "Tn-Seq"),
            LibraryStrategy::Wcs => write!(f, "WCS"),
            LibraryStrategy::Wga => write!(f, "WGA"),
            LibraryStrategy::Wgs => write!(f, "WGS"),
            LibraryStrategy::Wxs => write!(f, "WXS"),
        }
    }
}

impl Distribution<LibraryStrategy> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LibraryStrategy {
        match rng.gen_range(0..37) {
            0 => LibraryStrategy::Amplicon,
            1 => LibraryStrategy::AtacSeq,
            2 => LibraryStrategy::BisulfiteSeq,
            3 => LibraryStrategy::ChiaPet,
            4 => LibraryStrategy::ChipSeq,
            5 => LibraryStrategy::Clone,
            6 => LibraryStrategy::Cloneend,
            7 => LibraryStrategy::Cts,
            8 => LibraryStrategy::DnaSeq,
            9 => LibraryStrategy::DnaseHypersensitivity,
            10 => LibraryStrategy::Est,
            11 => LibraryStrategy::FaireSeq,
            12 => LibraryStrategy::Finishing,
            13 => LibraryStrategy::FlCdna,
            14 => LibraryStrategy::HiC,
            15 => LibraryStrategy::MbdSeq,
            16 => LibraryStrategy::MedipSeq,
            17 => LibraryStrategy::MirnaSeq,
            18 => LibraryStrategy::MnaseSeq,
            19 => LibraryStrategy::MreSeq,
            20 => LibraryStrategy::NcrnaSeq,
            21 => LibraryStrategy::Other,
            22 => LibraryStrategy::Poolclone,
            23 => LibraryStrategy::RadSeq,
            24 => LibraryStrategy::RipSeq,
            25 => LibraryStrategy::RnaSeq,
            26 => LibraryStrategy::Selex,
            27 => LibraryStrategy::SnatacSeq,
            28 => LibraryStrategy::SsrnaSeq,
            29 => LibraryStrategy::SyntheticLongRead,
            30 => LibraryStrategy::TargetedCapture,
            31 => LibraryStrategy::TetheredChromatinConformationCapture,
            32 => LibraryStrategy::TnSeq,
            33 => LibraryStrategy::Wcs,
            34 => LibraryStrategy::Wga,
            35 => LibraryStrategy::Wgs,
            _ => LibraryStrategy::Wxs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(LibraryStrategy::Amplicon.to_string(), "AMPLICON");
        assert_eq!(LibraryStrategy::AtacSeq.to_string(), "ATAC-Seq");
        assert_eq!(LibraryStrategy::BisulfiteSeq.to_string(), "Bisulfite-Seq");
        assert_eq!(LibraryStrategy::ChiaPet.to_string(), "ChIA-PET");
        assert_eq!(LibraryStrategy::ChipSeq.to_string(), "ChIP-Seq");
        assert_eq!(LibraryStrategy::Clone.to_string(), "CLONE");
        assert_eq!(LibraryStrategy::Cloneend.to_string(), "CLONEEND");
        assert_eq!(LibraryStrategy::Cts.to_string(), "CTS");
        assert_eq!(LibraryStrategy::DnaSeq.to_string(), "DNA-Seq");
        assert_eq!(
            LibraryStrategy::DnaseHypersensitivity.to_string(),
            "DNase-Hypersensitivity"
        );
        assert_eq!(LibraryStrategy::Est.to_string(), "EST");
        assert_eq!(LibraryStrategy::FaireSeq.to_string(), "FAIRE-seq");
        assert_eq!(LibraryStrategy::Finishing.to_string(), "FINISHING");
        assert_eq!(LibraryStrategy::FlCdna.to_string(), "FL-cDNA");
        assert_eq!(LibraryStrategy::HiC.to_string(), "Hi-C");
        assert_eq!(LibraryStrategy::MbdSeq.to_string(), "MBD-Seq");
        assert_eq!(LibraryStrategy::MedipSeq.to_string(), "MeDIP-Seq");
        assert_eq!(LibraryStrategy::MirnaSeq.to_string(), "miRNA-Seq");
        assert_eq!(LibraryStrategy::MnaseSeq.to_string(), "MNase-Seq");
        assert_eq!(LibraryStrategy::MreSeq.to_string(), "MRE-Seq");
        assert_eq!(LibraryStrategy::NcrnaSeq.to_string(), "ncRNA-Seq");
        assert_eq!(LibraryStrategy::Other.to_string(), "Other");
        assert_eq!(LibraryStrategy::Poolclone.to_string(), "POOLCLONE");
        assert_eq!(LibraryStrategy::RadSeq.to_string(), "RAD-Seq");
        assert_eq!(LibraryStrategy::RipSeq.to_string(), "RIP-Seq");
        assert_eq!(LibraryStrategy::RnaSeq.to_string(), "RNA-Seq");
        assert_eq!(LibraryStrategy::Selex.to_string(), "SELEX");
        assert_eq!(LibraryStrategy::SnatacSeq.to_string(), "snATAC-Seq");
        assert_eq!(LibraryStrategy::SsrnaSeq.to_string(), "ssRNA-seq");
        assert_eq!(
            LibraryStrategy::SyntheticLongRead.to_string(),
            "Synthetic-Long-Read"
        );
        assert_eq!(
            LibraryStrategy::TargetedCapture.to_string(),
            "Targeted-Capture"
        );
        assert_eq!(
            LibraryStrategy::TetheredChromatinConformationCapture.to_string(),
            "Tethered Chromatin Conformation Capture"
        );
        assert_eq!(LibraryStrategy::TnSeq.to_string(), "Tn-Seq");
        assert_eq!(LibraryStrategy::Wcs.to_string(), "WCS");
        assert_eq!(LibraryStrategy::Wga.to_string(), "WGA");
        assert_eq!(LibraryStrategy::Wgs.to_string(), "WGS");
        assert_eq!(LibraryStrategy::Wxs.to_string(), "WXS");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Amplicon).unwrap(),
            "\"AMPLICON\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::AtacSeq).unwrap(),
            "\"ATAC-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::BisulfiteSeq).unwrap(),
            "\"Bisulfite-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::ChiaPet).unwrap(),
            "\"ChIA-PET\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::ChipSeq).unwrap(),
            "\"ChIP-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Clone).unwrap(),
            "\"CLONE\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Cloneend).unwrap(),
            "\"CLONEEND\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Cts).unwrap(),
            "\"CTS\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::DnaSeq).unwrap(),
            "\"DNA-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::DnaseHypersensitivity).unwrap(),
            "\"DNase-Hypersensitivity\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Est).unwrap(),
            "\"EST\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::FaireSeq).unwrap(),
            "\"FAIRE-seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Finishing).unwrap(),
            "\"FINISHING\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::FlCdna).unwrap(),
            "\"FL-cDNA\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::HiC).unwrap(),
            "\"Hi-C\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::MbdSeq).unwrap(),
            "\"MBD-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::MedipSeq).unwrap(),
            "\"MeDIP-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::MirnaSeq).unwrap(),
            "\"miRNA-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::MnaseSeq).unwrap(),
            "\"MNase-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::MreSeq).unwrap(),
            "\"MRE-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::NcrnaSeq).unwrap(),
            "\"ncRNA-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Other).unwrap(),
            "\"Other\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Poolclone).unwrap(),
            "\"POOLCLONE\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::RadSeq).unwrap(),
            "\"RAD-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::RipSeq).unwrap(),
            "\"RIP-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::RnaSeq).unwrap(),
            "\"RNA-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Selex).unwrap(),
            "\"SELEX\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::SnatacSeq).unwrap(),
            "\"snATAC-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::SsrnaSeq).unwrap(),
            "\"ssRNA-seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::SyntheticLongRead).unwrap(),
            "\"Synthetic-Long-Read\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::TargetedCapture).unwrap(),
            "\"Targeted-Capture\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::TetheredChromatinConformationCapture).unwrap(),
            "\"Tethered Chromatin Conformation Capture\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::TnSeq).unwrap(),
            "\"Tn-Seq\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Wcs).unwrap(),
            "\"WCS\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Wga).unwrap(),
            "\"WGA\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Wgs).unwrap(),
            "\"WGS\""
        );
        assert_eq!(
            serde_json::to_string(&LibraryStrategy::Wxs).unwrap(),
            "\"WXS\""
        );
    }
}
