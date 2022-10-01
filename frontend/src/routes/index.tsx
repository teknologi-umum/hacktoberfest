import {
  $,
  component$,
  mutable,
  useStore,
  useStylesScoped$,
  useClientEffect$,
  useServerMount$,
} from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Countdown } from "~/components/countdown";
import { Header } from "~/components/header";
import { Label } from "~/components/label";
import { RepositoryCard } from "~/components/repository-card";
import { Repository } from "~/models/repository";
import { getFilteredRepositories } from "~/services/filter-repositories";
import { getCategoriesList } from "~/services/list-categories";
import { getRepositoriesList } from "~/services/list-repositories";
import { sortIssuesByDifficulty } from "~/services/sort-issues";
import styles from "~/styles/index.css?inline";

type State = {
  activeFilters: string[];
  categories: string[];
  repositories: Repository[];
  filteredRepositories: Repository[];
};

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore<State>({
    activeFilters: [],
    categories: [],
    repositories: [],
    filteredRepositories: [],
  });

  useServerMount$(async () => {
    const repositories = await getRepositoriesList();
    const sortedByDifficulty = await sortIssuesByDifficulty(repositories);
    state.repositories = sortedByDifficulty;
    state.filteredRepositories = sortedByDifficulty;
    state.categories = getCategoriesList(repositories);
  });

  useClientEffect$(async ({ track }) => {
    const filters = track(state, "activeFilters");
    const repositories = track(state, "repositories");
    state.filteredRepositories = await getFilteredRepositories(
      repositories,
      filters
    );
  });

  const toggleFilter$ = $((filter: string) => {
    const isFilterActive = state.activeFilters.includes(filter);
    state.activeFilters = isFilterActive
      ? state.activeFilters.filter((f) => f !== filter)
      : state.activeFilters.concat(filter);
  });

  return (
    <div>
      <Countdown />
      <Header />
      <p class="filter-tips">
        Kesusahan nyari issue? Klik aja filter di bawah biar gampang nyarinya!
      </p>
      <div class="filters">
        {state.categories.map((category) => {
          const isFilterActive = state.activeFilters.includes(category);
          return (
            <div onClick$={() => toggleFilter$(category)}>
              <Label text={category} isGlowing={mutable(isFilterActive)} />
            </div>
          );
        })}
      </div>
      <div class="card-container">
        {state.filteredRepositories.map((repo) => (
          <RepositoryCard
            full_name={mutable(repo.full_name)}
            html_url={mutable(repo.html_url)}
            description={mutable(repo.description)}
            languages={mutable(repo.languages)}
            issues={mutable(repo.issues)}
          />
        ))}
      </div>
    </div>
  );
});

export const head: DocumentHead = {
  title: "Hacktoberfest | Teknologi Umum",
};
