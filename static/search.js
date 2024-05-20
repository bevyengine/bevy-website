
/**
 * the actual sauce. sends the search off to elasticlunr and displays the results.
 */
function search_key_handler() {
    console.trace("search_key_handler()")
    let $dialog = document.getElementById("search-dialog")
    let $recent = $dialog.querySelector(".search-dialog__recent")
    let $results = $dialog.querySelector(".search-dialog__results")
    let query = $dialog.querySelector("form input").value;
    if (query.length > 0) {
        $recent.style.display = "none";
        $results.style.display = null;
    } else {
        $recent.style.display = "block";
        $results.style.display = "none";
        return;
    }
    // search_history_push(query);
    let result = SEARCH_INDEX.search(query, {
        fields: {
            title: { boost: 2 },
            body: { boost: 1 }
        },
        expand: true
    });
    if (result.length == 0) {
        console.trace("no results");
    }
    result = result.slice(0, SEARCH_RESULTS_LIMIT);
    $results.innerHTML = "";
    result.forEach(result => {
        let link = document.createElement("a");
        link.href = result.ref;
        link.role = "list-item";
        link.innerText = `${result.doc.title}`;
        $results.append(link);
    });
}

document.getElementById("search-dialog").addEventListener('click', function (event) {
    // based on https://stackoverflow.com/a/69421512
    var rect = this.getBoundingClientRect();
    if (!(rect.top <= event.clientY && event.clientY <= rect.top + rect.height
        && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)) {
        this.close();
    }
});

/// just a contrivance to clean up the global namespace
class __Search {
    RESULTS_LIMIT = 10;
    $dialog = document.getElementById("search-dialog")
    $input = document.getElementById("search-dialog__input")
    $search_tip_list = document.querySelector("#search-dialog aside ul");

    open() {
        this.$dialog.showModal();
        this.change_tip();
        this.search();
    }

    change_tip() {
        let length = this.$search_tip_list.children.length;
        let choice = Math.floor(Math.random() * length);
        console.trace("chose tip", choice);
        Array.from(this.$search_tip_list.children).forEach(
            (tip, i) => tip.setAttribute("data-chosen", i == choice
        ));
    }

    search() {
        /** @type {string} */
        const query = this.$input.value;
        console.trace("Search::handle:", query);
    }
}

const SEARCH = new __Search()