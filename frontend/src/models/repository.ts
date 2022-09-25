import type { Issue } from "./issue";

export type Repository = {
  full_name: string;
  html_url: string;
  description: string;
  languages: string[];
  stars_count: number;
  forks_count: number;
  topics: string[];
  created_at: string;
  updated_at: string;
  issues: Issue[];
};
