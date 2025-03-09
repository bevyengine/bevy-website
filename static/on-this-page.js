//! in nav.on-this-page, sets data-active=true on the link to the header you're currently looking at

if(window.location.hash == ""){
  otp_set_active(document.querySelector("main h2") || document.querySelector("main h3"));
} else {
  otp_set_active(window.location.hash.substring(1));
}

/**
 * remember which elements are on screen. IntersectionObserver only sends us updates.
 * @type {Map<Element, boolean>}
 */
let otp_state = new Map();

/**
 * give data-active=true to the page nav link which points to the given id; set all others to false.
 * accepts an id string or a DOM node which it will automatically retrieve the id of
 * @param {string | HTMLElement} id_or_node 
 */
function otp_set_active(id_or_node){
  let id = `#${id_or_node instanceof HTMLElement ? id_or_node.getAttribute("id") : id_or_node}`;

  document.querySelectorAll(".on-this-page a").forEach(a => {
    a.setAttribute("data-active", a.getAttribute("href") == id);
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
