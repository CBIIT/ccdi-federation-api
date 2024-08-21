<script setup lang="ts">
import * as d3 from "d3";
import {onMounted, computed, defineAsyncComponent, Ref, ref, watch} from "vue"
import { inBrowser } from 'vitepress';

import Api from "@/src/api";
import { ModelsSample as Sample } from "@/src/api/core";

const ApiCallBlock = inBrowser
  ? defineAsyncComponent(() => import('@/theme/components/api-call/Block.vue'))
  : () => null;

import * as Graph from "./09-25-2024-introducing-the-federation-api/graph";
import GraphPlaceholder from "./09-25-2024-introducing-the-federation-api/GraphPlaceholder.vue";
import { useDataStore } from "./09-25-2024-introducing-the-federation-api/store";

let api = new Api();
const data = useDataStore();
const samples: Ref<Sample[]> = ref([]);

const prepareByCount = (data: any): any => {
  let result = [];

  for (const [server, files] of Object.entries(data)) {
    if (files === null) {
      continue;
    }

    for (let entry of files.values) {
      entry.label = server;
      result.push({
        group: server,
        value: entry.value,
        count: entry.count
      });
    }
  }

  return result.filter((d) => d.count > 0);
}

const plotByCount = async (id: string, title: string, inputData: any, options: any) => {
  let data = prepareByCount(inputData);

  Graph.bar(id, title, data, options);

  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (
        mutation.attributeName === 'class' &&
        mutation.target instanceof HTMLElement &&
        mutation.target === document.documentElement
      ) {
        const hasDarkClass = mutation.target.classList.contains('dark');
        Graph.bar(id, title, data, options);
      }
    });
  });

  const htmlElement = document.documentElement;
  observer.observe(htmlElement, { attributes: true, attributeFilter: ['class'] });
}

const plotSubjectByCount = async (id: string, title: string, field: string, options: any) => {
  plotByCount(id, title, await api.countSubjectsBy(field), options);
}

const plotSampleByCount = async (id: string, title: string, field: string, options: any) => {
  plotByCount(id, title, await api.countSamplesBy(field), options);
}

const plotFileByCount = async (id: string, title: string, field: string, options: any) => {
  plotByCount(id, title, await api.countFilesBy(field), options);
}

onMounted(async() => {
  // MutationObserver to detect changes to the "dark" class on the <html> element and re-render

  plotFileByCount(
    "#filesCountedByType",
    "Files Counted by Type",
    "type",
    {
      scale: 1.05
    }
  );

  plotSampleByCount(
    "#samplesCountedByStrategy",
    "Samples Counted by Library Strategy",
    "library_strategy"
  );

  plotSubjectByCount(
    "#subjectsCountedByRace",
    "Subjects Counted by Race",
    "race",
    {
      margin: {
        bottom: 150,
      },
    }
  );
});

const osteosarcomas = computed(() => {
  return samples.value.filter((s) => {
    let diagnosis = s.metadata?.diagnosis?.value || null;

    return diagnosis !== null &&
           diagnosis !== undefined &&
           diagnosis.toLowerCase().includes("osteosarcoma");
  });
});

const neuroblastomas = computed(() => {
  return samples.value.filter((s) => {
    let diagnosis = s.metadata?.diagnosis?.value || null;

    return diagnosis !== null &&
           diagnosis !== undefined &&
           diagnosis.toLowerCase().includes("neuroblastoma");
  });
})

watch([osteosarcomas, neuroblastomas], ([osteosarcomas, neuroblastomas]) => {
  let osteoAges = osteosarcomas.map((d) => d.metadata?.age_at_diagnosis?.value || null);
  let thresholds = d3.ticks(...d3.nice(...d3.extent(osteoAges), 10), 40)
})
</script>

<ClientOnly>
<div class="text-4xl font-extrabold">Introducing the CCDI Federation API</div>
<div class="text-lg mt-1 dark:text-slate-300 text-slate-900">The CCDI Federation Developers</div>
<div class="dark:text-slate-400 text-slate-800 italic">September 25, 2024</div>

