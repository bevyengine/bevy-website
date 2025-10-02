"use strict";

const puppeteer = require("puppeteer");

const getContext = async () => {
	const browser = await puppeteer.launch(chromeLaunchConfig: {
        executablePath: '/usr/bin/chromium-browser',
        ignoreHTTPSErrors: true
    });
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
