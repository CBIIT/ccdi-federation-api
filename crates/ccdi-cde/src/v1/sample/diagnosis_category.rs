use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 16607972 1.00`**
///
/// This metadata element is defined by the caDSR as "The disease group which is
/// associated with the pediatric cancer diagnosis.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=16607972%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::DiagnosisCategory)]
pub enum DiagnosisCategory {
    /// `Atypical Teratoid/Rhabdoid Tumors`
    ///
    /// * **VM Long Name**: Atypical Teratoid/Rhabdoid Tumor
    /// * **VM Public ID**: 4722554
    /// * **Concept Code**: C6906
    /// * **Begin Date**:   02/05/2024
    ///
    /// An aggressive malignant embryonal neoplasm arising from the central
    /// nervous system. It is composed of cells with a large eccentric nucleus,
    /// prominent nucleolus, and abundant cytoplasm. It may be associated with
    /// loss of chromosome 22. The vast majority of cases occur in childhood.
    /// Symptoms include lethargy, vomiting, cranial nerve palsy, headache,
    /// and hemiplegia.
    #[serde(rename = "Atypical Teratoid/Rhabdoid Tumors")]
    AtypicalTeratoidRhabdoidTumors,


    /// `Choroid Plexus Tumors`
    ///
    /// * **VM Long Name**: Choroid Plexus Neoplasm
    /// * **VM Public ID**: 5097523
    /// * **Concept Code**: C3473
    /// * **Begin Date**:   01/19/2018
    ///
    /// An intraventricular papillary neoplasm that originates from the choroid
    /// plexus epithelium. It includes the choroid plexus papilloma, atypical
    /// choroid plexus papilloma, and choroid plexus carcinoma.
    #[serde(rename = "Choroid Plexus Tumors")]
    ChoroidPlexusTumors,


    /// `CNS Germ Cell Tumors`
    ///
    /// * **VM Long Name**: Central Nervous System Germ Cell Tumor
    /// * **VM Public ID**: 2578439
    /// * **Concept Code**: C5461
    /// * **Begin Date**:   04/03/2006
    ///
    /// Germ Cell Tumors of the CNS constitute a unique class of rare tumors that
    /// affect mainly children and adolescents. Their histopathological and
    /// biological profile largely corresponds to that of homologous germ cell
    /// neoplasms arising in the gonads and in other extragonadal sites.
    /// Germ Cell Tumors of the CNS include: Germinoma, Embryonal Carcinoma,
    /// Yolk Sac Tumor (Endodermal Sinus Tumor), Choriocarcinoma, Mature Teratoma,
    /// Immature Teratoma, Teratoma with Malignant Transformation and
    /// Mixed Germ Cell Tumors. (Adapted from WHO)
    #[serde(rename = "CNS Germ Cell Tumors")]
    CnsGermCellTumors,


    /// `CNS Sarcomas`
    ///
    /// * **VM Long Name**: Central Nervous System Sarcoma
    /// * **VM Public ID**: 14235234
    /// * **Concept Code**: C5153
    /// * **Begin Date**:   02/05/2024
    ///
    /// A sarcoma that arises from the central nervous system.
    #[serde(rename = "CNS Sarcomas")]
    CnsSarcomas,


    /// `Craniopharyngiomas`
    ///
    /// * **VM Long Name**: Craniopharyngioma
    /// * **VM Public ID**: 4265243
    /// * **Concept Code**: C2964
    /// * **Begin Date**:   01/19/2018
    ///
    /// A benign, partly cystic, epithelial tumor of the sellar region,
    /// presumably derived from Rathke pouch epithelium. It affects mainly
    /// children and young adults. There are two clinicopathological forms:
    /// adamantinomatous craniopharyngioma and papillary craniopharyngioma.
    /// The most significant factor associated with recurrence is the extent
    /// of surgical resection, with lesions greater than 5 cm in diameter
    /// carrying a markedly worse prognosis. (Adapted from WHO)
    #[serde(rename = "Craniopharyngiomas")]
    Craniopharyngiomas,


    /// `Ependymoma`
    ///
    /// * **VM Long Name**: Ependymoma
    /// * **VM Public ID**: 2578706
    /// * **Concept Code**: C3017
    /// * **Begin Date**:   04/23/2006
    ///
    /// Ependymoma (WHO grade II) is the most common ependymal neoplasm.
    /// It is a slow growing tumor of children and young adults and is usually
    /// located intraventricularly.  It often causes clinical symptoms by blocking
    /// CSF pathways. Key histological features include perivascular
    /// pseudorosettes and ependymal rosettes. (Adapted from WHO)
    #[serde(rename = "Ependymoma")]
    Ependymoma,


    /// `Glioneuronal and Neuronal Tumors`
    ///
    /// * **VM Long Name**: Glioneuronal and Neuronal Tumors
    /// * **VM Public ID**: 14235233
    /// * **Concept Code**: C4747
    /// * **Begin Date**:   02/05/2024
    ///
    /// A group of central nervous system neoplasms with a variable amount of
    /// neuronal and, less consistently, glial differentiation. They occur at
    /// a low frequency and usually carry a favorable prognosis. Representative
    /// examples include dysplastic cerebellar gangliocytoma, desmoplastic
    /// infantile ganglioglioma, desmoplastic infantile astrocytoma, and
    /// dysembryoplastic neuroepithelial tumor. (Adapted from WHO)
    #[serde(rename = "Glioneuronal and Neuronal Tumors")]
    GlioneuronalAndNeuronalTumors,


    /// `High-Grade Glioma`
    ///
    /// * **VM Long Name**: Malignant Brain Glioma
    /// * **VM Public ID**: 14235230
    /// * **Concept Code**: C4822
    /// * **Begin Date**:   01/21/2026
    ///
    /// A grade 3 or grade 4 glioma arising from the central nervous system.
    /// This category includes glioblastoma, anaplastic astrocytoma, anaplastic
    /// ependymoma, anaplastic oligodendroglioma, and anaplastic oligoastrocytoma.
    #[serde(rename = "High-Grade Glioma")]
    HighGradeGlioma,


    /// `Low-Grade Gliomas`
    ///
    /// * **VM Long Name**: Low Grade Glioma
    /// * **VM Public ID**: 7315250
    /// * **Concept Code**: C132067
    /// * **Begin Date**:   01/21/2026
    ///
    /// A grade I or grade II glioma arising from the central nervous system.
    /// This category includes pilocytic astrocytoma, diffuse astrocytoma,
    /// subependymal giant cell astrocytoma, ependymoma, oligodendroglioma,
    /// oligoastrocytoma, and angiocentric glioma.
    #[serde(rename = "Low-Grade Gliomas")]
    LowGradeGliomas,


    /// `Medulloblastoma`
    ///
    /// * **VM Long Name**: Medulloblastoma
    /// * **VM Public ID**: 4265264
    /// * **Concept Code**: C3222
    /// * **Begin Date**:   01/19/2018
    ///
    /// A malignant, invasive embryonal neoplasm arising from the cerebellum.
    /// It occurs predominantly in children and has the tendency to metastasize
    /// via the cerebrospinal fluid pathways. Signs and symptoms include truncal
    /// ataxia, disturbed gait, lethargy, headache, and vomiting. There are four
    /// histologic variants: anaplastic medulloblastoma, desmoplastic/nodular
    /// medulloblastoma, large cell medulloblastoma, and medulloblastoma with
    /// extensive nodularity.
    #[serde(rename = "Medulloblastoma")]
    Medulloblastoma,


    /// `Other CNS Embryonal Tumors`
    ///
    /// * **VM Long Name**: Central Nervous System Embryonal Tumor, Not Otherwise Specified
    /// * **VM Public ID**: 14741367
    /// * **Concept Code**: C5398
    /// * **Begin Date**:   10/20/2025
    ///
    /// A term that refers to central nervous system embryonal tumors which are
    /// not fully characterized.
    #[serde(rename = "Other CNS Embryonal Tumors")]
    OtherCnsEmbryonalTumors,


    /// `Myeloid Leukemia`
    ///
    /// * **VM Long Name**: Myeloid Leukemia
    /// * **VM Public ID**: 4903679
    /// * **Concept Code**: C3172
    /// * **Begin Date**:   01/19/2018
    ///
    /// A clonal proliferation of myeloid cells and their precursors in the
    /// bone marrow, peripheral blood, and spleen. When the proliferating cells
    /// are immature myeloid cells and myeloblasts, it is called acute myeloid
    /// leukemia. When the proliferating myeloid cells are neutrophils,
    /// it is called chronic myelogenous leukemia.
    #[serde(rename = "Myeloid Leukemia")]
    MyeloidLeukemia,


    /// `Lymphoblastic Leukemia`
    ///
    /// * **VM Long Name**: Acute Lymphoblastic Leukemia
    /// * **VM Public ID**: 3322383
    /// * **Concept Code**: C3167
    /// * **Begin Date**:   12/06/2011
    ///
    /// Leukemia with an acute onset, characterized by the presence of
    /// lymphoblasts in the bone marrow and the peripheral blood. It includes
    /// the acute B lymphoblastic leukemia and acute T lymphoblastic leukemia.
    #[serde(rename = "Lymphoblastic Leukemia")]
    LymphoblasticLeukemia,


    /// `Hodgkin Lymphoma`
    ///
    /// * **VM Long Name**: Hodgkin Lymphoma
    /// * **VM Public ID**: 4981811
    /// * **Concept Code**: C9357
    /// * **Begin Date**:   01/19/2018
    ///
    /// A lymphoma, previously known as Hodgkin's disease, characterized by the
    /// presence of large tumor cells in an abundant admixture of nonneoplastic
    /// cells. There are two distinct subtypes: nodular lymphocyte predominant
    /// Hodgkin lymphoma and classical Hodgkin lymphoma. Hodgkin lymphoma
    /// involves primarily lymph nodes.
    #[serde(rename = "Hodgkin Lymphoma")]
    HodgkinLymphoma,


    /// `Non-Hodgkin Lymphoma`
    ///
    /// * **VM Long Name**: Non-Hodgkin lymphoma
    /// * **VM Public ID**: 2568326
    /// * **Concept Code**: C3211
    /// * **Begin Date**:   04/13/2011
    ///
    /// Distinct from Hodgkin lymphoma both morphologically and biologically,
    /// Non-Hodgkin lymphoma (NHL) is characterized by the absence of Reed-Sternberg
    /// cells, can occur at any age, and usually presents as a localized or
    /// generalized lymphadenopathy associated with fever and weight loss.
    /// The clinical course varies according to the morphologic type.
    /// NHL is clinically classified as indolent, aggressive, or having a
    /// variable clinical course. NHL can be of B-or T-/NK-cell lineage.
    #[serde(rename = "Non-Hodgkin Lymphoma")]
    NonHodgkinLymphoma,


    /// `Lymphoproliferative Diseases`
    ///
    /// * **VM Long Name**: Lymphoproliferative Disorder
    /// * **VM Public ID**: 3235573
    /// * **Concept Code**: C9308
    /// * **Begin Date**:   10/20/2025
    ///
    /// A disorder characterized by proliferation of lymphocytes at various
    /// stages of differentiation. Lymphoproliferative disorders can be
    /// neoplastic (clonal, as in lymphomas and leukemias) or
    /// reactive (polyclonal, as in infectious mononucleosis). --2004
    #[serde(rename = "Lymphoproliferative Diseases")]
    LymphoproliferativeDiseases,


    /// `Soft Tissue Tumors`
    ///
    /// * **VM Long Name**: Soft Tissue Sarcoma
    /// * **VM Public ID**: 3384698
    /// * **Concept Code**: C9306
    /// * **Begin Date**:   01/19/2018
    ///
    /// A malignant mesenchymal neoplasm arising from muscle tissue,
    /// adipose tissue, blood vessels, fibrous tissue, or other supportive
    /// tissues excluding the bones.
    #[serde(rename = "Soft Tissue Tumors")]
    SoftTissueTumors,


    /// `Neuroblastoma`
    ///
    /// * **VM Long Name**: Neuroblastoma
    /// * **VM Public ID**: 4265259
    /// * **Concept Code**: C3270
    /// * **Begin Date**:   01/19/2018
    ///
    /// A neuroblastic tumor characterized by the presence of neuroblastic
    /// cells, the absence of ganglion cells, and the absence of a prominent
    /// Schwannian stroma formation.
    #[serde(rename = "Neuroblastoma")]
    Neuroblastoma,


    /// `Osteosarcoma`
    ///
    /// * **VM Long Name**: Osteosarcoma
    /// * **VM Public ID**: 2573004
    /// * **Concept Code**: C9145
    /// * **Begin Date**:   04/10/2019
    ///
    /// A usually aggressive malignant bone-forming mesenchymal neoplasm,
    /// predominantly affecting adolescents and young adults. It usually
    /// involves bones and less frequently extraosseous sites. It often
    /// involves the long bones (particularly distal femur, proximal tibia,
    /// and proximal humerus). Pain with or without a palpable mass is the
    /// most frequent clinical symptom. It may spread to other anatomic sites,
    /// particularly the lungs.
    #[serde(rename = "Osteosarcoma")]
    Osteosarcoma,


    /// `Renal Tumors`
    ///
    /// * **VM Long Name**: Renal Neoplasm
    /// * **VM Public ID**: 3210726
    /// * **Concept Code**: C3150
    /// * **Begin Date**:   01/19/2018
    ///
    /// A benign or malignant neoplasm affecting the kidney. Representative
    /// examples of benign renal neoplasms include fibroma, lipoma, oncocytoma,
    /// and juxtaglomerular cell tumor. Representative examples of malignant
    /// renal neoplasms include renal cell carcinoma, renal pelvis carcinoma,
    /// Wilms tumor, rhabdoid tumor, sarcoma, and lymphoma.
    #[serde(rename = "Renal Tumors")]
    RenalTumors,


    /// `Germ Cell Tumors`
    ///
    /// * **VM Long Name**: Germ Cell Tumor
    /// * **VM Public ID**: 4265239
    /// * **Concept Code**: C3708
    /// * **Begin Date**:   10/21/2019
    ///
    /// A benign or malignant, gonadal or extragonadal neoplasm that originates
    /// from germ cells. Representative examples include teratoma, seminoma,
    /// embryonal carcinoma, and yolk sac tumor.
    #[serde(rename = "Germ Cell Tumors")]
    GermCellTumors,


    /// `Ewings Sarcoma`
    ///
    /// * **VM Long Name**: Ewing Sarcoma
    /// * **VM Public ID**: 2593150
    /// * **Concept Code**: C4817
    /// * **Begin Date**:   07/31/2023
    ///
    /// A small round cell tumor that lacks morphologic, immunohistochemical,
    /// and electron microscopic evidence of neuroectodermal differentiation.
    /// It represents one of the two ends of the spectrum called
    /// Ewing sarcoma/peripheral neuroectodermal tumor. It affects mostly males
    /// under age 20, and it can occur in soft tissue or bone. Pain and the
    /// presence of a mass are the most common clinical symptoms.
    #[serde(rename = "Ewings Sarcoma")]
    EwingsSarcoma,


    /// `Liver Tumors`
    ///
    /// * **VM Long Name**: Liver Neoplasm
    /// * **VM Public ID**: 16607973
    /// * **Concept Code**: C7103
    /// * **Begin Date**:   10/20/2025
    ///
    /// A benign, premalignant, or malignant neoplasm that affects the liver
    /// parenchyma or intrahepatic bile ducts.
    #[serde(rename = "Liver Tumors")]
    LiverTumors,


    /// `Other Gliomas`
    ///
    /// * **VM Long Name**: Other Glioma
    /// * **VM Public ID**: 16965690
    /// * **Concept Code**: C3059
    /// * **Begin Date**:   01/21/2026
    ///
    /// Different than the one(s) previously specified or mentioned.
    /// A benign or malignant brain and spinal cord tumor that arises from
    /// glial cells (astrocytes, oligodendrocytes, ependymal cells).
    /// Tumors that arise from astrocytes are called astrocytic tumors or astrocytomas.
    /// Tumors that arise from oligodendrocytes are called oligodendroglial tumors.
    /// Tumors that arise from ependymal cells are called ependymomas.
    #[serde(rename = "Other Gliomas")]
    OtherGliomas,


    /// `Other Brain Tumors`
    ///
    /// * **VM Long Name**: Other Brain Neoplasm
    /// * **VM Public ID**: 16966535
    /// * **Concept Code**: C2907
    /// * **Begin Date**:   01/21/2026
    ///
    /// Different than the one(s) previously specified or mentioned.
    /// A benign or malignant neoplasm that arises from or metastasizes to the brain.
    #[serde(rename = "Other Brain Tumors")]
    OtherBrainTumors,


    /// `Other Solid Tumors`
    ///
    /// * **VM Long Name**: Other Childhood Solid Neoplasm
    /// * **VM Public ID**: 16966536
    /// * **Concept Code**: C9107
    /// * **Begin Date**:   01/21/2026
    ///
    /// Different than the one(s) previously specified or mentioned.
    /// A solid neoplasm (e.g., carcinoma, sarcoma) occurring in children.
    #[serde(rename = "Other Solid Tumors")]
    OtherSolidTumors,


    /// `Rhabdomyosarcoma`
    ///
    /// * **VM Long Name**: Rhabdomyosarcoma
    /// * **VM Public ID**: 16966538
    /// * **Concept Code**: C3359
    /// * **Begin Date**:   01/21/2026
    ///
    /// A rare aggressive malignant mesenchymal neoplasm that arises from striated
    /// (skeletal or cardiac) muscle cells. It usually occurs in children and young adults.
    #[serde(rename = "Rhabdomyosarcoma")]
    Rhabdomyosarcoma,


    /// `Rhabdoid Tumors`
    ///
    /// * **VM Long Name**: Rhabdoid Tumor
    /// * **VM Public ID**: 2962178
    /// * **Concept Code**: C3808
    /// * **Begin Date**:   01/19/2018
    ///
    /// An aggressive malignant embryonal neoplasm usually occurring during childhood.
    /// It is characterized by the presence of large cells with abundant cytoplasm,
    /// large eccentric nucleus, and a prominent nucleolus and it is associated with
    /// abnormalities of chromosome 22. It can arise from the central nervous system,
    /// kidney, and the soft tissues. The prognosis is poor.
    #[serde(rename = "Rhabdoid Tumors")]
    RhabdoidTumors,


    /// `Retinoblastoma`
    ///
    /// * **VM Long Name**: Retinoblastoma
    /// * **VM Public ID**: 16966537
    /// * **Concept Code**: C7541
    /// * **Begin Date**:   01/21/2026
    ///
    /// A malignant tumor that originates in the nuclear layer of the retina.
    /// As the most common primary tumor of the eye in children, retinoblastoma
    /// is still relatively uncommon, accounting for only 1% of all malignant tumors
    /// in pediatric patients. Approximately 95% of cases are diagnosed before age 5.
    /// These tumors may be multifocal, bilateral, congenital, inherited, or acquired.
    /// Seventy-five percent of retinoblastomas are unilateral; 60% occur sporadically.
    /// A predisposition to retinoblastoma has been associated with 13q14 cytogenetic
    /// abnormalities. Patients with the inherited form also appear to be at increased
    /// risk for secondary nonocular malignancies such as osteosarcoma,
    /// malignant fibrous histiocytoma, and fibrosarcoma.
    #[serde(rename = "Retinoblastoma")]
    Retinoblastoma,


    /// `Endocrine and Neuroendocrine Tumors`
    ///
    /// * **VM Long Name**: Endocrine Neoplasm And Neuroendocrine Neoplasm
    /// * **VM Public ID**: 16966539
    /// * **Concept Code**: C3809
    /// * **Begin Date**:   01/21/2026
    ///
    /// A benign or malignant neoplasm that affects the endocrine glands.
    /// An article which expresses the relation of connection or addition.
    /// It is used to conjoin a word with a word, a clause with a clause,
    /// or a sentence with a sentence. An epithelial neoplasm with neuroendocrine
    /// differentiation. This category includes neuroendocrine tumors,
    /// neuroendocrine carcinomas, and paragangliomas.
    #[serde(rename = "Endocrine and Neuroendocrine Tumors")]
    EndocrineAndNeuroendocrineTumors,


    /// `Other Hematopoietic Tumors`
    ///
    /// * **VM Long Name**: Other Hematopoietic and Lymphoid Cell Neoplasm
    /// * **VM Public ID**: 16966540
    /// * **Concept Code**: C27134
    /// * **Begin Date**:   01/21/2026
    ///
    /// Different than the one(s) previously specified or mentioned.
    /// A neoplasm that arises from hematopoietic and lymphoid cells.
    /// Representative examples include myeloproliferative neoplasms,
    /// myelodysplastic syndromes, leukemias, Hodgkin lymphomas, and non-Hodgkin lymphomas.
    #[serde(rename = "Other Hematopoietic Tumors")]
    OtherHematopoieticTumors,


}

impl CDE for DiagnosisCategory {}

impl std::fmt::Display for DiagnosisCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosisCategory::AtypicalTeratoidRhabdoidTumors => write!(f, "Atypical Teratoid/Rhabdoid Tumors"),
            DiagnosisCategory::ChoroidPlexusTumors => write!(f, "Choroid Plexus Tumors"),
            DiagnosisCategory::CnsGermCellTumors => write!(f, "CNS Germ Cell Tumors"),
            DiagnosisCategory::CnsSarcomas => write!(f, "CNS Sarcomas"),
            DiagnosisCategory::Craniopharyngiomas => write!(f, "Craniopharyngiomas"),
            DiagnosisCategory::Ependymoma => write!(f, "Ependymoma"),
            DiagnosisCategory::GlioneuronalAndNeuronalTumors => write!(f, "Glioneuronal and Neuronal Tumors"),
            DiagnosisCategory::HighGradeGlioma => write!(f, "High-Grade Glioma"),
            DiagnosisCategory::LowGradeGliomas => write!(f, "Low-Grade Gliomas"),
            DiagnosisCategory::Medulloblastoma => write!(f, "Medulloblastoma"),
            DiagnosisCategory::OtherCnsEmbryonalTumors => write!(f, "Other CNS Embryonal Tumors"),
            DiagnosisCategory::MyeloidLeukemia => write!(f, "Myeloid Leukemia"),
            DiagnosisCategory::LymphoblasticLeukemia => write!(f, "Lymphoblastic Leukemia"),
            DiagnosisCategory::HodgkinLymphoma => write!(f, "Hodgkin Lymphoma"),
            DiagnosisCategory::NonHodgkinLymphoma => write!(f, "Non-Hodgkin Lymphoma"),
            DiagnosisCategory::LymphoproliferativeDiseases => write!(f, "Lymphoproliferative Diseases"),
            DiagnosisCategory::SoftTissueTumors => write!(f, "Soft Tissue Tumors"),
            DiagnosisCategory::Neuroblastoma => write!(f, "Neuroblastoma"),
            DiagnosisCategory::Osteosarcoma => write!(f, "Osteosarcoma"),
            DiagnosisCategory::RenalTumors => write!(f, "Renal Tumors"),
            DiagnosisCategory::GermCellTumors => write!(f, "Germ Cell Tumors"),
            DiagnosisCategory::EwingsSarcoma => write!(f, "Ewings Sarcoma"),
            DiagnosisCategory::LiverTumors => write!(f, "Liver Tumors"),
            DiagnosisCategory::OtherGliomas => write!(f, "Other Gliomas"),
            DiagnosisCategory::OtherBrainTumors => write!(f, "Other Brain Tumors"),
            DiagnosisCategory::OtherSolidTumors => write!(f, "Other Solid Tumors"),
            DiagnosisCategory::Rhabdomyosarcoma => write!(f, "Rhabdomyosarcoma"),
            DiagnosisCategory::RhabdoidTumors => write!(f, "Rhabdoid Tumors"),
            DiagnosisCategory::Retinoblastoma => write!(f, "Retinoblastoma"),
            DiagnosisCategory::EndocrineAndNeuroendocrineTumors => write!(f, "Endocrine and Neuroendocrine Tumors"),
            DiagnosisCategory::OtherHematopoieticTumors => write!(f, "Other Hematopoietic Tumors"),
        }
    }
}

impl Distribution<DiagnosisCategory> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DiagnosisCategory {
        match rng.gen_range(0..=30) {            
            0 => DiagnosisCategory::AtypicalTeratoidRhabdoidTumors,
            1 => DiagnosisCategory::ChoroidPlexusTumors,
            2 => DiagnosisCategory::CnsGermCellTumors,
            3 => DiagnosisCategory::CnsSarcomas,
            4 => DiagnosisCategory::Craniopharyngiomas,
            5 => DiagnosisCategory::Ependymoma,
            6 => DiagnosisCategory::GlioneuronalAndNeuronalTumors,
            7 => DiagnosisCategory::HighGradeGlioma,
            8 => DiagnosisCategory::LowGradeGliomas,
            9 => DiagnosisCategory::Medulloblastoma,
            10 => DiagnosisCategory::OtherCnsEmbryonalTumors,
            11 => DiagnosisCategory::MyeloidLeukemia,
            12 => DiagnosisCategory::LymphoblasticLeukemia,
            13 => DiagnosisCategory::HodgkinLymphoma,
            14 => DiagnosisCategory::NonHodgkinLymphoma,
            15 => DiagnosisCategory::LymphoproliferativeDiseases,
            16 => DiagnosisCategory::SoftTissueTumors,
            17 => DiagnosisCategory::Neuroblastoma,
            18 => DiagnosisCategory::Osteosarcoma,
            19 => DiagnosisCategory::RenalTumors,
            20 => DiagnosisCategory::GermCellTumors,
            21 => DiagnosisCategory::EwingsSarcoma,
            22 => DiagnosisCategory::LiverTumors,
            23 => DiagnosisCategory::OtherGliomas,
            24 => DiagnosisCategory::OtherBrainTumors,
            25 => DiagnosisCategory::OtherSolidTumors,
            26 => DiagnosisCategory::Rhabdomyosarcoma,
            27 => DiagnosisCategory::RhabdoidTumors,
            28 => DiagnosisCategory::Retinoblastoma,
            29 => DiagnosisCategory::EndocrineAndNeuroendocrineTumors,
            _ => DiagnosisCategory::OtherHematopoieticTumors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(DiagnosisCategory::AtypicalTeratoidRhabdoidTumors.to_string(), "Atypical Teratoid/Rhabdoid Tumors");
        assert_eq!(DiagnosisCategory::ChoroidPlexusTumors.to_string(), "Choroid Plexus Tumors");
        assert_eq!(DiagnosisCategory::CnsGermCellTumors.to_string(), "CNS Germ Cell Tumors");
        assert_eq!(DiagnosisCategory::CnsSarcomas.to_string(), "CNS Sarcomas");
        assert_eq!(DiagnosisCategory::Craniopharyngiomas.to_string(), "Craniopharyngiomas");
        assert_eq!(DiagnosisCategory::Ependymoma.to_string(), "Ependymoma");
        assert_eq!(DiagnosisCategory::GlioneuronalAndNeuronalTumors.to_string(), "Glioneuronal and Neuronal Tumors");
        assert_eq!(DiagnosisCategory::HighGradeGlioma.to_string(), "High-Grade Glioma");
        assert_eq!(DiagnosisCategory::LowGradeGliomas.to_string(), "Low-Grade Gliomas");
        assert_eq!(DiagnosisCategory::Medulloblastoma.to_string(), "Medulloblastoma");
        assert_eq!(DiagnosisCategory::OtherCnsEmbryonalTumors.to_string(), "Other CNS Embryonal Tumors");
        assert_eq!(DiagnosisCategory::MyeloidLeukemia.to_string(), "Myeloid Leukemia");
        assert_eq!(DiagnosisCategory::LymphoblasticLeukemia.to_string(), "Lymphoblastic Leukemia");
        assert_eq!(DiagnosisCategory::HodgkinLymphoma.to_string(), "Hodgkin Lymphoma");
        assert_eq!(DiagnosisCategory::NonHodgkinLymphoma.to_string(), "Non-Hodgkin Lymphoma");
        assert_eq!(DiagnosisCategory::LymphoproliferativeDiseases.to_string(), "Lymphoproliferative Diseases");
        assert_eq!(DiagnosisCategory::SoftTissueTumors.to_string(), "Soft Tissue Tumors");
        assert_eq!(DiagnosisCategory::Neuroblastoma.to_string(), "Neuroblastoma");
        assert_eq!(DiagnosisCategory::Osteosarcoma.to_string(), "Osteosarcoma");
        assert_eq!(DiagnosisCategory::RenalTumors.to_string(), "Renal Tumors");
        assert_eq!(DiagnosisCategory::GermCellTumors.to_string(), "Germ Cell Tumors");
        assert_eq!(DiagnosisCategory::EwingsSarcoma.to_string(), "Ewings Sarcoma");
        assert_eq!(DiagnosisCategory::LiverTumors.to_string(), "Liver Tumors");
        assert_eq!(DiagnosisCategory::OtherGliomas.to_string(), "Other Gliomas");
        assert_eq!(DiagnosisCategory::OtherBrainTumors.to_string(), "Other Brain Tumors");
        assert_eq!(DiagnosisCategory::OtherSolidTumors.to_string(), "Other Solid Tumors");
        assert_eq!(DiagnosisCategory::Rhabdomyosarcoma.to_string(), "Rhabdomyosarcoma");
        assert_eq!(DiagnosisCategory::RhabdoidTumors.to_string(), "Rhabdoid Tumors");
        assert_eq!(DiagnosisCategory::Retinoblastoma.to_string(), "Retinoblastoma");
        assert_eq!(DiagnosisCategory::EndocrineAndNeuroendocrineTumors.to_string(), "Endocrine and Neuroendocrine Tumors");
        assert_eq!(DiagnosisCategory::OtherHematopoieticTumors.to_string(), "Other Hematopoietic Tumors");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::AtypicalTeratoidRhabdoidTumors).unwrap(),
            "\"Atypical Teratoid/Rhabdoid Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::ChoroidPlexusTumors).unwrap(),
            "\"Choroid Plexus Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::CnsGermCellTumors).unwrap(),
            "\"CNS Germ Cell Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::CnsSarcomas).unwrap(),
            "\"CNS Sarcomas\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Craniopharyngiomas).unwrap(),
            "\"Craniopharyngiomas\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Ependymoma).unwrap(),
            "\"Ependymoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::GlioneuronalAndNeuronalTumors).unwrap(),
            "\"Glioneuronal and Neuronal Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::HighGradeGlioma).unwrap(),
            "\"High-Grade Glioma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::LowGradeGliomas).unwrap(),
            "\"Low-Grade Gliomas\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Medulloblastoma).unwrap(),
            "\"Medulloblastoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::OtherCnsEmbryonalTumors).unwrap(),
            "\"Other CNS Embryonal Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::MyeloidLeukemia).unwrap(),
            "\"Myeloid Leukemia\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::LymphoblasticLeukemia).unwrap(),
            "\"Lymphoblastic Leukemia\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::HodgkinLymphoma).unwrap(),
            "\"Hodgkin Lymphoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::NonHodgkinLymphoma).unwrap(),
            "\"Non-Hodgkin Lymphoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::LymphoproliferativeDiseases).unwrap(),
            "\"Lymphoproliferative Diseases\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::SoftTissueTumors).unwrap(),
            "\"Soft Tissue Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Neuroblastoma).unwrap(),
            "\"Neuroblastoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Osteosarcoma).unwrap(),
            "\"Osteosarcoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::RenalTumors).unwrap(),
            "\"Renal Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::GermCellTumors).unwrap(),
            "\"Germ Cell Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::EwingsSarcoma).unwrap(),
            "\"Ewings Sarcoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::LiverTumors).unwrap(),
            "\"Liver Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::OtherGliomas).unwrap(),
            "\"Other Gliomas\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::OtherBrainTumors).unwrap(),
            "\"Other Brain Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::OtherSolidTumors).unwrap(),
            "\"Other Solid Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Rhabdomyosarcoma).unwrap(),
            "\"Rhabdomyosarcoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::RhabdoidTumors).unwrap(),
            "\"Rhabdoid Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::Retinoblastoma).unwrap(),
            "\"Retinoblastoma\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::EndocrineAndNeuroendocrineTumors).unwrap(),
            "\"Endocrine and Neuroendocrine Tumors\""
        );
        assert_eq!(
            serde_json::to_string(&DiagnosisCategory::OtherHematopoieticTumors).unwrap(),
            "\"Other Hematopoietic Tumors\""
        );
    }
}
