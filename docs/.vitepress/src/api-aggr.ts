import axios from "axios";
import { AxiosResponse } from "axios";
//TODO remove
import {
  ModelsSample as Sample,
  ModelsSubject as Subject,
  ModelsFile as File,
  ResponsesByCountSubjectResults,
  ResponsesByCountSampleResults,
  ResponsesByCountFileResults,
} from "../../.vitepress/src/api/core";

/**
 * Supported HTTP methods for the API.
 */
export enum HttpMethod {
  Get = "GET",
}

/**
 * Primary entities within the API.
 */
export enum PrimaryEntity {
  Subject = "subject",
  Sample = "sample",
  File = "file",
}

/**
 * The configuration for source servers.
 */
export interface ServerConfig {
  // The URL of the server.
  url: string;

  // The maximum number of results per page during pagination.
  perPage: number;

  // Whether or not the server is considered a "source" server.
  sourceServer: boolean;
}

export const defaultServers: { [host: string]: ServerConfig } = {
  "NCI Data Federation Resource": {
    url: "https://federation.ccdi.cancer.gov/api/v1",
    perPage: 100,
    sourceServer: false,
  }
};

class Api {
  // The hosts that the API will query.
  private hosts: { [host: string]: ServerConfig };

  /**
   * Creates a new API instance.
   *
   * If no hosts are provided, an API will be instantiated that points to all of
   * the source servers tracked in `defaultServers`.
   *
   * @param hosts the hosts to include in the API calls.
   */
  constructor(hosts?: { [host: string]: ServerConfig }) {
    this.hosts =
      hosts ||
      Object.entries(defaultServers)
        .filter(([key, value]) => value.sourceServer)
        .reduce((acc: { [host: string]: ServerConfig }, [key, value]) => {
          acc[key] = value;
          return acc;
        }, {});
  }

  /**
   * Runs a single GET requests across the federation endpoints and returns
   * results nested by the name of the server. `null` represents a failed request.
   *
   * @param path the endpoint
   * @returns the results of the query keyed by server name
   */
  async get<T>(path: string): Promise<{ [host: string]: T | null }> {
    const responses: { [host: string]: T } = {};

    const fetch = Object.entries(this.hosts).map(async ([name, url]) => {
      try {
        const response: AxiosResponse<T> = await axios.get<T>(`${url}${path}`);
        responses[name] = response.data;
      } catch (error) {
        responses[name] = null as any;
      }
    });

    await Promise.all(fetch);

    return responses;
  }

  /**
   * Collects a primary entity by a particular field.
   *
   * @param kind the kind of entity to collect
   * @param field the field to count by
   * @returns counts of the entities grouped by the field
   */
  async countEntityBy<
    T extends
      | ResponsesByCountSubjectResults
      | ResponsesByCountSampleResults
      | ResponsesByCountFileResults
  >(kind: PrimaryEntity, field: string): Promise<Record<string, T>> {
    const responses: { [host: string]: T } = {};
    let path = null;

    switch (kind) {
      case PrimaryEntity.Subject:
        path = `/subject/by/${field}/count`;
        break;
      case PrimaryEntity.Sample:
        path = `/sample/by/${field}/count`;
        break;
      case PrimaryEntity.File:
        path = `/file/by/${field}/count`;
        break;
      default:
        throw new Error("Unhandled entity:", kind);
    }

    const fetch = Object.entries(this.hosts).map(async ([name, server]) => {
      try {
        let url = `${server.url}${path}`;
        const response: AxiosResponse<T> = await axios.get<T>(url);
        responses[name] = response.data;
      } catch (error) {
        responses[name] = null as any;
      }
    });

    await Promise.all(fetch);

    return responses;
  }

  /**
   * Count subjects by a field.
   *
   * @param field the field to count by.
   * @returns counts of subjects grouped by the field.
   */
  async countSubjectsBy(
    field: string
  ): Promise<Record<string, ResponsesByCountSubjectResults>> {
    return this.countEntityBy(PrimaryEntity.Subject, field);
  }

