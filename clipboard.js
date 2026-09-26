(() => {
    'use strict';
    document.addEventListener('DOMContentLoaded', () => {
        // Don't add the "copy to clipboard" button if we can't copy to clipboard :)
        if (!navigator.clipboard || !navigator.clipboard.writeText) {
            return;
        }

        // Downloaded from https://dazzleui.gumroad.com/l/dazzleiconsfree
        // Author: Dazzle UI
        // License: CC-BY
        // Modifications:
        // - Removing `width` & `height` to adapt to the container size.
        // - Replacing `stroke="black"` with `stroke="currentColor"` to respect dark/light modes.
        const svg_clone = '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M8 8H7.2C6.0799 8 5.51984 8 5.09202 8.21799C4.71569 8.40973 4.40973 8.71569 4.21799 9.09202C4 9.51984 4 10.0799 4 11.2V16.8C4 17.9201 4 18.4802 4.21799 18.908C4.40973 19.2843 4.71569 19.5903 5.09202 19.782C5.51984 20 6.0799 20 7.2 20H12.8C13.9201 20 14.4802 20 14.908 19.782C15.2843 19.5903 15.5903 19.2843 15.782 18.908C16 18.4802 16 17.9201 16 16.8V16M11.2 16H16.8C17.9201 16 18.4802 16 18.908 15.782C19.2843 15.5903 19.5903 15.2843 19.782 14.908C20 14.4802 20 13.9201 20 12.8V7.2C20 6.0799 20 5.51984 19.782 5.09202C19.5903 4.71569 19.2843 4.40973 18.908 4.21799C18.4802 4 17.9201 4 16.8 4H11.2C10.0799 4 9.51984 4 9.09202 4.21799C8.71569 4.40973 8.40973 4.71569 8.21799 5.09202C8 5.51984 8 6.07989 8 7.2V12.8C8 13.9201 8 14.4802 8.21799 14.908C8.40973 15.2843 8.71569 15.5903 9.09202 15.782C9.51984 16 10.0799 16 11.2 16Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>';
        const svg_check = '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M4 12.6111L8.92308 17.5L20 6.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>';

        function ready(btn) {
            btn.ariaLabel = 'Copy';
            btn.innerHTML = svg_clone;
        }

        function success(btn) {
            btn.ariaLabel = 'Copied!';
            btn.innerHTML = svg_check;
        }

        // Add button to every `<pre><code>`
        [].forEach.call(document.querySelectorAll('pre:has(code)'), function(pre) {
            const btn = document.createElement('button');
            btn.classList.add('copy');
            ready(btn);
            btn.onclick = () => navigator.clipboard.writeText(pre.firstChild.innerText).then(() => {
                // Temporarily show a check mark to visually indicate success
                success(btn);
                setTimeout(() => ready(btn), 2000);
            });
            pre.appendChild(btn);
        });
    });
})();
