import { darkTheme } from "./dark-theme";
import { lightTheme } from "./light-theme";

const themes = new Map([
  ["light", lightTheme],
  ["dark", darkTheme],
]);

const themeNames = ["dark", "light"];

export { themes, themeNames };
