import { $ } from "@builder.io/qwik";
import type { Repository } from "~/models/repository";

export const getFilteredRepositories = $(
  (repositories: Repository[], filters: string[]): Repository[] => {
    if (filters.length < 1) return repositories;

    const filteredRepositories = repositories
      // filter issues that has matching category with the filters
      .map((repository) => {
        const filteredIssues =
          filters.length < 1
            ? repository.issues
            : repository.issues.filter((issue) =>
                issue.labels.some((label) => filters.includes(label.name))
              );
        return { ...repository, issues: filteredIssues };
      })
      // remove repository with 0 issue after filtering
      .filter((repository) => repository.issues.length > 0)
      // sort label prioritized by difficulty
      .map((repository) => {
        return {
          ...repository,
          issues: repository.issues.map((issue) => ({
            ...issue,
            labels: issue.labels.sort((label) =>
              label.name.startsWith("difficulty") ? -1 : 0
            ),
          })),
        };
      })
      // sort by recency
      .sort((a, b) =>
        new Date(a.updated_at) > new Date(b.updated_at) ? -1 : 1
      );

    return filteredRepositories;
  }
);
