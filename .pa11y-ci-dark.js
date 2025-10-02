"use strict";

const puppeteer = require("puppeteer");

const getContext = async () => {
	const browser = await puppeteer.launch({
        executablePath: '/usr/bin/chromium-browser',
        /* Disabling the sandbox is a bit dangerous and stupid,
        but at this point we've ran out of options. The proper
        way to solve this issue requires permissions on the
        GitHub machine runner we can't have for security reasons. */
        args: ['--no-sandbox']
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
