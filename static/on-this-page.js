//! in nav.on-this-page, sets data-active=true on the li which you're currently looking at

/**
 * remember which elements are on screen. * IntersectionObserver only sends us updates.
 * @type {Map<Element, boolean>}
 */
let otp_state = new Map();

/**
 * give data-active=true to the page nav link which points to the given id; set all others to false.
 * accepts an id string or a DOM node which it will automatically retrieve the id of
 * @param {string | HTMLElement} id_or_node 
 */
function otp_set_active(id_or_node){
  let id = "";
  if(typeof id_or_node == "object"){
    id = id_or_node.getAttribute("id");
  } else {
    id = id_or_node;
  }
  document.querySelectorAll(".on-this-page li").forEach((li) => {
    li.setAttribute("data-active", li.getAttribute("data-fragment") == id);
  });
}

let otp_observer =  new IntersectionObserver(
  entries => {
    entries.forEach(entry => {
      otp_state.set(entry.target, entry.isIntersecting);
    });
    let intersecting = Array.from(otp_state).filter(([k,v]) => v == true);
    intersecting.sort(([k,v]) => k.clientTop)
    if (intersecting.length > 0) {
      otp_set_active(intersecting[0][0]);
    }
  }, {
    rootMargin: "0px 0px 20% 0px",
    threshold: 1.0,
  });

otp_set_active(document.querySelector("main h2"));
document.querySelectorAll("main h2, main h3")
  .forEach(h => otp_observer.observe(h));
