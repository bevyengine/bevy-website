import Fuse from 'https://cdn.jsdelivr.net/npm/fuse.js@7.0.0/dist/fuse.mjs'

document.getElementById("search-dialog").addEventListener('click', function (event) {
    // allow clicking out of the search dialog
    // based on https://stackoverflow.com/a/69421512
    var rect = this.getBoundingClientRect();
    if (event.target == this && !(rect.top <= event.clientY && event.clientY <= rect.top + rect.height
        && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)) {
        this.close();
    }
});

document.addEventListener("keydown", event => {
    if (event.ctrlKey && event.key == "k") {
        event.preventDefault();
        SEARCH.open();
    }
})

class Search {
    RESULTS_LIMIT = 10;
    FUSE_OPTIONS = {
        keys: ["title", "body"],
        includeMatches: true,
        minMatchCharLength: 3,
    };
    index = null;
    previous_search = null;
    $dialog = document.getElementById("search-dialog")
    $input = document.getElementById("search-dialog__input")
    $results = document.getElementById("search-dialog__results")
    $select = document.getElementById("search-dialog__select")
    $search_tip_list = document.querySelector("#search-dialog aside ul");

    async setup() {
        this.index ??= await (await fetch("/search_index.en.json")).json();
        this.fuse ??= new Fuse(this.index, this.FUSE_OPTIONS);
        console.debug("fetched search index", this.index);
    }

    async open() {
        if(this.$dialog.getAttribute("open") !== null) {
            // already open
            this.$input.focus();
        } else {
            this.$dialog.showModal();
            this.change_tip();
            this.change_placeholder();
            await this.setup();
            await this.search();
        }
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

    change_placeholder() {
        let second_part = "";
        switch (this.$select.value) {
            case "docs":
                second_part = "'s Documentation";
                break;
            case "assets":
                second_part = "'s Assets";
                break;
            case "examples":
                second_part = "'s Examples";
                break;
        }
        this.$input.setAttribute("placeholder", `Search Bevy${second_part}...`);
    }

    async search() {
        let current_path = window.location.pathname.split("/");
        /** @type {string} */
        const query = this.$input.value;
        console.debug(`search: "${query}"`);
        if (query == this.previous_search) {
            // cursor movements trigger this function
            return;
        }
        this.previous_search = query;
        if (query.length == 0) {
            return;
        }
        /** @type {{item: any}[]} */
        let results = this.fuse.search(query);
        console.debug(results);
        let results_limit = results.slice(0, this.RESULTS_LIMIT);
        this.$results.innerHTML = "";
        results_limit.forEach((result) => {
            let a = document.createElement("a");
            a.innerText = result.item.title;
            a.href = result.item.path;
            this.$results.appendChild(a);
        });
    }

}

window.SEARCH = new Search()