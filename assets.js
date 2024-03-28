let filters_state = {
    search_terms: [''],
    version: '*'
}

const check_filters = (filters) => (node) => {
    let results = filters.map(filter => filter(node));
    return results.every(val => val)
}

const pass_filters = check_filters([
    filter_search_terms,
    filter_version
]);

function filter_assets() {
    document.querySelectorAll('.asset-card').forEach(asset => {
        asset.parentElement.style.display = pass_filters(asset) ? 'block' : 'none'
    })
}

//  ------------    Search terms Filtering

const searchElement = document.querySelector('#assets-search')
const searchUpdateDelay = 350

// Prevents `callback` from being called more than once within `wait` milliseconds.
function debounce(callback, wait) {
    let timeoutId
    return (...args) => {
        clearTimeout(timeoutId)
        timeoutId = setTimeout(() => callback(...args), wait)
    }
}

searchElement.addEventListener("input", debounce(_ => {
    filters_state.search_terms = searchElement.value.toLowerCase().split(' ');
    filter_assets()
    hideEmptySubSections()
    hideEmptySections()
    updateSuggestionLinks()
}, searchUpdateDelay))

function filter_search_terms(asset_node) {
    const fullText = asset_node.text.toLowerCase()
    return filters_state.search_terms.every((term) => fullText.includes(term))
}

function hideEmptySubSections() {
    for (const itemGrid of document.querySelectorAll('.item-grid')) {
        const cardInGrid = [...itemGrid.querySelectorAll('.asset-card')]
        const areAllHidden = (cardInGrid.every((card) => card.parentElement.style.display === 'none'))
        itemGrid.style.display = areAllHidden ? 'none' : 'grid'
        itemGrid.previousElementSibling.style.display = areAllHidden ? 'none' : 'block'
    }
}

function hideEmptySections() {
    document.querySelectorAll('.asset-section').forEach(section => {
        let nextElement = section.nextElementSibling
        while (nextElement && (nextElement.classList.contains('asset-subsection') || nextElement.classList.contains('item-grid'))) {
            if (nextElement.style.display !== 'none') {
                section.style.display = 'block'
                return
            }
            nextElement = nextElement.nextElementSibling
        }
        section.style.display = 'none'
    })
}

// A dictionary of 'suggestion-id': ['base-url', 'query-suffix'].
const suggestionsUrls = {
    '#suggestion-github': ['https://github.com/bevyengine/bevy/discussions', '?discussions_q='],
    '#suggestion-cheatbook': ['https://bevy-cheatbook.github.io/', '?search='],
    '#suggestion-docs': ['https://docs.rs/bevy/latest/bevy/', '?search='],
}

function updateSuggestionLinks() {
    const searchValue = searchElement.value.toLowerCase()

    if (searchValue === '') {
        for (const [linkId, [uriBase, _]] of Object.entries(suggestionsUrls)) {
            // No search value, just set base url
            document.querySelector(linkId).href = uriBase
        }
    } else {
        for (const [linkId, [uriBase, query]] of Object.entries(suggestionsUrls)) {
            // Some search value, add query
            document.querySelector(linkId).href = uriBase + query + encodeURIComponent(searchValue)
        }
    }
}

//  ------------    Version Filtering
const version_always_show = ['*', 'main', '']

function sort_versions(a, b) {
    let a1 = a.split('.').map(i => i.padStart(3, '0')).join('');
    let b1 = b.split('.').map(i => i.padStart(3, '0')).join('');
    return b1 - a1;
}

function normalize_version(raw_version) {
    let version = raw_version?.replace(/^[^\d]+/, '').replace(/[^\d]+$/, '');
    let normalized_version = version ? Array.from({ ...version.split('.'), length: 3 }, (v, i) => v ?? 0).join('.') : '*'
    return normalized_version;
}

function filter_version(asset_node) {
    let name = asset_node.querySelector('.asset-card__title').innerHTML;

    let tag = asset_node.querySelector('.asset-card__tags .asset-card__bevy-versions .asset-card__tag');
    if (filters_state.version === 'all_versions') {
        return true
    }
    else if (tag) {
        let raw_item_value = tag.innerText;
        let normalized_version = normalize_version(raw_item_value);
        return [filters_state.version, ...version_always_show].includes(normalized_version);
    } else return true
}

let versionsSelect = document.querySelector('#assets-filter');
if (versionsSelect) {
    let versionsQuery = document.querySelectorAll('.asset-card .asset-card__bevy-versions .asset-card__tag');
    [...new Set([...versionsQuery]
        .map((item) => {
            let raw_version = item?.innerText;
            let normalized_version = normalize_version(raw_version);
            return normalized_version
        })
        .filter(i => i)
        .filter(i => !version_always_show.includes(i))
        .sort(sort_versions)
    )].forEach(i => {
        var opt = document.createElement('option');
        opt.value = i;
        opt.innerHTML = i;
        versionsSelect.appendChild(opt);
    })

}

document
    .querySelector('#assets-filter')
    .addEventListener("change", (item) => {
        filters_state.version = item.target.value;
        filter_assets();
    })
