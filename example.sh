#!/bin/bash
URL="http://localhost:8080/users/"
HTTPCODE=`curl -s -o /dev/null -s -w "%{http_code}\n" $URL`
if [ $HTTPCODE = "401" ]; then
    echo "Access denied to $URL"
fi

echo "With Token for: $URL"
curl -s -H "Authorization: Bearer Kenneth" $URL | jq
