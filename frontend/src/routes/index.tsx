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
import { getRepositoriesList } from "~/services/list-repositories";
import styles from "../styles/index.css";

export const FAKE_FILTERS = [
  "difficulty: easy",
  "difficulty: medium",
  "difficulty: hard",
  "good first issue",
  "help wanted",
];

type State = {
  activeFilters: string[];
};

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore<State>({ activeFilters: [] });

  const repositoriesResource = useResource$<Repository[]>(
    async ({ track, cleanup }) => {
      const abortController = new AbortController();
      cleanup(() => abortController.abort());
      track(state, "activeFilters");
      return getRepositoriesList({
        signal: abortController.signal,
        filters: state.activeFilters,
      });
    }
  );

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
                  description={mutable(repo.description)}
                  full_name={mutable(repo.full_name)}
                  html_url={mutable(repo.html_url)}
                  issues={mutable(repo.issues)}
                  languages={mutable(repo.languages)}
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
