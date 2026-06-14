//! For elements with `[data-highlight-intrapage-link]`, sets
//! `data-highlight-intrapage-link-active=true` to the descendant `<a>` elements
//! (for intra-page navigation) that link to the top-most visible heading.

if(window.location.hash == ""){
  otp_set_active(document.querySelector("main h2") || document.querySelector("main h3"));
} else {
  otp_set_active(window.location.hash.substring(1));
}

/**
 * Remember which elements are on screen. `IntersectionObserver` only sends us updates.
 * @type {Map<Element, boolean>}
 */
let otp_state = new Map();

/**
 * Set `data-highlight-intrapage-link-active=true` to the page links which point
 * to the given id; set all others to `false`. Accepts an id string or a DOM
 * node which it will automatically retrieve the id of.
 */
function otp_set_active(id_or_node){
  let id = `#${id_or_node instanceof HTMLElement ? id_or_node.getAttribute("id") : id_or_node}`;

  document.querySelectorAll("[data-highlight-intrapage-link] a").forEach(a => {
    const href = a.getAttribute("href");

    if (href?.includes('#')) {
      const fragment = href.substring(href.indexOf('#'));
      a.setAttribute("data-highlight-intrapage-link-active", String(id === fragment));
    }
  });
}

let headerHeight = getComputedStyle(document.body).getPropertyValue(
  "--layout-header-height"
);

let otp_observer =  new IntersectionObserver(
  entries => {
    entries.forEach(entry => {
      otp_state.set(entry.target, entry.isIntersecting);
    });

    let intersecting = Array.from(otp_state)
      .filter(([_el, inter]) => inter)
      .map(([el, _inter]) => el);

    intersecting.sort((a, b) => a.getBoundingClientRect().y - b.getBoundingClientRect().y);

    if (intersecting.length > 0) {
      otp_set_active(intersecting[0]);
    }
  }, {
    rootMargin: `-${headerHeight} 0px 20% 0px`,
    threshold: 1.0,
  });

document.querySelectorAll("main h2, main h3")
  .forEach(h => otp_observer.observe(h));
