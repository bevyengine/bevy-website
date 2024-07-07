import Fuse from 'https://cdn.jsdelivr.net/npm/fuse.js@7.0.0/dist/fuse.mjs'

document.getElementById("search-dialog").addEventListener('click', function (event) {
    // allow clicking out of the search dialog
    // based on https://stackoverflow.com/a/69421512
    var rect = this.getBoundingClientRect();
    if (!(rect.top <= event.clientY && event.clientY <= rect.top + rect.height
        && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)) {
        this.close();
    }
});

document.addEventListener("keydown", event => {
    if(event.ctrlKey && event.key == "k") {
        event.preventDefault();
        SEARCH.open();
    }
})

class Search {
    RESULTS_LIMIT = 10;
    $dialog = document.getElementById("search-dialog")
    $input = document.getElementById("search-dialog__input")
    $results = document.getElementById("search-dialog__results")
    $search_tip_list = document.querySelector("#search-dialog aside ul");
    // <script> tags are contrived such that elasticlunr and searchIndex will be ready
    // someday, hopefully: switch to .mjs for searchIndex and elasticlunr
    index = elasticlunr.Index.load(window.searchIndex);

    open() {
        this.$dialog.showModal();
        this.change_tip();
        this.search();
    }

    close() {
        this.$dialog.close();
    }

    change_tip() {
        let length = this.$search_tip_list.children.length;
        let choice = Math.floor(Math.random() * length);
        console.debug("chose tip", choice);
        Array.from(this.$search_tip_list.children).forEach(
            (tip, i) => tip.setAttribute("data-chosen", i == choice
            ));
    }

    async search() {
        let current_path = window.location.pathname.split("/");
        /** @type {string} */
        const query = this.$input.value;
        console.debug(`search: "${query}"`);
        if (query.length == 0) {
            return;
        }
        /** @type any[] */
        let results = this.index.search(query, {});
        results.forEach(result => {
            result.ref = new URL(result.ref).pathname;
            let similarity = result.ref
                .split("/")
                .map((part, i) => part == current_path[i])
                .filter(v => v).length - 1;
            result.score += 0.1 * similarity;
        });
        results.sort((a, b) => b.score - a.score);
        this.$results.innerHTML = "";
        results.slice(0, this.RESULTS_LIMIT).forEach(result => {
            const a = document.createElement("a");
            a.innerText = `${result.doc.title}`;
            a.role = "listitem";
            a.href = result.ref;
            a.setAttribute("data-score", result.score)
            this.$results.appendChild(a)
        })
    }
}

let SEARCH = new Search()
export default SEARCH;