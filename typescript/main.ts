import { day01 } from "./year_2024/index.ts";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  console.log(day01.part1(day01.EXAMPLE_INPUT));
}
