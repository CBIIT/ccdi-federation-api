/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

/**
 * **`caDSR CDE 11524544 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A stable unique
 * alphanumeric identifier assigned to a study and any objects by the database
 * of Genotypes and Phenotypes (dbGaP).".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11524544%20and%20ver_nr=1>
 */
export type CdeV1DepositionDbgapPhsAccession = string;

/**
 * **`caDSR CDE 11280338 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A free text field that can
 * be used to document the content and other details about an electronic file
 * that may not be captured elsewhere.". No permissible values are defined for
 * this CDE.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11280338%20and%20ver_nr=1>
 */
export type CdeV1FileDescription = string;

/**
 * **`caDSR CDE 11284037 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The literal label for an
 * electronic data file.". No permissible values are defined for this CDE.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11284037%20and%20ver_nr=1>
 * @example "File001.txt"
 */
export type CdeV1FileName = string;

/**
 * **`caDSR CDE 11479876 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The measure (in bytes) of
 * how much space a data file takes up on a storage medium.". No permissible
 * values are defined for this CDE.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11479876%20and%20ver_nr=1>
 * @min 0
 */
export type CdeV1FileSize = number;

/**
 * **`caDSR CDE 11416926 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A defined organization or
 * layout representing and structuring data in a computer file.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11416926%20and%20ver_nr=1>
 */
export enum CdeV1FileType {
  HIC = "HIC",
  BEDPEFormat = "BEDPE Format",
  Mtx = "mtx",
  MzIdentML = "mzIdentML",
  MEX = "MEX",
  CRAI = "CRAI",
  CEL = "CEL",
  MzXML = "mzXML",
  IDAT = "IDAT",
  IDF = "IDF",
  JPEG = "JPEG",
  JPEG2000 = "JPEG 2000",
  JSON = "JSON",
  MAF = "MAF",
  YAML = "YAML",
  ZIP = "ZIP",
  SVS = "SVS",
  TAR = "TAR",
  ThermoRAW = "Thermo RAW",
  TIFF = "TIFF",
  TSV = "TSV",
  TXT = "TXT",
  VCF = "VCF",
  XLS = "XLS",
  XLSX = "XLSX",
  XML = "XML",
  MATLABScript = "MATLAB Script",
  NIFTIFormat = "NIFTI Format",
  RMarkdown = "R Markdown",
  RFileFormat = "R File Format",
  PythonScriptFormat = "Python Script Format",
  SequenceRecordFormat = "Sequence Record Format",
  SVG = "SVG",
  OMETIFF = "OME-TIFF",
  PDF = "PDF",
  PlainTextDataFormat = "Plain Text Data Format",
  PNG = "PNG",
  SDRF = "SDRF",
  MAGETAB = "MAGE-TAB",
  MAT = "MAT",
  MPEG4 = "MPEG-4",
  MzML = "mzML",
  GenBankFormat = "GenBank Format",
  GFF3 = "GFF3",
  GPR = "GPR",
  GTF = "GTF",
  GZIPFormat = "GZIP Format",
  HDF5 = "HDF5",
  HTML = "HTML",
  BIOM = "BIOM",
  CRAM = "CRAM",
  CSV = "CSV",
  DICOM = "DICOM",
  DOCX = "DOCX",
  DSV = "DSV",
  FASTA = "FASTA",
  FASTQ = "FASTQ",
  GCTResFormat = "GCT/Res Format",
  AVI = "AVI",
  BAI = "BAI",
  BAM = "BAM",
  BCRBiotab = "BCR Biotab",
  BED = "BED",
  Bedgraph = "bedgraph",
  BigBed = "bigBed",
  BigWig = "bigWig",
  BinaryFormat = "Binary Format",
  ADF = "ADF",
}

/**
 * **`caDSR CDE 11556150 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A 32-character hexadecimal
 * number that is computed on a file.". No permissible values are defined for
 * this CDE.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11556150%20and%20ver_nr=1>
 */
export type CdeV1FileChecksumMD5 = string;

/**
 * **`caDSR CDE 14528051 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A sequence of characters
 * used to uniquely identify, name, or characterize the study funding
 * organization.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14528051%20and%20ver_nr=1>
 */
export type CdeV1NamespaceStudyFundingId = string;

/**
 * **`caDSR CDE 12960571 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A sequence of characters
 * used to identify, name, or characterize a pediatric study.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12960571%20and%20ver_nr=1>
 */
export enum CdeV1NamespaceStudyId {
  AALL0232 = "AALL0232",
  AALL0331 = "AALL0331",
  AALL03B1 = "AALL03B1",
  AALL0434 = "AALL0434",
  AALL08B1 = "AALL08B1",
  AAML03P1 = "AAML03P1",
  AAML0531 = "AAML0531",
  AAML1031 = "AAML1031",
  AEIOPAML2002 = "AEIOPAML2002",
  AEWS0031 = "AEWS0031",
  AEWS0331 = "AEWS0331",
  AEWS07P1 = "AEWS07P1",
  AEWS1031 = "AEWS1031",
  AEWS1221 = "AEWS1221",
  AGCT0132 = "AGCT0132",
  AGCT01P1 = "AGCT01P1",
  AGCT0521 = "AGCT0521",
  AHOD0031 = "AHOD0031",
  AHOD03P1 = "AHOD03P1",
  AHOD0431 = "AHOD0431",
  AHOD0831 = "AHOD0831",
  AHOD1221 = "AHOD1221",
  AHOD1331 = "AHOD1331",
  AIEOPLAM92 = "AIEOPLAM92",
  AMLBFMRegistry2012 = "AMLBFM-Registry2012",
  AMLBFM1998 = "AMLBFM1998",
  AMLBFM2004 = "AMLBFM2004",
  AMLBFM2012 = "AMLBFM2012",
  AMLBFMRegistry2017 = "AMLBFMRegistry2017",
  AOST0121 = "AOST0121",
  AOST01P1 = "AOST01P1",
  AOST0221 = "AOST0221",
  AOST0331EURAMOS1 = "AOST0331/EURAMOS1",
  AOST1321 = "AOST1321",
  AOST1421 = "AOST1421",
  CCG782 = "CCG-782",
  CCG7942 = "CCG-7942",
  DBAML01 = "DBAML01",
  EE99 = "EE99",
  EICESS92 = "EICESS92",
  GC1 = "GC1",
  GC2 = "GC2",
  GOG0078 = "GOG0078",
  GOG0090 = "GOG0090",
  GOG0116 = "GOG0116",
  INT133 = "INT133",
  JACLSAML99 = "JACLSAML99",
  JPLSGAML05 = "JPLSGAML05",
  MRCAML12 = "MRCAML12",
  MRCAML15 = "MRCAML15",
  NOPHOAML2004 = "NOPHOAML2004",
  NOPHOAML2012 = "NOPHOAML2012",
  OS2006 = "OS2006",
  P9749 = "P9749",
  P9754 = "P9754",
  POG9049 = "POG9049",
  PPLLSGAML98 = "PPLLSGAML98",
  REGOBONE = "REGOBONE",
  Sarcome13OS2016 = "Sarcome13/OS2016",
  SCFEELAM02 = "SCFEELAM02",
  SJCRHAML02 = "SJCRHAML02",
  TCGM2004 = "TCGM2004",
  TE04 = "TE04",
  TE05 = "TE05",
  TE08 = "TE08",
  TE09 = "TE09",
  TE13 = "TE13",
  TE20 = "TE20",
  TE22 = "TE22",
  TGM85 = "TGM85",
  TGM90 = "TGM90",
  TGM95 = "TGM95",
  TIP = "TIP",
}

/**
 * **`caDSR CDE 11459810 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The acronym or abbreviated
 * form of the title for a research data collection. Example – GLIOMA01".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11459810%20and%20ver_nr=1>
 */
export type CdeV1NamespaceStudyName = string;

/**
 * **`caDSR CDE 11459812 v2.00`**
 *
 * This metadata element is defined by the caDSR as "The acronym or
 * abbreviated form of the title for a research data collection".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11459812%20and%20ver_nr=2>
 */
export type CdeV2NamespaceStudyShortTitle = string;

/**
 * **`caDSR CDE 12662779 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A sequence of characters
 * used to identify, name, or characterize the laboratory, institute, or
 * consortium that provided the information.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12662779%20and%20ver_nr=1>
 */
export enum CdeV1OrganizationInstitution {
  AIEOP = "AIEOP",
  BFMSG = "BFM-SG",
  BOCG = "BOCG",
  C3P = "C3P",
  CBTN = "CBTN",
  CCLG = "CCLG",
  CHLA = "CHLA",
  COG = "COG",
  COSSGPOH = "COSS-GPOH",
  CRCTU = "CRCTU",
  CWS = "CWS",
  DCOG = "DCOG",
  DePICT = "DePICT",
  DFCI = "DFCI",
  EORTC = "EORTC",
  EpSSG = "EpSSG",
  EuRBG = "EuRBG",
  EUROEWING = "EURO-EWING",
  FSG = "FSG",
  GALOP = "GALOP",
  GEIS = "GEIS",
  GPOH = "GPOH",
  IDIPGR = "IDIPGR",
  ISG = "ISG",
  JACLS = "JACLS",
  JCCG = "JCCG",
  JINCS = "JINCS",
  JNBSG = "JNBSG",
  JPLSG = "JPLSG",
  MRC = "MRC",
  NOPHO = "NOPHO",
  NRGOncology = "NRG-Oncology",
  PNOC = "PNOC",
  PPLLSG = "PPLLSG",
  RBTR = "RBTR",
  SFCE = "SFCE",
  SIOPMMT = "SIOP MMT",
  SIOPE = "SIOPE",
  SIOPEN = "SIOPEN",
  SJCRH = "SJCRH",
  SOPOBE = "SOPOBE",
  SSG = "SSG",
  Treehouse = "Treehouse",
  UCL = "UCL",
  UK = "UK",
}

/**
 * **`caDSR CDE 12217251 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The stage or period of an
 * individual's treatment process during which relevant observations were
 * recorded.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12217251%20and%20ver_nr=1>
 */
export enum CdeV1SampleDiseasePhase {
  PostMortem = "Post-Mortem",
  NotReported = "Not Reported",
  Unknown = "Unknown",
  InitialDiagnosis = "Initial Diagnosis",
  Progression = "Progression",
  Refractory = "Refractory",
  Relapse = "Relapse",
  RelapseProgression = "Relapse/Progression",
}

/**
 * **`caDSR CDE 6273393 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The overall strategy for
 * the collection of double stranded DNA fragments flanked by oligonucleotide
 * sequence adapters to enable their analysis by high-throughput sequencing.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6273393%20and%20ver_nr=1>
 */
export enum CdeV1SampleLibraryStrategy {
  AMPLICON = "AMPLICON",
  ATACSeq = "ATAC-Seq",
  BisulfiteSeq = "Bisulfite-Seq",
  ChIAPET = "ChIA-PET",
  ChIPSeq = "ChIP-Seq",
  CLONE = "CLONE",
  CLONEEND = "CLONEEND",
  CTS = "CTS",
  DNASeq = "DNA-Seq",
  DNaseHypersensitivity = "DNase-Hypersensitivity",
  EST = "EST",
  FAIRESeq = "FAIRE-seq",
  FINISHING = "FINISHING",
  FLCDNA = "FL-cDNA",
  HiC = "Hi-C",
  MBDSeq = "MBD-Seq",
  MeDIPSeq = "MeDIP-Seq",
  MiRNASeq = "miRNA-Seq",
  MNaseSeq = "MNase-Seq",
  MRESeq = "MRE-Seq",
  NcRNASeq = "ncRNA-Seq",
  Other = "Other",
  POOLCLONE = "POOLCLONE",
  RADSeq = "RAD-Seq",
  RIPSeq = "RIP-Seq",
  RNASeq = "RNA-Seq",
  SELEX = "SELEX",
  SnATACSeq = "snATAC-Seq",
  SsRNASeq = "ssRNA-seq",
  SyntheticLongRead = "Synthetic-Long-Read",
  TargetedCapture = "Targeted-Capture",
  TetheredChromatinConformationCapture = "Tethered Chromatin Conformation Capture",
  TnSeq = "Tn-Seq",
  WCS = "WCS",
  WGA = "WGA",
  WGS = "WGS",
  WXS = "WXS",
}

/**
 * **`caDSR CDE 14688604 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The category assigned
 * to the cytologic atypia found in cellular molecules, cells, tissues, organs,
 * body fluids, or body excretory products."
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14688604%20and%20ver_nr=1>
 */
export enum CdeV1SampleTissueType {
  NotReported = "Not Reported",
  Normal = "Normal",
  Peritumoral = "Peritumoral",
  Tumor = "Tumor",
  Unknown = "Unknown",
}

/**
 * **`caDSR CDE 12922545 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The classification of a
 * tumor based primarily on histopathological characteristics.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12922545%20and%20ver_nr=1>
 */
export enum CdeV1SampleTumorClassification {
  Metastatic = "Metastatic",
  NotReported = "Not Reported",
  Primary = "Primary",
  Regional = "Regional",
  Unknown = "Unknown",
}

/**
 * **`caDSR CDE 11326261 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The microscopic anatomy of
 * normal and abnormal cells and tissues of the specimen as captured in the
 * morphology codes of the International Classification of Diseases for
 * Oncology, 3rd Edition (ICD-O-3)."
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11326261%20and%20ver_nr=1>
 */
