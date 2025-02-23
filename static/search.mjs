class Search {
  CLASS_VISIBLE = "search--visible";

  constructor() {
    this.searchEl = document.querySelector("[data-search]");
    this.searchBackdropEl = document.querySelector("[data-search-backdrop]");
    this.searchDialogEl = document.querySelector("[data-search-dialog]");
  }

  isOpen() {
    return this.searchEl.classList.contains(this.CLASS_VISIBLE);
  }

  show() {
    this.searchEl.classList.add(this.CLASS_VISIBLE);
  }
}

window.addEventListener('load', () => {
  const search = new Search();

  window.addEventListener('keydown', (event) => {
    search.show();
    // if (event.code === "Escape" && !isHidden) {
    //   pagefindEl.classList.add("hidden");
    //   event.stopPropagation();
    //   event.preventDefault();
    // }
  
    // if (event.code === "KeyS" && isHidden) {
    //   pagefindEl.classList.remove("hidden");
  
    //   new PagefindUI({
    //     element: pagefindEl,
    //     showSubResults: true,
    //     showImages: false,
    //     autofocus: true,
    //   });
  });
});

// const pagefindEl = document.querySelector("[data-pagefind-search-wrapper]");

// pagefindEl.addEventListener("click", (event) => {
//   if (event.target === pagefindEl) {
//     pagefindEl.classList.add("hidden");
//   }
// });

// window.addEventListener("keypress", 
// });
