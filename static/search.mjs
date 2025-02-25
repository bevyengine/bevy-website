// @ts-check
/**
 * @typedef Pagefind
 * @prop {(term: string) => Promise<any>} search
 */

function debounce(callback, wait) {
  let timeoutId;

  return (...args) => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => callback(...args), wait);
  };
}

class Search {
  /** @readonly */
  CLASS_VISIBLE = "search--visible";

  constructor(
    /** @type {Pagefind} */ pagefind,
    /** @type {HTMLElement} */ searchEl,
    /** @type {HTMLElement} */ searchBackdropEl,
    /** @type {HTMLElement} */ searchDialogEl,
    /** @type {HTMLElement} */ searchResultsEl,
    /** @type {HTMLElement} */ searchCloseEl,
    /** @type {HTMLInputElement} */ searchInputEl,
    /** @type {HTMLTemplateElement} */ searchNoResultsTplEl,
    /** @type {HTMLTemplateElement} */ searchResultTplEl
  ) {
    /** @private @readonly @property {Pagefind} */
    this.pagefind = pagefind;
    /** @private @readonly @property {HTMLElement} */
    this.searchEl = searchEl;
    /** @private @readonly @property {HTMLElement} */
    this.searchBackdropEl = searchBackdropEl;
    /** @private @readonly @property {HTMLElement} */
    this.searchDialogEl = searchDialogEl;
    /** @private @readonly @property {HTMLElement} */
    this.searchResultsEl = searchResultsEl;
    /** @private @readonly @property {HTMLElement} */
    this.searchCloseEl = searchCloseEl;
    /** @private @readonly @property {HTMLInputElement} */
    this.searchInputEl = searchInputEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.searchNoResultsTplEl = searchNoResultsTplEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.searchResultTplEl = searchResultTplEl;

    // Setup event listeners
    window.addEventListener("keydown", (event) => {
      if (["ArrowRight", "ArrowLeft"].includes(event.code)) {
        console.info(event.code);
        event.stopPropagation();
        event.preventDefault();
      }

      if (event.code === "Escape" && this.isOpen()) {
        event.stopPropagation();
        event.preventDefault();
        this.hide();
      }

      if (event.code === "KeyS" && !this.isOpen()) {
        event.stopPropagation();
        event.preventDefault();
        this.show();
      }
    });

    this.searchInputEl.addEventListener(
      "input",
      debounce(async () => {
        const term = this.searchInputEl.value.trim();
        this.searchResultsEl.innerHTML = "";

        if (term === "") {
          return;
        }

        try {
          const { results } = await this.pagefind.search(term);

          console.info(results);

          if (results.length) {
            const data = await Promise.all(
              results.map((result) => result.data())
            );
            console.info(data);
            data.forEach((item) => {
              this.searchResultsEl.appendChild(this.createResultEl(item));
            });
          } else {
            const noResultsEl =
              this.searchNoResultsTplEl.content.cloneNode(true);
            this.searchResultsEl.appendChild(noResultsEl);
          }
        } catch (err) {
          console.error(`Failed to search for term: "${term}"`, err);
        }
      }, 500)
    );

    this.searchCloseEl.addEventListener("click", () => {
      this.hide();
    });

    this.searchBackdropEl.addEventListener("click", () => {
      this.hide();
    });
  }

  /**
   * @private
   * @returns DocumentFragment
   */
  createResultEl(/** @type {any} */ item) {
    /** @type {DocumentFragment} */
    const resultEl = this.searchResultTplEl.content.cloneNode(true);
    const titleEl = resultEl.querySelector("[data-search-result-title]");
    const subResultsEl = resultEl.querySelector(
      "[data-search-result-sub-results]"
    );

    if (titleEl) {
      titleEl.setAttribute("href", item.url);
      titleEl.innerHTML = item.meta.title;
    }

    if (subResultsEl) {
      subResultsEl.innerHTML =
        "<ul>" +
        item.sub_results
          .map(
            (sub) =>
              `<li><a href="${sub.url}">${sub.title}</a>: ${sub.excerpt}</a></li>`
          )
          .join("\n") +
        "</ul>";
    }

    return resultEl;
  }

  /**
   * @returns {void}
   */
  hide() {
    this.searchEl.classList.remove(this.CLASS_VISIBLE);
  }

  /**
   * @returns {boolean}
   */
  isOpen() {
    return this.searchEl.classList.contains(this.CLASS_VISIBLE);
  }

  /**
   * @returns {void}
   */
  show() {
    this.searchEl.classList.add(this.CLASS_VISIBLE);
    this.searchInputEl.focus();
    this.searchInputEl.value = "";
  }
}

window.addEventListener("load", async () => {
  const searchEl = document.querySelector("[data-search]");
  const searchBackdropEl = document.querySelector("[data-search-backdrop]");
  const searchDialogEl = document.querySelector("[data-search-dialog]");
  const searchResultsEl = document.querySelector("[data-search-results]");
  const searchCloseEl = document.querySelector("[data-search-close]");
  const searchInputEl = document.querySelector("[data-search-input]");
  const searchNoResultsTplEl = document.querySelector(
    "[data-search-no-results-tpl]"
  );
  const searchResultTplEl = document.querySelector("[data-search-result-tpl]");

  if (
    searchEl instanceof HTMLElement &&
    searchBackdropEl instanceof HTMLElement &&
    searchDialogEl instanceof HTMLElement &&
    searchResultsEl instanceof HTMLElement &&
    searchCloseEl instanceof HTMLElement &&
    searchInputEl instanceof HTMLInputElement &&
    searchNoResultsTplEl instanceof HTMLTemplateElement &&
    searchResultTplEl instanceof HTMLTemplateElement
  ) {
    try {
      // @ts-ignore
      const pagefind = await import("/pagefind/pagefind.js");
      await pagefind.options({ baseUrl: "/" });

      new Search(
        pagefind,
        searchEl,
        searchBackdropEl,
        searchDialogEl,
        searchResultsEl,
        searchCloseEl,
        searchInputEl,
        searchNoResultsTplEl,
        searchResultTplEl
      );
    } catch (err) {
      console.error("Failed to initialize Pagefind.", err);
    }
  } else {
    console.error("Search elements not found.");
  }
});
