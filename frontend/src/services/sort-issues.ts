import { $ } from "@builder.io/qwik";
import type { Label } from "~/models/label";
import type { Repository } from "~/models/repository";

export const sortIssuesByDifficulty = $((repositories: Repository[]) => {
  function calculateScore(label: Label) {
    if (label.name === "difficulty: easy") return 1;
    if (label.name === "difficulty: medium") return 2;
    if (label.name === "difficulty: hard") return 3;
    return 0;
  }

  return repositories.map((repository) => ({
    ...repository,
    issues: repository.issues
      .map((issue) => ({
        ...issue,
        labels: issue.labels.sort((label) => {
          return label.name.startsWith("difficulty") ? -1 : 0;
        }),
      }))
      .sort((a, b) => {
        if (a.labels.every((label) => !label.name.startsWith("difficulty: "))) {
          return 1;
        }

        const aScore = a.labels.reduce(
          (acc, label) => acc + calculateScore(label),
          0
        );
        const bScore = b.labels.reduce(
          (acc, label) => acc + calculateScore(label),
          0
        );
        return aScore > bScore ? 1 : -1;
      }),
  }));
});
