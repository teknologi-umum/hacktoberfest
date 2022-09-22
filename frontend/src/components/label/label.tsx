import { component$, type QwikJSX, useStylesScoped$ } from "@builder.io/qwik";
import styles from "./label.css";

export const COLOUR_MAP: Record<string, string> = {
  easy: "green",
  medium: "yellow",
  hard: "red",
  bug: "red",
  "help wanted": "purple",
  "good first issue": "blue",
  enhancement: "cyan",
};

export function parseLabel(label: string) {
  const PREFIX = "difficulty: ";
  const isDifficultyLabel = label.startsWith(PREFIX);
  const strippedLabel = isDifficultyLabel ? label.slice(PREFIX.length) : label;
  return {
    text: strippedLabel,
    colour: strippedLabel in COLOUR_MAP ? COLOUR_MAP[strippedLabel] : "white",
  };
}

type LabelProps = {
  text: string;
  isGlowing?: boolean;
};

export default component$((props: LabelProps) => {
  useStylesScoped$(styles);

  const label = parseLabel(props.text);

  return (
    <div class={`label ${label.colour} ${props.isGlowing ? "glow" : ""}`}>
      {label.text}
    </div>
  );
});
