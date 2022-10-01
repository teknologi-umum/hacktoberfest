import { $ } from "@builder.io/qwik";
import { Contributor } from "~/models/contributor";

export type SortedContributor = Contributor & { isTopContributor: boolean };

export const TOP_CONTRIBUTOR_THRESHOLD = 10;

export const sortAndTagContributorByPRs = $(
  (contributors: Contributor[]): SortedContributor[] => {
    const sorted = contributors.sort((a, b) => {
      const difference =
        b.merged_pulls + b.pending_pulls - (a.merged_pulls + a.pending_pulls);

      if (difference !== 0) {
        return difference;
      } else {
        return b.merged_pulls - a.merged_pulls;
      }
    });

    /**
     * any contributor after the 10th with same total PRs AND same merged PRs
     * is still considered top contributor
     */
    const lastTopIndex = Math.min(TOP_CONTRIBUTOR_THRESHOLD, sorted.length);
    const lastTopContributor = sorted[lastTopIndex - 1];

    let sortedAndTagged = sorted.map((contributor) => {
      const isTopContributor =
        contributor.merged_pulls + contributor.pending_pulls >=
          lastTopContributor.merged_pulls + lastTopContributor.pending_pulls &&
        contributor.merged_pulls >= lastTopContributor.merged_pulls;

      return { ...contributor, isTopContributor };
    });

    return sortedAndTagged;
  }
);
