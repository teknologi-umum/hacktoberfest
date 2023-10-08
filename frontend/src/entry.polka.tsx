import { createQwikCity } from "@builder.io/qwik-city/middleware/node";
import polka from "polka";
import sirv from "sirv";
import { fileURLToPath } from "url";
import { join } from "path";
import qwikCityPlan from "@qwik-city-plan";
import render from "./entry.ssr";

// Directories where the static assets are located
const DIST_DIR = join(fileURLToPath(import.meta.url), "..", "..", "dist");
const BUILD_DIR = join(DIST_DIR, "build");
const ONE_YEAR = 1000 * 60 * 60 * 24 * 365; // 1 year

// Create the Qwik City express middleware
const { router, notFound } = createQwikCity({ render, qwikCityPlan });

// Create the express server
// https://expressjs.com/
const app = polka();

// Static asset handlers
// https://expressjs.com/en/starter/static-files.html
app.use("/build", sirv(BUILD_DIR, { maxAge: ONE_YEAR, immutable: true }));
app.use(sirv(DIST_DIR));

// Use Qwik City's page and endpoint request handler
app.use(router);

// Use Qwik City's 404 handler
app.use(notFound);

// Start the express server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`Qwik running on localhost:${PORT}`));
