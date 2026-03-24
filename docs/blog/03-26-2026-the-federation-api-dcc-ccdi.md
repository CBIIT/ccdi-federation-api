<script setup lang="ts">
import {onMounted, computed, defineAsyncComponent, Ref, ref, watch} from "vue"

</script>
<ClientOnly>
<div class="text-4xl font-extrabold">CCDI Data Now Discoverable via the Data Federation Resource</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">CCDI Federation Resource Team</div>
<div class="dark:text-slate-400 text-slate-800 italic">March 26, 2026</div>

CCDI data ([CCDI Data Node V1.0.0][dcc-root]) is now available as a new participating node in the CCDI Data Federation Resource, enabling programmatic discovery through the Federation API. This integration connects harmonized CCDI metadata to the broader federated ecosystem, allowing researchers to search participant-, sample-, and file-level metadata alongside other pediatric cancer resources (Kids First Data Resource Center, Pediatric Cancer Data Commons, St. Jude Cloud, Treehouse Childhood Cancer Initiative, CCDI ecDNA, and the Indiana Pediatric Solid Tumor Program at the Indiana University Simon Comprehensive Cancer Center (IUSCCC)). The API provides standardized metadata to support secure, cross-resource discovery while maintaining required governance and access controls.

                                
Through the Federation API, users can discover data assets across multiple participating resources through a single, unified interface. Controlled-access data remain protected and are accessible only in accordance with the policies and procedures of the participating data resources.

To access the CCDI Data Federation Resource API, please click [here][spec-aggr].

To access participating nodes API, please click [here][spec-nodes].


[dcc-root]: https://dcc.ccdi.cancer.gov
[spec-aggr]: https://cbiit.github.io/ccdi-federation-api-aggregation
[spec-nodes]: https://cbiit.github.io/ccdi-federation-api-spec
[ResourceAPI-main]: https://ccdi.cancer.gov/data-federation-resource
</ClientOnly>
