<script setup lang="ts">
import * as d3 from "d3";
import {onMounted, computed, defineAsyncComponent, Ref, ref, watch} from "vue"
import { inBrowser } from 'vitepress';

import ApiAggr from "@/src/api-aggr";

const ApiCallBlockAggr = inBrowser
  ? defineAsyncComponent(() => import('@/theme/components/api-call-diagnosis/Block.vue'))
  : () => null;

import DataTable from '@/theme/components/api-call-diagnosis/DataTable.vue';
import DataTableSample from '@/theme/components/api-call-diagnosis/DataTableSample.vue';
//import * as Graph from "./07-23-2025-the-federation-api-cpi/graph";
//import GraphPlaceholder from "./07-23-2025-the-federation-api-cpi/GraphPlaceholder.vue";
import { useDataStore } from "./08-26-2025-the-federation-api-diagnosis/store";

let api = new ApiAggr();
const data = useDataStore();

</script>
<ClientOnly>
<div class="text-4xl font-extrabold">New Diagnosis Endpoints to Improve Disease Querying</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">The CCDI Federation Resource API Team</div>
<div class="dark:text-slate-400 text-slate-800 italic">August 26, 2025</div>

The CCDI Data Federation is making it easier for researchers and clinicians to explore disease-related data. To support this goal, we have introduced new “experimental” search endpoints that make it easier to find data by diagnosis name from all current participating nodes. 

## Key Features {#key-features}

>**Simple search by diagnosis**: quickly search and filter patient samples and subject records by diagnosis using flexible, case-insensitive partial matches.

>**Clearer diagnosis listings**: subject records now include a dedicated field that lists all associated diagnoses, summarizing key information in one place for easier reference.

## How It Works {how-it-works}

The following endpoints now support disease at diagnosis exploration:

### Sample Diagnosis Endpoint

-	Endpoint: `/sample-diagnosis?search=TERM`
-	Accepts a required search term for diagnosis.
-	Performs case-insensitive, substring matching on both harmonized and unharmonized terms.
-	All existing sample endpoint parameters are supported.
-	Output matches the structure of the standard sample endpoint.

### Subject Diagnosis Endpoint

-	Endpoint: `/subject-diagnosis?search=TERM`
-	Accepts a required search term for diagnosis.
-	Searches all associated diagnoses for each subject (subjects can have multiple diagnoses).
-	Similar to sample diagnosis endpoint, performs case-insensitive substring matching on both harmonized and unharmonized terms.
-	All existing subject endpoint parameters are supported.
-	Output matches the structure of the standard subject endpoint.

### Associated Diagnosis Metadata
-	Add a harmonized "associated_diagnoses" field to the subject response. This field is an array of the diagnosis terms, making it easy to understand all diagnoses linked with a subject at a glance.
-	All diagnoses must be included in "associated_diagnoses" array for the subject, supporting more comprehensive queries.

### Example 1: Subject Counts by Diagnosis
Retrieve summary counts and additional metadata using the /subject-diagnosis endpoint. Results include a JSON object listing diagnoses alongside counts of unique participants associated with each diagnosis. This is helpful for identifying which pediatric cancer subtypes are most prevalent in the federation’s data.

<ApiCallBlockAggr
  description="API request: Group subjects by associated diagnosis"
  method="GET"
  path="/subject/by/associated_diagnoses/count"
/>

In this next example, retrieve the first 10 subjects with diagnoses matching "neuroblastoma". 

<ApiCallBlockAggr
  description="API request: Neuroblastoma subjects (10 results per node)"
  method="GET"
  path="/subject-diagnosis?search=neuroblastoma&page=1&per_page=10"
/>

#### Search Subject Diagnoses Data Table {#cpi-response-data}

<DataTable :endpoint="'https://federation.ccdi.cancer.gov/api/v1/subject-diagnosis?search=neuroblastoma&page=1&per_page=10'" :payload="Subject"/>

### Example 2: Sample Counts by Diagnosis
Retrieve summary counts and additional metadata using the `/sample-diagnosis` endpoint.

<ApiCallBlockAggr
  description="API request: Summary counts"
  method="GET"
  path="/sample/by/diagnosis/count"
/>

In this next example, use the `/sample-diagnosis` endpoint to retrieve detailed sample information for the first 6 samples that match for term “glioma”. 

<ApiCallBlockAggr
  description="API Request: Sample diagnosis “glioma” (6 results per node)"
  method="GET"
  path="/sample-diagnosis?search=glioma&page=1&per_page=6"
/>

#### Search Sample Diagnoses Data Table {#table-response-data-1}
<DataTableSample :endpoint="'https://federation.ccdi.cancer.gov/api/v1/sample-diagnosis?search=glioma&per_page=6&page=1'" :payload="Sample"/>

</ClientOnly>