Today, we're announcing the launch of the Childhood Cancer Data Initiative
(CCDI) Federation API 🎉! Foundationally, we've created this API to facilitate
real-time discovery of research data being released across the pediatric cancer
ecosystem. Our hope is that this API will enable pediatric cancer researchers
around the world to quickly identify data that is relevant to their research and
ultimately speed the advancement of cures for these rare diseases.

In this blog post, we dive into the technical underpinnings of [the API
specification][spec] including (a) considerations for the API's design, (b) the
novel ideas the API contributes regarding data discoverability and indexing, and
(c) how you can start leveraging the API today. To that end, a number of
examples exist throughout this page whereby you can query the APIs live and get
responses back—we hope you'll try them out!

For more information about the broader initiative, please visit [the dedicated
site][ccdi-main] for the Childhood Cancer Data Initiative.

## Overview

The project is split into two overarching components:

- The **CCDI Data Federation API Specification** ([link][spec]) defines the
  complete specification that distributed members (typically, organizations
  wishing to index their data within the federation) implement. The
  specification is publicly developed, and all of the discussions and source
  code are available on [GitHub](https://github.com/cbiit/ccdi-federation-api).
- The **CCDI Data Federation Resource**
  ([link](https://cbiit.github.io/ccdi-federation-api-aggregation/)) is a
  service developed and operated by the NCI that pulls data from each federation
  member's source server and makes that data available via a single API call.
  The source code for this service is similarly available on
  [GitHub](https://github.com/cbiit/ccdi-federation-api-aggregation).

The intention is that federation members independently develop, operate, and
make available a service, colloquially referred to as a "source server", that
implements the aforementioned specification. The Data Federation Resource keeps
an authoritative list of known source servers and provides a single endpoint for
querying this information distributed across the ecosystem. Users can choose
between using individual source servers or using the Data Federation Resource
according to their use-case.

To date, the following federation members have implemented `v1.0.0` of the
specification.

| Project                                 | Organization                          |                         Server URL                         |
| :-------------------------------------- | :------------------------------------ | :--------------------------------------------------------: |
| [Kids First DRC]                        | Children's Hospital of Philadelphia   |     [Link](https://ccdi.kidsfirstdrc.org/api/v1/info)      |
| [Pediatric Cancer Data Commons]         | University of Chicago                 | [Link](https://ccdifederation.pedscommons.org/api/v1/info) |
| [St. Jude Cloud]                        | St. Jude Children's Research Hospital |       [Link](https://ccdi.stjude.cloud/api/v1/info)        |
| [Treehouse Childhood Cancer Initiative] | University of California Santa Cruz   |   [Link](https://ccdi.treehouse.gi.ucsc.edu/api/v1/info)   |

Using these APIs, you can create dynamic explorations of the data available in
the CCDI ecosystem. To that end, this blog post renders graphs with the most
up-to-date data from these APIs _live_—we hope the embedded examples and graphs
demonstrate how powerful such a federated data ecosystem can be.

## Exploring the API

At present, the design of the API centers around a set of three _primary_
entities within the specification.

![An overview of the architecture of the primary
entities.](./09-25-2024-introducing-the-federation-api/architecture-overview.png)

The complete list of API endpoints provided are listed in [the
specification][spec]. That said, accessing the API is designed to be intuitive.
For example, you can access the `/info` endpoint to learn about the source
server, including things like the server's name, the version of the
specification it implements, and when data was last updated within the server.

<ApiCallBlock
  description="Query information about a source server within the federation."
  method="GET"
  path="/info"
/>

#### Subjects

Each **Subject** represents a persistent, top-level entity under which data in
the API is organized. Generally speaking, subjects are individuals from which
data was collected (i.e., participants). That said, the term "subject" was
specifically chosen to encapsulate other types of entities, such as
patient-derived xenografts, cell lines, and organoids.

::: tip NOTE
Subjects have a <a target="_blank" rel="noopener noreferrer"
href="/ccdi-federation-api/specification.html#model/modelssubjectkind">`models.subject.Kind`</a>
that allows you to determine the exact kind of each subject.
:::

<ApiCallBlock
  description="Retrieve the first ten subjects known about by each source server."
  method="GET"
  path="/subject?per_page=10"
/>

Using the various endpoints, you can easily create visualizations that
summarize, say, demographics information on the landscape of available pediatric
cancer research data. For example, here's all subjects counted by
[race](https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#race).

<GraphPlaceholder id="subjectsCountedByRace" />

#### Samples

Each **Sample** represents a biological entity measured at a single point in
time upon which an experiment has been performed. The current design is akin to
an _experimental_ sample and each entry encompasses the concepts of the physical,
biological specimen, the experiment run on that specimen, and the time point at
which the experiment was done. This approach simplifies the design for v1 of the
API but carries an important caveat: samples are not necessarily unique cases or
biospecimens, and you should be careful not to interpret them as such. As we
work towards modeling biospecimens and diagnoses more formally in future
versions of the API, we expect to improve the flexibility and conciseness with
which you can ask a variety of different questions.

::: warning

Today, querying of samples—both within a source server and across source
servers—carries some important caveats that you should be aware of. You can read
more about this in the [**Limitations**](#limitations) section at the end of the
blog post.

:::

<ApiCallBlock
  description="Retrieve the first ten samples known about by each source server."
  method="GET"
  note="The Pediatric Cancer Data Commons (PCDC) does not support sample-level
  information. Querying that endpoint with this API call will result in an error."
  path="/sample?per_page=10"
/>

With this in mind, we can similarly query all samples known by each source
server. For example, using these endpoints, we can easily do things like
observing the breakdown of omics experiments across the ecosystem.

<GraphPlaceholder id="samplesCountedByStrategy" />

### Files

**Files** represent individual data files associated with one or more samples.
Common file types include unaligned reads (`FASTQ`), aligned reads
(`BAM`/`CRAM`), and variant calling files (`VCF`/`gVCF`). Several other [file
types](https://github.com/CBIIT/ccdi-federation-api/wiki/File-Metadata-Fields#type)
are supported and are documented in more detail at each source server.

<ApiCallBlock
  description="Try getting the server info from any of the CCDI source servers."
  method="GET"
  note="The Pediatric Cancer Data Commons (PCDC) does not support file-level
  information. Querying that endpoint with this API call will result in an error."
  path="/file?per_page=1"
/>

The provided endpoints allow for multiple views into the landscape of files
within the pediatric cancer data ecosystem. For example, here's a breakdown of
the number of files for each type of data shared through the API.

<GraphPlaceholder id="filesCountedByType" />

### Gateways and Links

The primary goal of the API specification is to enable streamlined discovery of
research data shared by federation members and provide relevant metadata.
Typically, this is done through the `File` entities discussed above, but the
question of how these files may be accessed by end users still remains.

The requirements for accessing data within the ecosystem fall into three broad
categories:

- **Open access**, which means that the files can be downloaded immediately
  without authentication or authorization.
- **Registered access**, which means that authentication (but not authorization)
  is required to access the files (e.g., you have to sign up for an account, but
  you don't need any special permissions beyond that).
- **Controlled access** data, which means that data access must be explicitly
  granted through some data request process.

Each site has different file sharing procedures and generally will have
different requirements for authentication and authorization.

To capture and describe this information, the API specification introduces the
concept of a `Gateway`: a novel mechanism for defining the authentication and
authorization requirements for accessing a particular set of data. The available
gateway kinds today are `Open`, `Registered`, `Controlled`, and `Closed` (used
for files that are not _yet_ available, but will be made available at some point
in the future).

`Gateway`s wrap `Link`s, which describe how the data can be accessed once you've
fulfilled the authentication and authorization requirements for that data. There
are a variety of different avenues for data access supported by the
specification, including:

- `Direct` links, which describe an exact URL that may be accessed. This is
  useful is the most straightforward case where the data is available by
  visiting the provided URL.
- `Approximate` links, which provide a URL and then subsequent instructions
  to follow to access the data. This is useful for when (a) an exact link does
  not exist (e.g., "go to this page and use the filters on the right") and (b)
  when describing how to access data that cannot be made available through a
  single link (e.g., requesting all of the data for a particular sample).
- `Informational` links, which point to an external webpage where you can
  learn more about how to access the data. This is useful when the source server
  itself does not have direct knowledge of how to access the data, but it can
  direct you to a URL where you can learn more.
- `MailTo` links, which support listing an email address that you can
  contact about accessing the data.

That being said, this list may not be all inclusive today, and growing the list
of supported link types is an area of future discussion.

Together, the `Gateway` concept (covering _how_ to access the data) combined
with the `Link` concept (covering _where_ to access the data) provides an
expressive vocabulary for articulating how data may be accessed within the
ecosystem. Our goal is to improve the user experience of accessing data shared
by various federation members and to make it easier to build automated data
access pipelines that seamlessly download the latest pediatric cancer research
data as it becomes available. Several examples of how to assign and interpret
these `Gateway`s/`Link`s are available [in the corresponding documentation]
(/gateways-and-links).

### Limitations

Being the first version of the API specification, there are several important
notes to understand when leveraging the API regarding its limitations. We expect
the list of notes to grow as we receive feedback and questions on this blog
post.

#### Interpretation of Samples

As discussed in the description of `Sample`s above, this entity covers
essentially everything between a `Subject` and a `File`. This means that you
must take great care when interpreting the results from any `/sample` endpoints.
For example,

- When a single case has been profiled by whole-genome, whole-exome, and
  RNA-Seq, the server is expected to return three sample entries—not one.
  Importantly though, the identifier for each sample should be made unique via
  the identifier. Further, the
  [`library_strategy`](https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#library_strategy)
  can be used to understand which strategy was used to sequence each sample.
- When the same tumor-banked biospecimen is sequenced by two independent
  researchers working on different research projects and data from both projects
  is included in the source server, that server is expected to return two sample
  entries in that instance—not one. Importantly though, the identifier for the
  sample should be made unique between the two entries when also considering the
  namespace in the identifier (different projects should have different
  namespaces).

This also means that simply counting the number of _cases_ of a particular
diagnosis is not as simple as using the `GET /sample/by/diagnosis/count`
endpoint. When counting samples within a particular source server, you should
consider deduplicating samples across namespaces (if you don't want the same
biological entity sequenced across multiple projects to be counted multiple
times) and/or deduplicating by experiment (if you don't want multiple
experiments being performed against the same biological entity to be counted
multiple times). Even in doing so, a comprehensive deduplication of cases across
the ecosystem is not currently possible for a variety of other reasons,
including the fact that no facility exists to deduplicate samples across source
servers.

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

_Thank you to the following individuals who co-authored or reviewed this blog
post (in alphabetical order by last name): Holly Beale, Melissa Cline, Brian
Furner, Ellen Kephart, Geoff Lyle, Clay McLeod, Chris Nemarich, Delaram
Rahbarinia, Michael Rusch, Stephanie Sandor, Jobin Sunny, and Sam Volchenboum._

[ccdi-main]: https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative
[spec]: https://cbiit.github.io/ccdi-federation-api/specification.html
[Kids First DRC]: https://kidsfirstdrc.org/
[St. Jude Cloud]: https://stjude.cloud
[Pediatric Cancer Data Commons]: https://commons.cri.uchicago.edu/pcdc/
[Treehouse Childhood Cancer Initiative]: https://treehousegenomics.ucsc.edu/

</ClientOnly>
