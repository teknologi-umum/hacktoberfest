import { it, expect, describe } from "vitest";
import { createDOM } from "@builder.io/qwik/testing";

import { ContributorCard } from "~/components/contributor-card";

import { sampleSortedContributor } from "./contributors.fixture";

it("should render contributor username as <a>", async () => {
  const { render, screen } = await createDOM();

  await render(<ContributorCard {...sampleSortedContributor} />);

  const username = screen.querySelector(".contributor-card__username");

  expect(username?.tagName).toBe("A");
  expect(username?.getAttribute("href")).toBe("https://github.com/johnsmith");
});

describe("PR grids", () => {
  it("should render as is if count < 30", async () => {
    const lessThan30Props = {
      ...sampleSortedContributor,
      merged_pulls: 10,
      pending_pulls: 2,
    };

    const { render, screen } = await createDOM();

    await render(<ContributorCard {...lessThan30Props} />);

    expect(
      screen.querySelectorAll(
        ".contributor-card__stats-bar:is(.merged,.pending,.completed)"
      )
    ).toHaveLength(12);
  });

  it("should be limited to maximum count of 30", async () => {
    const moreThan30Props = {
      ...sampleSortedContributor,
      merged_pulls: 21,
      pending_pulls: 42,
    };

    const { render, screen } = await createDOM();

    await render(<ContributorCard {...moreThan30Props} />);

    expect(
      screen.querySelectorAll(
        ".contributor-card__stats-bar:is(.merged,.pending,.completed)"
      )
    ).toHaveLength(30);
  });
});
