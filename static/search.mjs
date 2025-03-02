// @ts-check
import { debounce } from "/tools.js";

/**
 * @typedef Pagefind
 * @prop {(term: string, options: any) => Promise<any>} search
 */
/**
 * @typedef Category
 * @prop {string} id
 * @prop {string} name
 * @prop {boolean} checked
 */

/**
 * @returns string
 */
function getCategoryId(/** @type {string} */ category) {
  return category.toLowerCase().replace(/\s+/g, "");
}

class Search {
  /** @private @readonly */
  CLASS_SEARCHING = "search--searching";
  /** @private @readonly */
  CLASS_VISIBLE = "search--visible";

  // Up/down arrows navigation
  /** @private @type {HTMLElement[] | undefined} */
  results = undefined;
  /** @private @type {HTMLElement | undefined} */
  activeResult = undefined;

  constructor(
    /** @type {Pagefind} */ pagefind,
    /** @type {SearchCategories} */ categories,
    /** @type {HTMLElement} */ searchEl,
    /** @type {HTMLElement} */ backdropEl,
    /** @type {HTMLElement} */ dialogEl,
    /** @type {HTMLElement} */ categoriesEl,
    /** @type {HTMLElement} */ contentEl,
    /** @type {HTMLElement} */ resultsEl,
    /** @type {HTMLElement} */ noResultsEl,
    /** @type {HTMLElement} */ closeEl,
    /** @type {HTMLInputElement} */ inputEl,
    /** @type {HTMLElement} */ clearFilterEl,
    /** @type {SearchTpl} */ searchTpl
  ) {
    /** @private @readonly @property {Pagefind} */
    this.pagefind = pagefind;
    /** @private @readonly @property {SearchCategories} */
    this.categories = categories;
    /** @private @readonly @property {HTMLElement} */
    this.searchEl = searchEl;
    /** @private @readonly @property {HTMLElement} */
    this.backdropEl = backdropEl;
    /** @private @readonly @property {HTMLElement} */
    this.dialogEl = dialogEl;
    /** @private @readonly @property {HTMLElement} */
    this.categoriesEl = categoriesEl;
    /** @private @readonly @property {HTMLElement} */
    this.contentEl = contentEl;
    /** @private @readonly @property {HTMLElement} */
    this.resultsEl = resultsEl;
    /** @private @readonly @property {HTMLElement} */
    this.noResultsEl = noResultsEl;
    /** @private @readonly @property {HTMLElement} */
    this.closeEl = closeEl;
    /** @private @readonly @property {HTMLInputElement} */
    this.inputEl = inputEl;
    /** @private @readonly @property {HTMLElement} */
    this.clearFilterEl = clearFilterEl;
    /** @private @readonly @property {SearchTpl} */
    this.searchTpl = searchTpl;

    // Init Categories
    this.categories.getSorted().forEach((category) => {
      this.categoriesEl.appendChild(this.searchTpl.createCategoryEl(category));
    });

    this.categoriesEl.addEventListener("click", (event) => {
      if (event.target instanceof HTMLElement) {
        const categoryId = event.target.dataset.category;

        if (categoryId) {
          const checked = this.categories.toggle(categoryId);
          event.target.classList.toggle("search-category--active", checked);
          this.search();
        }
      }
    });

    // Setup global event listeners
    window.addEventListener("keydown", (event) => {
      // Close with `Escape`
      if (event.code === "Escape" && this.isOpen()) {
        event.stopPropagation();
        event.preventDefault();
        this.hide();
      }

      // Results keyboard navigations
      if (event.code === "ArrowDown" && this.results) {
        this.focusNext();
        event.preventDefault();
      }

      if (event.code === "ArrowUp" && this.results) {
        this.focusPrevious();
        event.preventDefault();
      }

      // Open with `S`
      if (event.code === "KeyS" && this.inputEl !== document.activeElement) {
        event.stopPropagation();
        event.preventDefault();

        if (!this.isOpen()) {
          this.show();
        } else {
          this.inputEl.focus();
          this.inputEl.select();
        }
      }
    });

    // Same page navigation should close the dialog
    this.dialogEl.addEventListener("click", (event) => {
      if (
        event.target instanceof HTMLAnchorElement &&
        event.target.hasAttribute("data-search-result")
      ) {
        const maybeUrl = event.target.getAttribute("href");

        if (maybeUrl) {
          const newPath = maybeUrl.split("#")[0];

          if (window.location.pathname === newPath) {
            this.hide();
          }
        }
      }
    });

    this.inputEl.addEventListener(
      "input",
      debounce(() => this.search(), 500)
    );

    this.clearFilterEl.addEventListener("click", () => this.clearInput());
    this.closeEl.addEventListener("click", () => this.hide());
    this.backdropEl.addEventListener("click", () => this.hide());
  }

