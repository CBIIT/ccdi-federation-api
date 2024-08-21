import { defineStore } from "pinia";
import { PrimaryEntity } from "../../.vitepress/src/api";

import {
  ModelsSample as Sample,
  ModelsSubject as Subject,
  ModelsFile as File,
} from "../../.vitepress/src/api/core";

interface EntityState<T> {
  [host: string]: { data: T[] | null; completed: boolean };
}

interface State {
  subject: EntityState<Subject>;
  sample: EntityState<Sample>;
  file: EntityState<File>;
}

export const useDataStore = defineStore("data", {
  state: (): State => ({
    subject: {} as EntityState<Subject>,
    sample: {} as EntityState<Sample>,
    file: {} as EntityState<File>,
  }),
  actions: {
    update<T>(kind: PrimaryEntity, host: string, data: T[]) {
      const state = this[kind] as EntityState<T>;
      if (!state[host]) {
        state[host] = { data: [], completed: false };
      }
      state[host].data = data;
    },
    markComplete(type: "subject" | "sample" | "file", host: string) {
      const state = this[type];
      if (state[host]) {
        state[host].completed = true;
      }
    },
  },
  getters: {
    subjects(state: State): Subject[] {
      return Object.values(state.subject)
        .filter((entity) => entity.data !== null)
        .map((entity) => entity.data as Subject[])
        .reduce((acc, data) => acc.concat(data), []);
    },
    samples(state: State): Sample[] {
      return Object.values(state.sample)
        .filter((entity) => entity.data !== null)
        .map((entity) => entity.data as Sample[])
        .reduce((acc, data) => acc.concat(data), []);
    },
    files(state: State): File[] {
      return Object.values(state.file)
        .filter((entity) => entity.data !== null)
        .map((entity) => entity.data as File[])
        .reduce((acc, data) => acc.concat(data), []);
    },
  },
});
