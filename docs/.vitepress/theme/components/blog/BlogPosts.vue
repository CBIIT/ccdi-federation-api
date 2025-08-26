<template>
  <div>
    <h1>Blog</h1>

    <ul>
      <BlogEntry
        title="Introducing the Federation API"
        author="The CCDI Federation Developers"
        link="/ccdi-federation-api/blog/09-25-2024-introducing-the-federation-api"
      />
    </ul>
    <ul>
      <BlogEntry1
        title="Federation Resource API with CPI"
        author="The CCDI Federation Resource API Team"
        link="/ccdi-federation-api/blog/07-23-2025-the-federation-api-cpi"
      />
    </ul>
    <ul>
      <BlogEntry2
        title="CCDI ecDNA joins the Federation API"
        author="The CCDI Federation Resource API Team"
        link="/ccdi-federation-api/blog/08-06-2025-the-federation-api-ecdna"
      />
    </ul>
    <ul>
      <BlogEntry3
        title="New Diagnosis Endpoints"
        author="The CCDI Federation Resource API Team"
        link="/ccdi-federation-api/blog/08-26-2025-the-federation-api-diagnosis"
      />
    </ul>
  </div>
</template>
<script setup lang="ts">
import BlogEntry from "./BlogEntry.vue";
import BlogEntry1 from "./BlogEntry1.vue";
import BlogEntry2 from "./BlogEntry2.vue";
import BlogEntry3 from "./BlogEntry3.vue";
</script>

<style scoped></style>
