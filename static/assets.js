const searchElement = document.querySelector('#assets-search')

searchElement.addEventListener("input", (_) => {
    // TODO add timer
    // TODO rename methods
    filterSearchTerms()
    hideEmptySubSections()
    hideEmptySections()
    updateSuggestionLinks()
    // TODO apps section does not disappear beause added one last element that isn't empty
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

const suggestionsUrls = {
    '#suggestion-github': 'https://github.com/bevyengine/bevy/discussions?discussions_q=',
    '#suggestion-cheatbook': 'https://bevy-cheatbook.github.io/?search=',
    '#suggestion-docs': 'https://dev-docs.bevyengine.org/bevy/index.html?search=',
}

function updateSuggestionLinks() {
    const searchValue = searchElement.value.toLowerCase();
    document.querySelector('#suggestions-footer').style.display = searchValue === "" ? 'none' : 'block'
    for (const [linkId, uriBase] of Object.entries(suggestionsUrls)) {
        document.querySelector(linkId).href=uriBase + encodeURIComponent(searchValue)
    }
}