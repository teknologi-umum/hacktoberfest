import { $ } from "@builder.io/qwik";
import { API_BASE_URL } from "~/env";
import { Contributor } from "~/models/contributor";

export const getContributorsList = $(async (): Promise<Contributor[]> => {
  const url = new URL("/contrib", API_BASE_URL);
  const response = await fetch(url);
  return response.json();
});
