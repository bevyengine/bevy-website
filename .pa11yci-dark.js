"use strict";

const puppeteer = require("puppeteer");
const fs = require("node:fs");

const getContext = async () => {
	const browser = await puppeteer.launch({});
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