  /**
   * @private
   * @returns {void}
   */
  clearInput() {
    this.inputEl.value = "";
    this.inputEl.focus();
    this.clearFilterEl.classList.add("hidden");
    this.resetContentState(true);
  }

  /**
   * @private
   * @returns {void}
   */
  focusNext() {
    if (!this.results) {
      return;
    }

    if (!this.activeResult) {
      this.activeResult = this.results[0];
    } else {
      const idx = this.results.indexOf(this.activeResult);
      const next = this.results[idx + 1];

      if (next) {
        this.activeResult = next;
      } else {
        this.activeResult = this.results[0];
      }
    }

    this.activeResult.focus();
  }

  /**
   * @private
   * @returns {void}
   */
  focusPrevious() {
    if (!this.results) {
      return;
    }

    if (!this.activeResult) {
      this.activeResult = this.results[this.results.length - 1];
    } else {
      const idx = this.results.indexOf(this.activeResult);
      const prev = this.results[idx - 1];

      if (prev) {
        this.activeResult = prev;
      } else {
        this.activeResult = this.results[this.results.length - 1];
      }
    }

    this.activeResult.focus();
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
    const term = this.inputEl.value.trim();

    this.clearFilterEl.classList.toggle("hidden", term === "");

    if (term === "") {
      this.resetContentState(true);
      return;
    }

    let caretPosition = this.inputEl.selectionStart;

    try {
      this.searchEl.classList.add(this.CLASS_SEARCHING);
      this.inputEl.disabled = true;

      const { results: allResults } = await this.pagefind.search(term, {
        filters: {
          any: this.categories
            .getSorted()
            .filter((category) => category.checked)
            .map(({ name }) => ({ category: name })),
        },
      });

      // Reduce results set
      const maxResults = 20;
      const minScore = 0.5;
      const results = allResults
        .filter(({ score }) => score > minScore)
        .slice(0, maxResults);

      if (results.length > 0) {
        await this.showResults(results);
      } else {
        this.showNoResults(term);
      }
    } catch (err) {
      console.error(`Failed to search for term: "${term}"`, err);
    } finally {
      this.searchEl.classList.remove(this.CLASS_SEARCHING);
      this.inputEl.disabled = false;
      this.inputEl.focus();
      this.inputEl.setSelectionRange(caretPosition, caretPosition);
    }
  }

  /**
   * @private
   * @returns {void}
   */
  resetContentState(/** @type {boolean} */ hideContentWrapper = false) {
    this.contentEl.classList.toggle("hidden", hideContentWrapper);
    this.contentEl.scrollTop = 0;
    this.resultsEl.innerHTML = "";
    this.resultsEl.classList.add("hidden");
    this.noResultsEl.classList.add("hidden");
  }

  /**
   * @returns {void}
   */
  show() {
    this.searchEl.classList.add(this.CLASS_VISIBLE);
    this.inputEl.focus();
    this.inputEl.value = "";
    this.resetContentState(true);
  }

  /**
   * @private
   * @returns {Promise<void>}
   */
  async showResults(/** @type any[] */ results) {
    const data = await Promise.all(results.map((result) => result.data()));

    this.resetContentState();
    this.resultsEl.classList.remove("hidden");

    data.forEach((item) => {
      this.resultsEl.appendChild(this.searchTpl.createResultEl(item));
    });

    this.results = Array.from(
      this.resultsEl.querySelectorAll("[data-search-result]")
    );
  }

  /**
   * @private
   * @returns {void}
   */
  showNoResults(/** @type {string} */ term) {
    this.resetContentState();
    const termEl = this.noResultsEl.querySelector("[data-search-term]");
    this.noResultsEl.classList.remove("hidden");

    if (termEl) {
      termEl.innerHTML = term;
    }
  }
}

class SearchCategories {
  /** @private @readonly */
  STORAGE_KEY = "bevy-search-categories";

