import type { Label } from "./label";
import { User } from "./user";

export type Issue = {
  title: string;
  html_url: string;
  comments: number;
  user: User;
  labels: Label[];
  created_at: string;
  updated_at: string;
};
