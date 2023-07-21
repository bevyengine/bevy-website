// clamp a number to the given range
function clamp(val, min, max) {
  return Math.min(Math.max(val, min), max)
}

// inserts the input element required to activate image_compare components
//
// Usage in a document should look like:
// ```html
// <main>
//   <div class="image-compare" style="aspect-ratio: 16 / 9"
//   data-title-a="Apples" data-title-b="Oranges">
//     <img class="image-a" alt="Apples" src="apples.png" />
//     <img class="image-b" alt="Oranges" src="oranges.png" />
//   </div>
// </main>
// ```
//
// The `image_compare` scss component should be used.
//
// Ideally the `aspect-ratio` should be set, but it will fallback to 16/9.
// You can provide `--slider-min`, `--slider-max`, `--slider-value` styles
// which will set the minimum, maximum, & starting value of the slider. They
// default to 7%, 93%, and 50% respectively.
function enable_image_compare() {
  const image_compares = document.querySelectorAll("div.image-compare");
  for (const img_cmp of image_compares) {
    // insert the input only when js is running
    let style = window.getComputedStyle(img_cmp);
    const slider = document.createElement('input');
    slider.type = "range";
    slider.min = style.getPropertyValue('--slider-min').replace('%', '');
    slider.max = style.getPropertyValue('--slider-max').replace('%', '');
    slider.value = style.getPropertyValue('--slider-value').replace('%', '');
    img_cmp.appendChild(slider);
    // setup callback
    slider.addEventListener("input", (event) => {
      img_cmp.style.setProperty('--slider-value', clamp(slider.value, slider.min, slider.max) + "%");
    });
  }
}

export { enable_image_compare };
