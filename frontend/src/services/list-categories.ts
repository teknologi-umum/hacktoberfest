import type { Repository } from "~/models/repository";

export function getCategoriesList(repositories: Repository[]): string[] {
  const uniqueLabels = new Set<string>();

  for (const repository of repositories) {
    for (const issue of repository.issues) {
      for (const label of issue.labels) {
        // ignore hacktoberfest label since they all should have it
        if (label.name === "hacktoberfest") continue;
        uniqueLabels.add(label.name);
      }
    }
  }

  return [...uniqueLabels];
}
