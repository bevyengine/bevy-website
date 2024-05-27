document.getElementById("search-dialog").addEventListener('click', function (event) {
    // allow clicking out of the search dialog
    // based on https://stackoverflow.com/a/69421512
    var rect = this.getBoundingClientRect();
    if (!(rect.top <= event.clientY && event.clientY <= rect.top + rect.height
        && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)) {
        this.close();
    }
});

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

    change_tip() {
        let length = this.$search_tip_list.children.length;
        let choice = Math.floor(Math.random() * length);
        console.debug("chose tip", choice);
        Array.from(this.$search_tip_list.children).forEach(
            (tip, i) => tip.setAttribute("data-chosen", i == choice
            ));
    }

    async search() {
        let fetched = await (await fetch("/search_index.en.json")).json()
        /** @param {string} path */
        function section_prio(path) {
            if (path.startsWith("/examples")) {
                return 0.5;
            }
            return 1;
        }
        /** @type {string} */
        const query = this.$input.value;
        console.debug(`search: "${query}"`);
        if (query.length == 0) {
            return;
        }
        /** @type any[] */
        let results = this.index.search(query);
        console.debug(results);
        this.$results.innerHTML = "";
        results.slice(0, this.RESULTS_LIMIT).forEach(result => {
            const a = document.createElement("a");
            console.debug(result)
            a.innerText = `${result.doc.title}`;
            a.role = "listitem";
            a.href = result.ref;
            this.$results.appendChild(a)
        })
    }

    sort_key(result) {
        if(result.item.path.startsWith("/examples")) {
            return result.score / 2;
        }
        return result.score;
    }
}

export default new Search()