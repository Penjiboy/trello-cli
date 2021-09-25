# Commands

* `board`
  * `get-all`
  * `create-new` `{name}` `{workspace}?`
  * `select` `{name}`
    * `label`
      * `get-all`
      * `create-new` `{name}`
      * `delete` `{label-id}`
    * `list`
      * `get-all`
      * `create-new` `{name}`
      * `delete` `{list-id}`
      * `select` `{name}`
        * `card-count`
        * `get-all`
        * `due-dates`
        * `create-new` `{name}`
        * `move-to-list` `{destination-list-name}`
        * `select-card` `{name}`
          * `get-description`
          * `edit-description` `{description}`
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
