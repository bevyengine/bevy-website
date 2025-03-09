// these helpers should be totally optional. if someone disables this javascript in their browser,
// core functionality should not be affected.

function navigateDocsNext(/** @type {KeyboardEvent} */ e) {
    const searchEl = document.querySelector(`[data-search]`);

    if (searchEl && searchEl.classList.contains('search--visible')) {
        return;
    }

    if (e.key == 'ArrowLeft') {
        const previous = document.querySelector("[data-docs-nav-previous]");

        if (previous instanceof HTMLAnchorElement) {
            previous.click();
        }

    } else if (e.key == 'ArrowRight') {
        const next = document.querySelector("[data-docs-nav-next]");

        if (next instanceof HTMLAnchorElement) {
            next.click();
        }
    }
}

window.addEventListener('load', function () {
    window.addEventListener('keydown', navigateDocsNext);
});
