import { $ } from "@builder.io/qwik";
import { API_BASE_URL, BROWSER_API_BASE_URL } from "~/env";
import type { Repository } from "~/models/repository";

type RepositoriesListOptions = {
  signal: AbortSignal;
  filters: string[];
};

export const CACHE_MAP = new Map();
export const getRepositoriesList = $(
  async ({ filters, signal }: RepositoriesListOptions) => {
    let repositories: Repository[] = [];
    if (!CACHE_MAP.has("repo")) {
      let fetchURL: URL;
      if (import.meta.env.SSR) {
        fetchURL = new URL("/repo", API_BASE_URL);
      } else {
        fetchURL = new URL("/repo", BROWSER_API_BASE_URL);
      }
      
      const response = await fetch(fetchURL, { signal });
      repositories = await response.json();
      CACHE_MAP.set("repo", repositories);
    } else {
      repositories = CACHE_MAP.get("repo");
    }

    const filteredRepositories = repositories
      .map((repository) => {
        const filteredIssues =
          filters.length < 1
            ? repository.issues
            : repository.issues.filter((issue) =>
                issue.labels.some((label) => filters.includes(label.name))
              );
        return { ...repository, issues: filteredIssues };
      })
      .filter((repository) => repository.issues.length > 0)
      .map((repository) => {
        // sort label prioritized by difficulty
        return {
          ...repository,
          issues: repository.issues.map((issue) => ({
            ...issue,
            labels: issue.labels.sort((label) => {
              if (label.name.startsWith("difficulty")) return -1;
              return 0;
            }),
          })),
        };
      });

    return filteredRepositories;
  }
);
