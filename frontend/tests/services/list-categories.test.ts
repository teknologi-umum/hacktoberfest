import { expect, it } from "vitest";
import { getCategoriesList } from "~/services/list-categories";
import { sampleRepositories } from "./repositories.fixture";

it("should get labels from the repositories", () => {
  const categories = getCategoriesList(sampleRepositories);

  expect(categories).toContain("scope: frontend");
  expect(categories).toContain("difficulty: easy");
  expect(categories).toContain("difficulty: medium");
  expect(categories).toContain("enhancement");
  expect(categories).toContain("good first issue");
  expect(categories).toContain("help wanted");
});

it("should not contain duplicate entries", () => {
  const categories = getCategoriesList(sampleRepositories);

  expect(
    categories.filter((category) => category === "difficulty: easy")
  ).toHaveLength(1);
  expect(
    categories.filter((category) => category === "good first issue")
  ).toHaveLength(1);
  expect(
    categories.filter((category) => category === "help wanted")
  ).toHaveLength(1);
});

it('should exclude "hacktoberfest" labels', () => {
  const categories = getCategoriesList(sampleRepositories);

  expect(categories).not.toContain("hacktoberfest");
});