export interface CdeV1SampleTumorTissueMorphology {
  /** The ICD-O-3 code. */
  icd_o_3: string;
}

/**
 * **`caDSR CDE 6380049 v1.00`**
 *
 * This metadata element is defined by the caDSR as "A unique subject
 * identifier within a site and a study.". No permissible values are defined
 * for this CDE.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6380049%20and%20ver_nr=1>
 * @example "SubjectName001"
 */
export type CdeV1SubjectName = string;

/**
 * **`caDSR CDE 2192199 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The text for reporting
 * information about race based on the Office of Management and Budget (OMB)
 * categories.". Upon examination of the large number of projects using the
 * term, it appears to be the preferred term for the general concept of race.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192199%20and%20ver_nr=1>
 */
export enum CdeV1SubjectRace {
  NotAllowedToCollect = "Not allowed to collect",
  NativeHawaiianOrOtherPacificIslander = "Native Hawaiian or other Pacific Islander",
  NotReported = "Not Reported",
  Unknown = "Unknown",
  AmericanIndianOrAlaskaNative = "American Indian or Alaska Native",
  Asian = "Asian",
  BlackOrAfricanAmerican = "Black or African American",
  White = "White",
}

/**
 * **`caDSR CDE 6343385 v1.00`**
 *
 * This metadata element is defined by the caDSR as "Sex of the subject as
 * determined by the investigator." In particular, this field does not dictate
 * the time period: whether it represents sex at birth, sex at sample
 * collection, or any other determined time point. Further, the descriptions
 * for F and M suggest that this term can represent either biological sex,
 * cultural gender roles, or both. Thus, this field cannot be assumed to
 * strictly represent biological sex.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6343385%20and%20ver_nr=1>
 */
export enum CdeV1SubjectSex {
  U = "U",
  F = "F",
  M = "M",
  UNDIFFERENTIATED = "UNDIFFERENTIATED",
}

/**
 * **`caDSR CDE 2847330 v1.00`**
 *
 * This metadata element is defined by the caDSR as "The response to a question
 * that describes a participant's survival status."
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2847330%20and%20ver_nr=1>
 */
export enum CdeV1SubjectVitalStatus {
  NotReported = "Not reported",
  Alive = "Alive",
  Dead = "Dead",
  Unknown = "Unknown",
  Unspecified = "Unspecified",
}

/**
 * **`caDSR CDE 8028962 v2.00`**
 *
 * This metadata element is defined by the caDSR as "Text term that represents
 * the method used to maintain the sample or biospecimen in a viable state.".
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=8028962%20and%20ver_nr=2>
 */
export enum CdeV2SamplePreservationMethod {
  Value80DegreesC = "-80 degrees C",
  Cryopreserved = "Cryopreserved",
  EDTA = "EDTA",
  FFPE = "FFPE",
  FormalinFixedBuffered = "Formalin Fixed - Buffered",
  FormalinFixedUnbuffered = "Formalin Fixed - Unbuffered",
  Fresh = "Fresh",
  FreshDissociated = "Fresh Dissociated",
  FreshDissociatedAndSingleCellSorted = "Fresh Dissociated and Single Cell Sorted",
  FreshDissociatedAndSingleCellSortedIntoPlates = "Fresh Dissociated and Single Cell Sorted into Plates",
  Frozen = "Frozen",
  LiquidNitrogen = "Liquid Nitrogen",
  NotReported = "Not Reported",
  OCT = "OCT",
  SnapFrozen = "Snap Frozen",
  Unknown = "Unknown",
}

/**
 * **`caDSR CDE 2192217 v2.00`**
 *
 * This metadata element is defined by the caDSR as "The text for reporting
 * information about ethnicity based on the Office of Management and Budget
 * (OMB) categories." Upon examination of the large number of projects using
 * the term, it appears to be the preferred term for the general concept of
 * ethnicity.
 *
 * Link:
 * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192217%20and%20ver_nr=2>
 */
export enum CdeV2SubjectEthnicity {
  NotAllowedToCollect = "Not allowed to collect",
  HispanicOrLatino = "Hispanic or Latino",
  NotHispanicOrLatino = "Not Hispanic or Latino",
  Unknown = "Unknown",
  NotReported = "Not reported",
}

/** A metadata field. */
export type FieldUnharmonizedField = FieldOwnedField | FieldUnownedField;

export interface FieldOwnedField {
  /** The value of the metadata field. */
  value: any;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
  /** Whether or not the field is owned by the source server. */
  owned?: boolean;
}

