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
<div class="text-4xl font-extrabold">CCDI Federation API with CPI</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">The CCDI Federation Developers</div>
<div class="dark:text-slate-400 text-slate-800 italic">June 25, 2025</div>

In this blog post, we consider a new API endpoint "subject-mapping" implemented by [the Federation Resource API][spec-aggr]. You can query the API live and get
responses back—we hope you'll try them out! This endpoint integrates [the Federation Resource API][spec-aggr] with [CCDI Participant Index (CPI)][CPI-main].

For more information about the broader initiative, please visit [the dedicated
site][ccdi-main] for the Childhood Cancer Data Initiative.

For more information about CCDI Participant Index, please visit [CCDI Participant Index (CPI)][CPI-spec] website.

## Overview

The **CCDI Data Federation Resource**
  ([link](https://cbiit.github.io/ccdi-federation-api-aggregation/)) is a
  service developed and operated by the NCI that pulls data from each federation
  member's source server and makes that data available via a single API call.
  The source code for this service is available on
  [GitHub](https://github.com/cbiit/ccdi-federation-api-aggregation).

To date, the following federation members have implemented `v1.1.0` of the
specification.

| Project                                 | Organization                          |                         Server URL                         |
| :-------------------------------------- | :------------------------------------ | :--------------------------------------------------------: |
| [Kids First DRC]                        | Children's Hospital of Philadelphia   |     [Link](https://ccdi.kidsfirstdrc.org/api/v1/info)      |
| [Pediatric Cancer Data Commons]         | University of Chicago                 | [Link](https://ccdifederation.pedscommons.org/api/v1/info) |
| [St. Jude Cloud]                        | St. Jude Children's Research Hospital |       [Link](https://ccdi.stjude.cloud/api/v1/info)        |
| [Treehouse Childhood Cancer Initiative] | University of California Santa Cruz   |   [Link](https://ccdi.treehouse.gi.ucsc.edu/api/v1/info)   |
| [Childhood Cancer Catalog of ecDNA CCDI API server] | Sanford Burnham Prebys Medical Discovery Institute   |   [Link](https://ccdi-ecdna.org/ccdi-federation/api/v1/info)   |


## Exploring the API

The complete list of API endpoints provided are listed in [the
specification][spec-aggr]. You can access the `/info` endpoint to learn about the source
servers, including the server's name, the version of the
specification it implements, and when data was last updated within the server.

<ApiCallBlockAggr
  description="Query information about a source server within the federation."
  method="GET"
  path="/info"
/>

### Subject Mapping: Federation Integration with the CCDI Participant Index Service
The CPI finds the participants' IDs retrieved by the Federation based on request search parameters and returns CPI search response with domain ID/ID pairs and all mapped organizational domain ID/ID pairs for each found participants ID. You can find more information in [the CPI specification][getAssociatedParticipantIds].

<ApiCallBlockAggr
  description="Map the first two subjects participants IDs known about by each source server where sex is F."
  method="GET"
  path="/subject-mapping?per_page=2&sex=F"
/>

### Summary

With these examples laid out in this blog post, we hope we've convinced you that
a real-time, federated data ecosystem can dramatically improve data discovery
time (and, subsequentially, time to knowledge generation). To reiterate, all of
the data displayed in this blog post was collected from each source server
when you loaded the page. As we continue to expand the concepts that the API
covers, we hope to greatly improve the experience of discovering data within the
pediatric cancer community.

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
