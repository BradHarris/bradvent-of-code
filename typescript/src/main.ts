import { years } from "./years.ts";
// get year and day from args
const [targetYear, targetDay] = Deno.args;

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  for (const [year, days] of Object.entries(years)) {
    if (targetYear != null && year !== targetYear) continue;

    const report: Record<string, Record<string, string>> = {};
    let totalTime = 0;
    for (let i = 0; i < days.length; i++) {
      if (targetDay != null && i !== Number(targetDay) - 1) continue;
      const day = days[i];
      const now1 = performance.now();
      const part1 = await day.part1(day.INPUT);
      const part1Time = performance.now() - now1;
      totalTime += part1Time;
      const now2 = performance.now();
      const part2 = await day.part2(day.INPUT);
      const part2Time = performance.now() - now2;
      totalTime += part2Time;
      report[`day ${i + 1}`] = {
        [`part 1`]: part1,
        [`part 1 time`]: `${part1Time.toFixed(2)} ms`,
        [`part 2`]: part2,
        [`part 2 time`]: `${part2Time.toFixed(2)} ms`,
      };
    }
    console.log(`\n${year}`);
    console.table(report);
    console.log(`Total time: ${totalTime.toFixed(2)} ms\n`);
  }
}
