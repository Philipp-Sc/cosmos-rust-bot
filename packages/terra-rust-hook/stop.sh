#!/bin/bash

if [ -f ./my-hook.pid ]; then
    cat ./my-hook.pid
    kill `cat ./my-hook.pid`
    rm ./my-hook.pid
    echo "process stopped"
else 
    echo "process not running"
fi

