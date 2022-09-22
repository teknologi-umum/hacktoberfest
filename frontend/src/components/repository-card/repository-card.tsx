import { component$, useStore, useStylesScoped$ } from "@builder.io/qwik";
import { GithubIcon } from "../icons/ic_github";
import { IssueCard, type Issue } from "../issue-card";
import styles from "./repository-card.css";

export type Repository = {
  full_name: string;
  html_url: string;
  description: string;
  languages: string[];
  issues: Issue[];
};

export default component$((props: Repository) => {
  const state = useStore({
    isIssueVisible: true,
  });

  useStylesScoped$(styles);

  return (
    <div class="card-wrapper">
      <div
        class="repository-card"
        onClick$={() => (state.isIssueVisible = !state.isIssueVisible)}
      >
        <div class="repository-card__left-content">
          <GithubIcon />
          <div class="repository-card__detail">
            <a class="repository-card__title" href={props.html_url}>
              {props.full_name}
            </a>
            <p class="repository-card__description">{props.description}</p>
          </div>
        </div>
      </div>
      <div class={`issues ${state.isIssueVisible ? "visible" : "invisible"}`}>
        {props.issues.map((issue) => (
          <IssueCard {...issue} />
        ))}
      </div>
    </div>
  );
});
