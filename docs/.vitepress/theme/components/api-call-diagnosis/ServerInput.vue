<template>
  <!-- API Call Block / Form / Server Label -->
  <div class="flex items-center justify-between text-sm">
    <label
      for="url"
      class="block mb-2 font-medium text-slate-900 dark:text-[#DFDFD7]"
    >
      Server URL
    </label>
    <span class="mr-1">
      <div
        v-if="!recentlyCopiedUrl"
        @click="copyUrl()"
        class="flex text-blue-500 text-xs ml-4 cursor-pointer select-none"
      >
        <ClipboardIcon class="w-4 mr-1" />
        Copy URL
      </div>
      <div
        v-if="recentlyCopiedUrl"
        class="flex text-blue-500 text-xs ml-4 cursor-pointer select-none"
      >
        <CheckIcon class="w-4 mr-1 text-green-600" />
        Copied
      </div>
    </span>
  </div>

  <!-- API Call Block / Form / Server Input -->
  <div
    class="flex bg-slate-50 dark:bg-[#24292F] border border-solid border-slate-200 dark:border-[#1F2327] mb-6 rounded-lg focus-within:border-blue-500 focus-within:ring-2 focus-within:ring-blue-500 overflow-hidden"
  >
    <!-- Globe Icon -->
    <div class="inset-y-0 start-0 flex pointer-events-none">
      <div
        class="inline-flex items-center px-2 text-sm text-slate-900 border-r dark:border-[#202128] bg-slate-100 dark:bg-[#24292F] rounded-e-0 rounded-s-lg"
      >
        <GlobeEuropeAfricaIcon
          class="h-5 w-5 text-slate-500 dark:text-slate-400"
        />
      </div>
    </div>

    <div
      v-if="props.method === HttpMethod.Get"
      class="flex items-center border-2 border-green-600 bg-green-500 text-white text-xs font-mono font-extrabold px-2 cursor-default"
    >
      GET
    </div>

    <!-- Server Text Box-->
    <div class="flex grow overflow-hidden">
      <input
        id="url"
        class="text-slate-900 dark:text-[#DFDFD7] text-sm w-full p-2.5"
        v-model="apiStore.host"
        placeholder="https://federation.ccdi.cancer.gov/api/v1"
      />
      <button
        class="border-solid border-r border-gray-200 dark:border-[#202128] pr-2"
        @click="dropdown = !dropdown"
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 9l-7 7-7-7"
          ></path>
        </svg>
      </button>
      <div
        class="bg-slate-100 dark:bg-[#24292F] flex items-center px-2 font-mono font-semibold text-nowrap text-sm dark:text-[#DFDFD7]"
      >
        {{ props.path }}
      </div>
    </div>
    <div
      v-if="dropdown"
      class="absolute right-0 mt-12 bg-white dark:bg-[#202128] border border-slate-200 dark:border-[#2E2E32] rounded-lg shadow-lg z-10"
    >
      <ul class="!px-2 !mt-2">
        <li
          v-for="[name, server] in Object.entries(defaultServers)"
          :key="name"
          @click="
            apiStore.setHost(server.url);
            dropdown = false;
          "
          class="text-sm px-2 py-2 hover:bg-slate-100 dark:hover:bg-[#24292F] cursor-pointer list-none"
        >
          <div class="text-md font-bold text-slate-900 dark:text-[#D7D7D7]">
            {{ name }}
            <span
              class="ml-0.5 px-1 py-0.5 bg-green-500 dark:bg-green-700 rounded-full text-[#DFDFD7]"
              style="font-size: 10px"
              >v1</span
            >
          </div>
          <span class="text-sm font-mono">{{ server.url }}</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  ClipboardIcon,
  CheckIcon,
  GlobeEuropeAfricaIcon,
} from "@heroicons/vue/24/outline";

import { defaultServers, HttpMethod } from "@/src/api-aggr";
import { useApiStore } from "@/stores/apiStore";

const props = defineProps({
  method: {
    type: String as unknown as () => HttpMethod,
    required: true,
    default: HttpMethod.Get,
    validator: (value: string): boolean =>
      Object.values(HttpMethod).includes(value as HttpMethod),
  },
  path: {
    type: String,
    required: true,
  },
});

const dropdown = ref(false);
const apiStore = useApiStore();
const recentlyCopiedUrl = ref(false);

function copyUrl() {
  if (!navigator.clipboard) {
    alert(
      "Your browser does not support copying. \
      Either manually copy the text or try another browser."
    );
    return;
  }

  let url = apiStore.getPath(props.path);
  navigator.clipboard.writeText(url);

  recentlyCopiedUrl.value = true;

  setTimeout(() => {
    recentlyCopiedUrl.value = false;
  }, 4000);
}
</script>

<style scoped></style>
