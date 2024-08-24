---
title: 'Search'
api: 'POST /api/search'
description: 'Perform a search query to retrieve relevant results'
---

# Search

Perform a search query to retrieve relevant results based on the provided parameters.

## Request Body

<ParamField body="query" type="string" required>
  The search query string
</ParamField>

<ParamField body="search_type" type="string" required>
  The type of search to perform (e.g., "hybrid")
</ParamField>

<ParamField body="dataset_id" type="string" required>
  The ID of the dataset to search within
</ParamField>

<ParamField body="slim_chunks" type="boolean">
  Whether to return slim chunks or full chunks
</ParamField>

<ParamField body="highlight_results" type="boolean" required>
  Whether to highlight matching terms in the results
</ParamField>

<ParamField body="recency_bias" type="number">
  The bias towards more recent results (0-10)
</ParamField>

<ParamField body="filters" type="object">
  Filters to apply to the search results
  
  <Expandable title="Filter object">
    <ParamField body="must" type="array">
      Conditions that must be met
    </ParamField>
    <ParamField body="must_not" type="array">
      Conditions that must not be met
    </ParamField>
    <ParamField body="should" type="array">
      Conditions that should be met
    </ParamField>
  </Expandable>
</ParamField>

<ParamField body="get_total_pages" type="boolean">
  Whether to return the total number of pages (default: true)
</ParamField>

<ParamField body="group_size" type="integer">
  The number of chunks to group together (default: 3)
</ParamField>

<ParamField body="highlight_delimiters" type="array of string">
  Delimiters to use for highlighting (default: ["?", ",", ".", "!"])
</ParamField>

<ParamField body="highlight_threshold" type="number">
  The threshold for highlighting (default: 0.8)
</ParamField>

<ParamField body="highlight_max_length" type="integer">
  The maximum length of highlighted text (default: 100)
</ParamField>

<ParamField body="highlight_max_num" type="integer">
  The maximum number of highlights (default: 5)
</ParamField>

<ParamField body="page" type="integer">
  The page number of results to retrieve (default: 1)
</ParamField>

<ParamField body="page_size" type="integer">
  The number of results per page (default: 10)
</ParamField>

<ParamField body="score_threshold" type="number">
  The minimum score threshold for results (default: 0.5)
</ParamField>

<ParamField body="get_collisions" type="boolean">
  Whether to return collision information (default: false)
</ParamField>

## Response

<ResponseField name="200: OK" type="object">
  Successful search response

  <Expandable title="Properties">
    <ResponseField name="title" type="string" required>
      The title of the search result
    </ResponseField>

    <ResponseField name="abstract_chunk" type="object">
      The abstract chunk of the search result
      <Expandable title="AbstractChunk properties">
        <ResponseField name="content" type="string">
          The content of the abstract chunk
        </ResponseField>
        <ResponseField name="score" type="number">
          The relevance score of the abstract chunk
        </ResponseField>
      </Expandable>
    </ResponseField>

    <ResponseField name="doi" type="string">
      The DOI (Digital Object Identifier) of the result
    </ResponseField>

    <ResponseField name="id" type="string">
      The unique identifier of the search result
    </ResponseField>

    <ResponseField name="metadata" type="object">
      Additional metadata for the search result
    </ResponseField>
  </Expandable>
</ResponseField>

<ResponseField name="400: Bad Request" type="object">
  Invalid request parameters
</ResponseField>

## Example Requestpython
import requests
import json
url = "http://0.0.0.0:8000/api/search"
payload = {
"query": "chemical ligands and their interactions",
"search_type": "hybrid",
"dataset_id": "your_dataset_id",
"highlight_results": True,
"recency_bias": 3,
"filters": {
"must": [{"field": "type", "value": "journal-article"}],
"must_not": [],
"should": []
},
"page_size": 10,
"page": 1
}
headers = {
"Content-Type": "application/json"
}
response = requests.post(url, headers=headers, data=json.dumps(payload))
print(f"Status: {response.status_code}")
print(f"Response: {response.json()}")

## Example Response
json
{
"title": "Chemical Ligands and Their Interactions: A Comprehensive Review",
"abstract_chunk": {
"content": "This review explores the various types of chemical ligands and their interactions with target molecules. We discuss the importance of ligand binding in drug discovery and molecular recognition processes.",
"score": 0.89
},
"doi": "10.1234/example.5678",
"id": "abc123def456",
"metadata": {
"authors": ["John Doe", "Jane Smith"],
"publication_date": "2023-04-15",
"journal": "Journal of Chemical Interactions"
}
}
