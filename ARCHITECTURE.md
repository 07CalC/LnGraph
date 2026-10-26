uses unix filesystem symbolik links to establish relationship between nodes

## folder structure

```
lnGraph/
  - Person/
    - PersonA/
      - properties.json  ## contains properties of PersonA
      - Follows/
        - PersonB/
          - target/          ## contains symbolic link to PersonB
          - properties.json  ## contains properties of the edge Follows
    - PersonB/
      - properties.json  ## contains properties of PersonB
      - Follows/
        - PersonC/
          - target/          ## contains symbolic link to PersonC
          - properties.json  ## contains properties of the edge Follows
    - PersonC/
      - properties.json  ## contains properties of PersonC
      - Follows/
        - PersonA/
          - target/          ## contains symbolic link to PersonA
          - properties.json  ## contains properties of the edge Follows

```
