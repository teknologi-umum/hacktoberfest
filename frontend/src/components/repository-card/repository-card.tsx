import {
  component$,
  mutable,
  useStore,
  useStylesScoped$,
} from "@builder.io/qwik";
import { GithubIcon } from "../icons/ic_github";
import { IssueCard, type Issue } from "../issue-card";
import { LANGUAGE_ICON_MAPPING } from "./language-icon-mapping";
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
        <div class="repository-card__right-content">
          {props.languages.map((language) => (
            <div class="repository-card__language">
              {LANGUAGE_ICON_MAPPING[language.toLowerCase()]}
            </div>
          ))}
        </div>
      </div>
      <div class={`issues ${state.isIssueVisible ? "visible" : "invisible"}`}>
        {props.issues.map((issue) => (
          <IssueCard
            {...issue}
            title={mutable(issue.title)}
            html_url={mutable(issue.html_url)}
            labels={mutable(issue.labels)}
          />
        ))}
      </div>
    </div>
  );
});
