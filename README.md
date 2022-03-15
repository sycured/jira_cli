# jira_cli

CLI tool using Jira REST API

## Top-level arguments

Required for all subcommands

| CLI short flag | CLI long flag | environment variable name |
|:--------------:|:-------------:|:-------------------------:|
|      `-d`      |  `--domain`   |       `JIRA_DOMAIN`       |
|      `-t`      |   `--token`   |       `JIRA_TOKEN`        |
|      `-u`      |   `--user`    |        `JIRA_USER`        |

## Subcommands

### add_version

|    argument    | environment variable name |
|:--------------:|:-------------------------:|
|  `issue_key`   |                           |
| `version_name` |    `JIRA_VERSION_NAME`    |

`JIRA_VERSION_NAME` exists because when I'm adding the same tag to a different issue, I can loop more quickly using this
syntax:

```shell
while read line; do jira_cli add_version $line; done < /tmp/issues_list
```

Where /tmp/issues_list is like:

```text
SYY-1025
SYY-1026
SYY-1030
SYY-1035
```

### create_project

|           argument           |                       default value                       | specific list of values |
|:----------------------------:|:---------------------------------------------------------:|:-----------------------:|
|        `project_name`        |                                                           |                         |
|        `project_key`         |                                                           |                         |
| `jira_project_leadaccountid` |                                                           |                         |
|        `project_type`        |                         software                          |            x            |
|      `project_template`      | `com.pyxis.greenhopper.jira:gh-simplified-agility-kanban` |            x            |

To see the list of possible values:

```shell
jira_cli help create_project
```

or

```shell
jira_cli create_project -h
```

### create_version

|    argument    | environment variable name |
|:--------------:|:-------------------------:|
| `version_name` |                           |
|  `project_id`  |     `JIRA_PROJECT_ID`     |

When creating multiple versions for the same project, it's quicker to do it like:

```shell
jira_cli create_version v1.0
jira_cli create_version v1.1
jira_cli create_version v1.2
```

### delete_project

|   argument    |
|:-------------:|
| `project_key` |

This subcommand has a confirmation prompt to be sure that you want to delete the project.

### get_account_id

|    argument     |
|:---------------:|
| `email_address` |

### get_project_id

|   argument    |
|:-------------:|
| `project_key` |

### set_project_feature_state

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
jira_cli set_project_feature_state SYY jsw.agility.releases ENABLED
```