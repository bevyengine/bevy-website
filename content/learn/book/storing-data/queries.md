+++
title = "Queries"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

Queries are your primary tool for interacting with the Bevy world,
allowing you to carefully and efficiently read and write component data from matching entities.

ANATOMY OF TRAITS

COMBINE AS TUPLES

LOOK AT DOCS FOR IMPLEMENTORS

## Query::get

ENTITY QUERY DATA

QUERY GET

## Working with singleton entities

QUERY::SINGLE

CONTRAST TO SINGLE SYSTEM PARAM

## Accessing multiple items from the same query

QUERY::GET_MANY

## Multiple queries in a single system

MUTABILITY RULES.
AVOID WITH WITHOUT
PARAMSET

## Disabling entities

DEFAULT QUERY FILTERS

DISABLED COMPONENT

## Working with complex queries

DERIVE QUERYDATA / QUERYFILTER

USE SYSTEMPARAM DERIVE INSTEAD

CONTRAST TO TYPE ALIASES

## Advanced query tools

- change detection
- Option
- Has and AnyOf
- Or
- EntityMut, EntityRef, FilteredEntityMut, FilteredEntityRef, EntityMutExcept, EntityRefExcept
