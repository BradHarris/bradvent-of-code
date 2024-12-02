import { days } from "./year_2024/index.ts";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
    const report: Record<string, Record<string, string>> = {};
    for (let i = 0; i < days.length; i++) {
        const day = days[i];
        report[`day ${i + 1}`] = {
            [`part 1 example`]: day.part1(day.EXAMPLE_INPUT),
            [`part 2 example`]: day.part2(day.EXAMPLE_INPUT),
            [`part 1`]: day.part1(day.INPUT),
            [`part 2`]: day.part2(day.INPUT),
        };
    }
    console.table(report);
}