  constructor(
    /** @type {string[]} */ pagefindCategories,
    /** @type {string[]} */ categoriesOrder,
    /** @type {string[]} */ defaultCheckedCategories
  ) {
    // Log warnings if order/checked categories don't match with the Pagefind categories
    [...categoriesOrder, ...defaultCheckedCategories].forEach((category) => {
      if (!pagefindCategories.includes(category)) {
        console.warn(
          `Category "${category}" not found in Pagefind search index.`
        );
      }
    });

    /** @private @readonly @property {string[]}*/
    this.order = categoriesOrder;

    const checkedCategories = this.getInitialCheckedCategories(
      defaultCheckedCategories
    );

    /** @private @readonly @property {Record<string, Category>}*/
    this.categories = Object.fromEntries(
      pagefindCategories.map((name) => {
        const id = getCategoryId(name);
        return [id, { id, name, checked: checkedCategories.includes(name) }];
      })
    );
  }

  /**
   * @returns {Category[]}
   */
  getSorted() {
    // Sort by `order` or by `category` name ASC if not found
    return Object.values(this.categories).sort((a, b) => {
      const aIndex = this.order.indexOf(a.name);
      const bIndex = this.order.indexOf(b.name);

      return (
        (aIndex === -1 ? Infinity : aIndex) -
          (bIndex === -1 ? Infinity : bIndex) || a.name.localeCompare(b.name)
      );
    });
  }

  /**
   * @private
   * @returns {string[]}
   */
  getInitialCheckedCategories(/** @type {string[]} */ fallback) {
    try {
      // Load categories status from localStorage
      const savedCategoriesRaw = localStorage.getItem(this.STORAGE_KEY);

      if (savedCategoriesRaw) {
        const savedCategories = JSON.parse(savedCategoriesRaw);
        return Object.values(savedCategories)
          .filter(({ checked }) => checked)
          .map(({ name }) => name);
      }
    } catch {}

    return fallback;
  }

  /**
   * @returns {boolean}
   */
  toggle(/** @type {string} */ id) {
    const category = this.categories[id];
    category.checked = !category.checked;
    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(this.categories));

    return category.checked;
  }
}

class SearchTpl {
  constructor(
    /** @type {HTMLTemplateElement} */ categoryTplEl,
    /** @type {HTMLTemplateElement} */ resultCompactTplEl,
    /** @type {HTMLTemplateElement} */ resultTplEl,
    /** @type {HTMLTemplateElement} */ subResultTplEl
  ) {
    /** @private @readonly @property {HTMLTemplateElement} */
    this.categoryTplEl = categoryTplEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.resultCompactTplEl = resultCompactTplEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.resultTplEl = resultTplEl;
    /** @private @readonly @property {HTMLTemplateElement} */
    this.subResultTplEl = subResultTplEl;
  }

