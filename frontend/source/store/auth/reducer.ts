import {
  ActionType,
  SIGN_IN_SUCCESS,
  SIGN_IN_FAIL,
  SIGN_OUT,
  SIGN_UP_SUCCESS,
  SIGN_UP_FAIL,
  ISignUpFail,
  ISignUpSuccess,
  ISignInSuccess,
  ISignInFail,
} from "./action-creators";

export interface IAuthStore {
  isAuthorized: boolean;
  token?: string;
  login?: string;
  signUp?: {
    errorMsg?: string;
  };
  signIn?: {
    errorMsg: string;
  };
}

const DefaultState: IAuthStore = {
  isAuthorized: false,
};

const reducer = (state = DefaultState, action: ActionType): IAuthStore => {
  switch (action.type) {
    case SIGN_IN_SUCCESS: {
      const a = action as ISignInSuccess;
      return {
        ...state,
        isAuthorized: true,
        login: a.login,
        token: a.token,
      };
    }
    case SIGN_IN_FAIL: {
      const a = action as ISignInFail;
      return {
        ...state,
        signIn: {
          errorMsg: a.errorMsg,
        },
      };
    }
    case SIGN_OUT: {
      return DefaultState;
    }
    case SIGN_UP_FAIL: {
      const a = action as ISignUpFail;
      return {
        ...state,
        signUp: {
          errorMsg: a.errorMsg,
        },
      };
    }
    case SIGN_UP_SUCCESS: {
      const a = action as ISignUpSuccess;
      return {
        ...state,
        isAuthorized: true,
        token: a.token,
        login: a.login,
      };
    }
    default:
      return state;
  }
};

export { reducer };
