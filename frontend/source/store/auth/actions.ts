import { Action, AnyAction, Dispatch } from "redux";
import {
  ActionType,
  signInSuccess,
  signInFail,
  signOut,
  signUpFail,
  signUpSuccess,
} from "./action-creators";
import { ThunkAction } from "redux-thunk";
import { IAuthStore } from "./reducer";
import { httpService } from "common/http-service";

// TODO: Warning! I sort of don't understand how this code works...

const actionSignIn =
  (
    login: string,
    password: string
  ): ThunkAction<void, IAuthStore, unknown, AnyAction> =>
  async (dispatch) => {
    console.warn("Sign in login %s, sign in password %s", login, password);
    try {
      const result = await httpService.post({
        url: `login?login=${login}&password=${password}`,
      });
      dispatch(signInSuccess(login, result.data.token));
    } catch (e) {
      dispatch(signInFail(e.toString()));
    }
  };

const actionSignOut =
  () =>
  (dispatch: Dispatch<ActionType>): Action => {
    return dispatch(signOut());
  };

const actionSignUp =
  (
    login: string,
    password: string
  ): ThunkAction<void, IAuthStore, unknown, AnyAction> =>
  async (dispatch) => {
    return new Promise(async () => {
      try {
        const result = await httpService.post({
          url: `registration?login=${login}&password=${password}`,
        });
        dispatch(signUpSuccess(result.data.token, login));
      } catch (e) {
        dispatch(signUpFail(e.toString()));
      }
    });
  };

export { actionSignIn, actionSignOut, actionSignUp };
