import { $ } from "@builder.io/qwik";
import { API_BASE_URL } from "~/env";
import type { Repository } from "~/models/repository";

type RepositoriesListOptions = {
  signal: AbortSignal;
  state: {
    activeFilters: string[];
    repositories: Repository[];
  };
};

export const getRepositoriesList = $(
  async ({ state, signal }: RepositoriesListOptions) => {
    let repositories: Repository[] = [];
    if (state.repositories.length < 1) {
      let url = new URL("/repo", API_BASE_URL);
      const response = await fetch(url, { signal });
      repositories = await response.json();
      state.repositories = repositories;
    } else {
      repositories = state.repositories;
    }

    const filteredRepositories = repositories
      .map((repository) => {
        const filteredIssues =
          state.activeFilters.length < 1
            ? repository.issues
            : repository.issues.filter((issue) =>
                issue.labels.some((label) => state.activeFilters.includes(label.name))
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
