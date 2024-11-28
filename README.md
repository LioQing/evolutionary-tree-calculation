# Evolutionary Tree Calculation

[![GitHub Pages](https://github.com/LioQing/evolutionary-tree-calculation/actions/workflows/github-pages.yml/badge.svg)](https://github.com/LioQing/evolutionary-tree-calculation/actions/workflows/github-pages.yml)

An evolutionary distinctiveness calculator for my friend doing biology research.

## Usage

I don't have much information on why and what, but here is a how - simply upload your JSON file and you will get the list of names and values sorted ascendingly.

Here is a template JSON file:
```json
{
  "root": {
    "children": [
      {
        "name": "Node Name 1",
        "length": 12
      },
      {
        "children": [
          {
            "name": "Node Name 2",
            "length": 34
          },
          {
            "name": "Node Name 3",
            "length": 56
          }
        ],
        "length": 78
      }
    ]
  }
}
```

**Note**: any node without a `length` property will automatically be given a value of `0`.
