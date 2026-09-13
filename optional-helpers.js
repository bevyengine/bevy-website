// these helpers should be totally optional. if someone disables this javascript in their browser,
// core functionality should not be affected.

function navigateDocsNext(e) {
    if (e.key == 'ArrowLeft') {
        var previous = document.querySelector("[data-docs-nav-previous]");
        if (previous) {
            previous.click();
        }

    } else if (e.key == 'ArrowRight') {
        var next = document.querySelector("[data-docs-nav-next]");
        if (next) {
            next.click();
        }
    }
}

window.addEventListener('load', function () {
    window.addEventListener('keydown', navigateDocsNext);
});
