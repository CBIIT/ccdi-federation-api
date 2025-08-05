<script setup lang="ts">
import {onMounted, computed, defineAsyncComponent, Ref, ref, watch} from "vue"

</script>
<ClientOnly>
<div class="text-4xl font-extrabold">CCDI ecDNA added to the CCDI Data Federation Resource</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">The CCDI Federation Resource API Team</div>
<div class="dark:text-slate-400 text-slate-800 italic">August 06, 2025</div>

We’re pleased to spotlight the newest addition to the CCDI Data Federation: The Childhood Cancer Catalog of Circular Extrachromosomal DNA ([CCDI ecDNA][Childhood Cancer Catalog of ecDNA CCDI API server]), developed by the Sanford Burnham Prebys Medical Discovery Institute.

<br/>
<div style="display: flex; justify-content: center;">
  <img src="./08-06-2025-the-federation-api-ecdna/Federation_CCDI_Data_Access.bf1845cb.png" alt="Diagram illustrating CCDI Data Federation users choosing to access individual source nodes such as KidsFirst, PCDC, St Jude Cloud, Treehouse, and CCDI ecDNA, or using the Federation Service to query data across all nodes at once. The diagram shows arrows connecting users to both individual nodes and the Federation Service, emphasizing flexible data access. The environment is clean and technical, with a collaborative and innovative tone. Text in the image: KidsFirst, PCDC, St Jude Cloud, Treehouse, CCDI ecDNA, Federation Service." style="width: 80%; display: block; margin: 0 auto;" />
</div>
<br/>
<div style="text-align: center; font-size: .8rem; color: #767676;">
The CCDI Federation Resource ecosystem: users can directly access individual source nodes (KidsFirst, PCDC, St. Jude Cloud, Treehouse, CCDI ecDNA) or utilize the aggregation capabilities of the Federation Service to query data across all nodes simultaneously.
</div>

## About CCDI ecDNA {#about-ccdi-ecdna}

The CCDI ecDNA platform is a dedicated pediatric cancer genomics resource focused on cataloging and analyzing circular extrachromosomal DNA (ecDNA) found in pediatric cancers. Powered by meticulously curated amplicon data from over 3,200 patients and 3,800 biosamples, all derived from whole-genome sequencing datasets contributed by major pediatric cancer databases, CCDI ecDNA is a powerful asset for researchers and clinicians interested in the role of ecDNA in tumor evolution and amplification.

## How to Access {how-to-access}

CCDI ecDNA has been included as an additional member of the Federation Resource and as a Participating node in the CCDI Data Federation. Researchers can leverage this new Data Federation member, along with the original CCDI Data Federation Members as follows:

To access the CCDI Data Federation Resource API, please click [here][spec-aggr].

To access participating nodes API, please click [here][spec-nodes].

Additional information related to the CCDI ecDNA’s API implementation is available [here][ecdna-api].

We’re thrilled to welcome CCDI ecDNA as the latest Federation member. Stay tuned for future updates as we continue to grow the CCDI Data Federation and expand the suite of tools supporting pediatric cancer research.

For more information, visit the [CCDI Data Federation Resource page][ResourceAPI-main].

_Note: This announcement blog highlights an achievement and technical milestone that took place earlier this year. While we are publishing this blog update today, access and integration have been in place since March 2025. The CCDI Data Federation was expanded in June 2025 to fully incorporate the CCDI ecDNA server as an official, searchable resource alongside other Federation members._


[onGitHub]: https://github.com/cbiit/ccdi-federation-api
[ccdi-main]: https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative
[spec]: https://cbiit.github.io/ccdi-federation-api/specification.html
[spec-aggr]: https://cbiit.github.io/ccdi-federation-api-aggregation
[spec-nodes]: https://cbiit.github.io/ccdi-federation-api-spec
[Childhood Cancer Catalog of ecDNA CCDI API server]: https://ccdi-ecdna.org/
[ecdna-api]: https://ccdi-ecdna.org/ccdi-federation-api-documentation
[ResourceAPI-main]: https://ccdi.cancer.gov/data-federation-resource
</ClientOnly>
