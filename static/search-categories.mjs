// @ts-check
/** @import {Category} from "./search.mjs" */

/**
 * @returns string
 */
function getCategoryId(/** @type {string} */ category) {
  return category.toLowerCase().replace(/\s+/g, "");
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

export { getCategoryId, SearchCategories };
