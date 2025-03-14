// @ts-check
/** @import {Pagefind} from "./search.mjs" */
/** @import {SearchCategories} from "./search-categories.mjs" */
/** @import {SearchTpl} from "./search-tpl.mjs" */
import { debounce } from "/tools.js";

class SearchDialog {
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
      const isSomeTextInputFocused =
        document.activeElement instanceof HTMLInputElement ||
        document.activeElement instanceof HTMLTextAreaElement;

      if (event.code === "KeyS" && !isSomeTextInputFocused) {
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

    // Open the dialog when any "trigger" element is clicked
    window.addEventListener("click", (event) => {
      if (
        event.target instanceof HTMLElement &&
        event.target.hasAttribute("data-search-open")
      ) {
        this.show();
      }
    });
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
    this.clearFilterEl.classList.add("hidden");
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

export { SearchDialog };
