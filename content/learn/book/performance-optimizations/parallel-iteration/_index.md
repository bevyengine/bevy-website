+++
title = "Parallel Iteration"
weight = 2
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
[extra]
status = 'hidden'
+++

{% todo() %}

* Remind users that systems run in parallel automatically, advise to make systems smaller
* Explain that `.for_each` is faster, and link to an explainer on why
* Demonstrate how to use `.par_for_each`
* Discuss costs of branching in a tight loop
* Discuss how to tune the hyperparameters for parallel iteration
{% end %}
