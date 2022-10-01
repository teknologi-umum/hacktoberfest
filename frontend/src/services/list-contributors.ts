import { $ } from "@builder.io/qwik";
import { API_BASE_URL } from "~/env";
import { Contributor } from "~/models/contributor";

export const getContributorsList = $(async (): Promise<Contributor[]> => {
  // const url = new URL("/contrib", API_BASE_URL);
  // const response = await fetch(url);
  // return response.json();
  return [
    {
      full_name: "rubichandrap",
      profile_url: "https://github.com/rubichandrap",
      merged_pulls: 4,
      pending_pulls: 2,
    },
    {
      full_name: "afman42",
      profile_url: "https://github.com/afman42",
      merged_pulls: 1,
      pending_pulls: 9,
    },
    {
      full_name: "krowter",
      profile_url: "https://github.com/krowter",
      merged_pulls: 3,
      pending_pulls: 5,
    },
    {
      full_name: "farhan443",
      profile_url: "https://github.com/farhan443",
      merged_pulls: 25,
      pending_pulls: 6,
    },
    {
      full_name: "jason-wihardja",
      profile_url: "https://github.com/jason-wihardja",
      merged_pulls: 0,
      pending_pulls: 1,
    },
    {
      full_name: "wasd123",
      profile_url: "https://github.com/wasd123",
      merged_pulls: 10,
      pending_pulls: 10,
    },
    {
      full_name: "qwerty",
      profile_url: "https://github.com/qwerty",
      merged_pulls: 1,
      pending_pulls: 9,
    },
    {
      full_name: "asdf",
      profile_url: "https://github.com/asdf",
      merged_pulls: 5,
      pending_pulls: 5,
    },
    {
      full_name: "jklmn",
      profile_url: "https://github.com/jklmn",
      merged_pulls: 4,
      pending_pulls: 6,
    },
    {
      full_name: "poiuyt",
      profile_url: "https://github.com/poiuyt",
      merged_pulls: 2,
      pending_pulls: 1,
    },
  ];
});
