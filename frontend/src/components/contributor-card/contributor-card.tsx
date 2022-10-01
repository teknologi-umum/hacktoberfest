import { component$, useStylesScoped$ } from "@builder.io/qwik";
import { Contributor } from "~/models/contributor";
import styles from "./contributor-card.css";

export function formatPRUnit(count: number) {
  return count === 1 ? "PR" : "PRs";
}

export default component$((props: Contributor) => {
  const hasCompletedHacktoberfest = props.merged_pulls >= 4;

  useStylesScoped$(styles);

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
            {Array(props.merged_pulls)
              .fill(null)
              .map(() => (
                <div
                  class={`contributor-card__stats-bar ${
                    hasCompletedHacktoberfest ? "completed" : "merged"
                  }`}
                />
              ))}
            {Array(props.pending_pulls)
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

      {hasCompletedHacktoberfest && (
        <p class="contributor-card__completed-description">
          Ayo. Terus kirimkan PR terbaikmu untuk jadi Top Contributor!
        </p>
      )}
    </div>
  );
});
