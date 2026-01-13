// @ts-check
import { SearchCategories } from "./search-categories.mjs";
import { SearchDialog } from "./search-dialog.mjs";
import { SearchTpl } from "./search-tpl.mjs";

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

/** @type {SearchDialog | undefined} */
let searchDialog;

/** @returns boolean */
function isSearchOpen() {
  return searchDialog?.isOpen() ?? false;
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

      searchDialog = new SearchDialog(
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

export { isSearchOpen };
