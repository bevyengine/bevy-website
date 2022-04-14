// these helpers should be totally optional. if someone disables this javascript in their browser,
// core functionality should not be affected.

function navigateBookNext(e) {
    if (e.key == 'ArrowLeft') {
        var previous = document.querySelector("[data-book-nav-previous]");
        if (previous) {
            previous.click();
        }

    } else if (e.key == 'ArrowRight') {
        var next = document.querySelector("[data-book-nav-next]");
        if (next) {
            next.click();
        }
    }
}

window.addEventListener('load', function () {
    window.addEventListener('keydown', navigateBookNext);
});