  /**
   * @returns {DocumentFragment}
   */
  createResultEl(/** @type {any} */ item) {
    const isCompact =
      item.sub_results.length === 1 && item.sub_results[0].url === item.url;

    const resultEl = /** @type {DocumentFragment} */ (
      isCompact
        ? this.resultCompactTplEl.content.cloneNode(true)
        : this.resultTplEl.content.cloneNode(true)
    );

    const titleEl = resultEl.querySelector("[data-title]");
    const categoryEl = resultEl.querySelector("[data-category]");

    if (titleEl) {
      titleEl.innerHTML = item.meta.title;
    }

    if (categoryEl) {
      if (item.filters.category.length > 0) {
        const category = item.filters.category[0];
        categoryEl.innerHTML = category;
        categoryEl.classList.add(`search-category--${getCategoryId(category)}`);
      } else {
        categoryEl.classList.add("hidden");
      }
    }

    if (isCompact) {
      const wrapperEl = resultEl.querySelector("[data-wrapper]");
      const excerptEl = resultEl.querySelector("[data-excerpt]");

      wrapperEl?.setAttribute("href", item.url);

      if (excerptEl) {
        excerptEl.innerHTML = item.sub_results[0].excerpt;
      }
    } else {
      const subResultsEl = resultEl.querySelector("[data-sub-results]");
      const moreEl = resultEl.querySelector("[data-more]");
      const maxSubResults = 5;

      titleEl?.setAttribute("href", item.url);

      if (subResultsEl) {
        item.sub_results.slice(0, maxSubResults).forEach((sub) => {
          subResultsEl.appendChild(
            this.createSubResultEl(sub.title, sub.url, sub.excerpt)
          );
        });
      }

      if (moreEl && item.sub_results.length > maxSubResults) {
        moreEl.innerHTML = `+${item.sub_results.length - maxSubResults} more`;
        moreEl.classList.remove("hidden");
      }
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
    const subResultEl = /** @type {DocumentFragment} */ (
      this.subResultTplEl.content.cloneNode(true)
    );
    const linkEl = subResultEl.querySelector("[data-link]");
    const titleEl = subResultEl.querySelector("[data-title]");
    const subResultsEl = subResultEl.querySelector("[data-excerpt]");

    if (linkEl) {
      linkEl.setAttribute("href", url);
    }

    if (titleEl) {
      titleEl.innerHTML = title.replace(/[#\s]+$/, "");
    }

    if (subResultsEl) {
      subResultsEl.innerHTML = excerpt;
    }

    return subResultEl;
  }

  /**
   * @returns {DocumentFragment}
   */
  createCategoryEl(/** @type {Category} */ category) {
    const el = /** @type {DocumentFragment} */ (
      this.categoryTplEl.content.cloneNode(true)
    );
    const categoryEl = el.querySelector("[data-category]");

    if (categoryEl instanceof HTMLElement) {
      categoryEl.innerHTML = category.name;
      categoryEl.dataset.category = category.id;
      categoryEl.classList.add(`search-category--${category.id}`);

      if (category.checked) {
        categoryEl.classList.add("search-category--active");
      }
    }

    return el;
  }
}

window.addEventListener("load", async () => {
  const getEl = (/** @type {string} */ id) =>
    document.querySelector(`[data-search-${id}]`);
  const searchEl = document.querySelector(`[data-search]`);
  const backdropEl = getEl("backdrop");
  const dialogEl = getEl("dialog");
  const categoriesEl = getEl("categories");
  const contentEl = getEl("content");
  const resultsEl = getEl("results");
  const noResultsEl = getEl("no-results");
  const closeEl = getEl("close");
  const inputEl = getEl("input");
  const clearFilterEl = getEl("clear-filter");
  const categoryTplEl = getEl("category-tpl");
  const resultCompactTplEl = getEl("result-compact-tpl");
  const resultTplEl = getEl("result-tpl");
  const subResultTplEl = getEl("sub-result-tpl");

  if (
    searchEl instanceof HTMLElement &&
    backdropEl instanceof HTMLElement &&
    dialogEl instanceof HTMLElement &&
    categoriesEl instanceof HTMLElement &&
    contentEl instanceof HTMLElement &&
    resultsEl instanceof HTMLElement &&
    noResultsEl instanceof HTMLElement &&
    closeEl instanceof HTMLElement &&
    inputEl instanceof HTMLInputElement &&
    clearFilterEl instanceof HTMLElement &&
    categoryTplEl instanceof HTMLTemplateElement &&
    resultCompactTplEl instanceof HTMLTemplateElement &&
    resultTplEl instanceof HTMLTemplateElement &&
    subResultTplEl instanceof HTMLTemplateElement
  ) {
    try {
      // @ts-ignore
      const pagefind = await import("/pagefind/pagefind.js");
      const filters = await pagefind.filters();
      const categories = new SearchCategories(
        Object.keys(filters.category),
        [
          "Quick Start",
          "Examples",
          "Migrations",
          "News",
          "Contribute",
          "Errors",
        ],
        ["Quick Start", "Examples"]
      );

      await pagefind.options({
        baseUrl: "/",
        // ranking: {
        //   pageLength: 0.5, // Favor longer pages (default: 0.75)
        //   termSaturation: 1.0, // Saturate faster repeating terms (default: 1.4)
        //   termSimilarity: 1.5, // Make stricter matches (default: 1.0)
        // },
      });

      new Search(
        pagefind,
        categories,
        searchEl,
        backdropEl,
        dialogEl,
        categoriesEl,
        contentEl,
        resultsEl,
        noResultsEl,
        closeEl,
        inputEl,
        clearFilterEl,
        new SearchTpl(
          categoryTplEl,
          resultCompactTplEl,
          resultTplEl,
          subResultTplEl
        )
      );
    } catch (err) {
      console.error("Failed to initialize Pagefind.", err);
    }
  } else {
    console.error(
      "Not all the elements needed to build the Search dialog were found."
    );
  }
});
