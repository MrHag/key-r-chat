import { combineReducers } from "redux";
import { IUserStore, reducer as userReducer } from "./user/reducer";

export interface IAppStore {
  user: IUserStore;
}

export const rootReducer = combineReducers({
  user: userReducer,
});
