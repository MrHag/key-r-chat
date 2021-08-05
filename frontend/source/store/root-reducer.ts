import { combineReducers } from "redux";
import { IAuthStore, reducer as authReducer } from "./auth/reducer";
import { ISettingsStore, reducer as settingsReducer } from "./settings/reducer";

export interface IAppStore {
  auth: IAuthStore;
  settings: ISettingsStore;
}

export const rootReducer = combineReducers({
  auth: authReducer,
  settings: settingsReducer,
});
