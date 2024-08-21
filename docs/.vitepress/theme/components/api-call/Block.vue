<template>
  <!-- API Call Block -->
  <div
    class="bg-white dark:bg-[#202128] border border-slate-200 dark:border-[#2E2E32] rounded-md shadow-md my-10 overflow-hidden"
  >
    <div
      class="flex px-5 py-3 items-center cursor-pointer select-none dark:bg-[#202128]"
      @click="isOpen = !isOpen"
    >
      <!-- Icon and call to action. -->
      <CodeBracketSquareIcon class="w-6 mr-1" />
      <span class="font-bold mr-4">Try It!</span>

      <!-- API Call Block / Header -->
      <span class="grow dark:text-[#DFDFD7]">
        {{ props.description }}
      </span>

      <!-- Chevron -->
      <ChevronDownIcon v-if="!isOpen" class="w-4" />
      <ChevronUpIcon v-if="isOpen" class="w-4" />
    </div>

    <div>
      <div
        class="flex items-center bg-cyan-100 border border-cyan-400 text-cyan-700 dark:bg-indigo-300 dark:border-indigo-600 dark:text-indigo-800 px-4 py-3 relative"
        role="alert"
        v-if="props.note && !noteClosed && isOpen"
      >
        <InformationCircleIcon class="w-10 mr-1" />
        <strong class="font-bold mr-4">Note</strong>
        <span class="grow block sm:inline">{{ props.note }}</span>
        <XMarkIcon
          class="w-10 cursor-pointer ml-5"
          @click="noteClosed = true"
        />
      </div>

      <!-- API Call Block / Form -->
      <div
        v-show="isOpen"
        class="px-5 border-t-2 border-slate-100 dark:border-[#2E2E32] pt-3"
      >
        <!-- API Call Block / Form / Server -->
        <ServerInput :method="method" :path="path" />

        <div v-if="method === HttpMethod.Get">
          <div id="get-response">
            <div
              class="flex text-sm px-5 py-3 font-mono text-slate-300 bg-[#1F2327] items-center justify-between"
            >
              <span class="bg-[#282e34] px-2 py-1 rounded-sm">Response</span>
              <div v-if="!isLoading">
                <div
                  v-if="!recentlyCopiedResponse"
                  @click="copyResponse()"
                  class="flex text-blue-500 text-xs ml-4 cursor-pointer select-none"
                >
                  <ClipboardIcon class="w-4 mr-1" />
                  Copy
                </div>
                <div
                  v-if="recentlyCopiedResponse"
                  class="flex text-blue-500 text-xs ml-4 cursor-pointer select-none"
                >
                  <CheckIcon class="w-4 mr-1 text-green-600" />
                  Copied
                </div>
              </div>
            </div>

            <!-- API Call Block / Form / Response / Editor -->
            <div class="relative">
              <div id="response-editor" ref="editorElement"></div>
              <div
                v-if="isLoading"
                class="absolute top-0 left-0 right-0 bottom-0 bg-black bg-opacity-50 flex justify-center items-center z-10"
              >
                <div class="text-center">
                  <div
                    class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-e-transparent align-[-0.125em] text-surface motion-reduce:animate-[spin_1.5s_linear_infinite] dark:text-[#DFDFD7]"
                    role="status"
                  >
                    <span
                      class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                      >Loading...</span
                    >
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div
          class="flex items-center bg-red-100 border border-red-400 text-red-700 dark:bg-red-300 dark:border-red-600 dark:text-red-800 px-4 py-3 mt-3 rounded relative"
          role="alert"
          v-if="error && isOpen"
        >
          <ExclamationCircleIcon class="w-6 mr-1" />
          <strong class="font-bold mr-4">Error</strong>
          <span class="grow block sm:inline">{{ error }}</span>
        </div>
        <div class="flex items-end justify-end" @click="dispatch(method)">
          <span
            class="my-3 p-2 text-[#DFDFD7] font-bold bg-blue-500 border border-blue-600 shadow-md rounded-lg cursor-pointer"
          >
            Submit
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, Ref, watch } from "vue";
import { createHighlighter } from "shiki";
import { shikiToMonaco } from "@shikijs/monaco";
import {
  CheckIcon,
  ChevronUpIcon,
  ChevronDownIcon,
  ClipboardIcon,
  CodeBracketSquareIcon,
  ExclamationCircleIcon,
  InformationCircleIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";
import axios from "axios";
import * as monaco from "monaco-editor";

import { HttpMethod } from "@/src/api";
import ServerInput from "@/theme/components/api-call/ServerInput.vue";
import { useApiStore } from "@/stores/apiStore";

// @ts-ignore
import JSONWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";

self.MonacoEnvironment = {
  getWorker(_workerId, _label) {
    return new JSONWorker();
  },
};

const props = defineProps({
  // A description to show in the box before it's opened.
  description: {
    type: String,
    required: true,
  },

  note: String,

  // The HTTP method for this call.
  method: {
    type: String as () => HttpMethod,
    required: true,
    default: HttpMethod.Get,
    validator: (value: string): boolean =>
      Object.values(HttpMethod).includes(value as HttpMethod),
  },

  path: {
    type: String,
    required: true,
  },

  // A JSON object to POST (if POST is selected).
  data: Object,
});

const apiStore = useApiStore();

const response = ref(JSON.stringify({}, null, 2));
const editorElement = ref<HTMLElement | null>(null);

const isLoading = ref(false);
const noteClosed = ref(false);
const isOpen = ref(false);
const recentlyCopiedResponse = ref(false);
const error = ref<string | null>(null);

let editor: monaco.editor.IStandaloneCodeEditor | null = null;

async function initEditor(
  container: HTMLElement,
  content: Ref<any>,
  options?: monaco.editor.IStandaloneEditorConstructionOptions
): Promise<monaco.editor.IStandaloneCodeEditor> {
  try {
    const highlighter = await createHighlighter({
      themes: ["github-dark"],
      langs: ["json"],
    });

    shikiToMonaco(highlighter, monaco);
  } catch (err) {
    console.error("Error creating Shiki highlighter:", err);
  }

  const editor = monaco.editor.create(container, {
    value: content.value,
    language: "json",
    scrollBeyondLastLine: false,
    theme: "github-dark",
    automaticLayout: true,
    minimap: { enabled: false },
    stickyScroll: { enabled: false },
    ...options,
  });

  container.style.height = "380px";

  return editor;
}

async function dispatch(method: HttpMethod) {
  isLoading.value = true;
  error.value = null;

  try {
    let url = apiStore.getPath(props.path);
    let result = null;

    switch (method) {
      case HttpMethod.Get:
        result = await axios.get(url, {
          validateStatus: function (_) {
            return true;
          },
        });
        response.value = JSON.stringify(result.data, null, 2);
        break;
      default:
        isLoading.value = false;
        error.value = `Request to server failed. See the log for more info.`;
        console.error("Unhandled HTTP method:", method);
        return;
    }

    isLoading.value = false;

    if (result.status !== 200) {
      error.value = `Bad response (status code: ${result.status}). Does the backend support this endpoint?`;
    }
  } catch (err) {
    isLoading.value = false;
    error.value = `Request to server failed. See the log for more info.`;
    console.error(err);
  }
}

function copyResponse() {
  if (!navigator.clipboard) {
    alert(
      "Your browser does not support copying. \
      Either manually copy the text or try another browser."
    );
    return;
  }

  if (!editor) {
    alert("Editor not found! This is an internal error.");
  }

  navigator.clipboard.writeText(editor!.getValue());
  recentlyCopiedResponse.value = true;

  setTimeout(() => {
    recentlyCopiedResponse.value = false;
  }, 4000);
}

onMounted(async () => {
  // SAFETY: this will always exist in `onMounted()`, as every variation of this
  // element provides a `responseEditor` ref.
  editor = await initEditor(editorElement.value!, response, {
    readOnly: true,
    domReadOnly: true,
  });
});

watch(response, (value) => {
  editor?.setValue(value);
});

onBeforeUnmount(() => {
  editor?.dispose();
});
</script>
