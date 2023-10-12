import { $ } from "@builder.io/qwik";
import { API_BASE_URL } from "~/env";
import { Contributor } from "~/models/contributor";

export const getContributorsList = $(async () => {
  const url = new URL("/contrib", API_BASE_URL);
  const response = await fetch(url);
  const contributors: Contributor[] = await response.json();

  return contributors.filter(({ full_name }) => !full_name.includes("[bot]"));
});
