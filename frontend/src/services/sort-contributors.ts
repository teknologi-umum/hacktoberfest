import { $ } from "@builder.io/qwik";
import { Contributor } from "~/models/contributor";

export const sortContributorByPRs = $((contributors: Contributor[]) => {
  return contributors.sort((a, b) => {
    const difference =
      b.merged_pulls + b.pending_pulls - (a.merged_pulls + a.pending_pulls);

    if (difference !== 0) {
      return difference;
    } else {
      return b.merged_pulls - a.merged_pulls;
    }
  });
});
