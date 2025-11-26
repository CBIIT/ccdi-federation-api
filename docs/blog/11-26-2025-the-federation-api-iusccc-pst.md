<script setup lang="ts">
import {onMounted, computed, defineAsyncComponent, Ref, ref, watch} from "vue"

</script>
<ClientOnly>
<div class="text-4xl font-extrabold">IUSCCC's Pediatric Solid Tumor Program added to the
CCDI Data Federation Resource</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">The CCDI Federation Resource API Team</div>
<div class="dark:text-slate-400 text-slate-800 italic">November 26, 2025</div>


We’re pleased to spotlight the newest addition to the CCDI Data Federation: 
The Indiana Pediatric Solid Tumor program at the Indiana University Simon Comprehensive Cancer 
Center (IUSCCC).

**About The IUSCCC Pediatric Solid Tumor Program**

The Sarcoma and Solid Tumor Program at Riley Children’s Health, operated through the 
Indiana University Simon Comprehensive Cancer Center, specializes in diagnosing, 
treating, and researching childhood solid tumors - including sarcomas and kidney tumors. 
The program uses advanced care protocols and participates in collaborative research with 
the Children's Oncology Group (COG) and develops new therapies. 

**How to Access**

The CCDI Federation Resource allows users to directly access individual source nodes or 
use the aggregation capabilities of the Federation Resource to query data across all nodes 
simultaneously.  Researchers can leverage this new Data Federation member, along with 
the current CCDI Data Federation Members (KidsFirst, PCDC, St. Jude Cloud, Treehouse, 
CCDI ecDNA) as follows: 

To access the CCDI Data Federation Resource API, please click [here][spec-aggr].

To access participating nodes API, please click [here][spec-nodes].

Stay tuned for future updates as we continue to expand the CCDI Data Federation with new 
resources and tools supporting pediatric cancer research community.

For more information, visit the  [CCDI Data Federation Resource page][ResourceAPI-main].

*This announcement blog highlights a technical milestone that took place earlier this 
year. While we are publishing this blog update today, access and integration have been 
in place since September 2025. The CCDI Data Federation was expanded to fully 
incorporate the Indiana Precision Oncology CCDI API as an official, searchable 
resource alongside other Federation members.*


[spec-aggr]: https://cbiit.github.io/ccdi-federation-api-aggregation
[spec-nodes]: https://cbiit.github.io/ccdi-federation-api-spec
[ResourceAPI-main]: https://ccdi.cancer.gov/data-federation-resource
</ClientOnly>
