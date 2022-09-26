import {
  component$,
  mutable,
  useStore,
  useStylesScoped$,
} from "@builder.io/qwik";
import type { Repository } from "~/models/repository";
import { GithubIcon } from "../icons/ic_github";
import { IssueCard } from "../issue-card";
import { LANGUAGE_ICON_MAPPING } from "./language-icon-mapping";
import styles from "./repository-card.css";

type RepositoryProps = Repository;
export default component$((props: RepositoryProps) => {
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
          <div class="repository-card__gh-logo">
            <GithubIcon />
          </div>
          <div class="repository-card__detail">
            <a class="repository-card__title" href={props.html_url}>
              {props.full_name}
            </a>
            <p class="repository-card__description">{props.description}</p>
          </div>
        </div>
        <div class="repository-card__right-content">
          {props.languages
            .sort()
            .slice(0, 8)
            .map((language) => (
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
