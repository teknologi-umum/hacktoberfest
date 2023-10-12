import { expect, it, vi } from "vitest";
import { getContributorsList } from "~/services/list-contributors";
import { sampleContributors } from "./contributors.fixture";

global.fetch = vi.fn();

it("should exclude [bot] users", async () => {
  vi.mocked(fetch).mockResolvedValue({
    ok: true,
    json: () => Promise.resolve(sampleContributors),
  } as Response);

  const contributors = await getContributorsList();

  expect(contributors.map((c) => c.full_name)).not.toContain("dependabot[bot]");
});
