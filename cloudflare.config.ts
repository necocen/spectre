import { defineConfig } from "cf/config";

export default defineConfig({
	worker: {
		name: "spectre",
		compatibilityDate: "2026-10-07",
		workersDev: true,
		assets: {
			notFoundHandling: "single-page-application",
		},
		domains: [
			"spectre.necocen.info",
		],
	},
});
