import { expect, it } from "vitest";
import { createDOM } from "@builder.io/qwik/testing";

import { IssueCard } from "~/components/issue-card";

import { sampleIssue } from "./contributors.fixture";

const defaultProps = sampleIssue;

it("should only display first 3 labels", async () => {
  const { render, screen } = await createDOM();

  await render(<IssueCard {...defaultProps} />);

  expect(screen.querySelectorAll(".label")).toHaveLength(3);
});
