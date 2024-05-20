const SEARCH_HISTORY_LOCALSTORAGE_KEY = "search_history"
const SEARCH_HISTORY_MAX = 5;
/** 
 * <script> tags are ordered such that this will work without checking
 * types stolen from https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/elasticlunr/index.d.ts
 * @typedef {{score: number, ref: string, doc: {title: string, body: string}}} SearchResult
 * @type {{search: (query: string) => SearchResult[]}}
 */
const SEARCH_INDEX = elasticlunr.Index.load(searchIndex);

/**
 * opens the search dialog, updates "recent searches", and randomizes the dialog tip
 */
function search_open() {
    document.getElementById("search-dialog").showModal();
    search_history_show();
    search_tip();
    search_key_handler();
}

/**
 * get history from localStorage
 * clears history and returns [] if anything goes wrong
 * @returns {string[]}
 */
function search_history_get() {
    let history = localStorage.getItem(SEARCH_HISTORY_LOCALSTORAGE_KEY) ?? "[]";
    try {
        history = JSON.parse(history);
    } catch {
        console.warn(`Expected localStorage ${SEARCH_HISTORY_LOCALSTORAGE_KEY} to parse as json, but it was`, history, "clearing.");
        localStorage.removeItem(SEARCH_HISTORY_LOCALSTORAGE_KEY);
        return [];
    }
    console.trace("history from localStorage:", history)
    if (!Array.isArray(history)) {
        console.warn(`Expected localStorage ${SEARCH_HISTORY_LOCALSTORAGE_KEY} to be an array, but it was`, history, "clearing.")
        localStorage.removeItem(SEARCH_HISTORY_LOCALSTORAGE_KEY);
        return [];
    }
    return history;
}

/**
 * add `search` to history in localStorage. does not update #search-dialog. returns the new history array.
 * @param {string} search 
 * @returns {string[]}
 */
function search_history_push(search) {
    let history = search_history_get();
    history.splice(0, 0, search);
    if (history.length > SEARCH_HISTORY_MAX) {
        history.pop()
    }
    localStorage.setItem(SEARCH_HISTORY_LOCALSTORAGE_KEY, JSON.stringify(history));
    return history
}

/**
 * gets search history from localStorage and updates the list in #search-dialog
 */
function search_history_show() {
    let history = search_history_get();
    let $parent = document.getElementById("search-dialog").querySelector(".search-dialog__recent");
    let $none = $parent.querySelector(".search-dialog__recent__none");
    let $some = $parent.querySelector(".search-dialog__recent__some");
    let $list = $some.querySelector("ul");
    if (history.length == 0) {
        $some.style.display = "none";
        $none.style.display = null;
    } else {
        $none.style.display = "none";
        $some.style.display = null;
        $list.innerHTML = ""; // remove children
        history.forEach(item => {
            let li = document.createElement("li");
            li.innerText = item;
            $list.appendChild(li)
        });
    }
}

/**
 * show a new search dialog tip
 */
function search_tip() {
    let $list = document.querySelector("#search-dialog aside ul");
    let length = $list.children.length;
    let choice = Math.floor(Math.random() * length);
    console.trace("chose tip", choice);
    Array.from($list.children).forEach((n, i) => n.setAttribute("data-chosen", i==choice));
}

/**
 * the actual sauce. sends the search off to elasticlunr and displays the results.
 */
function search_key_handler(){
    let $dialog = document.getElementById("search-dialog")
    let $recent = $dialog.querySelector(".search-dialog__recent")
    let $results = $dialog.querySelector(".search-dialog__results")
    let query = $dialog.querySelector("form input").value;
    if(query.length > 0) {
        $recent.style.display = "none";
    } else {
        $recent.style.display = null;
        return;
    }
    let result = SEARCH_INDEX.search(query);
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