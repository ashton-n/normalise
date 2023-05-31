#!/bin/bash

max_proc=$1
times=()

for i in $(seq 1 $max_proc) 
do 
    output=$(mpirun -np $i target/release/normalise ~/Desktop/data/coms4040/ass4-test-data/ real_test)
    time=$(echo "$output" | grep -oE '[0-9]+\.[0-9]+')
    times+=("$time")
done

speed_up=()
for ((i = 0; i < max_proc; i++)) 
do
    result=$(echo "scale=9; ${times[0]} / ${times[$i]}" | bc)
    speed_up+=("$result")
done

for ((i = 0; i < max_proc; i++))
do
    echo "Execution time: ${times[$i]}ms speed up: ${speed_up[$i]}"
done