const searchElement = document.querySelector('#assets-search')

searchElement.addEventListener("input", (_) => {
    filterSearchTerms()
    hideEmptySubSections()
    hideEmptySections()
})

function filterSearchTerms() {
    const searchTerms = searchElement.value.toLowerCase().split(' ')
    for (const asset of document.querySelectorAll('.asset-card')) {
        const fullText = asset.text.toLowerCase()
        const searchMatch = searchTerms.every((term) => fullText.includes(term))
        asset.parentElement.style.display = searchMatch ? 'block' : 'none'
    }
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
        while (nextElement && !nextElement.classList.contains('asset-section')) {
            if (nextElement.style.display !== 'none') {
                section.style.display = 'block'
                return
            }
            nextElement = nextElement.nextElementSibling
        }
        section.style.display = 'none'
    })
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
        let selected_value = item.target.value;
        for (const asset of document.querySelectorAll('.asset-card')) {
            // let name = asset.querySelector('.asset-card__title').innerHTML;

            let tag = asset.querySelector('.asset-card__tags .asset-card__bevy-versions .asset-card__tag');
            if (selected_value === 'all_versions') {
                asset.parentElement.style.display = 'block'
            }
            else if (tag) {
                let raw_item_value = tag.innerText;
                let normalized_version = normalize_version(raw_item_value);
                // console.debug("<<<<", { name, normalized_version, selected_value }, [selected_value, ...version_always_show]);
                const searchMatch = [selected_value, ...version_always_show].includes(normalized_version);
                asset.parentElement.style.display = searchMatch ? 'block' : 'none'
            }
        }
    })
