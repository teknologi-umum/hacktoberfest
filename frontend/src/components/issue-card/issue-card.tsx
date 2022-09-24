import { component$, mutable, useStylesScoped$ } from "@builder.io/qwik";
import { Label } from "../label";
import styles from "./issue-card.css?inline";

type Label = {
  name: string;
};

export type Issue = {
  title: string;
  html_url: string;
  labels: Label[];
};

export default component$((props: Issue) => {
  useStylesScoped$(styles);

  return (
    <a class="issue-card" href={props.html_url}>
      <span class="issue-card__title">{props.title}</span>
      <div className="issue-card__labels">
        {props.labels.slice(0, 3).map((l) => (
          <Label text={mutable(l.name)} />
        ))}
      </div>
    </a>
  );
});
