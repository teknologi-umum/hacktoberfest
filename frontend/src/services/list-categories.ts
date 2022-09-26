import { Repository } from "~/models/repository";

export function getCategoriesList(repositories: Repository[]): string[] {
  return [
    ...new Set(
      repositories
        .map((repository) =>
          repository.issues
            .map((issue) => issue.labels.map((label) => label.name))
            .flat()
        )
        .flat()
    ),
  ];
}
