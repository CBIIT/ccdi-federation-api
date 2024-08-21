import { defineStore } from "pinia";

type State = {
  host: string | null;
};

export const useApiStore = defineStore("api", {
  state: (): State => {
    return {
      host: null,
    };
  },
  actions: {
    setHost(host: string) {
      this.host = host;
    },
    getPath(path: string): string {
      let host =
        this.host === null
          ? "https://federation.ccdi.cancer.gov/api/v1"
          : this.host;

      while (path.startsWith("/")) {
        path = path.slice(1);
      }

      return `${host}/${path}`;
    },
  },
});
