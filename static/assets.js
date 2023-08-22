const search = document.querySelector('#assets-search')

search.addEventListener("input", (_) => {
    const searchTerms = search.value.toLowerCase().split(' ')
    for (const asset of document.querySelectorAll('.asset-card')) {
        const fullText = asset.text.toLowerCase()
        const searchMatch = searchTerms.every((term) => fullText.includes(term))
        asset.style.display = searchMatch ? 'block' : 'none'
    }
})