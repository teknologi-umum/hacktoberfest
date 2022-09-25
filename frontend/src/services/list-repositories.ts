import { $ } from "@builder.io/qwik";
import { API_BASE_URL } from "~/env";
import type { Repository } from "~/models/repository";

type RepositoriesListOptions = {
  signal: AbortSignal;
  filters: string[];
};

export const CACHE_MAP = new Map();
export const getRepositoriesList = $(
  async ({ filters, signal }: RepositoriesListOptions) => {
    // emulate loading
    await new Promise((res) => setTimeout(() => res(void 0), 500));
    let repositories: Repository[] = [];
    if (!CACHE_MAP.has("repo")) {
      const response = await fetch(`${API_BASE_URL}/repo`, { signal });
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
      .filter((repository) => repository.issues.length > 0);

    return filteredRepositories;
  }
);
