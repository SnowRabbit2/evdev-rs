#! /bin/bash

dir=$(dirname $(readlink -f $0))
$dir/event-codes.rs.py $dir/*.h