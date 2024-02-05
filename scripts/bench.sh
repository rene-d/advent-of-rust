#!/bin/sh

d=$(dirname $0)

if [ -f /proc/cpuinfo ] ; then
    cat /proc/cpuinfo > cpuinfo.txt
fi

$d/runall.py -x 2019:25 -u me --no-slow --no-64 | tee results-fast-32-me
$d/runall.py -x 2019:25 -u me --no-slow         | tee results-fast-me
$d/runall.py -x 2019:25 -u me                   | tee results-me
$d/runall.py -x 2019:25       --no-slow --no-64 | tee results-fast-32
$d/runall.py -x 2019:25       --no-slow         | tee results-fast
$d/runall.py -x 2019:25                         | tee results

if [ -f $d/../run.log ] ; then
    cp $d/../run.log .
fi
