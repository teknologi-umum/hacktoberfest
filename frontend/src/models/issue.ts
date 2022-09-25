import type { Label } from "./label";

export type Issue = {
  title: string;
  html_url: string;
  labels: Label[];
};
