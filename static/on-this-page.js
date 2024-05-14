//! in the page-level navigation, sets data-active=true on the li which you're currently looking at

let page_nav_state = new Map();

/// give data-active=true to the page nav link which points to the given id; set all others to false.
/// accepts an id string or a DOM node which it will automatically retrieve the id of
function page_nav_set_active(id_or_node){
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

let page_nav_observer =  new IntersectionObserver(
  entries => {
    entries.forEach(entry => {
      page_nav_state.set(entry.target, entry.isIntersecting);
    });
    let intersecting = Array.from(page_nav_state).filter(([k,v]) => v == true);
    intersecting.sort(([k,v]) => k.clientTop)
    if (intersecting.length > 0) {
      page_nav_set_active(intersecting[0][0]);
    }
  }, {
    rootMargin: "0px 0px 20% 0px",
    threshold: 1.0,
  });

page_nav_set_active(document.querySelector("main h2"));
document.querySelectorAll("main h2, main h3")
  .forEach(h => page_nav_observer.observe(h));
