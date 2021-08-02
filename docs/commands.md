# Commands

* `board`
  * `get-all`
  * `create-new` `{name}` `{workspace}?`
  * `board-id` `{id}`
    * `label`
      * `get-all`
      * `create-new` `{name}`
      * `delete` `{label-id}`
    * `list`
      * `get-all`
      * `create-new` `{name}`
      * `delete` `{list-id}`
      * `list-id` `{id}`
        * `card-count`
        * `get-all`
        * `due-dates`
        * `create-new` `{name}`
        * `move-to-list` `{destination-list-name}`
        * `card-id` `{id}`
          * `get-description`
          * `edit-description` `{description}`
          * `get-checklists`
          * `create-checklist` `{name}`
          * `checklist-id` `{id}`
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