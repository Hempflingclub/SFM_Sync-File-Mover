#!/bin/bash

# Exit if No File is Specified
if [[ "$#" == '0' ]]; then
  echo -e 'ERROR: No File Specified!' && exit 1
fi

# File to Upload
FILE="@$1"

# Find the Best server to upload
SERVER=$(curl -s https://apiv2.gofile.io/getServer | jq -r '.data|.server')

UPLOAD=$(curl -F file=${FILE} https://${SERVER}.gofile.io/uploadFile)

LINK=$(echo $UPLOAD | jq -r '.data|.downloadPage')

# Print the link!
echo $LINK
# Writing Link into dl_link
echo $LINK > dl_link
echo " "
echo $LINK >> $GITHUB_STEP_SUMMARY
