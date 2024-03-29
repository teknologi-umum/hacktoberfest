import {
  $,
  component$,
  useStore,
  useStylesScoped$,
  useTask$,
  useVisibleTask$,
} from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Countdown } from "~/components/countdown";
import { Header } from "~/components/header";
import { Label } from "~/components/label";
import { RepositoryCard } from "~/components/repository-card";
import { ContributorCard } from "~/components/contributor-card";
import { Repository } from "~/models/repository";
import { getFilteredRepositories } from "~/services/filter-repositories";
import { getCategoriesList } from "~/services/list-categories";
import { getContributorsList } from "~/services/list-contributors";
import { getRepositoriesList } from "~/services/list-repositories";
import { sortIssuesByDifficulty } from "~/services/sort-issues";
import {
  sortAndTagContributorByPRs,
  SortedContributor,
} from "~/services/sort-contributors";
import styles from "~/styles/index.css?inline";

type State = {
  activeFilters: string[];
  categories: string[];
  repositories: Repository[];
  filteredRepositories: Repository[];
  contributors: SortedContributor[];
};

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore<State>({
    activeFilters: [],
    categories: [],
    repositories: [],
    filteredRepositories: [],
    contributors: [],
  });

  useTask$(async () => {
    const repositories = await getRepositoriesList();
    const sortedByDifficulty = await sortIssuesByDifficulty(repositories);
    state.repositories = sortedByDifficulty;
    state.filteredRepositories = sortedByDifficulty;
    state.categories = getCategoriesList(repositories);

    const contributors = await getContributorsList();
    const sortedByPRs = await sortAndTagContributorByPRs(contributors);
    state.contributors = sortedByPRs;
  });

  useVisibleTask$(async ({ track }) => {
    const filters = track(() => state.activeFilters);
    const repositories = track(() => state.repositories);
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
            <div key={category} onClick$={() => toggleFilter$(category)}>
              <Label text={category} isGlowing={isFilterActive} />
            </div>
          );
        })}
      </div>
      <div class="repository-card-container">
        {state.filteredRepositories.map((repo) => (
          <RepositoryCard
            key={repo.html_url}
            full_name={repo.full_name}
            html_url={repo.html_url}
            description={repo.description}
            languages={repo.languages}
            issues={repo.issues}
          />
        ))}
      </div>

      <p class="contributor-section-title">Top Contributors</p>
      <div class="contributor-card-container">
        {state.contributors.map((contrib) => (
          <ContributorCard
            key={contrib.full_name}
            full_name={contrib.full_name}
            profile_url={contrib.profile_url}
            merged_pulls={contrib.merged_pulls}
            pending_pulls={contrib.pending_pulls}
            isTopContributor={contrib.isTopContributor}
          />
        ))}
      </div>
    </div>
  );
});

export const head: DocumentHead = {
  title: "Hacktoberfest | Teknologi Umum",
};
