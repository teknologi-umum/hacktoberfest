import { $ } from "@builder.io/qwik";
import type { Repository } from "~/models/repository";

export const sortIssuesByDifficulty = $((repositories: Repository[]) => {
  return repositories.map((repository) => ({
    ...repository,
    issues: repository.issues.map((issue) => ({
      ...issue,
      labels: issue.labels.sort((label) => {
        return label.name.startsWith("difficulty") ? -1 : 0;
      }),
    })),
  }));
});
