import { $ } from "@builder.io/qwik";
import { API_BASE_URL } from "~/env";
import type { Repository } from "~/models/repository";

export const getRepositoriesList = $(async (): Promise<Repository[]> => {
  const url = new URL("/repo", API_BASE_URL);
  const response = await fetch(url);
  return response.json();
});
