import { component$, useStylesScoped$ } from "@builder.io/qwik";
import styles from "./issue-card.css?inline";

type Label = {
  name: string;
};

export type Issue = {
  title: string;
  html_url: string;
  labels: Label[];
};

export const COLOUR_MAP: Record<string, string> = {
  easy: "green",
  medium: "yellow",
  hard: "red",
};

export function parseDifficultyLabel(label: string) {
  const PREFIX = "difficulty: ";
  const isDifficultyLabel = label.startsWith(PREFIX);
  const difficulty = isDifficultyLabel ? label.slice(PREFIX.length) : label;
  return {
    text: difficulty,
    colour: isDifficultyLabel ? COLOUR_MAP[difficulty] : "white",
  };
}

export default component$((props: Issue) => {
  useStylesScoped$(styles);

  return (
    <a class="issue-card" href={props.html_url}>
      <span class="issue-card__title">{props.title}</span>
      <div className="issue-card__labels">
        {props.labels.slice(0, 3).map((l) => {
          const label = parseDifficultyLabel(l.name);
          return (
            <div class={`issue-card__label ${label.colour}`}>{label.text}</div>
          );
        })}
      </div>
    </a>
  );
});
