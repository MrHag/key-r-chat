import { ActionType, SET_THEME } from "./action-creators";

interface ISettingsStore {
  themeName: string;
}

const defaultStore: ISettingsStore = {
  themeName: "light",
};

const reducer = (state = defaultStore, action: ActionType): ISettingsStore => {
  switch (action.type) {
    case SET_THEME:
      return {
        ...state,
        themeName: action.payload.themeName,
      };
    default:
      return state;
  }
};

export { ISettingsStore, reducer };
