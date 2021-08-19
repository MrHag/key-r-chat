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

let InitialState: IAuthStore =
  !process.env.NODE_ENV || process.env.NODE_ENV === "development"
    ? {
        isAuthorized: true,
        login: "TestUser",
      }
    : {
        isAuthorized: false,
      };

const LOCAL_STORAGE_ITEM_NAME = "auth";

if (localStorage.getItem(LOCAL_STORAGE_ITEM_NAME)) {
  const inStorage = JSON.parse(localStorage.getItem(LOCAL_STORAGE_ITEM_NAME));
  console.log("inStorage = ", inStorage);
  InitialState = inStorage;
}

const reducer = (state = InitialState, action: ActionType): IAuthStore => {
  switch (action.type) {
    case SIGN_IN_SUCCESS: {
      const a = action as ISignInSuccess;

      const authState = {
        ...state,
        isAuthorized: true,
        login: a.login,
        token: a.token,
      };

      localStorage.setItem(LOCAL_STORAGE_ITEM_NAME, JSON.stringify(authState));

      return authState;
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
      localStorage.removeItem(LOCAL_STORAGE_ITEM_NAME);
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
