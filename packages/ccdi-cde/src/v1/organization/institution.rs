use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 12662779 v1.00`**
///
/// This metadata element is defined by the caDSR as "A sequence of characters
/// used to identify, name, or characterize the laboratory, institute, or
/// consortium that provided the information.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12662779%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::organization::Institution)]
#[allow(non_camel_case_types)]
pub enum Institution {
    /// `AIEOP`
    ///
    /// * **VM Long Name**: Italian Association of Pediatric Hematology and
    ///   Oncology
    /// * **VM Public ID**: 12935180
    /// * **Concept Code**: C168887
    /// * **Begin Date**:   02/28/2023
    ///
    /// A group from Bologna, Italy, comprised mainly of pediatricians,
    /// hematologists, oncologists, surgeons, biologists, nurses and
    /// psychologists who are devoted to addressing the problems of hematology,
    /// oncology and immunology in the child and adolescent.
    AIEOP,

    /// `BFM-SG`
    ///
    /// * **VM Long Name**: Berlin-Frankfurt-Munster Study Group
    /// * **VM Public ID**: 12935182
    /// * **Concept Code**: C168888
    /// * **Begin Date**:   02/28/2023
    ///
    /// A study group formed in 1975 by three individuals, Hansjorg Riehm in
    /// Berlin (B), Bernhard Kornhuber in Frankfurt (F) and Gunther Schellong in
    /// Munster (M), who initiated the first multicenter BFM trial. The BFM
    /// treatment concept was based on an intensive chemotherapeutic approach
    /// employing eight different drugs which led to a revolutionary increase in
    /// the survival of children and adolescents with acute lymphoblastic
    /// leukemia.
    #[serde(rename = "BFM-SG")]
    BFM_SG,

    /// `BOCG`
    ///
    /// * **VM Long Name**: Brazilian Group and Scandinavian Group
    /// * **VM Public ID**: 12935501
    /// * **Concept Code**: C180373
    /// * **Begin Date**:   02/28/2023
    ///
    /// A cooperative group of individuals working with pediatric osteosarcoma.
    BOCG,

    /// `C3P`
    ///
    /// * **VM Long Name**: Consortium for Childhood Cancer Predisposition
    /// * **VM Public ID**: 12662905
    /// * **Concept Code**: C192767
    /// * **Begin Date**:   12/20/2023
    ///
    /// A multicenter collaborative research group created to improve outcomes
    /// for children at high risk for cancer due to hereditary tumor
    /// predisposition. It has created a multicenter registry, database, and
    /// biorepository for pediatric cancer predisposition syndromes (CPS).
    C3P,

    /// `CBTN`
    ///
    /// * **VM Long Name**: Children's Brain Tumor Network
    /// * **VM Public ID**: 12938090
    /// * **Concept Code**: C192778
    /// * **Begin Date**:   02/28/2023
    ///
    /// A network of member institutions affiliated with Children's Hospital of
    /// Philadelphia that is involved in the collection of donated brain tumor
    /// tissue and other biosamples, and the development cloud-based data
    /// platforms, allowing researchers to access brain tumor data and specimen
    /// with the shared goal of improving health outcomes for children and young
    /// adults with brain tumors.
    CBTN,

    /// `CCLG`
    ///
    /// * **VM Long Name**: Children's Cancer and Leukaemia Group
    /// * **VM Public ID**: 12935206
    /// * **Concept Code**: C177327
    /// * **Begin Date**:   02/28/2023
    ///
    /// A children's cancer charity and United Kingdom and Ireland's
    /// professional association for those involved in the treatment and care of
    /// children with cancer.
    CCLG,

    /// `CHLA`
    ///
    /// * **VM Long Name**: Children's Hospital Los Angeles
    /// * **VM Public ID**: 12938094
    /// * **Concept Code**: C192782
    /// * **Begin Date**:   02/28/2023
    ///
    /// A nationally ranked, freestanding acute care children's hospital in the
    /// East Hollywood district of Los Angeles. Through a network of people and
    /// places across Southern California they share standards, knowledge, and
    /// people with experienced care partners to ensure that more children
    /// receive high-quality, specialized pediatrics.
    CHLA,

    /// `COG`
    ///
    /// * **VM Long Name**: Children's Oncology Group
    /// * **VM Public ID**: 2563903
    /// * **Concept Code**: C39353
    /// * **Begin Date**:   02/28/2023
    ///
    /// An NCI-supported clinical cooperative group formed by the merger of the
    /// four national pediatric cancer research organizations: the Children's
    /// Cancer Group, the Intergroup Rhabdomyosarcoma Study Group, the National
    /// Wilms Tumor Study Group, and the Pediatric Oncology Group. The primary
    /// objective of the organization is to conduct clinical trials of new
    /// therapies for childhood and adolescent cancer. COG develops and
    /// coordinates clinical trials conducted at the 238 member institutions
    /// that include cancer centers of all major universities and teaching
    /// hospitals throughout the U.S. and Canada, as well as sites in Europe and
    /// Australia. COG members include over 5000 cancer researchers.
    COG,

    /// `COSS-GPOH`
    ///
    /// * **VM Long Name**: Osteosarcoma Study Group of the German Society of
    ///   Pediatric Oncology and Hematology
    /// * **VM Public ID**: 12935500
    /// * **Concept Code**: C186743
    /// * **Begin Date**:   02/28/2023
    ///
    /// The section of the German Society of Pediatric Oncology and Hematology
    /// dedicated to the study of osteosarcoma.
    #[serde(rename = "COSS-GPOH")]
    COSS_GPOH,

    /// `CRCTU`
    ///
    /// * **VM Long Name**: Cancer Research UK Clinical Trials Unit
    /// * **VM Public ID**: 12935492
    /// * **Concept Code**: C174966
    /// * **Begin Date**:   02/28/2023
    ///
    /// A research organization in the United Kingdom specializing in the design
    /// and delivery of every aspect of clinical trials of all phases in adults
    /// and children, from concept to publication.
    CRCTU,

    /// `CWS`
    ///
    /// * **VM Long Name**: Cooperative Weichteilsarkom Studiengruppe
    /// * **VM Public ID**: 12938089
    /// * **Concept Code**: C192777
    /// * **Begin Date**:   02/28/2023
    ///
    /// An international pediatric soft tissue sarcoma study group, also known
    /// as the Cooperative Soft Tissue Sarcoma Group, that creates guidance for
    /// risk-adapted treatment of soft tissue sarcoma and soft tissue tumors in
    /// children, adolescents, and young adults in Europe.
    CWS,

    /// `DCOG`
    ///
    /// * **VM Long Name**: Dutch Childhood Oncology Group
    /// * **VM Public ID**: 12935183
    /// * **Concept Code**: C168889
    /// * **Begin Date**:   02/28/2023
    ///
    /// A collaborating partner in Innovative Therapies for Children with Cancer
    /// Consortium (ITCC), a European based consortium to promote the clinical
    /// evaluation of new anti-cancer compounds in children with cancer.
    DCOG,

    /// `DePICT`
    ///
    /// * **VM Long Name**: Defining Platforms for Individualized Cancer
    ///   Treatment
    /// * **VM Public ID**: 12938095
    /// * **Concept Code**: C192784
    /// * **Begin Date**:   02/28/2023
    ///
    /// An IRB approved observational trial opened in 2016 that is designed to
    /// monitor outcomes of Broward County, Florida residents with late-stage
    /// refractory cancer who undergo next generation sequencing with the goal
    /// of targeting the enrollment of underrepresented populations in oncology
    /// clinical trials to improve generalizability of clinical research.
    DePICT,

    /// `DFCI`
    ///
    /// * **VM Long Name**: DFCI
    /// * **VM Public ID**: 7789447
    /// * **Concept Code**: C177330
    /// * **Begin Date**:   02/28/2023
    ///
    /// An institute founded in 1947 in Boston, Massachusetts, committed to
    /// providing treatment for adults and children with cancer, and developing
    /// cures with cutting-edge research.
    DFCI,

    /// `EORTC`
    ///
    /// * **VM Long Name**: European Organization for Research and Treatment of
    ///   Cancer
    /// * **VM Public ID**: 2563904
    /// * **Concept Code**: C19741
    /// * **Begin Date**:   12/20/2023
    ///
    /// European Organization for Research and Treatment of Cancer
    EORTC,

    /// `EpSSG`
    ///
    /// * **VM Long Name**: European Paediatric Soft Tissue Sarcoma Study Group
    /// * **VM Public ID**: 12938087
    /// * **Concept Code**: C192774
    /// * **Begin Date**:   02/28/2023
    ///
    /// An international organization of healthcare professionals devoted to the
    /// care and treatment of children and young people with soft tissue
    /// sarcoma.
    EpSSG,

    /// `EuRBG`
    ///
    /// * **VM Long Name**: European Retinoblastoma Group
    /// * **VM Public ID**: 12938096
    /// * **Concept Code**: C192785
    /// * **Begin Date**:   02/28/2023
    ///
    /// A pan-European partnership between healthcare professionals, families,
    /// and patients affected by retinoblastoma with a goal of identifying and
    /// developing tools to address the problems with retinoblastoma management
    /// through the creation of a retinoblastoma network.
    EuRBG,

    /// `EURO-EWING`
    ///
    /// * **VM Long Name**: EURO-EWING Consortium
    /// * **VM Public ID**: 12935490
    /// * **Concept Code**: C174964
    /// * **Begin Date**:   02/28/2023
    ///
    /// A coalition of clinical study groups bringing together active clinicians
    /// and scientists in Europe dedicated to improving survival from Ewing
    /// sarcoma.
    #[serde(rename = "EURO-EWING")]
    EURO_EWING,

    /// `FSG`
    ///
    /// * **VM Long Name**: French Sarcoma Group
    /// * **VM Public ID**: 12935217
    /// * **Concept Code**: C174962
    /// * **Begin Date**:   02/28/2023
    ///
    /// A research organization in France involved in the understanding,
    /// diagnosis, treatment and prognosis of sarcoma in children and
    /// adolescents.
    FSG,

    /// `GALOP`
    ///
    /// * **VM Long Name**: The Latin American Group of Paediatric Oncology
    /// * **VM Public ID**: 12938097
    /// * **Concept Code**: C192786
    /// * **Begin Date**:   02/28/2023
    ///
    /// A multinational cooperative group for clinical trials in childhood
    /// tumours in Latin America, created in 2008 under the auspices of the
    /// Children Oncology group for childhood tumor clinical trials in Latin
    /// America.
    GALOP,

    /// `GEIS`
    ///
    /// * **VM Long Name**: Grupo Espanol de Investigacion en Sarcomas
    /// * **VM Public ID**: 12935502
    /// * **Concept Code**: C180341
    /// * **Begin Date**:   02/28/2023
    ///
    /// A scientific society formed by professionals from more than sixty
    /// medical centers across Spain. This group includes surgeons,
    /// pediatricians, oncologic radiation therapy specialists, pathologists and
    /// molecular researchers.
    GEIS,

    /// `GPOH`
    ///
    /// * **VM Long Name**: German Society for Pediatric Oncology and Hematology
    /// * **VM Public ID**: 12935214
    /// * **Concept Code**: C174960
    /// * **Begin Date**:   02/28/2023
    ///
    /// A research organization in Germany involved in the understanding,
    /// diagnosis, treatment and prognosis of pediatric cancers.
    GPOH,

    /// `IDIPGR`
    ///
    /// * **VM Long Name**: International Diffuse Intrinsic Pontine Glioma
    ///   Registry Case Report Form Questionnaire
    /// * **VM Public ID**: 12935504
    /// * **Concept Code**: C180886
    /// * **Begin Date**:   02/28/2023
    ///
    /// Questions used in the case report forms of the international diffuse
    /// intrinsic pontine glioma registry for pediatric cancers.
    IDIPGR,

    /// `ISG`
    ///
    /// * **VM Long Name**: Italian Sarcoma Group
    /// * **VM Public ID**: 12935491
    /// * **Concept Code**: C174965
    /// * **Begin Date**:   02/28/2023
    ///
    /// A independent, scientific non-profit association formed in 2002 with the
    /// purpose of improving the quality of treatment for sarcoma.
    ISG,

    /// `JACLS`
    ///
    /// * **VM Long Name**: Japan Association of Childhood Leukemia Study
    /// * **VM Public ID**: 12935495
    /// * **Concept Code**: C168890
    /// * **Begin Date**:   02/28/2023
    ///
    /// A study group in Japan investigating childhood leukemia.
    JACLS,

    /// `JCCG`
    ///
    /// * **VM Long Name**: Japan Children's Cancer Group Neuroblastoma
    ///   Committee
    /// * **VM Public ID**: 12935522
    /// * **Concept Code**: C192770
    /// * **Begin Date**:   02/28/2023
    ///
    /// A neuroblastoma-focused committee within the Japan Children's Cancer
    /// Group, which is a national clinical research group for childhood cancer
    /// that was established in 2014 and comprises more than 200 university,
    /// community, regional children's, and general hospitals.
    JCCG,

    /// `JINCS`
    ///
    /// * **VM Long Name**: Japanese Infantile Neuroblastoma Collaborative Study
    ///   Group
    /// * **VM Public ID**: 12951420
    /// * **Concept Code**: C192894
    /// * **Begin Date**:   03/01/2023
    ///
    /// A research cooperative based in Japan that conducted studies on infants
    /// with stage 1, 2, and 3 neuroblastoma between June 1994 and December
    /// 2004.
    JINCS,

    /// `JNBSG`
    ///
    /// * **VM Long Name**: Japan Neuroblastoma Study Group
    /// * **VM Public ID**: 12937991
    /// * **Concept Code**: C192771
    /// * **Begin Date**:   02/28/2023
    ///
    /// A study group established in 2006 with the aims of improving the cure
    /// rate of childhood neuroblastoma in Japan and minimizing the late effects
    /// of certain treatments.
    JNBSG,

    /// `JPLSG`
    ///
    /// * **VM Long Name**: Japanese Pediatric Leukemia/Lymphoma Study Group
    /// * **VM Public ID**: 12935496
    /// * **Concept Code**: C168891
    /// * **Begin Date**:   02/28/2023
    ///
    /// A study group in Japan investigating pediatric leukemia and lymphoma.
    JPLSG,

    /// `MRC`
    ///
    /// * **VM Long Name**: Medical Research Council
    /// * **VM Public ID**: 12935191
    /// * **Concept Code**: C168892
    /// * **Begin Date**:   02/28/2023
    ///
    /// A publicly funded organization that is part of United Kingdom Research
    /// and Innovation, and is dedicated to improving human health through
    /// world-class medical research.
    MRC,

    /// `NOPHO`
    ///
    /// * **VM Long Name**: Nordic Society of Pediatric Haematology and Oncology
    /// * **VM Public ID**: 12935203
    /// * **Concept Code**: C168893
    /// * **Begin Date**:   02/28/2023
    ///
    /// A collaborative group formed in 1984 with members from Denmark, Finland,
    /// Iceland, Lithuania, Norway and Sweden and whose goal was to secure that
    /// all Nordic children suffering from leukemia would receive optimal
    /// therapy wherever they lived.
    NOPHO,

    /// `NRG-Oncology`
    ///
    /// * **VM Long Name**: NRG-Oncology
    /// * **VM Public ID**: 12939860
    /// * **Concept Code**: C177331
    /// * **Begin Date**:   02/28/2023
    ///
    /// A leading protocol organization within the National Clinical Trials
    /// Network that seeks to improve the lives of cancer patients by conducting
    /// practice-changing, multi-institutional clinical and translational
    /// research.
    #[serde(rename = "NRG-Oncology")]
    NRG_Oncology,

    /// `PNOC`
    ///
    /// * **VM Long Name**: Pacific Pediatric Neuro-Oncology Consortium
    /// * **VM Public ID**: 12938091
    /// * **Concept Code**: C192779
    /// * **Begin Date**:   02/28/2023
    ///
    /// An international consortium committed to understanding how brain tumors
    /// develop in children, identifying personalized treatment strategies, and
    /// improving health outcomes
    PNOC,

    /// `PPLLSG`
    ///
    /// * **VM Long Name**: Polish Pediatric Leukemia/Lymphoma Study Group
    /// * **VM Public ID**: 12935204
    /// * **Concept Code**: C168894
    /// * **Begin Date**:   02/28/2023
    ///
    /// A collaborative group established in 1974 that had the original goal of
    /// implementing unified protocols with standardized diagnostic criteria and
    /// therapy regimens for Hodgkin's disease and acute lymphoblastic leukemia.
    /// The prospects were widened to include non-Hodgkin's lymphomas and acute
    /// myelogenous leukemia in 1983.
    PPLLSG,

    /// `RBTR`
    ///
    /// * **VM Long Name**: Rare Brain Tumor Registry
    /// * **VM Public ID**: 12938092
    /// * **Concept Code**: C192780
    /// * **Begin Date**:   02/28/2023
    ///
    /// An international registry spearheaded by Children’s National Hospital
    /// that seeks to create an infrastructure for the prospective collection of
    /// biospecimens and radiological and clinical data that would allow better
    /// understanding of the biologic mechanisms and therapeutic
    /// susceptibilities of childhood rare brain tumors and other rare entities.
    RBTR,

    /// `SFCE`
    ///
    /// * **VM Long Name**: Societe Francaise de Lutte contre les Cancers et
    ///   Leucemies de l'Enfant et de l'Adolescent
    /// * **VM Public ID**: 12935497
    /// * **Concept Code**: C177328
    /// * **Begin Date**:   02/28/2023
    ///
    /// An association founded in France in 2003 which promotes the organization
    /// of care and scientific research in the field of childhood and adolescent
    /// cancer.
    SFCE,

    /// `SIOP MMT`
    ///
    /// * **VM Long Name**: International Society of Paediatric Oncology
    ///   Malignant Mesenchymal Tumour Committee
    /// * **VM Public ID**: 12938088
    /// * **Concept Code**: C192775
    /// * **Begin Date**:   02/28/2023
    ///
    /// An international pediatric oncology committee that specializes in the
    /// research of malignant mesenchymal tumors. Along with the Associazione
    /// Italiana Ematologia Oncologia Pediatrica-Soft Tissue Sarcoma Committee
    /// (AIEOP-STSC), which was formerly known as the Italian Cooperative Group
    /// (ICG), SIOP-MMT founded the European pediatric Soft tissue sarcoma Study
    /// Group (EpSSG) and the Non-rhabdomyosarcoma Soft Tissue Sarcoma (NRSTS)
    /// study in 2005 with the goal of making pediatric non-rhabdomyosarcoma
    /// soft tissue sarcoma treatment uniform across Europe.
    #[serde(rename = "SIOP MMT")]
    SIOP_MMT,

    /// `SIOPE`
    ///
    /// * **VM Long Name**: European Society for Paediatric Oncology
    /// * **VM Public ID**: 12938093
    /// * **Concept Code**: C192781
    /// * **Begin Date**:   02/28/2023
    ///
    /// A pan-European organization representing professionals working in the
    /// field of childhood cancers that was originally established in 1998 as
    /// the European branch of the International Society of Paediatric Oncology
    /// (SIOP) and became an independent organization in 2007.
    SIOPE,

    /// `SIOPEN`
    ///
    /// * **VM Long Name**: International Society of Paediatric Oncology Europe
    ///   Neuroblastoma Group
    /// * **VM Public ID**: 12938072
    /// * **Concept Code**: C192772
    /// * **Begin Date**:   02/28/2023
    ///
    /// An internationally collaborative research network that brings together
    /// physicians, scientists, and other healthcare professionals from across
    /// Europe and beyond who are actively involved in the care of patients
    /// with, or research into, neuroblastoma to improve patient outcomes.
    SIOPEN,

    /// `SJCRH`
    ///
    /// * **VM Long Name**: St. Jude Children's Research Hospital
    /// * **VM Public ID**: 14793809
    /// * **Concept Code**: C39510
    /// * **Begin Date**:   02/27/2024
    ///
    /// The St. Jude Children's Research Hospital received its NCI designation
    /// in 1977 and was awarded status as a comprehensive cancer center by NCI
    /// in 2008. Research is focused specifically on childhood cancers, acquired
    /// and inherited immunodeficiencies and genetic disorders.
    SJCRH,

    /// `SOPOBE`
    ///
    /// * **VM Long Name**: Brazilian Society of Pediatric Oncology
    /// * **VM Public ID**: 12935499
    /// * **Concept Code**: C177329
    /// * **Begin Date**:   02/28/2023
    ///
    /// A society founded in 1981 committed to improve the prognosis of children
    /// and adolescents with cancer.
    SOPOBE,

    /// `SSG`
    ///
    /// * **VM Long Name**: Scandinavian Sarcoma Group
    /// * **VM Public ID**: 12935215
    /// * **Concept Code**: C174961
    /// * **Begin Date**:   02/28/2023
    ///
    /// A research organization in Scandinavia involved in the understanding,
    /// diagnosis, treatment and prognosis of sarcoma in children and
    /// adolescents.
    SSG,

    /// `Treehouse`
    ///
    /// * **VM Long Name**: Treehouse Childhood Cancer Initiative
    /// * **VM Public ID**: 14848142
    /// * **Concept Code**: C204768
    /// * **Begin Date**:   03/19/2024
    ///
    /// An initiative to find existing cancer drugs that might be applicable to
    /// childhood cancers. The group is analyzing the genomic data of children
    /// with cancer in the context of large data sets of both pediatric and
    /// adult cancers. This should identify situations where an available drug
    /// is predicted to work for a given patient’s tumor based on a similar
    /// genetic basis for the cancer.
    Treehouse,

    /// `UCL`
    ///
    /// * **VM Long Name**: University College of London
    /// * **VM Public ID**: 12935494
    /// * **Concept Code**: C174967
    /// * **Begin Date**:   02/28/2023
    ///
    /// A public research university located in London, England.
    UCL,

    /// `UK`
    ///
    /// * **VM Long Name**: United Kingdom Sarcoma Registry
    /// * **VM Public ID**: 12935488
    /// * **Concept Code**: C174963
    /// * **Begin Date**:   02/28/2023
    ///
    /// An information system in the United Kingdom designed for the collection,
    /// storage, and management of data on persons with sarcoma.
    UK,
}

impl CDE for Institution {}

impl std::fmt::Display for Institution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Institution::AIEOP => write!(f, "AIEOP"),
            Institution::BFM_SG => write!(f, "BFM-SG"),
            Institution::BOCG => write!(f, "BOCG"),
            Institution::C3P => write!(f, "C3P"),
            Institution::CBTN => write!(f, "CBTN"),
            Institution::CCLG => write!(f, "CCLG"),
            Institution::CHLA => write!(f, "CHLA"),
            Institution::COG => write!(f, "COG"),
            Institution::COSS_GPOH => write!(f, "COSS-GPOH"),
            Institution::CRCTU => write!(f, "CRCTU"),
            Institution::CWS => write!(f, "CWS"),
            Institution::DCOG => write!(f, "DCOG"),
            Institution::DePICT => write!(f, "DePICT"),
            Institution::DFCI => write!(f, "DFCI"),
            Institution::EORTC => write!(f, "EORTC"),
            Institution::EpSSG => write!(f, "EpSSG"),
            Institution::EuRBG => write!(f, "EuRBG"),
            Institution::EURO_EWING => write!(f, "EURO-EWING"),
            Institution::FSG => write!(f, "FSG"),
            Institution::GALOP => write!(f, "GALOP"),
            Institution::GEIS => write!(f, "GEIS"),
            Institution::GPOH => write!(f, "GPOH"),
            Institution::IDIPGR => write!(f, "IDIPGR"),
            Institution::ISG => write!(f, "ISG"),
            Institution::JACLS => write!(f, "JACLS"),
            Institution::JCCG => write!(f, "JCCG"),
            Institution::JINCS => write!(f, "JINCS"),
            Institution::JNBSG => write!(f, "JNBSG"),
            Institution::JPLSG => write!(f, "JPLSG"),
            Institution::MRC => write!(f, "MRC"),
            Institution::NOPHO => write!(f, "NOPHO"),
            Institution::NRG_Oncology => write!(f, "NRG-Oncology"),
            Institution::PNOC => write!(f, "PNOC"),
            Institution::PPLLSG => write!(f, "PPLLSG"),
            Institution::RBTR => write!(f, "RBTR"),
            Institution::SFCE => write!(f, "SFCE"),
            Institution::SIOP_MMT => write!(f, "SIOP MMT"),
            Institution::SIOPE => write!(f, "SIOPE"),
            Institution::SIOPEN => write!(f, "SIOPEN"),
            Institution::SJCRH => write!(f, "SJCRH"),
            Institution::SOPOBE => write!(f, "SOPOBE"),
            Institution::SSG => write!(f, "SSG"),
            Institution::Treehouse => write!(f, "Treehouse"),
            Institution::UCL => write!(f, "UCL"),
            Institution::UK => write!(f, "UK"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(Institution::AIEOP.to_string(), "AIEOP");
        assert_eq!(Institution::BFM_SG.to_string(), "BFM-SG");
        assert_eq!(Institution::BOCG.to_string(), "BOCG");
        assert_eq!(Institution::C3P.to_string(), "C3P");
        assert_eq!(Institution::CBTN.to_string(), "CBTN");
        assert_eq!(Institution::CCLG.to_string(), "CCLG");
        assert_eq!(Institution::CHLA.to_string(), "CHLA");
        assert_eq!(Institution::COG.to_string(), "COG");
        assert_eq!(Institution::COSS_GPOH.to_string(), "COSS-GPOH");
        assert_eq!(Institution::CRCTU.to_string(), "CRCTU");
        assert_eq!(Institution::CWS.to_string(), "CWS");
        assert_eq!(Institution::DCOG.to_string(), "DCOG");
        assert_eq!(Institution::DePICT.to_string(), "DePICT");
        assert_eq!(Institution::DFCI.to_string(), "DFCI");
        assert_eq!(Institution::EORTC.to_string(), "EORTC");
        assert_eq!(Institution::EpSSG.to_string(), "EpSSG");
        assert_eq!(Institution::EuRBG.to_string(), "EuRBG");
        assert_eq!(Institution::EURO_EWING.to_string(), "EURO-EWING");
        assert_eq!(Institution::FSG.to_string(), "FSG");
        assert_eq!(Institution::GALOP.to_string(), "GALOP");
        assert_eq!(Institution::GEIS.to_string(), "GEIS");
        assert_eq!(Institution::GPOH.to_string(), "GPOH");
        assert_eq!(Institution::IDIPGR.to_string(), "IDIPGR");
        assert_eq!(Institution::ISG.to_string(), "ISG");
        assert_eq!(Institution::JACLS.to_string(), "JACLS");
        assert_eq!(Institution::JCCG.to_string(), "JCCG");
        assert_eq!(Institution::JINCS.to_string(), "JINCS");
        assert_eq!(Institution::JNBSG.to_string(), "JNBSG");
        assert_eq!(Institution::JPLSG.to_string(), "JPLSG");
        assert_eq!(Institution::MRC.to_string(), "MRC");
        assert_eq!(Institution::NOPHO.to_string(), "NOPHO");
        assert_eq!(Institution::NRG_Oncology.to_string(), "NRG-Oncology");
        assert_eq!(Institution::PNOC.to_string(), "PNOC");
        assert_eq!(Institution::PPLLSG.to_string(), "PPLLSG");
        assert_eq!(Institution::RBTR.to_string(), "RBTR");
        assert_eq!(Institution::SFCE.to_string(), "SFCE");
        assert_eq!(Institution::SIOP_MMT.to_string(), "SIOP MMT");
        assert_eq!(Institution::SIOPE.to_string(), "SIOPE");
        assert_eq!(Institution::SIOPEN.to_string(), "SIOPEN");
        assert_eq!(Institution::SJCRH.to_string(), "SJCRH");
        assert_eq!(Institution::SOPOBE.to_string(), "SOPOBE");
        assert_eq!(Institution::SSG.to_string(), "SSG");
        assert_eq!(Institution::Treehouse.to_string(), "Treehouse");
        assert_eq!(Institution::UCL.to_string(), "UCL");
        assert_eq!(Institution::UK.to_string(), "UK");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&Institution::AIEOP).unwrap(),
            "\"AIEOP\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::BFM_SG).unwrap(),
            "\"BFM-SG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::BOCG).unwrap(),
            "\"BOCG\""
        );
        assert_eq!(serde_json::to_string(&Institution::C3P).unwrap(), "\"C3P\"");
        assert_eq!(
            serde_json::to_string(&Institution::CBTN).unwrap(),
            "\"CBTN\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::CCLG).unwrap(),
            "\"CCLG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::CHLA).unwrap(),
            "\"CHLA\""
        );
        assert_eq!(serde_json::to_string(&Institution::COG).unwrap(), "\"COG\"");
        assert_eq!(
            serde_json::to_string(&Institution::COSS_GPOH).unwrap(),
            "\"COSS-GPOH\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::CRCTU).unwrap(),
            "\"CRCTU\""
        );
        assert_eq!(serde_json::to_string(&Institution::CWS).unwrap(), "\"CWS\"");
        assert_eq!(
            serde_json::to_string(&Institution::DCOG).unwrap(),
            "\"DCOG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::DePICT).unwrap(),
            "\"DePICT\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::DFCI).unwrap(),
            "\"DFCI\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::EORTC).unwrap(),
            "\"EORTC\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::EpSSG).unwrap(),
            "\"EpSSG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::EuRBG).unwrap(),
            "\"EuRBG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::EURO_EWING).unwrap(),
            "\"EURO-EWING\""
        );
        assert_eq!(serde_json::to_string(&Institution::FSG).unwrap(), "\"FSG\"");
        assert_eq!(
            serde_json::to_string(&Institution::GALOP).unwrap(),
            "\"GALOP\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::GEIS).unwrap(),
            "\"GEIS\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::GPOH).unwrap(),
            "\"GPOH\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::IDIPGR).unwrap(),
            "\"IDIPGR\""
        );
        assert_eq!(serde_json::to_string(&Institution::ISG).unwrap(), "\"ISG\"");
        assert_eq!(
            serde_json::to_string(&Institution::JACLS).unwrap(),
            "\"JACLS\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::JCCG).unwrap(),
            "\"JCCG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::JINCS).unwrap(),
            "\"JINCS\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::JNBSG).unwrap(),
            "\"JNBSG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::JPLSG).unwrap(),
            "\"JPLSG\""
        );
        assert_eq!(serde_json::to_string(&Institution::MRC).unwrap(), "\"MRC\"");
        assert_eq!(
            serde_json::to_string(&Institution::NOPHO).unwrap(),
            "\"NOPHO\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::NRG_Oncology).unwrap(),
            "\"NRG-Oncology\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::PNOC).unwrap(),
            "\"PNOC\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::PPLLSG).unwrap(),
            "\"PPLLSG\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::RBTR).unwrap(),
            "\"RBTR\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::SFCE).unwrap(),
            "\"SFCE\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::SIOP_MMT).unwrap(),
            "\"SIOP MMT\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::SIOPE).unwrap(),
            "\"SIOPE\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::SIOPEN).unwrap(),
            "\"SIOPEN\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::SJCRH).unwrap(),
            "\"SJCRH\""
        );
        assert_eq!(
            serde_json::to_string(&Institution::SOPOBE).unwrap(),
            "\"SOPOBE\""
        );
        assert_eq!(serde_json::to_string(&Institution::SSG).unwrap(), "\"SSG\"");
        assert_eq!(
            serde_json::to_string(&Institution::Treehouse).unwrap(),
            "\"Treehouse\""
        );
        assert_eq!(serde_json::to_string(&Institution::UCL).unwrap(), "\"UCL\"");
        assert_eq!(serde_json::to_string(&Institution::UK).unwrap(), "\"UK\"");
    }
}
