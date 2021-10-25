# Commands

* `board`
  * `get-all` :white_check_mark:
  * `create-new` `{name}` :white_check_mark:
  * `select` `{name}` :white_check_mark:
* `label`
  * `get-all` :white_check_mark:
  * `create-new` `{name} {color}` :white_check_mark:
  * `delete` `{name}` :white_check_mark:
  * `update {name} {color}`​ :white_check_mark:
* `list`
  * `get-all` :white_check_mark:
  * `create-new` `{name}` :white_check_mark:
  * `delete` `{list-name}`
  * `select` `{name}` :white_check_mark:
    * `card-count`
    * `due-dates`
* `card`
  * `get-all` :white_check_mark:
  * `create` `{name}` :white_check_mark:
  * `select` `{name}` :white_check_mark:
    * `get-description` :white_check_mark:
    * `edit-description` `{description}` :white_check_mark:
    * `move-to-list` `{destination-list-name}` :white_check_mark:
    * `get-checklists`
    * `create-checklist` `{name}`
    * `select-checklist` `{name}`
      * `get-tasks`
      * `add-task`
      * `complete-task` `{id}`
    * `get-comments`
    * `add-comment` `{comment}`
    * `get-labels` :white_check_mark:
    * `add-label` `{name}` :white_check_mark:
    * `remove-label` `{name}` :white_check_mark:
    * `get-due-date` :white_check_mark:
    * `set-due-date` `{due-date}` :white_check_mark:
