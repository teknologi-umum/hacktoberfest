import { component$, useStylesScoped$ } from "@builder.io/qwik";
import { Issue } from "~/models/issue";
import { Label } from "../label";
import styles from "./issue-card.css?inline";

type IssueProps = Pick<Issue, "html_url" | "title" | "labels">;

export default component$((props: IssueProps) => {
  useStylesScoped$(styles);

  return (
    <a class="issue-card" href={props.html_url}>
      <span class="issue-card__title">{props.title}</span>
      <div class="issue-card__labels">
        {props.labels.slice(0, 3).map((l) => (
          <Label key={l.name} text={l.name} />
        ))}
      </div>
    </a>
  );
});
