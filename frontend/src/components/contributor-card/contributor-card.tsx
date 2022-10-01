import {
  component$,
  mutable,
  useStore,
  useStylesScoped$,
} from "@builder.io/qwik";
import { Contributor } from "~/models/contributor";
import styles from "./contributor-card.css";

export default component$((props: Contributor) => {
  useStylesScoped$(styles);

  return (
    <div class="contributor-card">
      <a
        className="contributor-card__username"
        href={props.profile_url}
        rel="noopener noreferrer"
      >
        {props.full_name}
      </a>
      <div class="contributor-card__stats-wrapper">
        {Array(props.merged_pulls)
          .fill(null)
          .map(() => (
            <div class="contributor-card__stats-merged" />
          ))}
        {Array(props.pending_pulls)
          .fill(null)
          .map(() => (
            <div class="contributor-card__stats-pending" />
          ))}
      </div>
    </div>
  );
});
