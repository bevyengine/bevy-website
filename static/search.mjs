// @ts-check
/**
 * @typedef Pagefind
 * @prop {(term: string, options: any) => Promise<any>} search
 */

function debounce(callback, wait) {
  let timeoutId;

  return (...args) => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => callback(...args), wait);
  };
}

class Search {
  /** @private @readonly */
  CLASS_VISIBLE = "search--visible";
  /** @private @readonly */
  tags = {
    "Quick Start": true,
    // Book: true,
    Examples: true,
    Migrations: true,
    News: false,
    Contribute: false,
    Errors: false,
  };

  constructor(
    /** @type {Pagefind} */ pagefind,
    /** @type {Set<string>} */ pagefindTags,
    /** @type {HTMLElement} */ searchEl,
    /** @type {HTMLElement} */ searchBackdropEl,
    /** @type {HTMLElement} */ searchDialogEl,
    /** @type {HTMLElement} */ searchResultsEl,
    /** @type {HTMLElement} */ searchCloseEl,
    /** @type {HTMLInputElement} */ searchInputEl,
    /** @type {SearchTpl} */ searchTpl
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
    /** @private @readonly @property {SearchTpl} */
    this.searchTpl = searchTpl;

    Object.keys(this.tags).forEach((tag) => {
      if (!pagefindTags.has(tag)) {
        console.warn(`Tag "${tag}" not found in Pagefind search index.`);
      }
    });

    // Setup global event listeners
    window.addEventListener("keydown", (event) => {
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

    // Ensure search input doesn't trigger global shortcuts
    this.searchInputEl.addEventListener("keydown", (event) => {
      if (["ArrowRight", "ArrowLeft"].includes(event.code)) {
        event.stopPropagation();
      }
    });

    this.searchInputEl.addEventListener(
      "input",
      debounce(() => this.search(), 500)
    );
    this.searchCloseEl.addEventListener("click", () => this.hide());
    this.searchBackdropEl.addEventListener("click", () => this.hide());
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
   * @private
   * @returns {Promise<void>}
   */
  async search() {
    const term = this.searchInputEl.value.trim();
    this.searchResultsEl.innerHTML = "";

    if (term === "") {
      return;
    }

    try {
      const { results } = await this.pagefind.search(term, {
        // filters: { tag: 'Migrations' },
      });

      console.info(results);

      if (results.length) {
        const maxResults = 10;
        const minScore = 0.5;
        const data = await Promise.all(
          results
            .filter(({ score }) => score > minScore)
            .slice(0, maxResults)
            .map((result) => result.data())
        );

        console.info(data);

        data.forEach((item) => {
          this.searchResultsEl.appendChild(this.searchTpl.createResultEl(item));
        });
      } else {
        this.searchResultsEl.appendChild(this.searchTpl.createNoResultsEl());
      }
    } catch (err) {
      console.error(`Failed to search for term: "${term}"`, err);
    }
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

class SearchTpl {
  constructor(
    /** @type {HTMLTemplateElement} */ searchNoResultsTplEl,
    /** @type {HTMLTemplateElement} */ searchResultTplEl,
    /** @type {HTMLTemplateElement} */ searchSubResultTplEl
  ) {
    /** @private @readonly @property {HTMLTemplateElement} */
    this.searchNoResultsTplEl = searchNoResultsTplEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.searchResultTplEl = searchResultTplEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.searchSubResultTplEl = searchSubResultTplEl;
  }

  /**
   * @returns {DocumentFragment}
   */
  createNoResultsEl() {
    return this.searchNoResultsTplEl.content.cloneNode(true);
  }

  /**
   * @returns {DocumentFragment}
   */
  createResultEl(/** @type {any} */ item) {
    /** @type {DocumentFragment} */
    const resultEl = this.searchResultTplEl.content.cloneNode(true);
    const titleEl = resultEl.querySelector("[data-title]");
    const subResultsEl = resultEl.querySelector("[data-sub-results]");
    const moreEl = resultEl.querySelector("[data-more]");
    const maxSubResults = 3;

    if (titleEl) {
      titleEl.setAttribute("href", item.url);
      titleEl.innerHTML = item.meta.title;
    }

    if (subResultsEl) {
      item.sub_results.slice(0, maxSubResults).forEach((sub) => {
        subResultsEl.appendChild(
          this.createSubResultEl(sub.title, sub.url, sub.excerpt)
        );
      });
    }

    if (moreEl && item.sub_results.length > maxSubResults) {
      moreEl.innerHTML = `+${item.sub_results.length - maxSubResults} more`;
      moreEl.classList.remove('hidden');
    }

    return resultEl;
  }

  /**
   * @private
   * @returns {DocumentFragment}
   */
  createSubResultEl(
    /** @type {string} */ title,
    /** @type {string} */ url,
    /** @type {string} */ excerpt
  ) {
    /** @type {DocumentFragment} */
    const subResultEl = this.searchSubResultTplEl.content.cloneNode(true);
    const titleEl = subResultEl.querySelector("[data-title]");
    const subResultsEl = subResultEl.querySelector("[data-excerpt]");

    if (titleEl) {
      titleEl.setAttribute("href", url);
      titleEl.innerHTML = title.replace(/[#\s]+$/, "");
    }

    if (subResultsEl) {
      subResultsEl.innerHTML = excerpt;
    }

    return subResultEl;
  }
}

window.addEventListener("load", async () => {
  const getEl = (/** @type {string} */ id) =>
    document.querySelector(`[data-search-${id}]`);
  const searchEl = getEl("wrapper");
  const searchBackdropEl = getEl("backdrop");
  const searchDialogEl = getEl("dialog");
  const searchResultsEl = getEl("results");
  const searchCloseEl = getEl("close");
  const searchInputEl = getEl("input");
  const searchNoResultsTplEl = getEl("no-results-tpl");
  const searchResultTplEl = getEl("result-tpl");
  const searchSubResultTplEl = getEl("sub-result-tpl");

  if (
    searchEl instanceof HTMLElement &&
    searchBackdropEl instanceof HTMLElement &&
    searchDialogEl instanceof HTMLElement &&
    searchResultsEl instanceof HTMLElement &&
    searchCloseEl instanceof HTMLElement &&
    searchInputEl instanceof HTMLInputElement &&
    searchNoResultsTplEl instanceof HTMLTemplateElement &&
    searchResultTplEl instanceof HTMLTemplateElement &&
    searchSubResultTplEl instanceof HTMLTemplateElement
  ) {
    try {
      // @ts-ignore
      const pagefind = await import("/pagefind/pagefind.js");
      const filters = await pagefind.filters();

      await pagefind.options({
        baseUrl: "/",
        ranking: {
          pageLength: 0.5, // Favor longer pages
          termSaturation: 1.0, // Saturate faster repeating terms
        },
      });

      new Search(
        pagefind,
        new Set(Object.keys(filters.tag)),
        searchEl,
        searchBackdropEl,
        searchDialogEl,
        searchResultsEl,
        searchCloseEl,
        searchInputEl,
        new SearchTpl(
          searchNoResultsTplEl,
          searchResultTplEl,
          searchSubResultTplEl
        )
      );
    } catch (err) {
      console.error("Failed to initialize Pagefind.", err);
    }
  } else {
    console.error("Search elements not found.");
  }
});
