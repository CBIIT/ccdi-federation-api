use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 12960571 v1.00`**
///
/// This metadata element is defined by the caDSR as "A sequence of characters
/// used to identify, name, or characterize a pediatric study.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12960571%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::namespace::StudyId)]
#[allow(non_camel_case_types)]
pub enum StudyId {
    /// `AALL0232`
    ///
    /// * **VM Long Name**: AALL0232 ALL Study Identifier
    /// * **VM Public ID**: 13041454
    /// * **Concept Code**: C178065
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AALL0232 assigned to a study in an ALL clinical trial.
    AALL0232,

    /// `AALL0331`
    ///
    /// * **VM Long Name**: AALL0331 ALL Study Identifier
    /// * **VM Public ID**: 13041453
    /// * **Concept Code**: C178095
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AALL0331 assigned to a study in an ALL clinical trial.
    AALL0331,

    /// `AALL03B1`
    ///
    /// * **VM Long Name**: AALL03B1 ALL Study Identifier
    /// * **VM Public ID**: 13041457
    /// * **Concept Code**: C178068
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AALL03B1 assigned to a study in an ALL clinical trial.
    AALL03B1,

    /// `AALL0434`
    ///
    /// * **VM Long Name**: AALL0434 ALL Study Identifier
    /// * **VM Public ID**: 13041455
    /// * **Concept Code**: C178066
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AALL0434 assigned to a study in an ALL clinical trial.
    AALL0434,

    /// `AALL08B1`
    ///
    /// * **VM Long Name**: AALL08B1 ALL Study Identifier
    /// * **VM Public ID**: 13041456
    /// * **Concept Code**: C178067
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AALL08B1 assigned to a study in an ALL clinical trial.
    AALL08B1,

    /// `AAML03P1`
    ///
    /// * **VM Long Name**: AAML03P1 AML Study Identifier
    /// * **VM Public ID**: 13029822
    /// * **Concept Code**: C168936
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AAML03P1, assigned to a study in an AML clinical trial.
    AAML03P1,

    /// `AAML0531`
    ///
    /// * **VM Long Name**: AAML0531 AML Study Identifier
    /// * **VM Public ID**: 13029823
    /// * **Concept Code**: C168937
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AAML0531, assigned to an AML study in a clinical trial.
    AAML0531,

    /// `AAML1031`
    ///
    /// * **VM Long Name**: AAML1031 AML Study Identifier
    /// * **VM Public ID**: 13029824
    /// * **Concept Code**: C168938
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AAML1031, assigned to a study in an AML clinical trial.
    AAML1031,

    /// `AEIOPAML2002`
    ///
    /// * **VM Long Name**: AEIOPLAM2002 AML Study Identifier
    /// * **VM Public ID**: 13036706
    /// * **Concept Code**: C168942
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AEIOPLAM2002, assigned to a study in an AML clinical
    /// trial.
    AEIOPAML2002,

    /// `AEWS0031`
    ///
    /// * **VM Long Name**: AEWS0031 EWS Study Identifier
    /// * **VM Public ID**: 13039357
    /// * **Concept Code**: C174970
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AEWS0031, assigned to a study in an EWS clinical trial.
    AEWS0031,

    /// `AEWS0331`
    ///
    /// * **VM Long Name**: AEWS0331 EWS Study Identifier
    /// * **VM Public ID**: 13039356
    /// * **Concept Code**: C174969
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AEWS0331, assigned to a study in an EWS clinical trial.
    AEWS0331,

    /// `AEWS07P1`
    ///
    /// * **VM Long Name**: AEWS07P1 EWS Study Identifier
    /// * **VM Public ID**: 13039361
    /// * **Concept Code**: C174974
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AEWS07P1, assigned to a study in an EWS clinical trial.
    AEWS07P1,

    /// `AEWS1031`
    ///
    /// * **VM Long Name**: AEWS1031 EWS Study Identifier
    /// * **VM Public ID**: 13039358
    /// * **Concept Code**: C174971
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AEWS1031, assigned to a study in an EWS clinical trial.
    AEWS1031,

    /// `AEWS1221`
    ///
    /// * **VM Long Name**: AEWS1221 EWS Study Identifier
    /// * **VM Public ID**: 13039355
    /// * **Concept Code**: C174968
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AEWS1221, assigned to a study in an EWS clinical trial.
    AEWS1221,

    /// `AGCT0132`
    ///
    /// * **VM Long Name**: AGCT0132 GCT Study Identifier
    /// * **VM Public ID**: 13049604
    /// * **Concept Code**: C177343
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AGCT0132, assigned to a study in an GCT clinical trial.
    AGCT0132,

    /// `AGCT01P1`
    ///
    /// * **VM Long Name**: AGCT01P1 GCT Study Identifier
    /// * **VM Public ID**: 13049603
    /// * **Concept Code**: C177342
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AGCT01P1, assigned to a study in an GCT clinical trial.
    AGCT01P1,

    /// `AGCT0521`
    ///
    /// * **VM Long Name**: AGCT0521 GCT Study Identifier
    /// * **VM Public ID**: 13049605
    /// * **Concept Code**: C177344
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AGCT0521, assigned to a study in an GCT clinical trial.
    AGCT0521,

    /// `AHOD0031`
    ///
    /// * **VM Long Name**: AHOD0031 HL Study Identifier
    /// * **VM Public ID**: 13036715
    /// * **Concept Code**: C185311
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AHOD0031, assigned to a study in a HL clinical trial.
    AHOD0031,

    /// `AHOD03P1`
    ///
    /// * **VM Long Name**: AHOD03P1 HL Study Identifier
    /// * **VM Public ID**: 13039354
    /// * **Concept Code**: C185314
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AHOD03P1, assigned to a study in a HL clinical trial.
    AHOD03P1,

    /// `AHOD0431`
    ///
    /// * **VM Long Name**: AHOD0431 HL Study Identifier
    /// * **VM Public ID**: 13036714
    /// * **Concept Code**: C185310
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AHOD0431, assigned to a study in a HL clinical trial.
    AHOD0431,

    /// `AHOD0831`
    ///
    /// * **VM Long Name**: AHOD0831 HL Study Identifier
    /// * **VM Public ID**: 13036713
    /// * **Concept Code**: C185308
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AHOD0831, assigned to a study in a HL clinical trial.
    AHOD0831,

    /// `AHOD1221`
    ///
    /// * **VM Long Name**: AHOD1221 HL Study Identifier
    /// * **VM Public ID**: 13039353
    /// * **Concept Code**: C185313
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AHOD1221, assigned to a study in a HL clinical trial.
    AHOD1221,

    /// `AHOD1331`
    ///
    /// * **VM Long Name**: AHOD1331 HL Study Identifier
    /// * **VM Public ID**: 13039352
    /// * **Concept Code**: C185312
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AHOD1331, assigned to a study in a HL clinical trial.
    AHOD1331,

    /// `AIEOPLAM92`
    ///
    /// * **VM Long Name**: AIEOPLAM92 AML Study Identifier
    /// * **VM Public ID**: 13056484
    /// * **Concept Code**: C173254
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AIEOPLAM92, assigned to a study in an AML clinical trial.
    AIEOPLAM92,

    /// `AMLBFM-Registry2012`
    ///
    /// * **VM Long Name**: AMLBFMRegistry2012 AML Study Identifier
    /// * **VM Public ID**: 13029827
    /// * **Concept Code**: C173251
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AMLBFMRegistry2012, assigned to a study in an AML
    /// clinical trial.
    #[serde(rename = "AMLBFM-Registry2012")]
    AMLBFM_Registry2012,

    /// `AMLBFM1998`
    ///
    /// * **VM Long Name**: AMLBFM1998 AML Study Identifier
    /// * **VM Public ID**: 13029829
    /// * **Concept Code**: C182032
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AMLBFM1998, assigned to a study in an AML clinical trial.
    AMLBFM1998,

    /// `AMLBFM2004`
    ///
    /// * **VM Long Name**: AMLBFM2004 AML Study Identifier
    /// * **VM Public ID**: 13029825
    /// * **Concept Code**: C168939
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AMLBFM2004, assigned to a study in an AML clinical trial.
    AMLBFM2004,

    /// `AMLBFM2012`
    ///
    /// * **VM Long Name**: AMLBFM2012 AML Study Identifier
    /// * **VM Public ID**: 13029826
    /// * **Concept Code**: C173250
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AMLBFM2012, assigned to a study in an AML clinical trial.
    AMLBFM2012,

    /// `AMLBFMRegistry2017`
    ///
    /// * **VM Long Name**: AMLBFMRegistry2017 AML Study Identifier
    /// * **VM Public ID**: 13029828
    /// * **Concept Code**: C182031
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AMLBFMRegistry2017, assigned to a study in an AML
    /// clinical trial.
    AMLBFMRegistry2017,

    /// `AOST0121`
    ///
    /// * **VM Long Name**: AOST0121 OS Study Identifier
    /// * **VM Public ID**: 13054330
    /// * **Concept Code**: C180358
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AOST0121 assigned to a study in an OS clinical trial.
    AOST0121,

    /// `AOST01P1`
    ///
    /// * **VM Long Name**: AOST01P1 OS Study Identifier
    /// * **VM Public ID**: 13054331
    /// * **Concept Code**: C180359
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AOST01P1 assigned to a study in an OS clinical trial.
    AOST01P1,

    /// `AOST0221`
    ///
    /// * **VM Long Name**: AOST0221 OS Study Identifier
    /// * **VM Public ID**: 13054329
    /// * **Concept Code**: C180360
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AOST0221 assigned to a study in an OS clinical trial.
    AOST0221,

    /// `AOST0331/EURAMOS1`
    ///
    /// * **VM Long Name**: AOST0331/EURAMOS1 OS Study Identifier
    /// * **VM Public ID**: 13052436
    /// * **Concept Code**: C180361
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AOST0331/EURAMOS1 assigned to a study in an OS clinical
    /// trial.
    #[serde(rename = "AOST0331/EURAMOS1")]
    AOST0331_EURAMOS1,

    /// `AOST1321`
    ///
    /// * **VM Long Name**: AOST1321 OS Study Identifier
    /// * **VM Public ID**: 13054332
    /// * **Concept Code**: C180362
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AOST1321 assigned to a study in an OS clinical trial.
    AOST1321,

    /// `AOST1421`
    ///
    /// * **VM Long Name**: AOST1421 OS Study Identifier
    /// * **VM Public ID**: 13054708
    /// * **Concept Code**: C180363
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier AOST1421 assigned to a study in an OS clinical trial.
    AOST1421,

    /// `CCG-782`
    ///
    /// * **VM Long Name**: CCG-782 OS Study Identifier
    /// * **VM Public ID**: 13054710
    /// * **Concept Code**: C180364
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier CCG-782 assigned to a study in an OS clinical trial.
    #[serde(rename = "CCG-782")]
    CCG_782,

    /// `CCG-7942`
    ///
    /// * **VM Long Name**: CCG-7942 OS Study Identifier
    /// * **VM Public ID**: 13054709
    /// * **Concept Code**: C180365
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier CCG-7942 assigned to a study in an OS clinical trial.
    #[serde(rename = "CCG-7942")]
    CCG_7942,

    /// `DBAML01`
    ///
    /// * **VM Long Name**: DBAML01 AML Study Identifier
    /// * **VM Public ID**: 13036708
    /// * **Concept Code**: C168944
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier DBAML01, assigned to a study in an AML clinical trial.
    DBAML01,

    /// `EE99`
    ///
    /// * **VM Long Name**: EE99 EWS Study Identifier
    /// * **VM Public ID**: 13039359
    /// * **Concept Code**: C174972
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier EE99, assigned to a study in an EWS clinical trial.
    EE99,

    /// `EICESS92`
    ///
    /// * **VM Long Name**: EICESS92 EWS Study Identifier
    /// * **VM Public ID**: 13039360
    /// * **Concept Code**: C174973
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier EICESS92, assigned to a study in an EWS clinical trial.
    EICESS92,

    /// `GC1`
    ///
    /// * **VM Long Name**: OLFM4 wt Allele
    /// * **VM Public ID**: 13041461
    /// * **Concept Code**: C113593
    /// * **Begin Date**:   03/08/2023
    ///
    /// Human OLFM4 wild-type allele is located in the vicinity of 13q14.3 and
    /// is approximately 23 kb in length. This allele, which encodes
    /// olfactomedin-4 protein, plays a role in apoptosis inhibition.
    GC1,

    /// `GC2`
    ///
    /// * **VM Long Name**: GC2 GCT Study Identifier
    /// * **VM Public ID**: 13041462
    /// * **Concept Code**: C177336
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier GC2, assigned to a study in an GCT clinical trial.
    GC2,

    /// `GOG0078`
    ///
    /// * **VM Long Name**: GOG0078 GCT Study Identifier
    /// * **VM Public ID**: 13049598
    /// * **Concept Code**: C177337
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier GOG0078, assigned to a study in an GCT clinical trial.
    GOG0078,

    /// `GOG0090`
    ///
    /// * **VM Long Name**: GOG0090 GCT Study Identifier
    /// * **VM Public ID**: 13049599
    /// * **Concept Code**: C177338
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier GOG0090, assigned to a study in an GCT clinical trial.
    GOG0090,

    /// `GOG0116`
    ///
    /// * **VM Long Name**: GOG0116 GCT Study Identifier
    /// * **VM Public ID**: 13049600
    /// * **Concept Code**: C177339
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier GOG0116, assigned to a study in an GCT clinical trial.
    GOG0116,

    /// `INT133`
    ///
    /// * **VM Long Name**: INT133 OS Study Identifier
    /// * **VM Public ID**: 13054328
    /// * **Concept Code**: C180366
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier INT133 assigned to a study in an OS clinical trial.
    INT133,

    /// `JACLSAML99`
    ///
    /// * **VM Long Name**: JACLSAML99 AML Study Identifier
    /// * **VM Public ID**: 13036707
    /// * **Concept Code**: C168943
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier JACLSAML99, assigned to a study in an AML clinical trial.
    JACLSAML99,

    /// `JPLSGAML05`
    ///
    /// * **VM Long Name**: JPLSGAML05 AML Study Identifier
    /// * **VM Public ID**: 13029831
    /// * **Concept Code**: C168941
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier JPLSGAML05, assigned to a study in an AML clinical trial.
    JPLSGAML05,

    /// `MRCAML12`
    ///
    /// * **VM Long Name**: MRCAML12 AML Study Identifier
    /// * **VM Public ID**: 13036709
    /// * **Concept Code**: C168945
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier MRCAML12, assigned to a study in an AML clinical trial.
    MRCAML12,

    /// `MRCAML15`
    ///
    /// * **VM Long Name**: MRCAML15 AML Study Identifier
    /// * **VM Public ID**: 13036710
    /// * **Concept Code**: C173252
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier MRCAML15, assigned to a study in an AML clinical trial.
    MRCAML15,

    /// `NOPHOAML2004`
    ///
    /// * **VM Long Name**: NOPHOAML2004 AML Study Identifier
    /// * **VM Public ID**: 13036711
    /// * **Concept Code**: C168946
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier NOPHOAML2004, assigned to a study in an AML clinical
    /// trial.
    NOPHOAML2004,

    /// `NOPHOAML2012`
    ///
    /// * **VM Long Name**: NOPHOAML2012 AML Study Identifier
    /// * **VM Public ID**: 13036712
    /// * **Concept Code**: C173253
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier NOPHOAML2012, assigned to a study in an AML clinical
    /// trial.
    NOPHOAML2012,

    /// `OS2006`
    ///
    /// * **VM Long Name**: OS2006 OS Study Identifier
    /// * **VM Public ID**: 13052438
    /// * **Concept Code**: C180367
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier OS2006 assigned to a study in an OS clinical trial.
    OS2006,

    /// `P9749`
    ///
    /// * **VM Long Name**: P9749 GCT Study Identifier
    /// * **VM Public ID**: 13049601
    /// * **Concept Code**: C177340
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier P9749, assigned to a study in an GCT clinical trial.
    P9749,

    /// `P9754`
    ///
    /// * **VM Long Name**: P9754 OS Study Identifier
    /// * **VM Public ID**: 13054327
    /// * **Concept Code**: C180368
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier P9754 assigned to a study in an OS clinical trial.
    P9754,

    /// `POG9049`
    ///
    /// * **VM Long Name**: POG9049 GCT Study Identifier
    /// * **VM Public ID**: 13049602
    /// * **Concept Code**: C177341
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier POG9049, assigned to a study in an GCT clinical trial.
    POG9049,

    /// `PPLLSGAML98`
    ///
    /// * **VM Long Name**: PPLLSGAML98 AML Study Identifier
    /// * **VM Public ID**: 13056483
    /// * **Concept Code**: C168947
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier PPLLSGAML98, assigned to a study in an AML clinical
    /// trial.
    PPLLSGAML98,

    /// `REGOBONE`
    ///
    /// * **VM Long Name**: REGOBONE OS Study Identifier
    /// * **VM Public ID**: 13054326
    /// * **Concept Code**: C180369
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier REGOBONE assigned to a study in an OS clinical trial.
    REGOBONE,

    /// `Sarcome13/OS2016`
    ///
    /// * **VM Long Name**: Sarcome13/OS2016 OS Study Identifier
    /// * **VM Public ID**: 13052437
    /// * **Concept Code**: C180370
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier Sarcome13/OS2016 assigned to a study in an OS clinical
    /// trial.
    #[serde(rename = "Sarcome13/OS2016")]
    Sarcome13_OS2016,

    /// `SCFEELAM02`
    ///
    /// * **VM Long Name**: SCFEELAM02 AML Study Identifier
    /// * **VM Public ID**: 13056485
    /// * **Concept Code**: C173255
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier SCFEELAM02, assigned to a study in an AML clinical trial.
    SCFEELAM02,

    /// `SJCRHAML02`
    ///
    /// * **VM Long Name**: SJCRHAML02 AML Study Identifier
    /// * **VM Public ID**: 13029830
    /// * **Concept Code**: C168940
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier SJCRHAML02, assigned to a study in an AML clinical trial.
    SJCRHAML02,

    /// `TCGM2004`
    ///
    /// * **VM Long Name**: TCGM2004 Germ Cell Tumor Study Identifier
    /// * **VM Public ID**: 13052435
    /// * **Concept Code**: C187201
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TCGM2004, assigned to a study in an GCT clinical trial.
    TCGM2004,

    /// `TE04`
    ///
    /// * **VM Long Name**: TE04
    /// * **VM Public ID**: 13049606
    /// * **Concept Code**: C20299
    /// * **Begin Date**:   03/08/2023
    ///
    /// Provider: Technion University, Haifa, Israel. No information from
    /// provider. This cell line meets the criteria for the use of human
    /// embryonic stem cells by federally funded researchers.
    TE04,

    /// `TE05`
    ///
    /// * **VM Long Name**: TE05 GCT Study Identifier
    /// * **VM Public ID**: 13049607
    /// * **Concept Code**: C177346
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TE05, assigned to a study in an GCT clinical trial.
    TE05,

    /// `TE08`
    ///
    /// * **VM Long Name**: TE08 GCT Study Identifier
    /// * **VM Public ID**: 13052430
    /// * **Concept Code**: C177347
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TE08, assigned to a study in an GCT clinical trial.
    TE08,

    /// `TE09`
    ///
    /// * **VM Long Name**: TE09 GCT Study Identifier
    /// * **VM Public ID**: 13052432
    /// * **Concept Code**: C177349
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TE09, assigned to a study in an GCT clinical trial.
    TE09,

    /// `TE13`
    ///
    /// * **VM Long Name**: TE13 GCT Study Identifier
    /// * **VM Public ID**: 13052433
    /// * **Concept Code**: C177350
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TE13, assigned to a study in an GCT clinical trial.
    TE13,

    /// `TE20`
    ///
    /// * **VM Long Name**: TE20 GCT Study Identifier
    /// * **VM Public ID**: 13052434
    /// * **Concept Code**: C177351
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TE20, assigned to a study in an GCT clinical trial.
    TE20,

    /// `TE22`
    ///
    /// * **VM Long Name**: TE22 GCT Study Identifier
    /// * **VM Public ID**: 13052431
    /// * **Concept Code**: C177348
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TE22, assigned to a study in an GCT clinical trial.
    TE22,

    /// `TGM85`
    ///
    /// * **VM Long Name**: TGM85 GCT Study Identifier
    /// * **VM Public ID**: 13041458
    /// * **Concept Code**: C177332
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TGM85, assigned to a study in an GCT clinical trial.
    TGM85,

    /// `TGM90`
    ///
    /// * **VM Long Name**: TGM90 GCT Study Identifier
    /// * **VM Public ID**: 13041459
    /// * **Concept Code**: C177333
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TGM90, assigned to a study in an GCT clinical trial.
    TGM90,

    /// `TGM95`
    ///
    /// * **VM Long Name**: TGM95 GCT Study Identifier
    /// * **VM Public ID**: 13041460
    /// * **Concept Code**: C177334
    /// * **Begin Date**:   03/08/2023
    ///
    /// The identifier TGM95, assigned to a study in an GCT clinical trial.
    TGM95,

    /// `TIP`
    ///
    /// * **VM Long Name**: Tip
    /// * **VM Public ID**: 6380282
    /// * **Concept Code**: C90069
    /// * **Begin Date**:   03/08/2023
    ///
    /// The pointed end of a linear structure.
    TIP,
}

impl CDE for StudyId {}

impl std::fmt::Display for StudyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudyId::AALL0232 => write!(f, "AALL0232"),
            StudyId::AALL0331 => write!(f, "AALL0331"),
            StudyId::AALL03B1 => write!(f, "AALL03B1"),
            StudyId::AALL0434 => write!(f, "AALL0434"),
            StudyId::AALL08B1 => write!(f, "AALL08B1"),
            StudyId::AAML03P1 => write!(f, "AAML03P1"),
            StudyId::AAML0531 => write!(f, "AAML0531"),
            StudyId::AAML1031 => write!(f, "AAML1031"),
            StudyId::AEIOPAML2002 => write!(f, "AEIOPAML2002"),
            StudyId::AEWS0031 => write!(f, "AEWS0031"),
            StudyId::AEWS0331 => write!(f, "AEWS0331"),
            StudyId::AEWS07P1 => write!(f, "AEWS07P1"),
            StudyId::AEWS1031 => write!(f, "AEWS1031"),
            StudyId::AEWS1221 => write!(f, "AEWS1221"),
            StudyId::AGCT0132 => write!(f, "AGCT0132"),
            StudyId::AGCT01P1 => write!(f, "AGCT01P1"),
            StudyId::AGCT0521 => write!(f, "AGCT0521"),
            StudyId::AHOD0031 => write!(f, "AHOD0031"),
            StudyId::AHOD03P1 => write!(f, "AHOD03P1"),
            StudyId::AHOD0431 => write!(f, "AHOD0431"),
            StudyId::AHOD0831 => write!(f, "AHOD0831"),
            StudyId::AHOD1221 => write!(f, "AHOD1221"),
            StudyId::AHOD1331 => write!(f, "AHOD1331"),
            StudyId::AIEOPLAM92 => write!(f, "AIEOPLAM92"),
            StudyId::AMLBFM_Registry2012 => write!(f, "AMLBFM-Registry2012"),
            StudyId::AMLBFM1998 => write!(f, "AMLBFM1998"),
            StudyId::AMLBFM2004 => write!(f, "AMLBFM2004"),
            StudyId::AMLBFM2012 => write!(f, "AMLBFM2012"),
            StudyId::AMLBFMRegistry2017 => write!(f, "AMLBFMRegistry2017"),
            StudyId::AOST0121 => write!(f, "AOST0121"),
            StudyId::AOST01P1 => write!(f, "AOST01P1"),
            StudyId::AOST0221 => write!(f, "AOST0221"),
            StudyId::AOST0331_EURAMOS1 => write!(f, "AOST0331/EURAMOS1"),
            StudyId::AOST1321 => write!(f, "AOST1321"),
            StudyId::AOST1421 => write!(f, "AOST1421"),
            StudyId::CCG_782 => write!(f, "CCG-782"),
            StudyId::CCG_7942 => write!(f, "CCG-7942"),
            StudyId::DBAML01 => write!(f, "DBAML01"),
            StudyId::EE99 => write!(f, "EE99"),
            StudyId::EICESS92 => write!(f, "EICESS92"),
            StudyId::GC1 => write!(f, "GC1"),
            StudyId::GC2 => write!(f, "GC2"),
            StudyId::GOG0078 => write!(f, "GOG0078"),
            StudyId::GOG0090 => write!(f, "GOG0090"),
            StudyId::GOG0116 => write!(f, "GOG0116"),
            StudyId::INT133 => write!(f, "INT133"),
            StudyId::JACLSAML99 => write!(f, "JACLSAML99"),
            StudyId::JPLSGAML05 => write!(f, "JPLSGAML05"),
            StudyId::MRCAML12 => write!(f, "MRCAML12"),
            StudyId::MRCAML15 => write!(f, "MRCAML15"),
            StudyId::NOPHOAML2004 => write!(f, "NOPHOAML2004"),
            StudyId::NOPHOAML2012 => write!(f, "NOPHOAML2012"),
            StudyId::OS2006 => write!(f, "OS2006"),
            StudyId::P9749 => write!(f, "P9749"),
            StudyId::P9754 => write!(f, "P9754"),
            StudyId::POG9049 => write!(f, "POG9049"),
            StudyId::PPLLSGAML98 => write!(f, "PPLLSGAML98"),
            StudyId::REGOBONE => write!(f, "REGOBONE"),
            StudyId::Sarcome13_OS2016 => write!(f, "Sarcome13/OS2016"),
            StudyId::SCFEELAM02 => write!(f, "SCFEELAM02"),
            StudyId::SJCRHAML02 => write!(f, "SJCRHAML02"),
            StudyId::TCGM2004 => write!(f, "TCGM2004"),
            StudyId::TE04 => write!(f, "TE04"),
            StudyId::TE05 => write!(f, "TE05"),
            StudyId::TE08 => write!(f, "TE08"),
            StudyId::TE09 => write!(f, "TE09"),
            StudyId::TE13 => write!(f, "TE13"),
            StudyId::TE20 => write!(f, "TE20"),
            StudyId::TE22 => write!(f, "TE22"),
            StudyId::TGM85 => write!(f, "TGM85"),
            StudyId::TGM90 => write!(f, "TGM90"),
            StudyId::TGM95 => write!(f, "TGM95"),
            StudyId::TIP => write!(f, "TIP"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(StudyId::AALL0232.to_string(), String::from("AALL0232"));
        assert_eq!(StudyId::AALL0331.to_string(), String::from("AALL0331"));
        assert_eq!(StudyId::AALL03B1.to_string(), String::from("AALL03B1"));
        assert_eq!(StudyId::AALL0434.to_string(), String::from("AALL0434"));
        assert_eq!(StudyId::AALL08B1.to_string(), String::from("AALL08B1"));
        assert_eq!(StudyId::AAML03P1.to_string(), String::from("AAML03P1"));
        assert_eq!(StudyId::AAML0531.to_string(), String::from("AAML0531"));
        assert_eq!(StudyId::AAML1031.to_string(), String::from("AAML1031"));
        assert_eq!(
            StudyId::AEIOPAML2002.to_string(),
            String::from("AEIOPAML2002")
        );
        assert_eq!(StudyId::AEWS0031.to_string(), String::from("AEWS0031"));
        assert_eq!(StudyId::AEWS0331.to_string(), String::from("AEWS0331"));
        assert_eq!(StudyId::AEWS07P1.to_string(), String::from("AEWS07P1"));
        assert_eq!(StudyId::AEWS1031.to_string(), String::from("AEWS1031"));
        assert_eq!(StudyId::AEWS1221.to_string(), String::from("AEWS1221"));
        assert_eq!(StudyId::AGCT0132.to_string(), String::from("AGCT0132"));
        assert_eq!(StudyId::AGCT01P1.to_string(), String::from("AGCT01P1"));
        assert_eq!(StudyId::AGCT0521.to_string(), String::from("AGCT0521"));
        assert_eq!(StudyId::AHOD0031.to_string(), String::from("AHOD0031"));
        assert_eq!(StudyId::AHOD03P1.to_string(), String::from("AHOD03P1"));
        assert_eq!(StudyId::AHOD0431.to_string(), String::from("AHOD0431"));
        assert_eq!(StudyId::AHOD0831.to_string(), String::from("AHOD0831"));
        assert_eq!(StudyId::AHOD1221.to_string(), String::from("AHOD1221"));
        assert_eq!(StudyId::AHOD1331.to_string(), String::from("AHOD1331"));
        assert_eq!(StudyId::AIEOPLAM92.to_string(), String::from("AIEOPLAM92"));
        assert_eq!(
            StudyId::AMLBFM_Registry2012.to_string(),
            String::from("AMLBFM-Registry2012")
        );
        assert_eq!(StudyId::AMLBFM1998.to_string(), String::from("AMLBFM1998"));
        assert_eq!(StudyId::AMLBFM2004.to_string(), String::from("AMLBFM2004"));
        assert_eq!(StudyId::AMLBFM2012.to_string(), String::from("AMLBFM2012"));
        assert_eq!(
            StudyId::AMLBFMRegistry2017.to_string(),
            String::from("AMLBFMRegistry2017")
        );
        assert_eq!(StudyId::AOST0121.to_string(), String::from("AOST0121"));
        assert_eq!(StudyId::AOST01P1.to_string(), String::from("AOST01P1"));
        assert_eq!(StudyId::AOST0221.to_string(), String::from("AOST0221"));
        assert_eq!(
            StudyId::AOST0331_EURAMOS1.to_string(),
            String::from("AOST0331/EURAMOS1")
        );
        assert_eq!(StudyId::AOST1321.to_string(), String::from("AOST1321"));
        assert_eq!(StudyId::AOST1421.to_string(), String::from("AOST1421"));
        assert_eq!(StudyId::CCG_782.to_string(), String::from("CCG-782"));
        assert_eq!(StudyId::CCG_7942.to_string(), String::from("CCG-7942"));
        assert_eq!(StudyId::DBAML01.to_string(), String::from("DBAML01"));
        assert_eq!(StudyId::EE99.to_string(), String::from("EE99"));
        assert_eq!(StudyId::EICESS92.to_string(), String::from("EICESS92"));
        assert_eq!(StudyId::GC1.to_string(), String::from("GC1"));
        assert_eq!(StudyId::GC2.to_string(), String::from("GC2"));
        assert_eq!(StudyId::GOG0078.to_string(), String::from("GOG0078"));
        assert_eq!(StudyId::GOG0090.to_string(), String::from("GOG0090"));
        assert_eq!(StudyId::GOG0116.to_string(), String::from("GOG0116"));
        assert_eq!(StudyId::INT133.to_string(), String::from("INT133"));
        assert_eq!(StudyId::JACLSAML99.to_string(), String::from("JACLSAML99"));
        assert_eq!(StudyId::JPLSGAML05.to_string(), String::from("JPLSGAML05"));
        assert_eq!(StudyId::MRCAML12.to_string(), String::from("MRCAML12"));
        assert_eq!(StudyId::MRCAML15.to_string(), String::from("MRCAML15"));
        assert_eq!(
            StudyId::NOPHOAML2004.to_string(),
            String::from("NOPHOAML2004")
        );
        assert_eq!(
            StudyId::NOPHOAML2012.to_string(),
            String::from("NOPHOAML2012")
        );
        assert_eq!(StudyId::OS2006.to_string(), String::from("OS2006"));
        assert_eq!(StudyId::P9749.to_string(), String::from("P9749"));
        assert_eq!(StudyId::P9754.to_string(), String::from("P9754"));
        assert_eq!(StudyId::POG9049.to_string(), String::from("POG9049"));
        assert_eq!(
            StudyId::PPLLSGAML98.to_string(),
            String::from("PPLLSGAML98")
        );
        assert_eq!(StudyId::REGOBONE.to_string(), String::from("REGOBONE"));
        assert_eq!(
            StudyId::Sarcome13_OS2016.to_string(),
            String::from("Sarcome13/OS2016")
        );
        assert_eq!(StudyId::SCFEELAM02.to_string(), String::from("SCFEELAM02"));
        assert_eq!(StudyId::SJCRHAML02.to_string(), String::from("SJCRHAML02"));
        assert_eq!(StudyId::TCGM2004.to_string(), String::from("TCGM2004"));
        assert_eq!(StudyId::TE04.to_string(), String::from("TE04"));
        assert_eq!(StudyId::TE05.to_string(), String::from("TE05"));
        assert_eq!(StudyId::TE08.to_string(), String::from("TE08"));
        assert_eq!(StudyId::TE09.to_string(), String::from("TE09"));
        assert_eq!(StudyId::TE13.to_string(), String::from("TE13"));
        assert_eq!(StudyId::TE20.to_string(), String::from("TE20"));
        assert_eq!(StudyId::TE22.to_string(), String::from("TE22"));
        assert_eq!(StudyId::TGM85.to_string(), String::from("TGM85"));
        assert_eq!(StudyId::TGM90.to_string(), String::from("TGM90"));
        assert_eq!(StudyId::TGM95.to_string(), String::from("TGM95"));
        assert_eq!(StudyId::TIP.to_string(), String::from("TIP"));
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&StudyId::AALL0232).unwrap(),
            "\"AALL0232\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AALL0331).unwrap(),
            "\"AALL0331\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AALL03B1).unwrap(),
            "\"AALL03B1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AALL0434).unwrap(),
            "\"AALL0434\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AALL08B1).unwrap(),
            "\"AALL08B1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AAML03P1).unwrap(),
            "\"AAML03P1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AAML0531).unwrap(),
            "\"AAML0531\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AAML1031).unwrap(),
            "\"AAML1031\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AEIOPAML2002).unwrap(),
            "\"AEIOPAML2002\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AEWS0031).unwrap(),
            "\"AEWS0031\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AEWS0331).unwrap(),
            "\"AEWS0331\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AEWS07P1).unwrap(),
            "\"AEWS07P1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AEWS1031).unwrap(),
            "\"AEWS1031\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AEWS1221).unwrap(),
            "\"AEWS1221\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AGCT0132).unwrap(),
            "\"AGCT0132\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AGCT01P1).unwrap(),
            "\"AGCT01P1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AGCT0521).unwrap(),
            "\"AGCT0521\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AHOD0031).unwrap(),
            "\"AHOD0031\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AHOD03P1).unwrap(),
            "\"AHOD03P1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AHOD0431).unwrap(),
            "\"AHOD0431\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AHOD0831).unwrap(),
            "\"AHOD0831\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AHOD1221).unwrap(),
            "\"AHOD1221\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AHOD1331).unwrap(),
            "\"AHOD1331\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AIEOPLAM92).unwrap(),
            "\"AIEOPLAM92\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AMLBFM_Registry2012).unwrap(),
            "\"AMLBFM-Registry2012\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AMLBFM1998).unwrap(),
            "\"AMLBFM1998\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AMLBFM2004).unwrap(),
            "\"AMLBFM2004\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AMLBFM2012).unwrap(),
            "\"AMLBFM2012\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AMLBFMRegistry2017).unwrap(),
            "\"AMLBFMRegistry2017\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AOST0121).unwrap(),
            "\"AOST0121\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AOST01P1).unwrap(),
            "\"AOST01P1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AOST0221).unwrap(),
            "\"AOST0221\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AOST0331_EURAMOS1).unwrap(),
            "\"AOST0331/EURAMOS1\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AOST1321).unwrap(),
            "\"AOST1321\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::AOST1421).unwrap(),
            "\"AOST1421\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::CCG_782).unwrap(),
            "\"CCG-782\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::CCG_7942).unwrap(),
            "\"CCG-7942\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::DBAML01).unwrap(),
            "\"DBAML01\""
        );
        assert_eq!(serde_json::to_string(&StudyId::EE99).unwrap(), "\"EE99\"");
        assert_eq!(
            serde_json::to_string(&StudyId::EICESS92).unwrap(),
            "\"EICESS92\""
        );
        assert_eq!(serde_json::to_string(&StudyId::GC1).unwrap(), "\"GC1\"");
        assert_eq!(serde_json::to_string(&StudyId::GC2).unwrap(), "\"GC2\"");
        assert_eq!(
            serde_json::to_string(&StudyId::GOG0078).unwrap(),
            "\"GOG0078\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::GOG0090).unwrap(),
            "\"GOG0090\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::GOG0116).unwrap(),
            "\"GOG0116\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::INT133).unwrap(),
            "\"INT133\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::JACLSAML99).unwrap(),
            "\"JACLSAML99\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::JPLSGAML05).unwrap(),
            "\"JPLSGAML05\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::MRCAML12).unwrap(),
            "\"MRCAML12\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::MRCAML15).unwrap(),
            "\"MRCAML15\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::NOPHOAML2004).unwrap(),
            "\"NOPHOAML2004\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::NOPHOAML2012).unwrap(),
            "\"NOPHOAML2012\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::OS2006).unwrap(),
            "\"OS2006\""
        );
        assert_eq!(serde_json::to_string(&StudyId::P9749).unwrap(), "\"P9749\"");
        assert_eq!(serde_json::to_string(&StudyId::P9754).unwrap(), "\"P9754\"");
        assert_eq!(
            serde_json::to_string(&StudyId::POG9049).unwrap(),
            "\"POG9049\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::PPLLSGAML98).unwrap(),
            "\"PPLLSGAML98\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::REGOBONE).unwrap(),
            "\"REGOBONE\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::Sarcome13_OS2016).unwrap(),
            "\"Sarcome13/OS2016\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::SCFEELAM02).unwrap(),
            "\"SCFEELAM02\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::SJCRHAML02).unwrap(),
            "\"SJCRHAML02\""
        );
        assert_eq!(
            serde_json::to_string(&StudyId::TCGM2004).unwrap(),
            "\"TCGM2004\""
        );
        assert_eq!(serde_json::to_string(&StudyId::TE04).unwrap(), "\"TE04\"");
        assert_eq!(serde_json::to_string(&StudyId::TE05).unwrap(), "\"TE05\"");
        assert_eq!(serde_json::to_string(&StudyId::TE08).unwrap(), "\"TE08\"");
        assert_eq!(serde_json::to_string(&StudyId::TE09).unwrap(), "\"TE09\"");
        assert_eq!(serde_json::to_string(&StudyId::TE13).unwrap(), "\"TE13\"");
        assert_eq!(serde_json::to_string(&StudyId::TE20).unwrap(), "\"TE20\"");
        assert_eq!(serde_json::to_string(&StudyId::TE22).unwrap(), "\"TE22\"");
        assert_eq!(serde_json::to_string(&StudyId::TGM85).unwrap(), "\"TGM85\"");
        assert_eq!(serde_json::to_string(&StudyId::TGM90).unwrap(), "\"TGM90\"");
        assert_eq!(serde_json::to_string(&StudyId::TGM95).unwrap(), "\"TGM95\"");
        assert_eq!(serde_json::to_string(&StudyId::TIP).unwrap(), "\"TIP\"");
    }
}
