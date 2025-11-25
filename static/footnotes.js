// The existing Zola footnote behavior is broken due to an issue with their dependency on pulldown-cmark, which has had an open issue for footnote handling since 2018
// This util is based on advice provided by the Zola community here: https://github.com/getzola/zola/issues/1070

// The DOMContentLoaded event fires when the initial HTML
// document has been completely loaded and parsed, without
// waiting for stylesheets, images, and subframes to finish loading.
document.addEventListener('DOMContentLoaded', (_event) => {
  const references = document.getElementsByClassName('footnote-reference')
  // For each footnote reference, set an id so we can refer to it from the definition.
  // If the definition had an id of 'some_id', then the reference has id `some_id_ref`.
  for (const reference of references) {
    const link = reference.firstChild
    const id = link.getAttribute('href').slice(1) // skip the '#'
    link.setAttribute('id', `${id}_ref`)
  }

  const footnotes = document.getElementsByClassName('footnote-definition')
  // For each footnote-definition, add an anchor element with an href to its corresponding reference.
  for (const footnote of footnotes) {
    // Remove original poorly placed and unlinked footnote id label
    const footnoteLabel = footnote.getElementsByTagName('sup')[0]
    footnoteLabel.remove()

    const id = footnote.getAttribute('id')
    const backReference = document.createElement('a')
    const paragraph = footnote.getElementsByTagName('p')[0]
    const superscript = document.createElement('sup')
    // Add back class to superscript tag that was deleted previously
    superscript.classList.add('footnote-definition-label')

    backReference.setAttribute('href', `#${id}_ref`)
    backReference.textContent = `${id} `

    superscript.append(backReference)
    paragraph.prepend(superscript)
  }
});
