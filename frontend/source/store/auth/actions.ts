import { Action, ActionCreator, AnyAction, Dispatch } from "redux";
import { ActionType, signIn, signOut } from "./action-creators";
import { ThunkAction } from "redux-thunk";
import { IAuthStore } from "./reducer";

// TODO: Warning! I sort of don't understand how this code works...

const actionSignIn: ActionCreator<
  ThunkAction<Promise<Action>, IAuthStore, unknown, AnyAction>
> = () => async (dispatch: Dispatch<ActionType>) => {
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
