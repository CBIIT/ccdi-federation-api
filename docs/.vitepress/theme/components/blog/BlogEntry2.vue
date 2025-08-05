<template>
    <li>
      <a :href="link"
        ><b>{{ title }}</b></a
      >
      by {{ author }}.
    </li>
</template>

<script setup lang="ts">
defineProps({
  title: {
    type: String,
    required: true,
  },
  author: {
    type: String,
    required: true,
  },
  link: {
    type: String,
    required: true,
  },
});
</script>

<style scoped></style>
