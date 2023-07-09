
// clamp a number to the given range
function clamp(val, min, max) {
  return Math.min(Math.max(val, min), max)
}

// inserts the input element required to activate image_compare components
//
// Usage in a document should look like:
// ```html
// <main>
//   <div class="image-compare" style="aspect-ratio: 16 / 9">
//     <img class="image-a" src="$url" />
//     <img class="image-b" src="$url" />
//   </div>
// </main>
// ```
//
// The `image_compare` scss component should be used.
//
// Ideally the `aspect-ratio` should be set, but it will
// fallback to 16/9.
function enable_image_compare() {
  const image_compares = document.querySelectorAll("div.image-compare");
  for (const img_cmp of image_compares) {
    // insert the input only when js is running
    const slider = document.createElement('input');
    slider.type = "range";
    slider.min = "0";
    slider.max = "100";
    slider.value = "50";
    img_cmp.appendChild(slider);
    // setup callback
    img_cmp.style.setProperty('--slider-value', clamp(slider.value, 7, 93) + "%");
    slider.addEventListener("input", (event) => {
      img_cmp.style.setProperty('--slider-value', clamp(slider.value, 7, 93) + "%");
    });
  }
}

export { enable_image_compare };
