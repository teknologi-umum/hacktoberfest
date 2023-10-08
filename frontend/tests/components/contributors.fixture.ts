import { Contributor } from "~/models/contributor";
import { Issue } from "~/models/issue";
import { User } from "~/models/user";
import { SortedContributor } from "~/services/sort-contributors";

const sampleContributor: Contributor = {
  full_name: "anyContributor",
  profile_url: "https://github.com/johnsmith",
  merged_pulls: 0,
  pending_pulls: 0,
};

export const sampleSortedContributor: SortedContributor = {
  ...sampleContributor,
  isTopContributor: false,
};

export const sampleIssue: Issue = {
  html_url: "issue-url.com",
  title: "anyIssue",
  comments: 10,
  user: {
    login: "anyUser",
    avatar_url: "https://user.com/avatar.jpg",
    html_url: "https://github.com/user",
  },
  created_at: "2022-10-14T18:20:51.139Z",
  updated_at: "2022-10-14T18:20:51.139Z",
  labels: [
    {
      name: "issue label 1",
      color: "red",
      description: "any issue description",
    },
    {
      name: "issue label 2",
      color: "red",
      description: "any issue description",
    },
    {
      name: "issue label 3",
      color: "red",
      description: "any issue description",
    },
    {
      name: "issue label 4",
      color: "red",
      description: "any issue description",
    },
  ],
};
