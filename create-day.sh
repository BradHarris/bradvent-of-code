#!/bin/bash

cp ./src/day_template.rs ./src/year_$1/day_$2.rs
echo "pub mod day_$2;" >> ./src/year_$1.rs