export interface FieldUnownedField {
  /** The value of the metadata field. */
  value: any;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedFileChecksums {
  /** A list of checksums for a file. */
  value: ModelsFileMetadataChecksums;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedFileDescription {
  /**
   * **`caDSR CDE 11280338 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A free text field that can
   * be used to document the content and other details about an electronic file
   * that may not be captured elsewhere.". No permissible values are defined for
   * this CDE.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11280338%20and%20ver_nr=1>
   */
  value: CdeV1FileDescription;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedFileSize {
  /**
   * **`caDSR CDE 11479876 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The measure (in bytes) of
   * how much space a data file takes up on a storage medium.". No permissible
   * values are defined for this CDE.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11479876%20and%20ver_nr=1>
   */
  value: CdeV1FileSize;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedFileType {
  /**
   * **`caDSR CDE 11416926 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A defined organization or
   * layout representing and structuring data in a computer file.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11416926%20and%20ver_nr=1>
   */
  value: CdeV1FileType;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedNamespaceStudyFundingId {
  /**
   * **`caDSR CDE 14528051 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A sequence of characters
   * used to uniquely identify, name, or characterize the study funding
   * organization.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14528051%20and%20ver_nr=1>
   */
  value: CdeV1NamespaceStudyFundingId;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedNamespaceStudyId {
  /**
   * **`caDSR CDE 12960571 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A sequence of characters
   * used to identify, name, or characterize a pediatric study.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12960571%20and%20ver_nr=1>
   */
  value: CdeV1NamespaceStudyId;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedNamespaceStudyName {
  /**
   * **`caDSR CDE 11459810 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The acronym or abbreviated
   * form of the title for a research data collection. Example – GLIOMA01".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11459810%20and%20ver_nr=1>
   */
  value: CdeV1NamespaceStudyName;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedNamespaceStudyShortTitle {
  /**
   * **`caDSR CDE 11459812 v2.00`**
   *
   * This metadata element is defined by the caDSR as "The acronym or
   * abbreviated form of the title for a research data collection".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11459812%20and%20ver_nr=2>
   */
  value: CdeV2NamespaceStudyShortTitle;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedOrganizationInstitution {
  /**
   * **`caDSR CDE 12662779 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A sequence of characters
   * used to identify, name, or characterize the laboratory, institute, or
   * consortium that provided the information.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12662779%20and%20ver_nr=1>
   */
  value: CdeV1OrganizationInstitution;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleAgeAtCollection {
  /**
   * The approximate age of collection in days.
   *
   * * When the age at collection is collected by the source server in days, the
   * number of days is reported directly.
   * * When the age at collection is collected by the source server in years, the
   * number of years is multiplied by 365.25 to arrive at an approximate number
   * of days.
   */
  value: ModelsSampleMetadataAgeAtCollection;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleAgeAtDiagnosis {
  /**
   * The approximate age of diagnosis in days.
   *
   * * When the age at diagnosis is collected by the source server in days, the
   * number of days is reported directly.
   * * When the age at diagnosis is collected by the source server in years, the
   * number of years is multiplied by 365.25 to arrive at an approximate number
   * of days.
   */
  value: ModelsSampleMetadataAgeAtDiagnosis;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleDiagnosis {
  /**
   * The diagnosis for a [`Sample`](crate::Sample).
   *
   * This value can be any permissible diagnosis in v1.7.2 of the CCDI Submission
   * Template. These values are from the value set **diagnosis_classification**
   * found in the 'Terms and Value Sets' tab from the [CCDI Submission Template
   * v1.7.2].
   *
   * To facilitate quick access to these values, we have provided a slimmed down
   * spreadsheet containing the valid diagnoses:
   *
   * 1. Download the spreadsheet linked below titled
   * 'CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx'.
   * 2. The permissible values are found in column A of the 'diagnosis' tab,
   * titled **diagnosis_category_term**
   *
   * [CCDI Submission Template v1.7.2]: https://github.com/CBIIT/ccdi-model/blob/682a99d93b66540bb880ce5899ba8096968a96cf/metadata-manifest/CCDI_Submission_Template_v1.7.2.xlsx
   * [CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx]: https://cbiit.github.io/ccdi-federation-api/assets/CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx
   */
  value: ModelsSampleMetadataDiagnosis;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleDiseasePhase {
  /**
   * **`caDSR CDE 12217251 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The stage or period of an
   * individual's treatment process during which relevant observations were
   * recorded.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12217251%20and%20ver_nr=1>
   */
  value: CdeV1SampleDiseasePhase;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleIdentifier {
  /**
   * A referenced identifier for a [`Sample`](crate::Sample).
   *
   * A referenced identifier is a reference to either an identifier whose owner is known
   * and operates an authoritative federation server containing that identifier (i.e., a
   * [linked identifier](linked::Identifier)) _or_ a reference to an identifier that is
   * generally known to be associated with the sample but does not have an associated
   * server that asserts ownership of the identifier (i.e., an [unlinked
   * identifier](unlinked::Identifier)).
   */
  value: ModelsSampleIdentifierReferencedIdentifier;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleLibraryStrategy {
  /**
   * **`caDSR CDE 6273393 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The overall strategy for
   * the collection of double stranded DNA fragments flanked by oligonucleotide
   * sequence adapters to enable their analysis by high-throughput sequencing.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6273393%20and%20ver_nr=1>
   */
  value: CdeV1SampleLibraryStrategy;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSamplePreservationMethod {
  /**
   * **`caDSR CDE 8028962 v2.00`**
   *
   * This metadata element is defined by the caDSR as "Text term that represents
   * the method used to maintain the sample or biospecimen in a viable state.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=8028962%20and%20ver_nr=2>
   */
  value: CdeV2SamplePreservationMethod;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleTissueType {
  /**
   * **`caDSR CDE 14688604 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The category assigned to the
   * cytologic atypia found in cellular molecules, cells, tissues, organs,
   * body fluids, or body excretory products."
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14688604%20and%20ver_nr=1>
   */
  value: CdeV1SampleTissueType;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleTumorClassification {
  /**
   * **`caDSR CDE 12922545 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The classification of a
   * tumor based primarily on histopathological characteristics.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12922545%20and%20ver_nr=1>
   */
  value: CdeV1SampleTumorClassification;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSampleTumorTissueMorphology {
  /**
   * **`caDSR CDE 11326261 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The microscopic anatomy of
   * normal and abnormal cells and tissues of the specimen as captured in the
   * morphology codes of the International Classification of Diseases for
   * Oncology, 3rd Edition (ICD-O-3)."
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11326261%20and%20ver_nr=1>
   */
  value: CdeV1SampleTumorTissueMorphology;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSubjectAgeAtVitalStatus {
  /**
   * The approximate age at vital status in days.
   *
   * * When the age at vital status is collected by the source server in days,
   * the number of days is reported directly.
   * * When the age at vital status is collected by the source server in years,
   * the number of years is multiplied by 365.25 to arrive at an approximate
   * number of days.
   */
  value: ModelsSubjectMetadataAgeAtVitalStatus;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSubjectEthnicity {
  /**
   * **`caDSR CDE 2192217 v2.00`**
   *
   * This metadata element is defined by the caDSR as "The text for reporting
   * information about ethnicity based on the Office of Management and Budget
   * (OMB) categories." Upon examination of the large number of projects using
   * the term, it appears to be the preferred term for the general concept of
   * ethnicity.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192217%20and%20ver_nr=2>
   */
  value: CdeV2SubjectEthnicity;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSubjectIdentifier {
  /**
   * A referenced identifier for a [`Subject`](crate::Subject).
   *
   * A referenced identifier is a reference to either an identifier whose owner is known
   * and operates an authoritative federation server containing that identifier (i.e., a
   * [linked identifier](linked::Identifier)) _or_ a reference to an identifier that is
   * generally known to be associated with the subject but does not have an associated
   * server that asserts ownership of the identifier (i.e., an [unlinked
   * identifier](unlinked::Identifier)).
   */
  value: ModelsSubjectIdentifierReferencedIdentifier;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSubjectRace {
  /**
   * **`caDSR CDE 2192199 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The text for reporting
   * information about race based on the Office of Management and Budget (OMB)
   * categories.". Upon examination of the large number of projects using the
   * term, it appears to be the preferred term for the general concept of race.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192199%20and%20ver_nr=1>
   */
  value: CdeV1SubjectRace;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSubjectSex {
  /**
   * **`caDSR CDE 6343385 v1.00`**
   *
   * This metadata element is defined by the caDSR as "Sex of the subject as
   * determined by the investigator." In particular, this field does not dictate
   * the time period: whether it represents sex at birth, sex at sample
   * collection, or any other determined time point. Further, the descriptions
   * for F and M suggest that this term can represent either biological sex,
   * cultural gender roles, or both. Thus, this field cannot be assumed to
   * strictly represent biological sex.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6343385%20and%20ver_nr=1>
   */
  value: CdeV1SubjectSex;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

export interface FieldUnownedSubjectVitalStatus {
  /**
   * **`caDSR CDE 2847330 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The response to a question
   * that describes a participant's survival status."
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2847330%20and%20ver_nr=1>
   */
  value: CdeV1SubjectVitalStatus;
  /**
   * The ancestors from which this field was derived.
   *
   * Ancestors should be provided as period (`.`) delimited paths
   * from the `metadata` key in the subject response object.
   */
  ancestors?: string[];
  details?: ModelsMetadataFieldDetails;
  /** A free-text comment field. */
  comment?: string;
}

/**
 * A map of unharmonized metadata fields.
 *
 * Unharmonized keys may be any valid JSON string.
 */
export type FieldsUnharmonized = Record<string, FieldUnharmonizedField> &
  object;

/**
 * A file.
 *
 * **Note:** the `samples` key **must** include only identifiers for
 * [`Samples`](super::Sample) that both (a) are listed in the
 * [`Sample`](super::Sample) index endpoint (`/sample`) and (b) are able to be
 * shown with the [`Sample`](super::Sample) show endpoint
 * (`/sample/{namespace}/{name}`).
 */
export interface ModelsFile {
  /** The primary name and namespace for a file within the source server. */
  id: ModelsFileIdentifier;
  /**
   * One or more samples that are associated with this [`File`] referred to
   * by their identifier(s).
   *
   * Each file must be associated with at least one
   * [`Sample`](super::Sample)—files that are not associated with any sample
   * are disallowed. Files may be associated with as many samples as is
   * necessary.
   *
   * **Note:** each identifier **must** match a [`Sample`](super::Sample)
   * that both (a) is listed in the [`Sample`](super::Sample) index endpoint
   * (`/sample`) and (b) is able to be shown with the
   * [`Sample`](super::Sample) show endpoint (`/sample/{namespace}/{name}`).
   */
  samples: ModelsSampleIdentifier[];
  /**
   * One or more [gateways](AnonymousOrReference) through which this file can
   * be accessed.
   *
   * Gateways can be either [anonymous](AnonymousOrReference::Anonymous)
   * ([gateways](crate::Gateway) with no name) or a
   * [refererence](AnonymousOrReference::Reference) to a [named
   * gateway](gateway::Named) ([gateways](crate::Gateway) with a name).
   *
   * **Anonymous** gateways are intended to be embedded directly within a
   * returned file in the `/file` response object. They have no name and are
   * only referred to by the file within which they are embedded.
   *
   * **Named** gateways, on the other hand, are included in the `gateways`
   * key in the `/file` response object and referred to by name within a
   * returned file in the `/file` response object. They are intended to be
   * used when more than one file references the same gateway. This mechanism
   * is available to ensure that the gateway object does not need to be
   * duplicated multiple times in the response in these cases.
   *
   * This field can contain multiple gateways to support scenarios where a
   * file is available through more than one mechanism. We expect that only
   * one gateway will be returned in most responses.
   *
   * **Note:** a file must have at least one gateway. If the file has no
   * gateway, then it should not be returned as part of this API.
   */
  gateways?: ModelsGatewayAnonymousOrReference[];
  metadata?: ModelsFileMetadata | null;
}

/** Gateways, which notify of resources that are external to the API. */
export type ModelsGateway =
  | {
      /**
       * A link to an external resource.
       *
       * A link communicates information about where a resource is located, alongside
       * additional context regarding how the link should be interpreted (via the
       * `kind` field). All [`Link`]s include a `url` field pointing to the external
       * resource. In the case of [`Link::Approximate`] and [`Link::MailTo`], a
       * required `instructions` field is included to instruct the user of what steps
       * to take after the link has been followed (see the definition of
       * [`Link::Approximate`] and [`Link::MailTo`] for more details, respectively).
       *
       * **Note:** the context of what resources are desired compared with what
       * resources the link represents is an important consideration when
       * constructing the correct [`Link`]. For example, if the desired resource is a
       * specific file, but the server can only construct a link to a cohort of
       * files, a [`Link::Approximate`] should be used. In contrast, if the desired
       * resource is the entire cohort of files, a [`Link::Direct`] should be used.
       *
       * **Note:** the link does not imply the access level or immediate availability
       * of the data—it only points a user to where they can _attempt_ to access the
       * data. Instead, [`Link`]s are always wrapped in a [`Gateway`](super::Gateway)
       * that communicates the access level or requirements. In other words, a
       * [`Link`] can absolutely require authentication or authorization before data
       * becomes accessible (and the corresponding [`Gateway`](super::Gateway) within
       * which the [`Link`] is embedded should reflect this).
       *
       * ## Examples
       *
       * * If the data is contained within a file where a direct link can be
       * constructed, whether that file is open access or controlled, then a
       * [`Link::Direct`] should be constructed with a link directly to that file.
       * * In the event that study data is deposited as a study within a larger data
       * repository, such as the database of Genotypes and Phenotypes (dbGaP) or
       * the European Genome-phenome Archive (EGA), and the `url` points to the
       * study page:
       * * If the desired resource is a specific file or subset of (but not all)
       * files within the study, a [`Link::Approximate`] should be returned.
       * This is because the link includes more files than what was
       * specifically requested—thus, instructions on how to filter to the
       * files requested must be communicated.
       * * If the desired resource is _every_ file in the study, then a
       * [`Link::Direct`] should be returned pointing to the study page.
       * * If the data is not immediately requestable through a webpage but there
       * exists an informational page on how to request the data using an
       * out-of-band process, then a [`Link::Informational`] should be used.
       * * If the data is available after contacting an email address, then a
       * [`Link::MailTo`] should be used.
       */
      link: ModelsGatewayLink;
      kind: "Open";
    }
  | {
      /**
       * A link to an external resource.
       *
       * A link communicates information about where a resource is located, alongside
       * additional context regarding how the link should be interpreted (via the
       * `kind` field). All [`Link`]s include a `url` field pointing to the external
       * resource. In the case of [`Link::Approximate`] and [`Link::MailTo`], a
       * required `instructions` field is included to instruct the user of what steps
       * to take after the link has been followed (see the definition of
       * [`Link::Approximate`] and [`Link::MailTo`] for more details, respectively).
       *
       * **Note:** the context of what resources are desired compared with what
       * resources the link represents is an important consideration when
       * constructing the correct [`Link`]. For example, if the desired resource is a
       * specific file, but the server can only construct a link to a cohort of
       * files, a [`Link::Approximate`] should be used. In contrast, if the desired
       * resource is the entire cohort of files, a [`Link::Direct`] should be used.
       *
       * **Note:** the link does not imply the access level or immediate availability
       * of the data—it only points a user to where they can _attempt_ to access the
       * data. Instead, [`Link`]s are always wrapped in a [`Gateway`](super::Gateway)
       * that communicates the access level or requirements. In other words, a
       * [`Link`] can absolutely require authentication or authorization before data
       * becomes accessible (and the corresponding [`Gateway`](super::Gateway) within
       * which the [`Link`] is embedded should reflect this).
       *
       * ## Examples
       *
       * * If the data is contained within a file where a direct link can be
       * constructed, whether that file is open access or controlled, then a
       * [`Link::Direct`] should be constructed with a link directly to that file.
       * * In the event that study data is deposited as a study within a larger data
       * repository, such as the database of Genotypes and Phenotypes (dbGaP) or
       * the European Genome-phenome Archive (EGA), and the `url` points to the
       * study page:
       * * If the desired resource is a specific file or subset of (but not all)
       * files within the study, a [`Link::Approximate`] should be returned.
       * This is because the link includes more files than what was
       * specifically requested—thus, instructions on how to filter to the
       * files requested must be communicated.
       * * If the desired resource is _every_ file in the study, then a
       * [`Link::Direct`] should be returned pointing to the study page.
       * * If the data is not immediately requestable through a webpage but there
       * exists an informational page on how to request the data using an
       * out-of-band process, then a [`Link::Informational`] should be used.
       * * If the data is available after contacting an email address, then a
       * [`Link::MailTo`] should be used.
       */
      link: ModelsGatewayLink;
      kind: "Registered";
    }
  | {
      /**
       * A link to an external resource.
       *
       * A link communicates information about where a resource is located, alongside
       * additional context regarding how the link should be interpreted (via the
       * `kind` field). All [`Link`]s include a `url` field pointing to the external
       * resource. In the case of [`Link::Approximate`] and [`Link::MailTo`], a
       * required `instructions` field is included to instruct the user of what steps
       * to take after the link has been followed (see the definition of
       * [`Link::Approximate`] and [`Link::MailTo`] for more details, respectively).
       *
       * **Note:** the context of what resources are desired compared with what
       * resources the link represents is an important consideration when
       * constructing the correct [`Link`]. For example, if the desired resource is a
       * specific file, but the server can only construct a link to a cohort of
       * files, a [`Link::Approximate`] should be used. In contrast, if the desired
       * resource is the entire cohort of files, a [`Link::Direct`] should be used.
       *
       * **Note:** the link does not imply the access level or immediate availability
       * of the data—it only points a user to where they can _attempt_ to access the
       * data. Instead, [`Link`]s are always wrapped in a [`Gateway`](super::Gateway)
       * that communicates the access level or requirements. In other words, a
       * [`Link`] can absolutely require authentication or authorization before data
       * becomes accessible (and the corresponding [`Gateway`](super::Gateway) within
       * which the [`Link`] is embedded should reflect this).
       *
       * ## Examples
       *
       * * If the data is contained within a file where a direct link can be
       * constructed, whether that file is open access or controlled, then a
       * [`Link::Direct`] should be constructed with a link directly to that file.
       * * In the event that study data is deposited as a study within a larger data
       * repository, such as the database of Genotypes and Phenotypes (dbGaP) or
       * the European Genome-phenome Archive (EGA), and the `url` points to the
       * study page:
       * * If the desired resource is a specific file or subset of (but not all)
       * files within the study, a [`Link::Approximate`] should be returned.
       * This is because the link includes more files than what was
       * specifically requested—thus, instructions on how to filter to the
       * files requested must be communicated.
       * * If the desired resource is _every_ file in the study, then a
       * [`Link::Direct`] should be returned pointing to the study page.
       * * If the data is not immediately requestable through a webpage but there
       * exists an informational page on how to request the data using an
       * out-of-band process, then a [`Link::Informational`] should be used.
       * * If the data is available after contacting an email address, then a
       * [`Link::MailTo`] should be used.
       */
      link: ModelsGatewayLink;
      kind: "Controlled";
    }
  | (ModelsGatewayClosed & {
      kind: "Closed";
    });

/** A namespace. */
export interface ModelsNamespace {
  /** An identifier for a namespace. */
  id: ModelsNamespaceIdentifier;
  /**
   * A description of a namespace.
   *
   * This description cannot exceed 2048 characters.
   */
  description: ModelsNamespaceDescription;
  /**
   * A support email address for entities contained within the namespace.
   *
   * This field is required to be a valid email address (both in format and
   * in terms of the email address being actively monitored).
   * @example "support@example.com"
   */
  contact_email: string;
  metadata?: ModelsNamespaceMetadata | null;
}

/**
 * An organization.
 *
 * Organizations own [`Namespaces`](super::Namespace) within a source server.
 */
export interface ModelsOrganization {
  /**
   * The identifier of an organization.
   *
   * The identifier **must** conform to
   * [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
   * matching the pattern `^[a-z0-9-]+$`. Any identifier that does not match this
   * pattern should be considered invalid by clients.
   *
   * **Note:** this field is asserted by the source server, but it is not
   * guaranteed to be authoritative across the federation (due to the
   * decentralized nature of organization and namespace allocation).
   *
   * **Note**: the regex for this field does not allow for any spaces because it is
   * anticipated that the field will be displayable as a repository (e.g.,
   * `example-organization/ExampleNamespace`).
   */
  identifier: ModelsOrganizationIdentifier;
  /**
   * The proper name of the organization as it should be displayed by clients.
   *
   * This name name cannot exceed 256 characters.
   *
   * This field is intended to be the proper name of the organization that mints
   * identifiers within a given namespace. That said, we have intentionally not
   * required that this be an organization specifically, as there may be exceptions
   * to this guideline. We recommend that you use an organization name here if you
   * can, but you may put whatever value is appropriate to describe the owner of the
   * namespace.
   *
   * It is recommended that you use title case for this field, though that is not
   * required.
   *
   * **Note:** this field is asserted by the source server, but it is not guaranteed
   * to be authoritative across the federation (due to the decentralized nature of
   * organization and namespace allocation).
   */
  name: ModelsOrganizationName;
  metadata?: ModelsOrganizationMetadata | null;
}

/**
 * A sample.
 *
 * **Note:** the `subject` identifier **must** match a
 * [`Subject`](super::Subject) that both (a) is listed in the
 * [`Subject`](super::Subject) index endpoint and (b) is able to be shown with
 * the [`Subject`](super::Subject) show endpoint.
 */
export interface ModelsSample {
  /**
   * An identifier for a [`Sample`](crate::Sample).
   *
   * [`Identifiers`](Identifier) serve two main purposes:
   *
   * 1. They represent the primary identifier for a [`Sample`](crate::Sample).
   * 2. They extended when referenced as [linked identifiers](linked::Identifier).
   */
  id: ModelsSampleIdentifier;
  /**
   * An identifier for a [`Subject`](crate::Subject).
   *
   * [`Identifiers`](Identifier) serve two main purposes:
   *
   * 1. They represent the primary identifier for a [`Subject`](crate::Subject).
   * 2. They extended when referenced as [linked identifiers](linked::Identifier).
   */
  subject: ModelsSubjectIdentifier;
  /**
   * One or more [gateways](AnonymousOrReference) through which this sample
   * can be accessed.
   *
   * Gateways can be either [anonymous](AnonymousOrReference::Anonymous)
   * ([gateways](crate::Gateway) with no name) or a
   * [refererence](AnonymousOrReference::Reference) to a [named
   * gateway](gateway::Named) ([gateways](crate::Gateway) with a name).
   *
   * **Anonymous** gateways are intended to be embedded directly within a
   * returned sample in the `/sample` response object. They have no name and
   * are only referred to by the sample within which they are embedded.
   *
   * **Named** gateways, on the other hand, are included in the `gateways`
   * key in the `/sample` response object and referred to by name within a
   * returned sample in the `/sample` response object. They are intended to
   * be used when more than one sample references the same gateway. This
   * mechanism is available to ensure that the gateway object does not need
   * to be duplicated multiple times in the response in these cases.
   *
   * This field can contain multiple gateways to support scenarios where a
   * sample is available through more than one mechanism. We expect that only
   * one gateway will be returned in most responses (if at all).
   */
  gateways?: ModelsGatewayAnonymousOrReference[];
  metadata?: ModelsSampleMetadata | null;
}

/** A subject. */
export interface ModelsSubject {
  /**
   * An identifier for a [`Subject`](crate::Subject).
   *
   * [`Identifiers`](Identifier) serve two main purposes:
   *
   * 1. They represent the primary identifier for a [`Subject`](crate::Subject).
   * 2. They extended when referenced as [linked identifiers](linked::Identifier).
   */
  id: ModelsSubjectIdentifier;
  /** A kind of [`Subject`](super::Subject). */
  kind: ModelsSubjectKind;
  /**
   * One or more [gateways](AnonymousOrReference) through which this subject
   * can be accessed.
   *
   * Gateways can be either [anonymous](AnonymousOrReference::Anonymous)
   * ([gateways](crate::Gateway) with no name) or a
   * [refererence](AnonymousOrReference::Reference) to a [named
   * gateway](gateway::Named) ([gateways](crate::Gateway) with a name).
   *
   * **Anonymous** gateways are intended to be embedded directly within a
   * returned subject in the `/subject` response object. They have no name
   * and are only referred to by the subject within which they are embedded.
   *
   * **Named** gateways, on the other hand, are included in the `gateways`
   * key in the `/subject` response object and referred to by name within a
   * returned subject in the `/subject` response object. They are intended to
   * be used when more than one subject references the same gateway. This
   * mechanism is available to ensure that the gateway object does not need
   * to be duplicated multiple times in the response in these cases.
   *
   * This field can contain multiple gateways to support scenarios where a
   * subject is available through more than one mechanism. We expect that
   * only one gateway will be returned in most responses (if at all).
   */
  gateways?: ModelsGatewayAnonymousOrReference[];
  metadata?: ModelsSubjectMetadata | null;
}

/**
 * A uniform resource locator (URL) according to the [URL
 * Standard](https://url.spec.whatwg.org/).
 */
export type ModelsUrl = string;

/** The primary name and namespace for a file within the source server. */
export interface ModelsFileIdentifier {
  /** An identifier for a namespace. */
  namespace: ModelsNamespaceIdentifier;
  /**
   * **`caDSR CDE 11284037 v1.00`**
   *
   * This metadata element is defined by the caDSR as "The literal label for an
   * electronic data file.". No permissible values are defined for this CDE.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11284037%20and%20ver_nr=1>
   */
  name: CdeV1FileName;
}

/** Metadata associated with a file. */
export type ModelsFileMetadata = ModelsMetadataCommonMetadata & {
  type: FieldUnownedFileType | null;
  size: FieldUnownedFileSize | null;
  checksums: FieldUnownedFileChecksums | null;
  description: FieldUnownedFileDescription | null;
  /**
   * A map of unharmonized metadata fields.
   *
   * Unharmonized keys may be any valid JSON string.
   */
  unharmonized?: FieldsUnharmonized;
};

/** A list of checksums for a file. */
export interface ModelsFileMetadataChecksums {
  md5?: CdeV1FileChecksumMD5 | null;
}

/** An anonymous [`Gateway`] or a reference to a named [`Gateway`]. */
export type ModelsGatewayAnonymousOrReference =
  | {
      /** Gateways, which notify of resources that are external to the API. */
      gateway: ModelsGateway;
      kind: "Anonymous";
    }
  | {
      /** The reference to a [`Named`] gateway. */
      gateway: string;
      kind: "Reference";
    };

/** A closed gateway. */
export type ModelsGatewayClosed = ModelsGatewayClosedStatus & {
  /**
   * A Markdown field (formatted according to the [CommonMark] standard) that
   * describes the gateway.
   *
   * At a minimum, a description of what the gateway is and why it is closed
   * is recommended.
   *
   * **Note:** this field is required for a closed gateway. This was an
   * intentional decision: gateways are intended to point users to external
   * resources. When that is not possible, a gateway's only purpose can be to
   * provide further information about the data source. Put another way, if
   * one is attempting to construct a closed gateway with no description, it
   * should be considered why a gateway is needed at all.
   *
   * [CommonMark]: https://commonmark.org
   */
  description: string;
};

/**
 * A link to an external resource.
 *
 * A link communicates information about where a resource is located, alongside
 * additional context regarding how the link should be interpreted (via the
 * `kind` field). All [`Link`]s include a `url` field pointing to the external
 * resource. In the case of [`Link::Approximate`] and [`Link::MailTo`], a
 * required `instructions` field is included to instruct the user of what steps
 * to take after the link has been followed (see the definition of
 * [`Link::Approximate`] and [`Link::MailTo`] for more details, respectively).
 *
 * **Note:** the context of what resources are desired compared with what
 * resources the link represents is an important consideration when
 * constructing the correct [`Link`]. For example, if the desired resource is a
 * specific file, but the server can only construct a link to a cohort of
 * files, a [`Link::Approximate`] should be used. In contrast, if the desired
 * resource is the entire cohort of files, a [`Link::Direct`] should be used.
 *
 * **Note:** the link does not imply the access level or immediate availability
 * of the data—it only points a user to where they can _attempt_ to access the
 * data. Instead, [`Link`]s are always wrapped in a [`Gateway`](super::Gateway)
 * that communicates the access level or requirements. In other words, a
 * [`Link`] can absolutely require authentication or authorization before data
 * becomes accessible (and the corresponding [`Gateway`](super::Gateway) within
 * which the [`Link`] is embedded should reflect this).
 *
 * ## Examples
 *
 * * If the data is contained within a file where a direct link can be
 * constructed, whether that file is open access or controlled, then a
 * [`Link::Direct`] should be constructed with a link directly to that file.
 * * In the event that study data is deposited as a study within a larger data
 * repository, such as the database of Genotypes and Phenotypes (dbGaP) or
 * the European Genome-phenome Archive (EGA), and the `url` points to the
 * study page:
 * * If the desired resource is a specific file or subset of (but not all)
 * files within the study, a [`Link::Approximate`] should be returned.
 * This is because the link includes more files than what was
 * specifically requested—thus, instructions on how to filter to the
 * files requested must be communicated.
 * * If the desired resource is _every_ file in the study, then a
 * [`Link::Direct`] should be returned pointing to the study page.
 * * If the data is not immediately requestable through a webpage but there
 * exists an informational page on how to request the data using an
 * out-of-band process, then a [`Link::Informational`] should be used.
 * * If the data is available after contacting an email address, then a
 * [`Link::MailTo`] should be used.
 */
export type ModelsGatewayLink =
  | {
      /**
       * A uniform resource locator (URL) according to the [URL
       * Standard](https://url.spec.whatwg.org/).
       */
      url: ModelsUrl;
      kind: "Direct";
    }
  | {
      /**
       * A uniform resource locator (URL) according to the [URL
       * Standard](https://url.spec.whatwg.org/).
       */
      url: ModelsUrl;
      /**
       * The manual instructions to follow after navigating to the URL.
       *
       * As much as is practical, instructions should be as specific as
       * possible to the desired data. We expect that generating dynamic
       * instructions based on the user's selection will require a
       * non-trivial amount of development effort rather than returning
       * generic set of instructions. The reason for this is because this
       * field is intended to be surfaced as tailored instructions with a
       * user interface detailing to the user _exactly_ how to retrieve their
       * desired data.
       *
       * For example, if a user is requesting whole-genome sequence BAM files
       * for a particular sample but the server can only construct a link to
       * _all_ files for the sample, the instructions should specify how to
       * filter down to only the whole-genome sequence BAM files
       * specifically—not how to operate filters within the interface
       * generally.
       */
      instructions: string;
      kind: "Approximate";
    }
  | {
      /**
       * A uniform resource locator (URL) according to the [URL
       * Standard](https://url.spec.whatwg.org/).
       */
      url: ModelsUrl;
      kind: "Informational";
    }
  | {
      /**
       * A uniform resource locator (URL) according to the [URL
       * Standard](https://url.spec.whatwg.org/).
       */
      url: ModelsUrl;
      /**
       * The instructions to follow when constructing the email request. At a
       * minimum, explaining what criteria are used in determining access,
       * what information is requested, and what to expect after the
       * email in terms of communication and timeline is recommended.
       */
      instructions: string;
      kind: "MailTo";
    };

/**
 * A named gateway.
 *
 * A named gateway is simply a [`Gateway`] with a name. Named gateways exist so
 * that multiple files in a response can refer to the same gateway (by name)
 * without duplicating the information for that gateway multiple times.
 *
 * **Note:** a _named_ gateway can only be included in a `gateways` response
 * object—they cannot be embedded directly within a [`File`](crate::File) in
 * the response.
 */
export type ModelsGatewayNamed = ModelsGateway & {
  /** The name. */
  name: string;
};

/** The status of a closed gateway. */
export type ModelsGatewayClosedStatus =
  | {
      status: "IndefinitelyClosed";
    }
  | {
      /**
       * If known, the ISO 8601 formatted, UTC-based date and time when the
       * the resource will become available.
       *
       * This field is intended to indicate to the consumer that they should
       * retry their request on or after the listed time to gain an updated
       * gateway definition.
       * @format date-time
       */
      available_at?: string | null;
      status: "AwaitingPublication";
    }
  | {
      /**
       * The ISO 8601 formatted, UTC-based date and time when the the
       * resource will become available.
       *
       * This field is intended to indicate to the consumer that they should
       * retry their request on or after the listed time to gain an updated
       * gateway definition.
       *
       * **Note:** for the gateway to have a kind of [`Status::Embargoed`],
       * by definition, a date at which the resource becomes available _must_
       * be known. If a date is not known, then the resource does not fit the
       * API's definition of "embargoed".
       * @format date-time
       */
      available_at: string;
      status: "Embargoed";
    };

/** Metadata that is common to all metadata blocks. */
export interface ModelsMetadataCommonMetadata {
  /**
   * Statements of deposition to public repositories for a given entity.
   *
   * **NOTE**: when you declare that a dataset has been deposited to a public
   * repository such as dbGaP or EGA, you should also include a gateway and
   * link pointing to where that entity can be found in the public
   * repository.
   */
  depositions: ModelsMetadataCommonDepositionAccession[];
}

/** An accession of a public repository where the data has been deposited. */
export type ModelsMetadataCommonDepositionAccession = {
  kind: "dbGaP";
  /**
   * **`caDSR CDE 11524544 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A stable unique
   * alphanumeric identifier assigned to a study and any objects by the database
   * of Genotypes and Phenotypes (dbGaP).".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11524544%20and%20ver_nr=1>
   */
  value: CdeV1DepositionDbgapPhsAccession;
};

/** A description for a metadata field. */
export type ModelsMetadataFieldDescription =
  | ModelsMetadataFieldDescriptionHarmonized
  | ModelsMetadataFieldDescriptionUnharmonized;

/** Details regarding the harmonization process. */
export interface ModelsMetadataFieldDetails {
  method?: ModelsMetadataFieldDetailsMethod | null;
  harmonizer?: ModelsMetadataFieldDetailsHarmonizer | null;
  url?: ModelsUrl | null;
}

/**
 * A harmonized metadata field description.
 *
 * Harmonized keys _must_ fit the regex pattern `^[a-z_]+$`.
 */
export interface ModelsMetadataFieldDescriptionHarmonized {
  /**
   * Whether or not this field is harmonized across the ecosystem.
   *
   * This will always be set to `true`.
   * @default true
   */
  harmonized: boolean;
  /**
   * A comma (`.`) delimited path to the field's location on the `metadata`
   * objects returned by the various subject endpoints.
   */
  path: string;
  /**
   * A uniform resource locator (URL) according to the [URL
   * Standard](https://url.spec.whatwg.org/).
   */
  wiki_url: ModelsUrl;
  standard?: ModelsMetadataFieldDescriptionHarmonizedStandard | null;
}

/**
 * An unharmonized metadata field description.
 *
 * Unharmonized keys may be any valid JSON string.
 */
export interface ModelsMetadataFieldDescriptionUnharmonized {
  /**
   * Whether or not this field is harmonized across the ecosystem.
   *
   * This will always be set to `false`.
   * @default false
   */
  harmonized: boolean;
  /**
   * A display name for this metadata field as _suggested_ by the server (this
   * is not considered authoritative and can be ignored by the client if it so
   * chooses). This is mainly to avoid naming collisions of common fields across
   * servers.
   */
  name?: string | null;
  /** A plain-text description of what the field represents. */
  description?: string | null;
  /**
   * A comma (`.`) delimited path to the field's location on the `metadata`
   * objects returned by the various subject endpoints.
   */
  path: string;
  /**
   * If the field is considered harmonized across the federation ecosystem, the
   * name of the standard to which the field is harmonized.
   *
   * If the field is _not_ harmonized across the federation ecosystem, then this
   * should be [`None`].
   */
  standard?: string | null;
  url?: ModelsUrl | null;
}

/** A standard to which a field is harmonized. */
export interface ModelsMetadataFieldDescriptionHarmonizedStandard {
  /** The name. */
  name: string;
  /**
   * A uniform resource locator (URL) according to the [URL
   * Standard](https://url.spec.whatwg.org/).
   */
  url: ModelsUrl;
}

/**
 * A statement on the expertise of the individual (or individuals) who are
 * assigning the harmonized values. This information can help data receivers
 * understand the context within which the data was harmonized (particularly in
 * data fields that may be constantly evolving or changing—for instance,
 * diagnosis).
 *
 * **NOTE:** if you find that there are types of harmonizers that are not
 * captured here, please make an issue on the GitHub repository so we can
 * support the value.
 */
export enum ModelsMetadataFieldDetailsHarmonizer {
  DomainExpert = "DomainExpert",
  CurationTeamMember = "CurationTeamMember",
}

/**
 * The method by which data was harmonized.
 *
 * **NOTE:** if you find that there are types of harmonization methods that are
 * not captured here, please make an issue on the GitHub repository so we can
 * support the value.
 */
export enum ModelsMetadataFieldDetailsMethod {
  Mapped = "Mapped",
}

/**
 * A description of a namespace.
 *
 * This description cannot exceed 2048 characters.
 * @example "A namespace owned by Example Organization."
 */
export type ModelsNamespaceDescription = string;

/** An identifier for a namespace. */
export interface ModelsNamespaceIdentifier {
  /**
   * The identifier of an organization.
   *
   * The identifier **must** conform to
   * [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
   * matching the pattern `^[a-z0-9-]+$`. Any identifier that does not match this
   * pattern should be considered invalid by clients.
   *
   * **Note:** this field is asserted by the source server, but it is not
   * guaranteed to be authoritative across the federation (due to the
   * decentralized nature of organization and namespace allocation).
   *
   * **Note**: the regex for this field does not allow for any spaces because it is
   * anticipated that the field will be displayable as a repository (e.g.,
   * `example-organization/ExampleNamespace`).
   */
  organization: ModelsOrganizationIdentifier;
  /**
   * The name of a namespace.
   *
   * Typically, this is going to represent a particular dataset within the source server.
   * The name **must** conform to the pattern `^[A-Za-z0-9-]+$`. Any name that does not
   * match this pattern should be considered invalid by clients.
   *
   * NOTE: the regex for this field does not allow for any spaces because it is
   * anticipated that the field will be displayable as a repository (e.g.,
   * `example-organization/ExampleNamespace`).
   */
  name: ModelsNamespaceIdentifierName;
}

/** Metadata associated with a namespace. */
export type ModelsNamespaceMetadata = ModelsMetadataCommonMetadata & {
  study_short_title: FieldUnownedNamespaceStudyShortTitle | null;
  study_name: FieldUnownedNamespaceStudyName | null;
  /** The study funding id. */
  study_funding_id: FieldUnownedNamespaceStudyFundingId[] | null;
  study_id: FieldUnownedNamespaceStudyId | null;
  /**
   * A map of unharmonized metadata fields.
   *
   * Unharmonized keys may be any valid JSON string.
   */
  unharmonized?: FieldsUnharmonized;
};

/**
 * The name of a namespace.
 *
 * Typically, this is going to represent a particular dataset within the source server.
 * The name **must** conform to the pattern `^[A-Za-z0-9-]+$`. Any name that does not
 * match this pattern should be considered invalid by clients.
 *
 * NOTE: the regex for this field does not allow for any spaces because it is
 * anticipated that the field will be displayable as a repository (e.g.,
 * `example-organization/ExampleNamespace`).
 */
export type ModelsNamespaceIdentifierName = string;

/**
 * The identifier of an organization.
 *
 * The identifier **must** conform to
 * [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
 * matching the pattern `^[a-z0-9-]+$`. Any identifier that does not match this
 * pattern should be considered invalid by clients.
 *
 * **Note:** this field is asserted by the source server, but it is not
 * guaranteed to be authoritative across the federation (due to the
 * decentralized nature of organization and namespace allocation).
 *
 * **Note**: the regex for this field does not allow for any spaces because it is
 * anticipated that the field will be displayable as a repository (e.g.,
 * `example-organization/ExampleNamespace`).
 */
export type ModelsOrganizationIdentifier = string;

/** Metadata associated with an organization. */
export type ModelsOrganizationMetadata = ModelsMetadataCommonMetadata & {
  /**
   * Institutions associated with an organization.
   *
   * **NOTE:** in this design, an organization in not always a single
   * institution—it may also represent a consortium of institutions, for
   * instance. This is necessary, since a namespace can be tied to one and
   * only one organization in the API specification. As such, if the above is
   * not true, there is no way to make a namespace where data is contributed
   * from multiple institutions.
   */
  institution: FieldUnownedOrganizationInstitution[] | null;
  /**
   * A map of unharmonized metadata fields.
   *
   * Unharmonized keys may be any valid JSON string.
   */
  unharmonized?: FieldsUnharmonized;
};

/**
 * The proper name of the organization as it should be displayed by clients.
 *
 * This name name cannot exceed 256 characters.
 *
 * This field is intended to be the proper name of the organization that mints
 * identifiers within a given namespace. That said, we have intentionally not
 * required that this be an organization specifically, as there may be exceptions
 * to this guideline. We recommend that you use an organization name here if you
 * can, but you may put whatever value is appropriate to describe the owner of the
 * namespace.
 *
 * It is recommended that you use title case for this field, though that is not
 * required.
 *
 * **Note:** this field is asserted by the source server, but it is not guaranteed
 * to be authoritative across the federation (due to the decentralized nature of
 * organization and namespace allocation).
 * @example "Example Organization"
 */
export type ModelsOrganizationName = string;

/**
 * An identifier for a [`Sample`](crate::Sample).
 *
 * [`Identifiers`](Identifier) serve two main purposes:
 *
 * 1. They represent the primary identifier for a [`Sample`](crate::Sample).
 * 2. They extended when referenced as [linked identifiers](linked::Identifier).
 */
export interface ModelsSampleIdentifier {
  /** An identifier for a namespace. */
  namespace: ModelsNamespaceIdentifier;
  /**
   * **`caDSR CDE 15100774 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A unique string of characters
   * used to identify a specimen.".
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=15100774%20and%20ver_nr=1>
   * @example "SampleName001"
   */
  name: string;
}

/** Metadata associated with a sample. */
export type ModelsSampleMetadata = ModelsMetadataCommonMetadata & {
  age_at_diagnosis: FieldUnownedSampleAgeAtDiagnosis | null;
  diagnosis: FieldUnownedSampleDiagnosis | null;
  disease_phase: FieldUnownedSampleDiseasePhase | null;
  tissue_type: FieldUnownedSampleTissueType | null;
  tumor_classification: FieldUnownedSampleTumorClassification | null;
  tumor_tissue_morphology: FieldUnownedSampleTumorTissueMorphology | null;
  age_at_collection: FieldUnownedSampleAgeAtCollection | null;
  library_strategy: FieldUnownedSampleLibraryStrategy | null;
  preservation_method: FieldUnownedSamplePreservationMethod | null;
  /**
   * The alternate identifiers for the sample.
   *
   * Note that this list of identifiers *must* include the main identifier
   * for the [`Sample`].
   */
  identifiers: FieldUnownedSampleIdentifier[] | null;
  /**
   * A map of unharmonized metadata fields.
   *
   * Unharmonized keys may be any valid JSON string.
   */
  unharmonized?: FieldsUnharmonized;
};

/**
 * A linked identifier for a [`Sample`](crate::Sample).
 *
 * Linked identifiers are identifiers that are able to be linked back to servers within
 * the federated ecosystem (i.e., the server that owns this identifier within the
 * ecosystem is known).
 */
export type ModelsSampleIdentifierLinkedIdentifier = ModelsSampleIdentifier & {
  /**
   * A uniform resource locator (URL) according to the [URL
   * Standard](https://url.spec.whatwg.org/).
   */
  server: ModelsUrl;
};

/**
 * A referenced identifier for a [`Sample`](crate::Sample).
 *
 * A referenced identifier is a reference to either an identifier whose owner is known
 * and operates an authoritative federation server containing that identifier (i.e., a
 * [linked identifier](linked::Identifier)) _or_ a reference to an identifier that is
 * generally known to be associated with the sample but does not have an associated
 * server that asserts ownership of the identifier (i.e., an [unlinked
 * identifier](unlinked::Identifier)).
 */
export type ModelsSampleIdentifierReferencedIdentifier =
  | (ModelsSampleIdentifierLinkedIdentifier & {
      type: "Linked";
    })
  | (ModelsSampleIdentifierUnlinkedIdentifier & {
      type: "Unlinked";
    });

/**
 * An unlinked identifier for a [`Sample`](crate::Sample).
 *
 * This represents an arbitrary identitier that cannot be linked to any source server
 * in the broader federated ecosystem. There are no restricted values for this
 * identifier.
 */
export interface ModelsSampleIdentifierUnlinkedIdentifier {
  name: string;
}

/**
 * The approximate age of collection in days.
 *
 * * When the age at collection is collected by the source server in days, the
 * number of days is reported directly.
 * * When the age at collection is collected by the source server in years, the
 * number of years is multiplied by 365.25 to arrive at an approximate number
 * of days.
 * @format float
 */
export type ModelsSampleMetadataAgeAtCollection = number;

/**
 * The approximate age of diagnosis in days.
 *
 * * When the age at diagnosis is collected by the source server in days, the
 * number of days is reported directly.
 * * When the age at diagnosis is collected by the source server in years, the
 * number of years is multiplied by 365.25 to arrive at an approximate number
 * of days.
 * @format float
 */
export type ModelsSampleMetadataAgeAtDiagnosis = number;

/**
 * The diagnosis for a [`Sample`](crate::Sample).
 *
 * This value can be any permissible diagnosis in v1.7.2 of the CCDI Submission
 * Template. These values are from the value set **diagnosis_classification**
 * found in the 'Terms and Value Sets' tab from the [CCDI Submission Template
 * v1.7.2].
 *
 * To facilitate quick access to these values, we have provided a slimmed down
 * spreadsheet containing the valid diagnoses:
 *
 * 1. Download the spreadsheet linked below titled
 * 'CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx'.
 * 2. The permissible values are found in column A of the 'diagnosis' tab,
 * titled **diagnosis_category_term**
 *
 * [CCDI Submission Template v1.7.2]: https://github.com/CBIIT/ccdi-model/blob/682a99d93b66540bb880ce5899ba8096968a96cf/metadata-manifest/CCDI_Submission_Template_v1.7.2.xlsx
 * [CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx]: https://cbiit.github.io/ccdi-federation-api/assets/CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx
 */
export type ModelsSampleMetadataDiagnosis = string;

/**
 * An identifier for a [`Subject`](crate::Subject).
 *
 * [`Identifiers`](Identifier) serve two main purposes:
 *
 * 1. They represent the primary identifier for a [`Subject`](crate::Subject).
 * 2. They extended when referenced as [linked identifiers](linked::Identifier).
 */
export interface ModelsSubjectIdentifier {
  /** An identifier for a namespace. */
  namespace: ModelsNamespaceIdentifier;
  /**
   * **`caDSR CDE 6380049 v1.00`**
   *
   * This metadata element is defined by the caDSR as "A unique subject
   * identifier within a site and a study.". No permissible values are defined
   * for this CDE.
   *
   * Link:
   * <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6380049%20and%20ver_nr=1>
   */
  name: CdeV1SubjectName;
}

/** A kind of [`Subject`](super::Subject). */
export enum ModelsSubjectKind {
  Participant = "Participant",
  PatientDerivedXenograft = "Patient Derived Xenograft",
  CellLine = "Cell Line",
  Organoid = "Organoid",
}

/** Metadata associated with a subject. */
export type ModelsSubjectMetadata = ModelsMetadataCommonMetadata & {
  sex: FieldUnownedSubjectSex | null;
  /** The race(s) of the subject. */
  race: FieldUnownedSubjectRace[] | null;
  ethnicity: FieldUnownedSubjectEthnicity | null;
  /**
   * The alternate identifiers for the subject.
   *
   * Note that this list of identifiers *must* include the main identifier
   * for the [`Subject`].
   */
  identifiers: FieldUnownedSubjectIdentifier[] | null;
  vital_status: FieldUnownedSubjectVitalStatus | null;
  age_at_vital_status: FieldUnownedSubjectAgeAtVitalStatus | null;
  /**
   * A map of unharmonized metadata fields.
   *
   * Unharmonized keys may be any valid JSON string.
   */
  unharmonized?: FieldsUnharmonized;
};

/**
 * A linked identifier for a [`Subject`](crate::Subject).
 *
 * Linked identifiers are identifiers that are able to be linked back to servers within
 * the federated ecosystem (i.e., the server that owns this identifier within the
 * ecosystem is known).
 */
export type ModelsSubjectIdentifierLinkedIdentifier =
  ModelsSubjectIdentifier & {
    /**
     * A uniform resource locator (URL) according to the [URL
     * Standard](https://url.spec.whatwg.org/).
     */
    server: ModelsUrl;
  };

/**
 * A referenced identifier for a [`Subject`](crate::Subject).
 *
 * A referenced identifier is a reference to either an identifier whose owner is known
 * and operates an authoritative federation server containing that identifier (i.e., a
 * [linked identifier](linked::Identifier)) _or_ a reference to an identifier that is
 * generally known to be associated with the subject but does not have an associated
 * server that asserts ownership of the identifier (i.e., an [unlinked
 * identifier](unlinked::Identifier)).
 */
export type ModelsSubjectIdentifierReferencedIdentifier =
  | (ModelsSubjectIdentifierLinkedIdentifier & {
      type: "Linked";
    })
  | (ModelsSubjectIdentifierUnlinkedIdentifier & {
      type: "Unlinked";
    });

/**
 * An unlinked identifier for a [`Subject`](crate::Subject).
 *
 * This represents an arbitrary identitier that cannot be linked to any source server
 * in the broader federated ecosystem. There are no restricted values for this
 * identifier.
 */
export interface ModelsSubjectIdentifierUnlinkedIdentifier {
  name: string;
}

/**
 * The approximate age at vital status in days.
 *
 * * When the age at vital status is collected by the source server in days,
 * the number of days is reported directly.
 * * When the age at vital status is collected by the source server in years,
 * the number of years is multiplied by 365.25 to arrive at an approximate
 * number of days.
 * @format float
 */
export type ModelsSubjectMetadataAgeAtVitalStatus = number;

/** A wrapper around one or more [errors](Kind). */
export interface ResponsesErrors {
  /** The errors within this response. */
  errors: ResponsesErrorKind[];
}

/** A response representing a single [`File`](models::File). */
export type ResponsesFile = ModelsFile & object;

/**
 * A response representing multiple files known about by the server.
 *
 * When no sort order is provided, files **must** be ordered by the primary
 * identifier. This means that, when comparing two identifiers:
 *
 * 1. The namespace organization field should be sorted alphabetically. If all
 * values for the namespace organization are equal, continue on to the next
 * sorting criteria.
 * 2. The namespace name field should be sorted alphabetically. If all
 * values for the namespace names are equal, continue on to the next
 * sorting criteria.
 * 3. The entity name should be sorted alphabetically.
 *
 * Since the `namespace` and `name` identifiers should always uniquely apply to
 * a single entity, this should always resolve to an ordering.
 *
 * If there is a provided sort order, use that instead.
 */
export interface ResponsesFiles {
  /** A summary of a paged entity response. */
  summary: ResponsesEntitySummary;
  /** The files. */
  data: ResponsesFile[];
  gateways?: ModelsGatewayNamed[];
}

/** A response for information regarding the server. */
export interface ResponsesInformation {
  /** Information that is specific to the server itself. */
  server: ResponsesInfoServerInformation;
  /** Information that is specific to the API that the server implements. */
  api: ResponsesInfoApiInformation;
  /** Information that is specific to the API that the server implements. */
  data: ResponsesInfoDataInformation;
}

export type ResponsesNamespace = ModelsNamespace;

/** A response for describing namespaces. */
export type ResponsesNamespaces = ModelsNamespace[];

export type ResponsesOrganization = ModelsOrganization;

/** A response for describing organizations. */
export type ResponsesOrganizations = ModelsOrganization[];

/** A response representing a single [`Sample`](models::Sample). */
export type ResponsesSample = ModelsSample & object;

/**
 * A response representing multiple samples known about by the server.
 *
 * When no sort order is provided, samples **must** be ordered by the primary
 * identifier. This means that, when comparing two identifiers:
 *
 * 1. The namespace organization field should be sorted alphabetically. If all
 * values for the namespace organization are equal, continue on to the next
 * sorting criteria.
 * 2. The namespace name field should be sorted alphabetically. If all values
 * for the namespace names are equal, continue on to the next sorting
 * criteria.
 * 3. The entity name should be sorted alphabetically.
 *
 * Since the `namespace` and `name` identifiers should always uniquely apply to
 * a single entity, this should always resolve to an ordering.
 *
 * If there is a provided sort order, use that instead.
 */
export interface ResponsesSamples {
  /** A summary of a paged entity response. */
  summary: ResponsesEntitySummary;
  /** The samples. */
  data: ModelsSample[];
  gateways?: ModelsGatewayNamed[];
}

/** A response representing a single [`Subject`](models::Subject). */
export type ResponsesSubject = ModelsSubject & object;

/**
 * A response representing multiple subjects known about by the server.
 *
 * When no sort order is provided, subjects **must** be ordered by the primary
 * identifier. This means that, when comparing two identifiers:
 *
 * 1. The namespace organization field should be sorted alphabetically. If all
 * values for the namespace organization are equal, continue on to the next
 * sorting criteria.
 * 2. The namespace name field should be sorted alphabetically. If all
 * values for the namespace names are equal, continue on to the next
 * sorting criteria.
 * 3. The entity name should be sorted alphabetically.
 *
 * Since the `namespace` and `name` identifiers should always uniquely apply to
 * a single entity, this should always resolve to an ordering.
 *
 * If there is a provided sort order, use that instead.
 */
export interface ResponsesSubjects {
  /** A summary of a paged entity response. */
  summary: ResponsesEntitySummary;
  /** The subjects. */
  data: ModelsSubject[];
  gateways?: ModelsGatewayNamed[];
}

/** A summary response for an entity endpoint. */
export interface ResponsesSummary {
  /** Counts included in a summary endpoint. */
  counts: ResponsesSummaryCounts;
}

/** A value along with the number of counted entities for that value. */
export interface ResponsesByCountValueCount {
  /** The value. */
  value: any;
  /**
   * The number of times the value was counted.
   * @min 0
   */
  count: number;
}

/**
 * A set of results from grouping [`Files`](ccdi_models::File) by a specified
 * metadata field and then summing the counts for each value (along with computing a
 * total count).
 */
export interface ResponsesByCountFileResults {
  /**
   * The total number of counts in this result set.
   * @min 0
   */
  total: number;
  /**
   * The total number of entries that are missing values. In this context,
   * "missing" means either (a) the individual metadata key is missing or (b)
   * the entire metadata object is missing.
   * @min 0
   */
  missing: number;
  /** The counts per value observed for the result set. */
  values: ResponsesByCountValueCount[];
}

/**
 * A set of results from grouping [`Samples`](ccdi_models::Sample) by a specified
 * metadata field and then summing the counts for each value (along with computing a
 * total count).
 */
export interface ResponsesByCountSampleResults {
  /**
   * The total number of counts in this result set.
   * @min 0
   */
  total: number;
  /**
   * The total number of entries that are missing values. In this context,
   * "missing" means either (a) the individual metadata key is missing or (b)
   * the entire metadata object is missing.
   * @min 0
   */
  missing: number;
  /** The counts per value observed for the result set. */
  values: ResponsesByCountValueCount[];
}

/**
 * A response for grouping [`Subject`](ccdi_models::Subject)s by a metadata field
 * and then summing the counts.
 */
export interface ResponsesByCountSubjectResults {
  /**
   * The total number of counts in this result set.
   * @min 0
   */
  total: number;
  /**
   * The total number of entries that are missing values. In this context,
   * "missing" means either (a) the individual metadata key is missing or (b)
   * the entire metadata object is missing.
   * @min 0
   */
  missing: number;
  /** The counts per value observed for the result set. */
  values: ResponsesByCountValueCount[];
}

/** Counts that summarize the contents of a paged entity response. */
export interface ResponsesEntityCounts {
  /**
   * The number of entities within the currently selected page in the result set.
   * @min 0
   */
  current: number;
  /**
   * The number of entities across all pages in the result set.
   * @min 0
   */
  all: number;
}

/** A summary of a paged entity response. */
export interface ResponsesEntitySummary {
  /** Counts that summarize the contents of a paged entity response. */
  counts: ResponsesEntityCounts;
}

/** A response indicating an error from the API. */
export type ResponsesErrorKind = (
  | {
      /** The HTTP method that was used in the request. */
      method: string;
      /** The route that was requested. */
      route: string;
      kind: "InvalidRoute";
    }
  | {
      /**
       * If known, the parameters that are invalid. If not known, pass `None`
       * to this field for a more general error message.
       */
      parameters?: string[] | null;
      /** A plain-text reason describing why the parameters are invalid. */
      reason: string;
      kind: "InvalidParameters";
    }
  | {
      /** The entity (or entities) that are not found. */
      entity: string;
      kind: "NotFound";
    }
  | {
      /** The entity (or entities) where data cannot be shared. */
      entity: string;
      /** The reason that the line-level data cannot be shared. */
      reason: string;
      kind: "UnshareableData";
    }
  | {
      /** The field that is not supported. */
      field: string;
      /** The reason that the field is not supported. */
      reason: string;
      kind: "UnsupportedField";
    }
) & {
  /**
   * A plain-text description of the error.
   *
   * This field is intended to be shown within a user interface or similar if
   * needed. Please use this field if you intend to pass the error along to a
   * user.
   */
  message: string;
};

/** Information that is specific to the API that the server implements. */
export interface ResponsesInfoApiInformation {
  /**
   * The version of the API that this server supports.
   * @example "v1.0.0"
   */
  api_version: string;
  /**
   * A URL pointing to the latest version of the Swagger documentation.
   *
   * Note that, at times, the latest version of the Swagger documentation may
   * not be in sync with the version of the API deployed for this server. The
   * intention of this field is not to link to a Swagger specification that
   * strictly matches this particular server, but rather, to point users to
   * where the specification is developed and hosted.
   * @default "https://cbiit.github.io/ccdi-federation-api/"
   */
  documentation_url: string;
}

/** Information that is specific to the API that the server implements. */
export interface ResponsesInfoDataInformation {
  /** The version of data published within this source server. */
  version: ResponsesInfoDataVersion;
  /**
   * The ISO 8601 formatted, UTC-based date and time when the data was last
   * updated.
   *
   * This represents the last _update_ time. In contrast to the
   * `data_version` field, this field is updated whenever a data update is
   * performed irrespective of whether there were actually changes in the
   * data.
   * @format date-time
   */
  last_updated: string;
  /**
   * A URL pointing to the wiki.
   *
   * The intention of this field is to make users aware that we maintain a
   * federation-wide wiki that describes the data elements in detail.
   * @default "https://github.com/CBIIT/ccdi-federation-api/wiki"
   */
  wiki_url: string;
  /**
   * If available, a link pointing to where users can learn more about the
   * data contained within this particular server.
   *
   * This is intended to be a server-specification documentation link, not
   * any link that is developed by the federation.
   * @default "https://docs.example.com"
   */
  documentation_url?: string | null;
}

/** The version of data published within this source server. */
export type ResponsesInfoDataVersion = ResponsesInfoDataVersionAbout & {
  /**
   * The value of the version.
   *
   * This field represents a free-text field where data is arbitrarily
   * versioned by the source server. Any versioning scheme is permissible.
   * @min 1
   * @example 1
   */
  version: number;
};

/** A description of how data is versioning within the source server. */
export type ResponsesInfoDataVersionAbout =
  | {
      /**
       * A free-text description of the data version included with the response
       * from the source server. This field is interpreted as Markdown (as
       * defined by the [CommonMark](https://commonmark.org/) specification).
       */
      about: string;
    }
  | {
      /**
       * A URL where one can learn more about the data versioning for this source
       * server.
       */
      about_url: string;
    };

/** Information that is specific to the server itself. */
export interface ResponsesInfoServerInformation {
  /**
   * The name of this server (if it has one).
   *
   * This is a free-text field describing the name of this server, if it has
   * one. The intention is to be able to describe the proper name of the
   * application.
   * @example "Example Server"
   */
  name?: string | null;
  /**
   * The version of this server (if it has one).
   *
   * Though there is explicitly no versioning dictated by the specification,
   * we recommend [Semantic Versioning v2.0](https://semver.org/) in the
   * absence of better options to align with the scheme used by the API. Note
   * that using the same versioning scheme does not mean that the version of
   * your server is recommended to be the same version as the API.
   * @example "v1.22"
   */
  version?: string | null;
  /**
   * A free-text string describing the owner of the namespace.
   *
   * This field is intended to be the proper name of the organization that
   * owns and operates the server. That said, we have intentionally not
   * required this restriction, as there may be exceptions to this guideline.
   * We recommend that you use an organization name here if you can, but you
   * may put whatever value is appropriate to describe the owner of the
   * server.
   *
   * It is recommended that you use title case for this field, though that is
   * not strictly required.
   * @example "Example Organization"
   */
  owner: string;
  /**
   * A support email address for the server.
   *
   * This field is required to be a valid email address (both in format and
   * in terms of the email address being actively monitored).
   * @example "support@example.com"
   */
  contact_email: string;
  /**
   * If desired, a link to a page intended to be consumed by a web browser
   * that describes more about the owner. This can be a link to your
   * organization's main web page or a link to a webpage describing the
   * project.
   * @example "https://example.com"
   */
  about_url?: string | null;
  /**
   * If your code base is open source and you want to advertise that, a link
   * to the repository where the code is stored.
   * @example "https://github.com/CBIIT/ccdi-federation-api"
   */
  repository_url?: string | null;
  /**
   * If available, a URL where users can report issues.
   * @example "https://github.com/CBIIT/ccdi-federation-api/issues"
   */
  issues_url?: string | null;
}

/** A response for describing metadata fields for a subject, sample, or file. */
export interface ResponsesMetadataFieldDescriptions {
  /** Field descriptions. */
  fields: ModelsMetadataFieldDescription[];
}

/** Counts included in a summary endpoint. */
export interface ResponsesSummaryCounts {
  /** @min 0 */
  total: number;
}

export type QueryParamsType = Record<string | number, any>;
export type ResponseFormat = keyof Omit<Body, "body" | "bodyUsed">;

export interface FullRequestParams extends Omit<RequestInit, "body"> {
  /** set parameter to `true` for call `securityWorker` for this request */
  secure?: boolean;
  /** request path */
  path: string;
  /** content type of request body */
  type?: ContentType;
  /** query params */
  query?: QueryParamsType;
  /** format of response (i.e. response.json() -> format: "json") */
  format?: ResponseFormat;
  /** request body */
  body?: unknown;
  /** base url */
  baseUrl?: string;
  /** request cancellation token */
  cancelToken?: CancelToken;
}

export type RequestParams = Omit<
  FullRequestParams,
  "body" | "method" | "query" | "path"
>;

export interface ApiConfig<SecurityDataType = unknown> {
  baseUrl?: string;
  baseApiParams?: Omit<RequestParams, "baseUrl" | "cancelToken" | "signal">;
  securityWorker?: (
    securityData: SecurityDataType | null
  ) => Promise<RequestParams | void> | RequestParams | void;
  customFetch?: typeof fetch;
}

export interface HttpResponse<D extends unknown, E extends unknown = unknown>
  extends Response {
  data: D;
  error: E;
}

type CancelToken = Symbol | string | number;

export enum ContentType {
  Json = "application/json",
  FormData = "multipart/form-data",
  UrlEncoded = "application/x-www-form-urlencoded",
  Text = "text/plain",
}

export class HttpClient<SecurityDataType = unknown> {
  public baseUrl: string = "https://ccdi.stjude.cloud/api/v1";
  private securityData: SecurityDataType | null = null;
  private securityWorker?: ApiConfig<SecurityDataType>["securityWorker"];
  private abortControllers = new Map<CancelToken, AbortController>();
  private customFetch = (...fetchParams: Parameters<typeof fetch>) =>
    fetch(...fetchParams);

  private baseApiParams: RequestParams = {
    credentials: "same-origin",
    headers: {},
    redirect: "follow",
    referrerPolicy: "no-referrer",
  };

  constructor(apiConfig: ApiConfig<SecurityDataType> = {}) {
    Object.assign(this, apiConfig);
  }

  public setSecurityData = (data: SecurityDataType | null) => {
    this.securityData = data;
  };

  protected encodeQueryParam(key: string, value: any) {
    const encodedKey = encodeURIComponent(key);
    return `${encodedKey}=${encodeURIComponent(
      typeof value === "number" ? value : `${value}`
    )}`;
  }

  protected addQueryParam(query: QueryParamsType, key: string) {
    return this.encodeQueryParam(key, query[key]);
  }

  protected addArrayQueryParam(query: QueryParamsType, key: string) {
    const value = query[key];
    return value.map((v: any) => this.encodeQueryParam(key, v)).join("&");
  }

  protected toQueryString(rawQuery?: QueryParamsType): string {
    const query = rawQuery || {};
    const keys = Object.keys(query).filter(
      (key) => "undefined" !== typeof query[key]
    );
    return keys
      .map((key) =>
        Array.isArray(query[key])
          ? this.addArrayQueryParam(query, key)
          : this.addQueryParam(query, key)
      )
      .join("&");
  }

  protected addQueryParams(rawQuery?: QueryParamsType): string {
    const queryString = this.toQueryString(rawQuery);
    return queryString ? `?${queryString}` : "";
  }

  private contentFormatters: Record<ContentType, (input: any) => any> = {
    [ContentType.Json]: (input: any) =>
      input !== null && (typeof input === "object" || typeof input === "string")
        ? JSON.stringify(input)
        : input,
    [ContentType.Text]: (input: any) =>
      input !== null && typeof input !== "string"
        ? JSON.stringify(input)
        : input,
    [ContentType.FormData]: (input: any) =>
      Object.keys(input || {}).reduce((formData, key) => {
        const property = input[key];
        formData.append(
          key,
          property instanceof Blob
            ? property
            : typeof property === "object" && property !== null
            ? JSON.stringify(property)
            : `${property}`
        );
        return formData;
      }, new FormData()),
    [ContentType.UrlEncoded]: (input: any) => this.toQueryString(input),
  };

  protected mergeRequestParams(
    params1: RequestParams,
    params2?: RequestParams
  ): RequestParams {
    return {
      ...this.baseApiParams,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...(this.baseApiParams.headers || {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {}),
      },
    };
  }

  protected createAbortSignal = (
    cancelToken: CancelToken
  ): AbortSignal | undefined => {
    if (this.abortControllers.has(cancelToken)) {
      const abortController = this.abortControllers.get(cancelToken);
      if (abortController) {
        return abortController.signal;
      }
      return void 0;
    }

    const abortController = new AbortController();
    this.abortControllers.set(cancelToken, abortController);
    return abortController.signal;
  };

  public abortRequest = (cancelToken: CancelToken) => {
    const abortController = this.abortControllers.get(cancelToken);

    if (abortController) {
      abortController.abort();
      this.abortControllers.delete(cancelToken);
    }
  };

  public request = async <T = any, E = any>({
    body,
    secure,
    path,
    type,
    query,
    format,
    baseUrl,
    cancelToken,
    ...params
  }: FullRequestParams): Promise<HttpResponse<T, E>> => {
    const secureParams =
      ((typeof secure === "boolean" ? secure : this.baseApiParams.secure) &&
        this.securityWorker &&
        (await this.securityWorker(this.securityData))) ||
      {};
    const requestParams = this.mergeRequestParams(params, secureParams);
    const queryString = query && this.toQueryString(query);
    const payloadFormatter = this.contentFormatters[type || ContentType.Json];
    const responseFormat = format || requestParams.format;

    return this.customFetch(
      `${baseUrl || this.baseUrl || ""}${path}${
        queryString ? `?${queryString}` : ""
      }`,
      {
        ...requestParams,
        headers: {
          ...(requestParams.headers || {}),
          ...(type && type !== ContentType.FormData
            ? { "Content-Type": type }
            : {}),
        },
        signal:
          (cancelToken
            ? this.createAbortSignal(cancelToken)
            : requestParams.signal) || null,
        body:
          typeof body === "undefined" || body === null
            ? null
            : payloadFormatter(body),
      }
    ).then(async (response) => {
      const r = response.clone() as HttpResponse<T, E>;
      r.data = null as unknown as T;
      r.error = null as unknown as E;

      const data = !responseFormat
        ? r
        : await response[responseFormat]()
            .then((data) => {
              if (r.ok) {
                r.data = data;
              } else {
                r.error = data;
              }
              return r;
            })
            .catch((e) => {
              r.error = e;
              return r;
            });

      if (cancelToken) {
        this.abortControllers.delete(cancelToken);
      }

      if (!response.ok) throw data;
      return data;
    });
  };
}

/**
 * @title CCDI Data Federation: Participating Nodes API
 * @version v1.0.0
 * @baseUrl https://ccdi.stjude.cloud/api/v1
 * @externalDocs https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative
 * @contact Childhood Cancer Data Initiative support email <NCIChildhoodCancerDataInitiative@mail.nih.gov>
 */
export class Api<
  SecurityDataType extends unknown
> extends HttpClient<SecurityDataType> {
  subject = {
    /**
     * @description Gets the subjects known by this server. ### Pagination This endpoint is paginated. Users may override the default pagination parameters by providing one or more of the pagination-related query parameters below. ### Filtering All harmonized (top-level) and unharmonized (nested under the `metadata.unharmonized` key) metadata fields are filterable. To achieve this, you can provide the field name as a [`String`]. Filtering follows the following rules: * For single-value metadata field, the subject is included in the results if its value _exactly_ matches the query string. Matches are case-sensitive. * For multiple-value metadata fields, the subject is included in the results if any of its values for the field _exactly_ match the query string (a logical OR [`||`]). Matches are case-sensitive. * When the metadata field is `null` (in the case of singlular or multiple-valued metadata fields) or empty, the subject is not included. * When multiple fields are provided as filters, a logical AND (`&&`) strings together the predicates. In other words, all filters must match for a subject to be returned. Note that this means that servers do not natively support logical OR (`||`) across multiple fields: that must be done by calling this endpoint with each of your desired queries and performing a set union of those subjects out of band. ### Ordering This endpoint has default ordering requirements—those details are documented in the `responses::Subjects` schema.
     *
     * @tags Subject
     * @name SubjectIndex
     * @summary Gets the subjects known by this server.
     * @request GET:/subject
     */
    subjectIndex: (
      query?: {
        /** Matches any subject where the `sex` field matches the string provided. */
        sex?: string;
        /**
         * Matches any subject where any member of the `race` field matches the
         * string provided.
         *
         * **Note:** a logical OR (`||`) is performed across the values when
         * determining whether the subject should be included in the results.
         */
        race?: string;
        /**
         * Matches any subject where the `ethnicity` field matches the string
         * provided.
         */
        ethnicity?: string;
        /**
         * Matches any subject where any member of the `identifiers` field matches
         * the string provided.
         *
         * **Note:** a logical OR (`||`) is performed across the values when
         * determining whether the subject should be included in the results.
         */
        identifiers?: string;
        /**
         * Matches any subject where the `vital_status` field matches the string
         * provided.
         */
        vital_status?: string;
        /**
         * Matches any subject where the `age_at_vital_status` field matches the
         * string provided.
         */
        age_at_vital_status?: string;
        /**
         * Matches any subject where any member of the `depositions` fields match
         * the string provided.
         *
         * **Note:** a logical OR (`||`) is performed across the values when
         * determining whether the subject should be included in the results.
         */
        depositions?: string;
        /**
         * All unharmonized fields should be filterable in the same manner as harmonized fields:
         *
         * * Filtering on a singular field should include the `Subject` in the results if the query exactly matches the value of that field for the `Subject` (case-sensitive).
         * * Filtering on field with multiple values should include the `Subject` in the results if the query exactly matches any of the values of the field for that `Subject` (case-sensitive).
         * * Unlike harmonized fields, unharmonized fields must be prefixed with `metadata.unharmonized`.
         *
         * **Note:** this query parameter is intended to be symbolic of any unharmonized field. Because of limitations within Swagger UI, it will show up as a query parameter that can be optionally be submitted as part of a request within Swagger UI. Please keep in mind that the literal query parameter `?metadata.unharmonized.<field>=value` is not supported, so attempting to use it within Swagger UI will not work!
         */
        metadataUnharmonizedField?: string;
        /**
         * The page to retrieve.
         *
         * This is a 1-based index of a page within a page set. The value of `page`
         * **must** default to `1` when this parameter is not provided.
         * @min 0
         */
        page?: number;
        /**
         * The number of results per page.
         *
         * Each server can select its own default value for `per_page` when this
         * parameter is not provided. That said, the convention within the
         * community is to use `100` as a default value if any value is equally
         * reasonable.
         * @min 0
         */
        per_page?: number;
      },
      params: RequestParams = {}
    ) =>
      this.request<ResponsesSubjects, ResponsesErrors>({
        path: `/subject`,
        method: "GET",
        query: query,
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the subject matching the provided id (if the subject exists).
     *
     * @tags Subject
     * @name SubjectShow
     * @summary Gets the subject matching the provided id (if the subject exists).
     * @request GET:/subject/{organization}/{namespace}/{name}
     */
    subjectShow: (
      organization: string,
      namespace: string,
      name: string,
      params: RequestParams = {}
    ) =>
      this.request<ResponsesSubject, ResponsesErrors>({
        path: `/subject/${organization}/${namespace}/${name}`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Groups the subjects by the specified metadata field and returns counts.
     *
     * @tags Subject
     * @name SubjectsByCount
     * @summary Groups the subjects by the specified metadata field and returns counts.
     * @request GET:/subject/by/{field}/count
     */
    subjectsByCount: (field: string, params: RequestParams = {}) =>
      this.request<ResponsesByCountSubjectResults, ResponsesErrors>({
        path: `/subject/by/${field}/count`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Reports summary information for the subjects known by this server.
     *
     * @tags Subject
     * @name SubjectSummary
     * @summary Reports summary information for the subjects known by this server.
     * @request GET:/subject/summary
     */
    subjectSummary: (params: RequestParams = {}) =>
      this.request<ResponsesSummary, any>({
        path: `/subject/summary`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
  sample = {
    /**
     * @description Gets the samples known by this server. ### Pagination This endpoint is paginated. Users may override the default pagination parameters by providing one or more of the pagination-related query parameters below. ### Filtering All harmonized (top-level) and unharmonized (nested under the `metadata.unharmonized` key) metadata fields are filterable. To achieve this, you can provide the field name as a [`String`]. Filtering follows the following rules: * For single-value metadata field, the sample is included in the results if its value _exactly_ matches the query string. Matches are case-sensitive. * For multiple-value metadata fields, the sample is included in the results if any of its values for the field _exactly_ match the query string (a logical OR [`||`]). Matches are case-sensitive. * When the metadata field is `null` (in the case of singlular or multiple-valued metadata fields) or empty, the sample is not included. * When multiple fields are provided as filters, a logical AND (`&&`) strings together the predicates. In other words, all filters must match for a sample to be returned. Note that this means that servers do not natively support logical OR (`||`) across multiple fields: that must be done by calling this endpoint with each of your desired queries and performing a set union of those samples out of band. ### Ordering This endpoint has default ordering requirements—those details are documented in the `responses::Samples` schema.
     *
     * @tags Sample
     * @name SampleIndex
     * @summary Gets the samples known by this server.
     * @request GET:/sample
     */
    sampleIndex: (
      query?: {
        /**
         * Matches any sample where the `disease_phase` field matches the string
         * provided.
         */
        disease_phase?: string;
        /**
         * Matches any sample where the `library_strategy` field matches the string
         * provided.
         */
        library_strategy?: string;
        /**
         * Matches any sample where the `preservation_method` field matches the string
         * provided.
         */
        preservation_method?: string;
        /**
         * Matches any sample where the `tissue_type` field matches the string
         * provided.
         */
        tissue_type?: string;
        /**
         * Matches any sample where the `tumor_classification` field matches the
         * string provided.
         */
        tumor_classification?: string;
        /**
         * Matches any sample where the `age_at_diagnosis` field matches the string
         * provided.
         */
        age_at_diagnosis?: string;
        /**
         * Matches any sample where the `age_at_collection` field matches the
         * string provided.
         */
        age_at_collection?: string;
        /**
         * Matches any sample where the `tumor_tissue_morphology` field matches the
         * string provided.
         */
        tumor_tissue_morphology?: string;
        /**
         * Matches any sample where any member of the `depositions` fields match
         * the string provided.
         *
         * **Note:** a logical OR (`||`) is performed across the values when
         * determining whether the sample should be included in the results.
         */
        depositions?: string;
        /**
         * All unharmonized fields should be filterable in the same manner as harmonized fields:
         *
         * * Filtering on a singular field should include the `Sample` in the results if the query exactly matches the value of that field for the `Sample` (case-sensitive).
         * * Filtering on field with multiple values should include the `Sample` in the results if the query exactly matches any of the values of the field for that `Sample` (case-sensitive).
         * * Unlike harmonized fields, unharmonized fields must be prefixed with `metadata.unharmonized`.
         *
         * **Note:** this query parameter is intended to be symbolic of any unharmonized field. Because of limitations within Swagger UI, it will show up as a query parameter that can be optionally be submitted as part of a request within Swagger UI. Please keep in mind that the literal query parameter `?metadata.unharmonized.<field>=value` is not supported, so attempting to use it within Swagger UI will not work!
         */
        metadataUnharmonizedField?: string;
        /**
         * The page to retrieve.
         *
         * This is a 1-based index of a page within a page set. The value of `page`
         * **must** default to `1` when this parameter is not provided.
         * @min 0
         */
        page?: number;
        /**
         * The number of results per page.
         *
         * Each server can select its own default value for `per_page` when this
         * parameter is not provided. That said, the convention within the
         * community is to use `100` as a default value if any value is equally
         * reasonable.
         * @min 0
         */
        per_page?: number;
      },
      params: RequestParams = {}
    ) =>
      this.request<ResponsesSamples, ResponsesErrors>({
        path: `/sample`,
        method: "GET",
        query: query,
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the sample matching the provided name (if the sample exists).
     *
     * @tags Sample
     * @name SampleShow
     * @summary Gets the sample matching the provided name (if the sample exists).
     * @request GET:/sample/{organization}/{namespace}/{name}
     */
    sampleShow: (
      organization: string,
      namespace: string,
      name: string,
      params: RequestParams = {}
    ) =>
      this.request<ResponsesSample, ResponsesErrors>({
        path: `/sample/${organization}/${namespace}/${name}`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Groups the samples by the specified metadata field and returns counts.
     *
     * @tags Sample
     * @name SamplesByCount
     * @summary Groups the samples by the specified metadata field and returns counts.
     * @request GET:/sample/by/{field}/count
     */
    samplesByCount: (field: string, params: RequestParams = {}) =>
      this.request<ResponsesByCountSampleResults, ResponsesErrors>({
        path: `/sample/by/${field}/count`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Reports summary information for the samples known by this server.
     *
     * @tags Sample
     * @name SampleSummary
     * @summary Reports summary information for the samples known by this server.
     * @request GET:/sample/summary
     */
    sampleSummary: (params: RequestParams = {}) =>
      this.request<ResponsesSummary, any>({
        path: `/sample/summary`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
  file = {
    /**
     * @description Gets the files known by this server. ### Pagination This endpoint is paginated. Users may override the default pagination parameters by providing one or more of the pagination-related query parameters below. ### Filtering All harmonized (top-level) and unharmonized (nested under the `metadata.unharmonized` key) metadata fields are filterable. To achieve this, you can provide the field name as a [`String`]. Filtering follows the following rules: * For single-value metadata field, the file is included in the results if its value _exactly_ matches the query string. Matches are case-sensitive. * For multiple-value metadata fields, the file is included in the results if any of its values for the field _exactly_ match the query string (a logical OR [`||`]). Matches are case-sensitive. * When the metadata field is `null` (in the case of singlular or multiple-valued metadata fields) or empty, the file is not included. * When multiple fields are provided as filters, a logical AND (`&&`) strings together the predicates. In other words, all filters must match for a file to be returned. Note that this means that servers do not natively support logical OR (`||`) across multiple fields: that must be done by calling this endpoint with each of your desired queries and performing a set union of those files out of band. ### Ordering This endpoint has default ordering requirements—those details are documented in the `responses::Files` schema.
     *
     * @tags File
     * @name FileIndex
     * @summary Gets the files known by this server.
     * @request GET:/file
     */
    fileIndex: (
      query?: {
        /** Matches any file where the `type` field matches the string provided. */
        type?: string;
        /** Matches any file where the `size` field matches the string provided. */
        size?: string;
        /**
         * Matches any file where the `checksums` field matches the string
         * provided.
         *
         * **Note:** a logical OR (`||`) is performed across the values when
         * determining whether the file should be included in the results.
         */
        checksums?: string;
        /**
         * Matches any file where the `description` field matches the string
         * provided.
         *
         * **Note:** a file is returned if the value provided is a substring of the
         * description.
         */
        description?: string;
        /**
         * Matches any file where any member of the `depositions` fields match
         * the string provided.
         *
         * **Note:** a logical OR (`||`) is performed across the values when
         * determining whether the sample should be included in the results.
         */
        depositions?: string;
        /**
         * All unharmonized fields should be filterable in the same manner as harmonized fields:
         *
         * * Filtering on a singular field should include the `File` in the results if the query exactly matches the value of that field for the `File` (case-sensitive).
         * * Filtering on field with multiple values should include the `File` in the results if the query exactly matches any of the values of the field for that `File` (case-sensitive).
         * * Unlike harmonized fields, unharmonized fields must be prefixed with `metadata.unharmonized`.
         *
         * **Note:** this query parameter is intended to be symbolic of any unharmonized field. Because of limitations within Swagger UI, it will show up as a query parameter that can be optionally be submitted as part of a request within Swagger UI. Please keep in mind that the literal query parameter `?metadata.unharmonized.<field>=value` is not supported, so attempting to use it within Swagger UI will not work!
         */
        metadataUnharmonizedField?: string;
        /**
         * The page to retrieve.
         *
         * This is a 1-based index of a page within a page set. The value of `page`
         * **must** default to `1` when this parameter is not provided.
         * @min 0
         */
        page?: number;
        /**
         * The number of results per page.
         *
         * Each server can select its own default value for `per_page` when this
         * parameter is not provided. That said, the convention within the
         * community is to use `100` as a default value if any value is equally
         * reasonable.
         * @min 0
         */
        per_page?: number;
      },
      params: RequestParams = {}
    ) =>
      this.request<ResponsesFiles, ResponsesErrors>({
        path: `/file`,
        method: "GET",
        query: query,
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the file matching the provided name (if the file exists).
     *
     * @tags File
     * @name FileShow
     * @summary Gets the file matching the provided name (if the file exists).
     * @request GET:/file/{organization}/{namespace}/{name}
     */
    fileShow: (
      organization: string,
      namespace: string,
      name: string,
      params: RequestParams = {}
    ) =>
      this.request<ResponsesFile, ResponsesErrors>({
        path: `/file/${organization}/${namespace}/${name}`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Groups the files by the specified metadata field and returns counts.
     *
     * @tags File
     * @name FilesByCount
     * @summary Groups the files by the specified metadata field and returns counts.
     * @request GET:/file/by/{field}/count
     */
    filesByCount: (field: string, params: RequestParams = {}) =>
      this.request<ResponsesByCountFileResults, ResponsesErrors>({
        path: `/file/by/${field}/count`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Reports summary information for the files known by this server.
     *
     * @tags File
     * @name FileSummary
     * @summary Reports summary information for the files known by this server.
     * @request GET:/file/summary
     */
    fileSummary: (params: RequestParams = {}) =>
      this.request<ResponsesSummary, any>({
        path: `/file/summary`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
  metadata = {
    /**
     * @description Gets the metadata fields for subjects that are supported by this server.
     *
     * @tags Metadata
     * @name MetadataFieldsSubject
     * @summary Gets the metadata fields for subjects that are supported by this server.
     * @request GET:/metadata/fields/subject
     */
    metadataFieldsSubject: (params: RequestParams = {}) =>
      this.request<ResponsesMetadataFieldDescriptions, any>({
        path: `/metadata/fields/subject`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the metadata fields for samples that are supported by this server.
     *
     * @tags Metadata
     * @name MetadataFieldsSample
     * @summary Gets the metadata fields for samples that are supported by this server.
     * @request GET:/metadata/fields/sample
     */
    metadataFieldsSample: (params: RequestParams = {}) =>
      this.request<ResponsesMetadataFieldDescriptions, any>({
        path: `/metadata/fields/sample`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the metadata fields for files that are supported by this server.
     *
     * @tags Metadata
     * @name MetadataFieldsFile
     * @summary Gets the metadata fields for files that are supported by this server.
     * @request GET:/metadata/fields/file
     */
    metadataFieldsFile: (params: RequestParams = {}) =>
      this.request<ResponsesMetadataFieldDescriptions, any>({
        path: `/metadata/fields/file`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
  namespace = {
    /**
     * @description Gets the namespaces known by this server.
     *
     * @tags Namespace
     * @name NamespaceIndex
     * @summary Gets the namespaces known by this server.
     * @request GET:/namespace
     */
    namespaceIndex: (params: RequestParams = {}) =>
      this.request<ResponsesNamespaces, any>({
        path: `/namespace`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the namespace matching the provided name (if it exists).
     *
     * @tags Namespace
     * @name NamespaceShow
     * @summary Gets the namespace matching the provided name (if it exists).
     * @request GET:/namespace/{organization}/{namespace}
     */
    namespaceShow: (
      organization: string,
      namespace: string,
      params: RequestParams = {}
    ) =>
      this.request<ResponsesNamespace, ResponsesErrors>({
        path: `/namespace/${organization}/${namespace}`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
  organization = {
    /**
     * @description Gets the organizations known by this server.
     *
     * @tags Organization
     * @name OrganizationIndex
     * @summary Gets the organizations known by this server.
     * @request GET:/organization
     */
    organizationIndex: (params: RequestParams = {}) =>
      this.request<ResponsesOrganizations, any>({
        path: `/organization`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * @description Gets the organization matching the provided name (if it exists).
     *
     * @tags Organization
     * @name OrganizationShow
     * @summary Gets the organization matching the provided name (if it exists).
     * @request GET:/organization/{name}
     */
    organizationShow: (name: string, params: RequestParams = {}) =>
      this.request<ResponsesOrganization, ResponsesErrors>({
        path: `/organization/${name}`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
  info = {
    /**
     * @description Gets the info for this server.
     *
     * @tags Info
     * @name InfoIndex
     * @summary Gets the info for this server.
     * @request GET:/info
     */
    infoIndex: (params: RequestParams = {}) =>
      this.request<ResponsesInformation, any>({
        path: `/info`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
}
