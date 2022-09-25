import type { Issue } from "./issue";

export type Repository = {
  full_name: string;
  html_url: string;
  description: string;
  languages: string[];
  issues: Issue[];
};
