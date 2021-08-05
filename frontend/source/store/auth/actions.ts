import { Action, AnyAction, Dispatch } from "redux";
import { ActionType, signIn, signOut } from "./action-creators";
import { ThunkAction } from "redux-thunk";
import { IAuthStore } from "./reducer";

// TODO: Warning! I sort of don't understand how this code works...

const actionSignIn =
  (
    login: string,
    password: string
  ): ThunkAction<void, IAuthStore, unknown, AnyAction> =>
  async (dispatch) => {
    console.warn("Sign in login %s, sign in password %s", login, password);
    return new Promise(() => {
      setTimeout(() => {
        dispatch(signIn());
      }, 0);
    });
  };

const actionSignOut =
  () =>
  (dispatch: Dispatch<ActionType>): Action => {
    return dispatch(signOut());
  };

export { actionSignIn, actionSignOut };
