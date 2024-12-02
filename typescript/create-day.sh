#!/bin/bash

mkdir -p ./src/year_$1
cp ./src/template.ts ./src/year_$1/day_$2.ts
echo "import * as day_$2 from './day_$2.ts';" >> ./src/index.ts
