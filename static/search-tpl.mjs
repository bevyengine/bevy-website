// @ts-check
/** @import {Category} from "./search.mjs" */
import { getCategoryId } from "./search-categories.mjs";

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
      if (item.filters.category?.length > 0) {
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

export { SearchTpl };
