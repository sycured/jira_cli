#!/usr/bin/env bash
set -e
export JIRA_CLI="./target/debug/jira_cli"
export JIRA_PROJECT_KEY="CITS"
export JIRA_VERSION_NAME="v0.0.0"

# version
$JIRA_CLI -V

# help
$JIRA_CLI help

# check_version
$JIRA_CLI check_version

# generate
$JIRA_CLI generate bash > /tmp/jira_cli.bash
$JIRA_CLI generate elvish > /tmp/jira_cli.elvish
$JIRA_CLI generate fish > /tmp/jira_cli.fish
$JIRA_CLI generate powershell > /tmp/jira_cli.ps1
$JIRA_CLI generate zsh > /tmp/jira_cli.zsh

# user
## get_id
JIRA_USER_ACCOUNT_ID=$($JIRA_CLI user get_account_id "$JIRA_USER")
export JIRA_USER_ACCOUNT_ID
echo "$JIRA_USER_ACCOUNT_ID"

# project
## create
echo "project create"
$JIRA_CLI project create citest CITS "$JIRA_USER_ACCOUNT_ID"
## get_id
echo "project get_id"
$JIRA_CLI project get_id $JIRA_PROJECT_KEY
JIRA_PROJECT_ID=$($JIRA_CLI project get_id $JIRA_PROJECT_KEY)
export JIRA_PROJECT_ID
## list_link_types
echo "list_link_types"
$JIRA_CLI issue list_link_types
## create_link_type
echo "create_link_type"
JIRA_LINK_TYPE_ID=$($JIRA_CLI i clt test test test | awk '{print $6}')
export JIRA_LINK_TYPE_ID
## update_link_type
echo "update_link_type"
$JIRA_CLI i ult "$JIRA_LINK_TYPE_ID" test "is tested by" "tests"
## get_link_type
echo "get_link_type"
$JIRA_CLI i glt "$JIRA_LINK_TYPE_ID"
## delete_link_type
echo "delete_link_type"
$JIRA_CLI i dlt "$JIRA_LINK_TYPE_ID"
## list_features
echo "project list_features"
$JIRA_CLI project list_features $JIRA_PROJECT_KEY
## set_feature_state
echo "project set_feature_state"
$JIRA_CLI project set_feature_state $JIRA_PROJECT_KEY jsw.agility.releases ENABLED
## new_version
echo "project new_version"
$JIRA_CLI project new_version
$JIRA_CLI project new_version test
## list_versions
echo "project list_versions"
$JIRA_CLI project list_versions

# issue
## list_types
echo "issue list_types"
$JIRA_CLI issue list_types $JIRA_PROJECT_KEY
## create
echo "issue create"
$JIRA_CLI issue create Task "little test" "little test using jira_cli" "$JIRA_USER_ACCOUNT_ID" $JIRA_PROJECT_KEY
$JIRA_CLI issue create Task "little test2" "little test using jira_cli" "$JIRA_USER_ACCOUNT_ID" $JIRA_PROJECT_KEY
$JIRA_CLI issue create Task "little test3" "little test using jira_cli" "$JIRA_USER_ACCOUNT_ID" $JIRA_PROJECT_KEY
$JIRA_CLI issue create Task "little test4" "little test using jira_cli" "$JIRA_USER_ACCOUNT_ID" $JIRA_PROJECT_KEY
$JIRA_CLI issue create Task "little test5" "little test using jira_cli" "$JIRA_USER_ACCOUNT_ID" $JIRA_PROJECT_KEY
## add_version
echo "issue add_version"
$JIRA_CLI issue add_version "$JIRA_PROJECT_KEY"-1 $JIRA_VERSION_NAME
$JIRA_CLI issue add_version "$JIRA_PROJECT_KEY"-1 test
JIRA_VERSION_NAME=test $JIRA_CLI issue add_version "$JIRA_PROJECT_KEY"-2,"$JIRA_PROJECT_KEY"-3
$JIRA_CLI i av "$JIRA_PROJECT_KEY"-4,"$JIRA_PROJECT_KEY"-5
## add_vote
echo "issue add_vote"
$JIRA_CLI issue add_vote "$JIRA_PROJECT_KEY"-1
$JIRA_CLI issue add_vote "$JIRA_PROJECT_KEY"-3
## add_label
echo "issue add_label"
$JIRA_CLI issue add_label "$JIRA_PROJECT_KEY"-1 "CI"
$JIRA_CLI issue add_label "$JIRA_PROJECT_KEY"-1 "CI2"
$JIRA_CLI issue al "$JIRA_PROJECT_KEY"-2,"$JIRA_PROJECT_KEY"-3 "CI3"
$JIRA_CLI issue al "$JIRA_PROJECT_KEY"-4,"$JIRA_PROJECT_KEY"-5 "CI4"
## remove_version
echo "issue remove_version"
$JIRA_CLI issue remove_version "$JIRA_PROJECT_KEY"-1 test
## remove_vote
echo "issue remove_vote"
$JIRA_CLI issue remove_vote "$JIRA_PROJECT_KEY"-1
## list_votes
echo "issue list_votes"
$JIRA_CLI issue list_votes "$JIRA_PROJECT_KEY"-3
## remove_label
echo "issue remove_label"
$JIRA_CLI issue remove_label "$JIRA_PROJECT_KEY"-1 "CI2"
$JIRA_CLI i rl "$JIRA_PROJECT_KEY"-2,"$JIRA_PROJECT_KEY"-3 "CI3"
## show_fixversions
echo "issue show_fixversions"
$JIRA_CLI issue show_fixversions "$JIRA_PROJECT_KEY"-1
## delete an issue
echo "issue delete"
$JIRA_CLI issue delete "$JIRA_PROJECT_KEY"-5

# labels
echo "labels"
$JIRA_CLI labels

# project - delete_project
echo "delete_project"
./test_project_destroy.exp