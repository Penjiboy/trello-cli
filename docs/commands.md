# Commands

* `board`
  * `get-all`
  * `create-new` `{name}` `{workspace}?`
  * `select` `{name}`
    * `label`
      * `get-all`
      * `create-new` `{name} {color}`
      * `delete` `{name}`
      * `update {name} {color}`
    * `list`
      * `get-all`
      * `create-new` `{name}`
      * `delete` `{list-name}`
      * `select` `{name}`
        * `card-count`
        * `get-cards`
        * `due-dates`
        * `create-card` `{name}`
        * `select-card` `{name}`
          * `get-description`
          * `edit-description` `{description}`
          * `move-to-list` `{destination-list-name}`
          * `get-checklists`
          * `create-checklist` `{name}`
          * `select-checklist` `{name}`
            * `get-tasks`
            * `add-task`
            * `complete-task` `{id}`
          * `get-comments`
          * `add-comment` `{comment}`
          * `get-labels`
          * `add-label` `{name}`
          * `remove-label` `{name}`
          * `get-due-date`
          * `set-due-date` `{due-date}`
