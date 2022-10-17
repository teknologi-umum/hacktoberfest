import { component$, useStylesScoped$ } from "@builder.io/qwik";
import { SortedContributor } from "~/services/sort-contributors";
import styles from "./contributor-card.css";

export function formatPRUnit(count: number) {
  return count === 1 ? "PR" : "PRs";
}

export default component$((props: SortedContributor) => {
  const hasCompletedHacktoberfest = props.merged_pulls >= 4;

  useStylesScoped$(styles);

  let mergedPRs = props.merged_pulls;
  let pendingPRs = props.pending_pulls;

  /**
   * beyond 30, the UI for the grid will be broken
   * but still need to be in proportion
   */
  const MAX_VISIBLE_PR = 30;
  const totalPRs = mergedPRs + pendingPRs;

  if (totalPRs > MAX_VISIBLE_PR) {
    mergedPRs = Math.ceil((mergedPRs / totalPRs) * MAX_VISIBLE_PR);
    pendingPRs = Math.floor((pendingPRs / totalPRs) * MAX_VISIBLE_PR);
  }

  return (
    <div
      class={`contributor-card ${hasCompletedHacktoberfest ? "completed" : ""}`}
    >
      <div>
        <a
          className="contributor-card__username"
          href={props.profile_url}
          rel="noopener noreferrer"
          target="__blank"
        >
          {props.full_name}
        </a>
        <div class="contributor-card__stats-wrapper">
          <div class="contributor-card__stats-bar">
            {Array(mergedPRs)
              .fill(null)
              .map(() => (
                <div
                  class={`contributor-card__stats-bar ${
                    hasCompletedHacktoberfest ? "completed" : "merged"
                  }`}
                />
              ))}
            {Array(pendingPRs)
              .fill(null)
              .map(() => (
                <div class="contributor-card__stats-bar pending" />
              ))}
          </div>
          <div class="contributor-card__stats-detail">
            {props.merged_pulls} Merged {formatPRUnit(props.merged_pulls)} |{" "}
            {props.pending_pulls} Pending {formatPRUnit(props.pending_pulls)}
            <br />
          </div>
        </div>
      </div>

      {hasCompletedHacktoberfest && !props.isTopContributor && (
        <p class="contributor-card__completed-description">
          Ayo. Terus kirimkan PR terbaikmu untuk jadi Top Contributor!
        </p>
      )}
    </div>
  );
});
