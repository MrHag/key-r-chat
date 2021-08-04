import { Action, ActionCreator, AnyAction, Dispatch } from "redux";
import { ActionType, signIn, signOut } from "./action-creators";
import { ThunkAction } from "redux-thunk";
import { IAppStore } from "../root-reducer";
import { IUserStore } from "./reducer";

const actionSignIn: ActionCreator<
  ThunkAction<Action, IUserStore, unknown, AnyAction>
> =
  () =>
  (dispatch: Dispatch<ActionType>): Action => {
    return dispatch(signIn());
  };

const actionSignOut =
  () =>
  (dispatch: Dispatch<ActionType>): Action => {
    return dispatch(signOut());
  };

export { actionSignIn, actionSignOut };
