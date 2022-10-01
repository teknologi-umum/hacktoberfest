import { $ } from "@builder.io/qwik";
import { Contributor } from "~/models/contributor";

export type SortedContributor = Contributor & { isTopContributor: boolean };

export const TOP_CONTRIBUTOR_THRESHOLD = 10;

export const sortContributorByPRs = $(
  (contributors: Contributor[]): SortedContributor[] => {
    return contributors
      .sort((a, b) => {
        const difference =
          b.merged_pulls + b.pending_pulls - (a.merged_pulls + a.pending_pulls);

        if (difference !== 0) {
          return difference;
        } else {
          return b.merged_pulls - a.merged_pulls;
        }
      })
      .map((contributor, i) => ({
        ...contributor,
        isTopContributor: i < TOP_CONTRIBUTOR_THRESHOLD,
      }));
  }
);
