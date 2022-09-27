import {
  $,
  component$,
  mutable,
  useResource$,
  useStore,
  useStylesScoped$,
  Resource,
} from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Countdown } from "~/components/countdown";
import { Header } from "~/components/header";
import { Label } from "~/components/label";
import { RepositoryCard } from "~/components/repository-card";
import { Repository } from "~/models/repository";
import { getCategoriesList } from "~/services/list-categories";
import { getRepositoriesList } from "~/services/list-repositories";
import styles from "~/styles/index.css";

type State = {
  activeFilters: string[];
  repositories: Repository[];
};

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore<State>({ activeFilters: [], repositories: [] });

  const repositoriesResource = useResource$<Repository[]>(async ({ track }) => {
    track(state, "activeFilters");
    return getRepositoriesList({ state });
  });

  const categoriesResource = useResource$<string[]>(async ({ cleanup }) => {
    const repositories = await getRepositoriesList({ state });
    return getCategoriesList(repositories);
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
        <Resource
          value={categoriesResource}
          onPending={() => (
            <span class="loading-text">Loading Categories...</span>
          )}
          onRejected={(error) => (
            <div>
              <span class="error-text">Failed to load categories</span>
              <p class="error-message">{error.message}</p>
            </div>
          )}
          onResolved={(filters) => (
            <>
              {filters.map((filter) => {
                const isFilterActive = state.activeFilters.includes(filter);
                return (
                  <div onClick$={() => toggleFilter$(filter)}>
                    <Label text={filter} isGlowing={mutable(isFilterActive)} />
                  </div>
                );
              })}
            </>
          )}
        />
      </div>
      <div class="card-container">
        <Resource
          value={repositoriesResource}
          onPending={() => (
            <span class="loading-text">Loading Repositories...</span>
          )}
          onRejected={(error) => (
            <div>
              <span class="error-text">Failed to load repositories</span>
              <p class="error-message">{error.message}</p>
            </div>
          )}
          onResolved={(repositoriesData: Repository[]) => (
            <>
              {repositoriesData.map((repo) => (
                <RepositoryCard
                  full_name={mutable(repo.full_name)}
                  html_url={mutable(repo.html_url)}
                  description={mutable(repo.description)}
                  languages={mutable(repo.languages)}
                  stars_count={mutable(repo.stars_count)}
                  forks_count={mutable(repo.forks_count)}
                  topics={mutable(repo.topics)}
                  created_at={mutable(repo.created_at)}
                  updated_at={mutable(repo.updated_at)}
                  issues={mutable(repo.issues)}
                />
              ))}
            </>
          )}
        />
      </div>
    </div>
  );
});

export const head: DocumentHead = {
  title: "Hacktoberfest | Teknologi Umum",
};
