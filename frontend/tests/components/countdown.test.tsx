import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createDOM } from "@builder.io/qwik/testing";

import { Countdown } from "~/components/countdown";

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe("message and countdown relative to time of the event", () => {
  it("should show time left until START_DATE if event is upcoming", async () => {
    const { render, screen } = await createDOM();

    vi.setSystemTime(new Date("27 Sept 2023 06:30:52+07:00"));

    await render(<Countdown />);

    expect(screen.querySelector(".countdown")?.textContent).toBe(
      "03 hari17 jam29 menit08 detiksampai Hacktoberfest dimulai"
    );
  });

  it("should show time left until END_DATE if event is ongoing", async () => {
    const { render, screen } = await createDOM();

    vi.setSystemTime(new Date("10 Oct 2023 15:25:02+07:00"));

    await render(<Countdown />);

    expect(screen.querySelector(".countdown")?.textContent).toBe(
      "21 hari08 jam34 menit58 detiksampai Hacktoberfest berakhir"
    );
  });

  it("should show time after END_DATE if is event over", async () => {
    const { render, screen } = await createDOM();

    vi.setSystemTime(new Date("3 Nov 2023 20:00:00+07:00"));

    await render(<Countdown />);

    expect(screen.querySelector(".countdown")?.textContent).toBe(
      "02 hari20 jam00 menit00 detiksejak Hacktoberfest berakhir"
    );
  });
});
