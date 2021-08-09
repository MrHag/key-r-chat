export const SET_THEME = "settings/SET_THEME";

export interface ISetTheme {
  type: string;
  payload: {
    themeName: string;
  };
}

const setTheme = (themeName: string): ISetTheme => ({
  type: SET_THEME,
  payload: {
    themeName,
  },
});

type ActionType = ISetTheme;

export { setTheme, ActionType };
