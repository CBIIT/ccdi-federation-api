<template>
  <div v-if="loading" class="loading-message" aria-live="polite" role="status"><span>Loading ...</span></div>
  <div v-else-if="error">Error: {{ error }}</div>
  <div v-else>
    <button
      class="toggle-table-btn"
      @click="showTable = !showTable"
      :aria-expanded="showTable.toString()"
      :aria-controls="'data-table-section'"
    >
      {{ showTable ? 'Hide' : 'Show' }}
    </button>
    <section v-show="showTable" id="data-table-section">
      <table class="data-table" role="table" aria-label="CPI Response Data Table">
        <caption class="sr-only">CPI Response Data Table: Domain Name, Participant ID, and Associated IDs</caption>
        <thead>
          <tr>
            <th scope="col">Domain Name</th>
            <th scope="col">Participant ID</th>
            <th scope="col">Associated IDs<br/><span class="visually-hidden">Domain : Participant</span></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in tableData" :key="item.participant_id" tabindex="0">
            <td>{{ item.domain_name }}</td>
            <td>{{ item.participant_id }}</td>
            <td>
              <ul>
                <li v-for="assoc in item.associated_ids" :key="assoc.participant_id">
                  <span>{{ assoc.domain_name }} : {{ assoc.participant_id }}</span>
                </li>
              </ul>
            </td>
          </tr>
        </tbody>
      </table>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const loading = ref(true);
const error = ref(null);
const tableData = ref([]);
const tableHeaders = ref([]);
const showTable = ref(false);

//REST endpoint example
//const REST_ENDPOINT = 'https://federation.ccdi.cancer.gov/api/v1/subject-mapping?per_page=1&sex=M'; 
const props = defineProps({
  endpoint: {
    type: String,
    required: true
  }
});
onMounted(async () => {
  try {
    const response = await fetch(props.endpoint);
    const data = await response.json();

    // participant_ids is an array of objects
    tableData.value = data.participant_ids; //attribute name participant_ids

    // Extract table headers dynamically from the first item in the array
    if (tableData.value.length > 0) {
      tableHeaders.value = Object.keys(tableData.value[0]);
      console.log(tableData.value[0]);
    }
  } catch (err) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
});
</script>

<style scoped>
.sr-only,
.visually-hidden {
  position: absolute !important;
  width: 1px !important;
  height: 1px !important;
  padding: 0 !important;
  margin: -1px !important;
  overflow: hidden !important;
  clip: rect(0, 0, 0, 0) !important;
  white-space: nowrap !important;
  border: 0 !important;
}
.data-table {
  width: 100%;
  border-collapse: collapse;
}

.data-table th,
.data-table td {
  border: 1px solid #222;
  padding: 8px;
  text-align: left;
}

.data-table th {
  background-color: #e6f0fa;
  color: #222;
}

.toggle-table-btn {
  margin-bottom: 1rem;
  padding: 0.5rem 1.2rem;
  font-size: 1rem;
  color: #DFDFD7;
  font-weight: bold;
  background: #3b82f6; /* Tailwind blue-500 */
  border: 1px solid #2563eb; /* Tailwind blue-600 */
  box-shadow: 0 1px 3px 0 rgba(0,0,0,0.1), 0 1px 2px 0 rgba(0,0,0,0.06);
  border-radius: 0.5rem;
  cursor: pointer;
  transition: background 0.2s;
}
.toggle-table-btn:focus {
  outline: 2px solid #1976d2;
  outline-offset: 2px;
}
.toggle-table-btn:hover {
  background: #2563eb;
}

.loading-message {
  font-style: italic;
  color: #444;
  margin-bottom: 1rem;
}

.data-table tr:focus {
  outline: 2px solid #1976d2;
  outline-offset: 2px;
}
</style>
