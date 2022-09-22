import {
  $,
  component$,
  mutable,
  useStore,
  useStylesScoped$,
} from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Header } from "~/components/header";
import { Label, parseLabel } from "~/components/label";
import { RepositoryCard, type Repository } from "~/components/repository-card";
import styles from "../styles/index.css";

export const FAKE_FILTERS = [
  "difficulty: easy",
  "difficulty: medium",
  "difficulty: hard",
  "good first issue",
  "help wanted",
];

export const FAKE_DATA: Repository[] = Array(3)
  .fill(0)
  .map((): Repository[] => [
    {
      full_name: "teknologi-umum/blog",
      html_url: "https://github.com/teknologi-umum/blog.git",
      description: "Blog for Teknologi Umum",
      languages: ["typescript", "javascript"],
      issues: [
        {
          title: "view post by category got error",
          html_url: "https://github.com/teknologi-umum/blog/issues/109",
          labels: [
            { name: "difficulty: medium" },
            { name: "good first issue" },
          ],
        },
        {
          title:
            "feat: add list a projects that were created by the organizations",
          html_url: "https://github.com/teknologi-umum/blog/issues/103",
          labels: [{ name: "difficulty: hard" }, { name: "help wanted" }],
        },
        {
          title: "article: random ideas from telegram",
          html_url: "https://github.com/teknologi-umum/blog/issues/96",
          labels: [{ name: "difficulty: easy" }, { name: "good first issue" }],
        },
      ],
    },
    {
      full_name: "teknologi-umum/bot",
      html_url: "https://github.com/teknologi-umum/bot.git",
      description: "Bot for a more interactive Teknologi Umum group",
      languages: ["javascript", "handlebars"],
      issues: [
        {
          title: 'Avoid term "dukun" completely',
          html_url: "https://github.com/teknologi-umum/bot/issues/161",
          labels: [{ name: "difficulty: medium" }, { name: "enhancement" }],
        },
      ],
    },
    {
      full_name: "teknologi-umum/pehape",
      html_url: "https://github.com/teknologi-umum/pehape.git",
      description: "PHP itu bukan pemberi harapan palsu",
      languages: ["C#", "Clojure", "Ruby", "Go", "Rust", "Typescript"],
      issues: [
        {
          title: "C#: Support imploding nested array",
          html_url: "https://github.com/teknologi-umum/pehape/issues/22",
          labels: [
            { name: "bug" },
            { name: "good first issue" },
            { name: "help wanted" },
          ],
        },
        {
          title: "Java implementation",
          html_url: "https://github.com/teknologi-umum/pehape/issues/16",
          labels: [{ name: "good first issue" }, { name: "help wanted" }],
        },
      ],
    },
  ])
  .flat();

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore({ activeFilters: [] as string[] });

  const toggleFilter$ = $((filter: string) => {
    const isFilterActive = state.activeFilters.includes(filter);
    state.activeFilters = isFilterActive
      ? state.activeFilters.filter((f) => f !== filter)
      : state.activeFilters.concat(filter);
  });

  return (
    <div>
      <Header />
      <div class="filters">
        {FAKE_FILTERS.map((filter) => {
          const isFilterActive = state.activeFilters.includes(filter);
          return (
            <div onClick$={() => toggleFilter$(filter)}>
              <Label text={filter} isGlowing={mutable(isFilterActive)} />
            </div>
          );
        })}
      </div>
      <div class="card-container">
        {FAKE_DATA.map((repository) => {
          const filteredIssues =
            state.activeFilters.length < 1
              ? repository.issues
              : repository.issues.filter((issue) =>
                  issue.labels.some((label) =>
                    state.activeFilters.includes(label.name)
                  )
                );

          if (filteredIssues.length < 1) return null;

          return (
            <RepositoryCard
              description={mutable(repository.description)}
              full_name={mutable(repository.full_name)}
              html_url={mutable(repository.html_url)}
              issues={mutable(filteredIssues)}
              languages={mutable(repository.languages)}
            />
          );
        })}
      </div>
    </div>
  );
});

export const head: DocumentHead = {
  title: "Hacktoberfest | Teknologi Umum",
};
