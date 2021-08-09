import { setTheme, ISetTheme } from "./action-creators";

const actionSetTheme = (themeName: string): ISetTheme => setTheme(themeName);

export { actionSetTheme };