  /**
   * Count samples by a field.
   *
   * @param field the field to count by.
   * @returns counts of samples grouped by the field.
   */
  async countSamplesBy(
    field: string
  ): Promise<Record<string, ResponsesByCountSampleResults>> {
    return this.countEntityBy(PrimaryEntity.Sample, field);
  }

  /**
   * Count files by a field.
   *
   * @param field the field to count by.
   * @returns counts of files grouped by the field.
   */
  async countFilesBy(
    field: string
  ): Promise<Record<string, ResponsesByCountFileResults>> {
    return this.countEntityBy(PrimaryEntity.File, field);
  }

  /**
   * Collects a set of primary entities by querying paginated results for all
   * servers.
   *
   * @param kind the kind of primary entity within the API.
   * @param progress a progress callback.
   * @returns an object where keys are the server name and values are an array
   * of the collected entities.
   */
  async collect<T extends Subject | Sample | File>(
    kind: PrimaryEntity,
    progress:
      | ((
          name: string,
          kind: PrimaryEntity,
          values: T[] | null,
          complete: boolean
        ) => Promise<void>)
      | null = null
  ): Promise<Record<string, T[]>> {
    const results: Record<string, T[]> = {};

    const sleep = (ms: number) =>
      new Promise((resolve) => setTimeout(resolve, ms));

    const inner = async (
      name: string,
      config: ServerConfig,
      page: number = 1,
      retries: number = 0
    ): Promise<void> => {
      const perPage = config.perPage || 1000;
      try {
        const response: AxiosResponse<{ data: T[] }> = await axios.get<{
          data: T[];
        }>(`${config.url}/${kind}?per_page=${perPage}&page=${page}`);

        if (!results[name]) {
          results[name] = [];
        }

        results[name].push(...response.data.data);

        if (response.data.data.length < perPage) {
          if (progress !== null) {
            await progress(name, kind, response.data.data, true);
          }

          return;
        }

        if (progress !== null) {
          await progress(name, kind, response.data.data, false);
        }
        await inner(name, config, page + 1);
      } catch (error) {
        if (retries == 3) {
          retries = retries + 1;
          await sleep(1000 * retries);
          await inner(name, config, page, retries);
        } else {
          if (progress !== null) {
            await progress(name, kind, null, true);
          }
        }
      }
    };

    await Promise.all(
      Object.entries(this.hosts).map(([host, config]) => inner(host, config))
    );

    return results;
  }

  /**
   * Collects all subjects from the source servers.
   *
   * @param progress a progress callback.
   * @returns the subjects for each source server.
   */
  async subjects(
    progress:
      | ((
          name: string,
          kind: PrimaryEntity,
          values: Subject[] | null,
          complete: boolean
        ) => Promise<void>)
      | null = null
  ): Promise<Record<string, Subject[]>> {
    return this.collect<Subject>(PrimaryEntity.Subject, progress);
  }

  /**
   * Collects all samples from the source servers.
   *
   * @param progress a progress callback.
   * @returns the samples for each source server.
   */
  async samples(
    progress:
      | ((
          name: string,
          kind: PrimaryEntity,
          values: Sample[] | null,
          complete: boolean
        ) => Promise<void>)
      | null = null
  ): Promise<Record<string, Sample[]>> {
    return this.collect<Sample>(PrimaryEntity.Sample, progress);
  }

  /**
   * Collects all files from the source servers.
   *
   * @param progress a progress callback.
   * @returns the files for each source server.
   */
  async files(
    progress:
      | ((
          name: string,
          kind: PrimaryEntity,
          values: File[] | null,
          complete: boolean
        ) => Promise<void>)
      | null = null
  ): Promise<Record<string, File[]>> {
    return this.collect<File>(PrimaryEntity.File, progress);
  }
}

export default Api;
