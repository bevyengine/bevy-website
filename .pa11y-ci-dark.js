"use strict";

const puppeteer = require("puppeteer");
const fs = require("node:fs");

let settings = {};
fs.readFile("./.pa11yci", (error, content) => {
	if (err) throw err;

	settings = JSON.parse(content);
}
)

console.log(settings.toString());

const getContext = async () => {
	const browser = await puppeteer.launch(settings.defaults.chromeLaunchConfig);
	const page = await browser.newPage();
	await page.emulateMediaFeatures([
		{ name: "prefers-color-scheme", value: "dark" },
	]);
	return { browser, page };
};

const context = getContext().then((c) => c);

module.exports = {
	defaults: {
		browser: context.browser,
		page: context.page,
	},
};
