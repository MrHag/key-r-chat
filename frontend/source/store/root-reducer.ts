import { combineReducers } from "redux";
import { IAuthStore, reducer as authReducer } from "./auth/reducer";

export interface IAppStore {
  auth: IAuthStore;
}

export const rootReducer = combineReducers({
  auth: authReducer,
});
