import { days as days2024 } from "./year_2024/index.ts";
import { days as days2015 } from "./year_2015/index.ts";
import { Day } from "./Day.ts";

export const years: Record<string, Day[]> = {
  2015: days2015,
  2024: days2024,
};
