#!/bin/bash

rm doc*

for i in {1..5}
do
    touch doc$i.txt
done

wordBank=('alpha' 'bravo' 'charlie' 'delta' 'echo' 'foxtrot' 'golf' 'hotel' 'india' 'juliet' 'kilo' 'lima' 'mike' 'november' 'oscar' 'papa' 'quebec' 'romeo'
'sierra' 'tango' 'uniform' 'victor' 'whiskey' 'xray' 'yankee' 'zulu')

for doc in *
do
    numOfWords=$(((RANDOM % 5) + 10))

    words=""
    for j in $(seq 0 $numOfWords)
    do
        index=$((RANDOM % 26))
        word=${wordBank[$index]}
        words+= $word
    done
    echo "$words" >> "$doc"
    
done

