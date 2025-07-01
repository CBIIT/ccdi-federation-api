<script setup lang="ts">
import * as d3 from "d3";
import {onMounted, computed, defineAsyncComponent, Ref, ref, watch} from "vue"
import { inBrowser } from 'vitepress';

import ApiAggr from "@/src/api-aggr";

const ApiCallBlockAggr = inBrowser
  ? defineAsyncComponent(() => import('@/theme/components/api-call-aggr/Block.vue'))
  : () => null;

import * as Graph from "./06-25-2025-the-federation-api-cpi/graph";
import GraphPlaceholder from "./06-25-2025-the-federation-api-cpi/GraphPlaceholder.vue";
import { useDataStore } from "./06-25-2025-the-federation-api-cpi/store";

let api = new ApiAggr();
const data = useDataStore();
const samples: Ref<Sample[]> = ref([]);

</script>

<ClientOnly>
<div class="text-4xl font-extrabold">NEW! Subject Mapping Endpoint: Integration with the Childhood Cancer Data Initiative Participant Index (CPI)</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">The CCDI Federation Developers</div>
<div class="dark:text-slate-400 text-slate-800 italic">June 25, 2025</div>

The CCDI Data Federation is excited to announce a new API endpoint "subject-mapping" available within the Federation Resource [the Federation Resource API][spec-aggr] that links Federation member participants with the [CCDI Participant Index (CPI)][CPI-main].

The CPI’s goal is to manage and share multiple cross-linked participant IDs that represent the same individual by connecting various participant IDs from different studies/research institutions (domains). Through the subject mapping endpoint, the CPI finds the subjects known by the Federation and returns domain ID/ID pairs and all mapped organizational domain ID/ID pairs for each found subject ID.
By mappings these identifiers, CPI enables researchers to connect data associated with the same individual across different datasets. This mapping capability enhances the discovery of multimodal data, facilitating the exploration of complex research questions, and ultimately supporting the development of innovative therapies for pediatric cancers.
Additional information related to the CPI can be found in the [CPI Participant index Documentation Site][CPI-spec].  
You can query this API endpoint live and get responses back — we hope you'll try it out!

<ApiCallBlockAggr
  description="Map the first two subjects participants IDs known about by each source server where sex is F."
  method="GET"
  path="/subject-mapping?per_page=2&sex=F"
/>

<ApiCallBlockAggr
  description="Map subject participants IDs where ID is SJ000008."
  method="GET"
  path="/subject-mapping?identifiers=SJ000008"
/>

To stay updated on our progress, please star and follow the project [on
GitHub](https://github.com/cbiit/ccdi-federation-api).


[ccdi-main]: https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative
[spec]: https://cbiit.github.io/ccdi-federation-api/specification.html
[spec-aggr]: https://cbiit.github.io/ccdi-federation-api-aggregation
[Kids First DRC]: https://kidsfirstdrc.org/
[St. Jude Cloud]: https://stjude.cloud
[Pediatric Cancer Data Commons]: https://commons.cri.uchicago.edu/pcdc/
[Treehouse Childhood Cancer Initiative]: https://treehousegenomics.ucsc.edu/
[Childhood Cancer Catalog of ecDNA CCDI API server]: https://ccdi-ecdna.org/
[CPI-main]: https://ccdi.cancer.gov/ccdi-participant-index
[getAssociatedParticipantIds]: https://participantindex-docs.ccdi.cancer.gov/#operation/getAssociatedParticipantIds
[CPI-spec]: https://participantindex-docs.ccdi.cancer.gov/
</ClientOnly>
