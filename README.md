# jira_cli

A little CLI for Jira using the REST API.

## Best Practices

Create a `.env` file in your parent project directory with the following variables:
- JIRA_ADMIN: my_jira.atlassian.net
- JIRA_USER: my_email_address
- JIRA_USER_ACCOUNT_ID: output from `jira_cli user get_account_id my_email_address`
- JIRA_TOKEN: my_api_token

If you work all time on the same project, you can also set:
- JIRA_PROJECT_KEY: my_project_key
- JIRA_PROJECT_ID: output from `jira_cli project get_id my_project_key`

After, you can use a tool like [dotenv](https://github.com/motdotla/dotenv) or a zsh plugin like [ohmyzsh plugin dotenv](https://github.com/ohmyzsh/ohmyzsh/blob/master/plugins/dotenv/dotenv.plugin.zsh) to load this file automatically.

For example, on my setup, it's like: `/Volumes/CS/companyname/.env`
Every git clone are in this folder `/Volumes/CS/companyname/` so no issue about putting the .env file in git (thing that you need to avoid).

## Top-level arguments

Required for all subcommands

| CLI short flag | CLI long flag | environment variable name |
|:--------------:|:-------------:|:-------------------------:|
|      `-d`      |  `--domain`   |       `JIRA_DOMAIN`       |
|      `-t`      |   `--token`   |       `JIRA_TOKEN`        |
|      `-u`      |   `--user`    |        `JIRA_USER`        |

## Subcommands

### generate

Generate autocompletion file for your shell

| argument | specific list of values |
|:--------:|:-----------------------:|
| `shell`  |            x            |

Possible values for `shell` (case sensitive):

- bash
- elvish
- fish
- powershell
- zsh

For example to generate zsh autocompletion and save it to a file: `jira_cli generate zsh > ~/.zsh/completion/_jira_cli`

### issue

#### add_label

Add label to an issue

|  argument   | environment variable name |
|:-----------:|:-------------------------:|
| `issue_key` |                           |
|   `label`   |       `JIRA_LABEL`        |

`JIRA_LABEL` exists because when I'm adding the same label to different issues, I can loop more quickly using this

```shell
export JIRA_LABEL=Security
while read line; do jira_cli issue add_label $line; done < /tmp/issues.txt
```

Where /tmp/issues.list is like:

```text
SYY-1025
SYY-1026
SYY-1030
SYY-1035
```

#### add_version

Add version to an issue

|    argument    | environment variable name |
|:--------------:|:-------------------------:|
|  `issue_key`   |                           |
| `version_name` |    `JIRA_VERSION_NAME`    |

`JIRA_VERSION_NAME` exists because when I'm adding the same tag to different issues, I can loop more quickly using this
syntax:

```shell
export JIRA_VERSION_NAME=v1.0.0
while read line; do jira_cli issue add_version $line; done < /tmp/issues_list
```

Where /tmp/issues.list is like:

```text
SYY-1025
SYY-1026
SYY-1030
SYY-1035
```

#### create

Create an issue

|       argument        | environment variable name | required |
|:---------------------:|:-------------------------:|:--------:|
|     `issue_type`      |                           |    x     |
|    `issue_summary`    |                           |    x     |
|  `issue_description`  |                           |    x     |
| `reporter_account_id` |                           |    x     |
|   `issue_priority`    |                           |          |
|     `project_key`     |    `JIRA_PROJECT_KEY`     |    x     |

To have the list of `issue_priority`, you have to use: `jira_cli isssue list_priorities`

Default value for `issue_priority` is the one defined in Jira.

Usage:
```shell
export JIRA_PROJECT_KEY=SYY
jira_cli issue create_issue Task "summary" "description" account_id
jira_cli issue create_issue Task "summary" "description" account_id Lowest
```

#### list_priorities

List issue priorities

Usage: `jira_cli issue list_priorities`

#### list_types

List issue types for this project

|   argument    |
|:-------------:|
| `project_key` |

Usage: `jira_cli issue list_types SYY`

#### show_fixversions

List all fix versions for a specific issue

|  argument   |
|:-----------:|
| `issue_key` |

Usage: `jira_cli issue show_fixversions SYY-1025`

### labels

List available labels for the global label field"

Usage: `jira_cli labels`

### project

#### create

Create project

|           argument           |                       default value                       | specific list of values |
|:----------------------------:|:---------------------------------------------------------:|:-----------------------:|
|        `project_name`        |                                                           |                         |
|        `project_key`         |                                                           |                         |
| `jira_project_leadaccountid` |                                                           |                         |
|        `project_type`        |                         software                          |            x            |
|      `project_template`      | `com.pyxis.greenhopper.jira:gh-simplified-agility-kanban` |            x            |

To see the list of possible values: `jira_cli project create_project -h`

Usage: `jira_cli project create "SYCURED_TEST" "SYY" lead_account_id`

#### ⚠️ delete_project

Aliases: destroy, destroy_project

Delete a project

|   argument    |
|:-------------:|
| `project_key` |

This subcommand has a confirmation prompt to be sure that you want to delete the project.

Usage:
```shell
jira_cli project delete_project SYY
jira_cli project destroy_project SYY
jira_cli project destroy SYY
````

#### get_id

Get project id

|   argument    |
|:-------------:|
| `project_key` |

Usage: `jira_cli project get_id SYY`

#### list_features

List project features

|   argument    |
|:-------------:|
| `project_key` |

Usage: `jira_cli project list_features SYY`

#### new_version

Create version for a project

|    argument    | environment variable name |
|:--------------:|:-------------------------:|
| `version_name` |    `JIRA_VERSION_NAME`    |
|  `project_id`  |     `JIRA_PROJECT_ID`     |

When creating multiple versions for the same project, it's quicker to do it like:

```shell
jira_cli project new_version v1.0
jira_cli project new_version v1.1
jira_cli project new_version v1.2
```

#### set_feature_state

Set project feature state

|    argument     | specific list of values |
|:---------------:|:-----------------------:|
|  `project_key`  |                         |
|  `feature_key`  |                         |
| `feature_state` |            x            |

Possible values for `feature_state` (case sensitive):

- ENABLED
- DISABLED
- COMING_SOON

For example, to enable Releases on SYY:

```shell
jira_cli project set_feature_state SYY jsw.agility.releases ENABLED
```

### user

#### get_account_id

Get account id from email_address

|    argument     |
|:---------------:|
| `email_address` |

Usage: `jira_cli user get_account_id xxxx@xxx.xx`