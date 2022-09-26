import { component$, mutable, useStylesScoped$ } from "@builder.io/qwik";
import { Issue } from "~/models/issue";
import { Label } from "../label";
import styles from "./issue-card.css";

type IssueProps = Issue;

export default component$((props: IssueProps) => {
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
