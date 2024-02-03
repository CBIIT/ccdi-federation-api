//! A type for a file.

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11416926 v1.00`**
///
/// This metadata element is defined by the caDSR as "A defined organization or
/// layout representing and structuring data in a computer file.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11416926%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema, Introspect,
)]
#[schema(as = cde::v1::file::Type)]
#[allow(clippy::upper_case_acronyms)]
pub enum Type {
    /// `HIC`
    ///
    /// * **VM Long Name**: HIC File Format
    /// * **VM Public ID**: 14728934
    /// * **Concept Code**: C203672
    /// * **Begin Date**:   01/25/2024
    ///
    /// An indexed binary format designed to permit fast random access to
    /// contact matrix heatmaps.
    #[serde(rename = "HIC")]
    HIC,

    /// `BEDPE Format`
    ///
    /// * **VM Long Name**: BEDPE Format
    /// * **VM Public ID**: 14535229
    /// * **Concept Code**: C184749
    /// * **Begin Date**:   10/16/2023
    ///
    /// A text file format used to store genomic paired-end sequencing data that
    /// includes coordinates and associated metadata.
    #[serde(rename = "BEDPE Format")]
    BEDPEFormat,

    /// `mtx`
    ///
    /// * **VM Long Name**: mtx
    /// * **VM Public ID**: 14534322
    /// * **Concept Code**: C201789
    /// * **Begin Date**:   10/16/2023
    ///
    /// A format for the storage of numerical or pattern matrices in a dense
    /// (array format) or sparse (coordinate format) representation.
    #[serde(rename = "mtx")]
    MTX,

    /// `mzIdentML`
    ///
    /// * **VM Long Name**: mzIdentML
    /// * **VM Public ID**: 14534321
    /// * **Concept Code**: C201788
    /// * **Begin Date**:   10/16/2023
    ///
    /// An exchange format for peptides and proteins identified from mass
    /// spectra.
    #[serde(rename = "mzIdentML")]
    MzIdentML,

    /// `MEX`
    ///
    /// * **VM Long Name**: MEX Format
    /// * **VM Public ID**: 14532371
    /// * **Concept Code**: C184778
    /// * **Begin Date**:   10/13/2023
    ///
    /// A minimal base ASCII file format to facilitate the exchange of data
    /// matrices.
    #[serde(rename = "MEX")]
    MEX,

    /// `CRAI`
    ///
    /// * **VM Long Name**: CRAI File
    /// * **VM Public ID**: 12517496
    /// * **Concept Code**: C192224
    /// * **Begin Date**:   02/01/2023
    ///
    /// An external index file created by the sequencing data archive alongside
    /// a compressed reference-oriented alignment map (CRAM) file. This index
    /// file is available in the same directory as the CRAM file and is required
    /// for selective access to the genomic sequence alignment data within the
    /// CRAM file.
    #[serde(rename = "CRAI")]
    CRAI,

    /// `CEL`
    ///
    /// * **VM Long Name**: Affymetrix CEL Format
    /// * **VM Public ID**: 12135194
    /// * **Concept Code**: C191737
    /// * **Begin Date**:   12/12/2022
    ///
    /// A type of file that stores the results of intensity calculations of
    /// pixel values for DNA microarray image analysis. The data includes an
    /// intensity value, standard deviation of the intensity, the number of
    /// pixels used to calculate the intensity value, a flag to indicate an
    /// outlier as calculated by the algorithm, and a user defined flag
    /// indicating whether the feature should be excluded from future analysis.
    /// The file also stores the previously stated data for each feature on the
    /// probe array.
    #[serde(rename = "CEL")]
    CEL,

    /// `mzXML`
    ///
    /// * **VM Long Name**: Mass Spectrometry Extensible Markup Language
    /// * **VM Public ID**: 11422143
    /// * **Concept Code**: C47924
    /// * **Begin Date**:   10/12/2022
    ///
    /// An XML based common file format for mass spectrometry data.
    #[serde(rename = "mzXML")]
    MzXML,

    /// `IDAT`
    ///
    /// * **VM Long Name**: IDAT Format
    /// * **VM Public ID**: 11495707
    /// * **Concept Code**: C184762
    /// * **Begin Date**:   10/26/2022
    ///
    /// A proprietary, encrypted XML-based, compressed electronic file format
    /// from Illumina Inc. for storing genome-wide profiling data. The data
    /// contains a summary of the intensity data generated from each probe used
    /// during a sequencing run.
    #[serde(rename = "IDAT")]
    IDAT,

    /// `IDF`
    ///
    /// * **VM Long Name**: Investigation Description Format
    /// * **VM Public ID**: 11422065
    /// * **Concept Code**: C172212
    /// * **Begin Date**:   10/26/2022
    ///
    /// A a tab-delimited containing information about an investigation,
    /// including its name, a description, the investigator's contact details,
    /// bibliographic references, and free text descriptions of any relevant
    /// protocols.
    #[serde(rename = "IDF")]
    IDF,

    /// `JPEG`
    ///
    /// * **VM Long Name**: JPEG
    /// * **VM Public ID**: 11422138
    /// * **Concept Code**: C48230
    /// * **Begin Date**:   10/26/2022
    ///
    /// A file compression format mostly used for color and greyscale
    /// photographs.
    #[serde(rename = "JPEG")]
    JPEG,

    /// `JPEG 2000`
    ///
    /// * **VM Long Name**: JPEG 2000 Format
    /// * **VM Public ID**: 11422135
    /// * **Concept Code**: C184768
    /// * **Begin Date**:   10/26/2022
    ///
    /// A discrete wavelet transform (DWT)-based compression standard electronic
    /// file format for storing video images.
    #[serde(rename = "JPEG 2000")]
    JPEG2000,

    /// `JSON`
    ///
    /// * **VM Long Name**: JSON Format
    /// * **VM Public ID**: 11422136
    /// * **Concept Code**: C184769
    /// * **Begin Date**:   10/26/2022
    ///
    /// A common open standard file format and data interchange format that uses
    /// human-readable text consisting of attribute-value pairs and arrays to
    /// store and transmit data.
    #[serde(rename = "JSON")]
    JSON,

    /// `MAF`
    ///
    /// * **VM Long Name**: Mutation Annotation Format
    /// * **VM Public ID**: 11422137
    /// * **Concept Code**: C172215
    /// * **Begin Date**:   10/26/2022
    ///
    /// A tab-delimited text file containing mutation information aggregated
    /// from variant call files (VCF) across a project, trial or study.
    #[serde(rename = "MAF")]
    MAF,

    /// `YAML`
    ///
    /// * **VM Long Name**: YAML Language
    /// * **VM Public ID**: 11426805
    /// * **Concept Code**: C190193
    /// * **Begin Date**:   10/12/2022
    ///
    /// A human-readable, tree-structured data-serialization language often used
    /// to create configuration files.
    #[serde(rename = "YAML")]
    YAML,

    /// `ZIP`
    ///
    /// * **VM Long Name**: ZIP Format
    /// * **VM Public ID**: 11426806
    /// * **Concept Code**: C190192
    /// * **Begin Date**:   10/12/2022
    ///
    /// An archive file format that supports lossless data compression. It is
    /// used to compress one or more files together into a single location.
    #[serde(rename = "ZIP")]
    ZIP,

    /// `SVS`
    ///
    /// * **VM Long Name**: Spectral View Settings Format
    /// * **VM Public ID**: 11426797
    /// * **Concept Code**: C172214
    /// * **Begin Date**:   10/12/2022
    ///
    /// An electronic file format based on TIFF that is used for storing
    /// multiple high-resolution images in a single electronic file.
    #[serde(rename = "SVS")]
    SVS,

    /// `TAR`
    ///
    /// * **VM Long Name**: TAR Format
    /// * **VM Public ID**: 11426798
    /// * **Concept Code**: C190189
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file format generated by the Unix command-line utility tar (tape
    /// archive). It is used for collecting many files into one archive file.
    #[serde(rename = "TAR")]
    TAR,

    /// `Thermo RAW`
    ///
    /// * **VM Long Name**: Thermo RAW Format
    /// * **VM Public ID**: 11426799
    /// * **Concept Code**: C190190
    /// * **Begin Date**:   10/12/2022
    ///
    /// A proprietary file format developed by Thermo Scientific to capture mass
    /// spectrometry data produced by Thermo mass spectrometers.
    #[serde(rename = "Thermo RAW")]
    ThermoRAW,

    /// `TIFF`
    ///
    /// * **VM Long Name**: TIFF
    /// * **VM Public ID**: 11426800
    /// * **Concept Code**: C70631
    /// * **Begin Date**:   10/12/2022
    ///
    /// A bitmap graphics file format utilizing tagged fields.
    #[serde(rename = "TIFF")]
    TIFF,

    /// `TSV`
    ///
    /// * **VM Long Name**: Tab-Separated Value Format
    /// * **VM Public ID**: 11426801
    /// * **Concept Code**: C164049
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file format where each line in the file contains a single piece of
    /// data and where each field or value in a line of data is separated from
    /// the next by a tab character.
    #[serde(rename = "TSV")]
    TSV,

    /// `TXT`
    ///
    /// * **VM Long Name**: Plain Text Data Format
    /// * **VM Public ID**: 11422197
    /// * **Concept Code**: C85873
    /// * **Begin Date**:   10/12/2022
    ///
    /// A data format consisting of readable textual material maintained as a
    /// sequential file.
    #[serde(rename = "TXT")]
    TXT,

    /// `VCF`
    ///
    /// * **VM Long Name**: Variant Call File Format
    /// * **VM Public ID**: 11426802
    /// * **Concept Code**: C172216
    /// * **Begin Date**:   10/12/2022
    ///
    /// A text-based electronic file used for storing gene sequence variation
    /// data. The first text section is composed of a header containing the
    /// metadata and keywords used in the file. This is followed by the body of
    /// the file which is tab-separated into eight mandatory data columns for
    /// each sample. Additionally, the body of the file can include an unlimited
    /// number of optional columns to record other sample-related data.
    #[serde(rename = "VCF")]
    VCF,

    /// `XLS`
    ///
    /// * **VM Long Name**: XLS Format
    /// * **VM Public ID**: 11426803
    /// * **Concept Code**: C190191
    /// * **Begin Date**:   10/12/2022
    ///
    /// A proprietary binary file format used to store spreadsheet data in
    /// Microsoft Excel.
    #[serde(rename = "XLS")]
    XLS,

    /// `XLSX`
    ///
    /// * **VM Long Name**: Excel Open XML Format
    /// * **VM Public ID**: 11426804
    /// * **Concept Code**: C164050
    /// * **Begin Date**:   10/12/2022
    ///
    /// A proprietary file format developed by Microsoft that allows the user to
    /// save a spreadsheet created in Excel in an open XML-format. The file then
    /// can be read and opened by other spreadsheet-compatible applications.
    #[serde(rename = "XLSX")]
    XLSX,

    /// `XML`
    ///
    /// * **VM Long Name**: Extensible Markup Language
    /// * **VM Public ID**: 2581637
    /// * **Concept Code**: C45967
    /// * **Begin Date**:   10/12/2022
    ///
    /// Extensible Markup Language
    #[serde(rename = "XML")]
    XML,

    /// `MATLAB Script`
    ///
    /// * **VM Long Name**: MATLAB Script File
    /// * **VM Public ID**: 11422140
    /// * **Concept Code**: C190179
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file that contains MATLAB commands and function calls.
    #[serde(rename = "MATLAB Script")]
    MATLABScript,

    /// `NIFTI Format`
    ///
    /// * **VM Long Name**: NIfTI Format
    /// * **VM Public ID**: 11422144
    /// * **Concept Code**: C190183
    /// * **Begin Date**:   10/12/2022
    ///
    /// An open file format used to store neuroscience and neuroradiology
    /// imaging data obtained by MRI. The raw image data in NIfTI is saved as a
    /// 3d image.
    #[serde(rename = "NIFTI Format")]
    NIFTIFormat,

    /// `R Markdown`
    ///
    /// * **VM Long Name**: R Markdown Format
    /// * **VM Public ID**: 11422201
    /// * **Concept Code**: C190187
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file format for making dynamic documents written in R. It is written
    /// in plain text format and contains chunks of embedded R code.
    #[serde(rename = "R Markdown")]
    RMarkdown,

    /// `R File Format`
    ///
    /// * **VM Long Name**: R Format
    /// * **VM Public ID**: 11422200
    /// * **Concept Code**: C190186
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file format used for writing scripts in the R programming language for
    /// execution within the R software environment, typically for statistical
    /// computation and graphics.
    #[serde(rename = "R File Format")]
    RFileFormat,

    /// `Python Script Format`
    ///
    /// * **VM Long Name**: Python Script Format
    /// * **VM Public ID**: 11422199
    /// * **Concept Code**: C190184
    /// * **Begin Date**:   10/12/2022
    ///
    /// A plain text file that stores python code.
    #[serde(rename = "Python Script Format")]
    PythonScriptFormat,

    /// `Sequence Record Format`
    ///
    /// * **VM Long Name**: Sequence Record Format
    /// * **VM Public ID**: 11422203
    /// * **Concept Code**: C190188
    /// * **Begin Date**:   10/12/2022
    ///
    /// Any file format designed to store molecular sequence data.
    #[serde(rename = "Sequence Record Format")]
    SequenceRecordFormat,

    /// `SVG`
    ///
    /// * **VM Long Name**: Scalable Vector Graphics
    /// * **VM Public ID**: 11422204
    /// * **Concept Code**: C85435
    /// * **Begin Date**:   10/12/2022
    ///
    /// A specification for describing vector graphics using extensible markup
    /// language.
    #[serde(rename = "SVG")]
    SVG,

    /// `OME-TIFF`
    ///
    /// * **VM Long Name**: Open Microscopy Environment Tagged Image File Format
    /// * **VM Public ID**: 11422196
    /// * **Concept Code**: C181618
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tagged image file format (TIFF) image file format that contains an
    /// Open Microscopy Environment (OME) XML metadata block in the file header
    /// for storing microscopy information (pixels and metadata) using the OME
    /// Data Model.
    #[serde(rename = "OME-TIFF")]
    OMETIFF,

    /// `PDF`
    ///
    /// * **VM Long Name**: Portable Document Format
    /// * **VM Public ID**: 2967828
    /// * **Concept Code**: C63805
    /// * **Begin Date**:   10/12/2022
    ///
    /// An open file format created and controlled by Adobe Systems, for
    /// representing two-dimensional documents in a device independent and
    /// resolution independent fixed-layout document format. Each PDF file
    /// encapsulates a complete description of a 2D document that includes the
    /// text, fonts, images, and 2D vector graphics that compose the document.
    /// PDF files do not encode information that is specific to the application
    /// software, hardware, or operating system used to create or view the
    /// document. This feature ensures that a valid PDF will render exactly the
    /// same regardless of its origin or destination (but depending on font
    /// availability when fonts are not encapsulated in the file).
    #[serde(rename = "PDF")]
    PDF,

    /// `Plain Text Data Format`
    ///
    /// * **VM Long Name**: Plain Text Data Format
    /// * **VM Public ID**: 11422197
    /// * **Concept Code**: C85873
    /// * **Begin Date**:   10/12/2022
    ///
    /// A data format consisting of readable textual material maintained as a
    /// sequential file.
    #[serde(rename = "Plain Text Data Format")]
    PlainTextDataFormat,

    /// `PNG`
    ///
    /// * **VM Long Name**: Portable Network Graphics
    /// * **VM Public ID**: 11422198
    /// * **Concept Code**: C85437
    /// * **Begin Date**:   10/12/2022
    ///
    /// An extensible file format for lossless data compression of images.
    #[serde(rename = "PNG")]
    PNG,

    /// `SDRF`
    ///
    /// * **VM Long Name**: Sample and Data Relationship Format
    /// * **VM Public ID**: 11422202
    /// * **Concept Code**: C172211
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-delimited file describing the relationships between samples,
    /// arrays, data and other objects used or produced during a microarray
    /// experiment.
    #[serde(rename = "SDRF")]
    SDRF,

    /// `MAGE-TAB`
    ///
    /// * **VM Long Name**: MAGE-TAB
    /// * **VM Public ID**: 2953599
    /// * **Concept Code**: C82937
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-delimited, spreadsheet-based format that can be used for
    /// annotating and communicating microarray data in a MIAME (Minimum
    /// Information About a Microarray Experiment) compliant fashion.
    #[serde(rename = "MAGE-TAB")]
    MAGETAB,

    /// `MAT`
    ///
    /// * **VM Long Name**: MAT Format
    /// * **VM Public ID**: 11422139
    /// * **Concept Code**: C190178
    /// * **Begin Date**:   10/12/2022
    ///
    /// A proprietary, binary data container format used by MATLAB software to
    /// store workspace variables.
    #[serde(rename = "MAT")]
    MAT,

    /// `MPEG-4`
    ///
    /// * **VM Long Name**: MP4 Format
    /// * **VM Public ID**: 11422141
    /// * **Concept Code**: C190180
    /// * **Begin Date**:   10/12/2022
    ///
    /// A multimedia file that is commonly used to store  video and audio data.
    #[serde(rename = "MPEG-4")]
    MPEG4,

    /// `mzML`
    ///
    /// * **VM Long Name**: Mass Spectrometry Markup Language
    /// * **VM Public ID**: 11422142
    /// * **Concept Code**: C190181
    /// * **Begin Date**:   10/12/2022
    ///
    /// An open-source format for mass spectrometer output files that was
    /// designed to be utilized and adapted as advances in mass spectrometry
    /// technology proceed. This was standardized by the Human Proteome
    /// Organization (HUPO)-Proteomics Standards Initiative (PSI) Mass
    /// Spectrometry Standards (MSS) Working Group.
    #[serde(rename = "mzML")]
    MzML,

    /// `GenBank Format`
    ///
    /// * **VM Long Name**: GenBank Format
    /// * **VM Public ID**: 11422129
    /// * **Concept Code**: C190174
    /// * **Begin Date**:   10/12/2022
    ///
    /// A type of flat file format that allows for the storage of DNA or protein
    /// sequences and annotations. It consists of an annotation section and a
    /// sequence section.
    #[serde(rename = "GenBank Format")]
    GenBankFormat,

    /// `GFF3`
    ///
    /// * **VM Long Name**: General Feature Format Version 3
    /// * **VM Public ID**: 11422130
    /// * **Concept Code**: C190175
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-separated format for sequence data. It uses one line per feature,
    /// each containing 9 columns of data, plus optional track definition lines.
    #[serde(rename = "GFF3")]
    GFF3,

    /// `GPR`
    ///
    /// * **VM Long Name**: GenePix Results Format
    /// * **VM Public ID**: 11422131
    /// * **Concept Code**: C190176
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-delimited text file format developed by Axon Instruments and used
    /// to export data from the microarray image analysis tool GenePix.
    #[serde(rename = "GPR")]
    GPR,

    /// `GTF`
    ///
    /// * **VM Long Name**: GTF Format
    /// * **VM Public ID**: 11422132
    /// * **Concept Code**: C190177
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-delimited text format based on the general feature format (GFF)
    /// but which also contains some additional information for each gene.
    #[serde(rename = "GTF")]
    GTF,

    /// `GZIP Format`
    ///
    /// * **VM Long Name**: Gzip File Format
    /// * **VM Public ID**: 2929849
    /// * **Concept Code**: C80220
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file format consisting of a 10-byte header containing a magic number,
    /// a version number, and a timestamp, a Deflate-compressed body, and an
    /// 8-byte footer containing a checksum and the length of the original
    /// uncompressed data.
    #[serde(rename = "GZIP Format")]
    GZIPFormat,

    /// `HDF5`
    ///
    /// * **VM Long Name**: Hierarchical Data Format 5
    /// * **VM Public ID**: 11422133
    /// * **Concept Code**: C184763
    /// * **Begin Date**:   10/12/2022
    ///
    /// A hierarchical, filesystem-like data format that can store metadata in
    /// the form of user-defined, named attributes, which are attached to groups
    /// and datasets, and representations of images and tables built up using
    /// datasets, groups and attributes.
    #[serde(rename = "HDF5")]
    HDF5,

    /// `HTML`
    ///
    /// * **VM Long Name**: Hypertext Markup Language
    /// * **VM Public ID**: 11422134
    /// * **Concept Code**: C142380
    /// * **Begin Date**:   10/12/2022
    ///
    /// A standard markup language used to display content on a web page, as
    /// specified by the World Wide Web Consortium (W3C).
    #[serde(rename = "HTML")]
    HTML,

    /// `BIOM`
    ///
    /// * **VM Long Name**: BIOM Format
    /// * **VM Public ID**: 11422059
    /// * **Concept Code**: C190169
    /// * **Begin Date**:   10/12/2022
    ///
    /// A general-use format for representing biological sample by observation
    /// contingency tables. It is designed for general use in broad areas of
    /// comparative -omics. The primary use of this format is to represent
    /// operational taxonomic unit (OTU) and metagenome tables.
    #[serde(rename = "BIOM")]
    BIOM,

    /// `CRAM`
    ///
    /// * **VM Long Name**: CRAM Format
    /// * **VM Public ID**: 11422061
    /// * **Concept Code**: C190170
    /// * **Begin Date**:   10/12/2022
    ///
    /// A more highly compressed alternative to the BAM and SAM DNA sequence
    /// alignment file formats. It uses reference based compression, with only
    /// base calls that differ from a designated reference sequence being
    /// stored.
    #[serde(rename = "CRAM")]
    CRAM,

    /// `CSV`
    ///
    /// * **VM Long Name**: Comma Separated Values Format
    /// * **VM Public ID**: 7797849
    /// * **Concept Code**: C182456
    /// * **Begin Date**:   10/12/2022
    ///
    /// A text file format that separates data elements with commas.
    #[serde(rename = "CSV")]
    CSV,

    /// `DICOM`
    ///
    /// * **VM Long Name**: DICOM
    /// * **VM Public ID**: 2967848
    /// * **Concept Code**: C49059
    /// * **Begin Date**:   10/12/2022
    ///
    /// A comprehensive set of standards for communications between medical
    /// imaging devices, including handling, storing and transmitting
    /// information in medical imaging. It includes a file format definition and
    /// a network communication protocol.
    #[serde(rename = "DICOM")]
    DICOM,

    /// `DOCX`
    ///
    /// * **VM Long Name**: DOCX Format
    /// * **VM Public ID**: 11422062
    /// * **Concept Code**: C190171
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file type that uses Open XML Document format to store data for
    /// Microsoft Word and other compatible programs.
    #[serde(rename = "DOCX")]
    DOCX,

    /// `DSV`
    ///
    /// * **VM Long Name**: DSV Format
    /// * **VM Public ID**: 11422063
    /// * **Concept Code**: C190172
    /// * **Begin Date**:   10/12/2022
    ///
    /// A delimiter-separated values file that contains plain text data divided
    /// into rows and columns by a designated special character.
    #[serde(rename = "DSV")]
    DSV,

    /// `FASTA`
    ///
    /// * **VM Long Name**: FASTA Format
    /// * **VM Public ID**: 11422064
    /// * **Concept Code**: C47845
    /// * **Begin Date**:   10/12/2022
    ///
    /// A sequence in FASTA format consists of a single-line description,
    /// followed by lines of sequence data. The first character of the
    /// description line is a greater-than (">") symbol in the first column.
    /// Sequences are represented in the standard IUB/IUPAC single letter amino
    /// acid and nucleic acid codes, with a single hyphen or dash being used to
    /// represent a gap of indeterminate length; in amino acid sequences asterix
    /// ("*") can represent a translation stop.
    #[serde(rename = "FASTA")]
    FASTA,

    /// `FASTQ`
    ///
    /// * **VM Long Name**: Investigation Description Format
    /// * **VM Public ID**: 11422065
    /// * **Concept Code**: C172212
    /// * **Begin Date**:   10/12/2022
    ///
    /// A a tab-delimited containing information about an investigation,
    /// including its name, a description, the investigator's contact details,
    /// bibliographic references, and free text descriptions of any relevant
    /// protocols.
    #[serde(rename = "FASTQ")]
    FASTQ,

    /// `GCT/Res Format`
    ///
    /// * **VM Long Name**: GCT/RES Format
    /// * **VM Public ID**: 11422066
    /// * **Concept Code**: C190173
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-delimited text file that contains gene expression data as a column
    /// for each sample, a row for each gene, and an expression value for each
    /// gene in each sample. RES differs from GCT by having labels for each
    /// gene's absent (A) versus present (P) calls as generated by Affymetrix's
    /// GeneChip software.
    #[serde(rename = "GCT/Res Format")]
    GCTResFormat,

    /// `AVI`
    ///
    /// * **VM Long Name**: AVI Format
    /// * **VM Public ID**: 11421997
    /// * **Concept Code**: C190162
    /// * **Begin Date**:   10/12/2022
    ///
    /// A multimedia container format for audio video interleaved (AVI) files
    /// that allows for synchronous audio-with-video playback.
    #[serde(rename = "AVI")]
    AVI,

    /// `BAI`
    ///
    /// * **VM Long Name**: BAI File
    /// * **VM Public ID**: 11421998
    /// * **Concept Code**: C190163
    /// * **Begin Date**:   10/12/2022
    ///
    /// An index file found in the same directory as the binary alignment map
    /// (BAM) file, a compressed binary version of a sequence alignment/map
    /// (SAM) file.
    #[serde(rename = "BAI")]
    BAI,

    /// `BAM`
    ///
    /// * **VM Long Name**: Binary Alignment Map
    /// * **VM Public ID**: 6356231
    /// * **Concept Code**: C153249
    /// * **Begin Date**:   10/12/2022
    ///
    /// A binary representation of a sequence alignment map (SAM), a compact and
    /// indexable representation of nucleotide sequence alignments, compressed
    /// by the BGZF (Blocked GNU Zip Format) library.
    #[serde(rename = "BAM")]
    BAM,

    /// `BCR Biotab`
    ///
    /// * **VM Long Name**: BCR Biotab Format
    /// * **VM Public ID**: 11421999
    /// * **Concept Code**: C190164
    /// * **Begin Date**:   10/12/2022
    ///
    /// Files that consist of aggregate clinical and biospecimen data across all
    /// cases of the given project. Biotab files are supplemental files that are
    /// available in the Genomic Data Commons (GDC) Legacy Archive as
    /// tab-delimited files on a project level basis. These may also include
    /// fields that are not available in the GDC application programming
    /// interface (API).
    #[serde(rename = "BCR Biotab")]
    BCRBiotab,

    /// `BED`
    ///
    /// * **VM Long Name**: BED Format
    /// * **VM Public ID**: 11422000
    /// * **Concept Code**: C153367
    /// * **Begin Date**:   10/12/2022
    ///
    /// A tab-delimited text file format that allows the specification of the
    /// sequence data that is displayed in an annotation track. The minimum
    /// required information is chromosome, start position, and end position.
    #[serde(rename = "BED")]
    BED,

    /// `bedgraph`
    ///
    /// * **VM Long Name**: Bedgraph Format
    /// * **VM Public ID**: 11422001
    /// * **Concept Code**: C190165
    /// * **Begin Date**:   10/12/2022
    ///
    /// A format that allows display of continuous-valued data in track format.
    /// This display type is useful for probability scores and transcriptome
    /// data.
    #[serde(rename = "bedgraph")]
    Bedgraph,

    /// `bigBed`
    ///
    /// * **VM Long Name**: BigBed Format
    /// * **VM Public ID**: 11422002
    /// * **Concept Code**: C190166
    /// * **Begin Date**:   10/12/2022
    ///
    /// A format for large sequence annotation tracks, similar to the textual
    /// browser extensible data (BED) format. The annotated items can be either
    /// simple or a linked collection of exons.
    #[serde(rename = "bigBed")]
    BigBed,

    /// `bigWig`
    ///
    /// * **VM Long Name**: BigWig Format
    /// * **VM Public ID**: 11422003
    /// * **Concept Code**: C190167
    /// * **Begin Date**:   10/12/2022
    ///
    /// A format for large sequence annotation tracks that consist of a value
    /// for each sequence position, similar to the textual wiggle (WIG) format.
    #[serde(rename = "bigWig")]
    BigWig,

    /// `Binary Format`
    ///
    /// * **VM Long Name**: Binary Format
    /// * **VM Public ID**: 11422004
    /// * **Concept Code**: C190168
    /// * **Begin Date**:   10/12/2022
    ///
    /// A file format in which file information is stored in the form of ones
    /// and zeros. It can be easily converted to text representations for
    /// visualization and editing.
    #[serde(rename = "Binary Format")]
    BinaryFormat,

    /// `ADF`
    ///
    /// * **VM Long Name**: Array Design Format
    /// * **VM Public ID**: 11421996
    /// * **Concept Code**: C172213
    ///
    /// A tab-delimited file defining how each array in an investigation is
    /// used. This data includes what sequence is located at each position in an
    /// array and what the annotation of this sequence is; cross-references to
    /// entries in a public repository can be used if available.
    #[serde(rename = "ADF")]
    ADF,
}

impl CDE for Type {}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::HIC => write!(f, "HIC"),
            Type::BEDPEFormat => write!(f, "BEDPE Format"),
            Type::MTX => write!(f, "mtx"),
            Type::MzIdentML => write!(f, "mzIdentML"),
            Type::MEX => write!(f, "MEX"),
            Type::CRAI => write!(f, "CRAI"),
            Type::CEL => write!(f, "CEL"),
            Type::MzXML => write!(f, "mzXML"),
            Type::IDAT => write!(f, "IDAT"),
            Type::IDF => write!(f, "IDF"),
            Type::JPEG => write!(f, "JPEG"),
            Type::JPEG2000 => write!(f, "JPEG 2000"),
            Type::JSON => write!(f, "JSON"),
            Type::MAF => write!(f, "MAF"),
            Type::YAML => write!(f, "YAML"),
            Type::ZIP => write!(f, "ZIP"),
            Type::SVS => write!(f, "SVS"),
            Type::TAR => write!(f, "TAR"),
            Type::ThermoRAW => write!(f, "Thermo RAW"),
            Type::TIFF => write!(f, "TIFF"),
            Type::TSV => write!(f, "TSV"),
            Type::TXT => write!(f, "TXT"),
            Type::VCF => write!(f, "VCF"),
            Type::XLS => write!(f, "XLS"),
            Type::XLSX => write!(f, "XLSX"),
            Type::XML => write!(f, "XML"),
            Type::MATLABScript => write!(f, "MATLAB Script"),
            Type::NIFTIFormat => write!(f, "NIFTI Format"),
            Type::RMarkdown => write!(f, "R Markdown"),
            Type::RFileFormat => write!(f, "R File Format"),
            Type::PythonScriptFormat => write!(f, "Python Script Format"),
            Type::SequenceRecordFormat => write!(f, "Sequence Record Format"),
            Type::SVG => write!(f, "SVG"),
            Type::OMETIFF => write!(f, "OME-TIFF"),
            Type::PDF => write!(f, "PDF"),
            Type::PlainTextDataFormat => write!(f, "Plain Text Data Format"),
            Type::PNG => write!(f, "PNG"),
            Type::SDRF => write!(f, "SDRF"),
            Type::MAGETAB => write!(f, "MAGE-TAB"),
            Type::MAT => write!(f, "MAT"),
            Type::MPEG4 => write!(f, "MPEG-4"),
            Type::MzML => write!(f, "mzML"),
            Type::GenBankFormat => write!(f, "GenBank Format"),
            Type::GFF3 => write!(f, "GFF3"),
            Type::GPR => write!(f, "GPR"),
            Type::GTF => write!(f, "GTF"),
            Type::GZIPFormat => write!(f, "GZIP Format"),
            Type::HDF5 => write!(f, "HDF5"),
            Type::HTML => write!(f, "HTML"),
            Type::BIOM => write!(f, "BIOM"),
            Type::CRAM => write!(f, "CRAM"),
            Type::CSV => write!(f, "CSV"),
            Type::DICOM => write!(f, "DICOM"),
            Type::DOCX => write!(f, "DOCX"),
            Type::DSV => write!(f, "DSV"),
            Type::FASTA => write!(f, "FASTA"),
            Type::FASTQ => write!(f, "FASTQ"),
            Type::GCTResFormat => write!(f, "GCT/Res Format"),
            Type::AVI => write!(f, "AVI"),
            Type::BAI => write!(f, "BAI"),
            Type::BAM => write!(f, "BAM"),
            Type::BCRBiotab => write!(f, "BCR Biotab"),
            Type::BED => write!(f, "BED"),
            Type::Bedgraph => write!(f, "bedgraph"),
            Type::BigBed => write!(f, "bigBed"),
            Type::BigWig => write!(f, "bigWig"),
            Type::BinaryFormat => write!(f, "Binary Format"),
            Type::ADF => write!(f, "ADF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::file::r#type::Type;

    #[test]
    fn it_serializes_correctly() {
        assert_eq!(Type::HIC.to_string(), "HIC");
        assert_eq!(Type::BEDPEFormat.to_string(), "BEDPE Format");
        assert_eq!(Type::MTX.to_string(), "mtx");
        assert_eq!(Type::MzIdentML.to_string(), "mzIdentML");
        assert_eq!(Type::MEX.to_string(), "MEX");
        assert_eq!(Type::CRAI.to_string(), "CRAI");
        assert_eq!(Type::CEL.to_string(), "CEL");
        assert_eq!(Type::MzXML.to_string(), "mzXML");
        assert_eq!(Type::IDAT.to_string(), "IDAT");
        assert_eq!(Type::IDF.to_string(), "IDF");
        assert_eq!(Type::JPEG.to_string(), "JPEG");
        assert_eq!(Type::JPEG2000.to_string(), "JPEG 2000");
        assert_eq!(Type::JSON.to_string(), "JSON");
        assert_eq!(Type::MAF.to_string(), "MAF");
        assert_eq!(Type::YAML.to_string(), "YAML");
        assert_eq!(Type::ZIP.to_string(), "ZIP");
        assert_eq!(Type::SVS.to_string(), "SVS");
        assert_eq!(Type::TAR.to_string(), "TAR");
        assert_eq!(Type::ThermoRAW.to_string(), "Thermo RAW");
        assert_eq!(Type::TIFF.to_string(), "TIFF");
        assert_eq!(Type::TSV.to_string(), "TSV");
        assert_eq!(Type::TXT.to_string(), "TXT");
        assert_eq!(Type::VCF.to_string(), "VCF");
        assert_eq!(Type::XLS.to_string(), "XLS");
        assert_eq!(Type::XLSX.to_string(), "XLSX");
        assert_eq!(Type::XML.to_string(), "XML");
        assert_eq!(Type::MATLABScript.to_string(), "MATLAB Script");
        assert_eq!(Type::NIFTIFormat.to_string(), "NIFTI Format");
        assert_eq!(Type::RMarkdown.to_string(), "R Markdown");
        assert_eq!(Type::RFileFormat.to_string(), "R File Format");
        assert_eq!(Type::PythonScriptFormat.to_string(), "Python Script Format");
        assert_eq!(
            Type::SequenceRecordFormat.to_string(),
            "Sequence Record Format"
        );
        assert_eq!(Type::SVG.to_string(), "SVG");
        assert_eq!(Type::OMETIFF.to_string(), "OME-TIFF");
        assert_eq!(Type::PDF.to_string(), "PDF");
        assert_eq!(
            Type::PlainTextDataFormat.to_string(),
            "Plain Text Data Format"
        );
        assert_eq!(Type::PNG.to_string(), "PNG");
        assert_eq!(Type::SDRF.to_string(), "SDRF");
        assert_eq!(Type::MAGETAB.to_string(), "MAGE-TAB");
        assert_eq!(Type::MAT.to_string(), "MAT");
        assert_eq!(Type::MPEG4.to_string(), "MPEG-4");
        assert_eq!(Type::MzML.to_string(), "mzML");
        assert_eq!(Type::GenBankFormat.to_string(), "GenBank Format");
        assert_eq!(Type::GFF3.to_string(), "GFF3");
        assert_eq!(Type::GPR.to_string(), "GPR");
        assert_eq!(Type::GTF.to_string(), "GTF");
        assert_eq!(Type::GZIPFormat.to_string(), "GZIP Format");
        assert_eq!(Type::HDF5.to_string(), "HDF5");
        assert_eq!(Type::HTML.to_string(), "HTML");
        assert_eq!(Type::BIOM.to_string(), "BIOM");
        assert_eq!(Type::CRAM.to_string(), "CRAM");
        assert_eq!(Type::CSV.to_string(), "CSV");
        assert_eq!(Type::DICOM.to_string(), "DICOM");
        assert_eq!(Type::DOCX.to_string(), "DOCX");
        assert_eq!(Type::DSV.to_string(), "DSV");
        assert_eq!(Type::FASTA.to_string(), "FASTA");
        assert_eq!(Type::FASTQ.to_string(), "FASTQ");
        assert_eq!(Type::GCTResFormat.to_string(), "GCT/Res Format");
        assert_eq!(Type::AVI.to_string(), "AVI");
        assert_eq!(Type::BAI.to_string(), "BAI");
        assert_eq!(Type::BAM.to_string(), "BAM");
        assert_eq!(Type::BCRBiotab.to_string(), "BCR Biotab");
        assert_eq!(Type::BED.to_string(), "BED");
        assert_eq!(Type::Bedgraph.to_string(), "bedgraph");
        assert_eq!(Type::BigBed.to_string(), "bigBed");
        assert_eq!(Type::BigWig.to_string(), "bigWig");
        assert_eq!(Type::BinaryFormat.to_string(), "Binary Format");
        assert_eq!(Type::ADF.to_string(), "ADF");
    }
}
