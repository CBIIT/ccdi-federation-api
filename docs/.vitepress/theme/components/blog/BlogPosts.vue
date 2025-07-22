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
        author="The CCDI Federation Developers"
        link="/ccdi-federation-api/blog/07-23-2025-the-federation-api-cpi"
      />
    </ul>
  </div>
</template>

<script setup lang="ts">
import BlogEntry from "./BlogEntry.vue";
import BlogEntry1 from "./BlogEntry1.vue";
</script>

<style scoped></style>
