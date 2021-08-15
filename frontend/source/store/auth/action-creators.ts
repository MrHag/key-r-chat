export const SIGN_IN_SUCCESS = "auth/SIGN_IN_SUCCESS";
export const SIGN_IN_FAIL = "auth/SIGN_IN_FAIL";
export const SIGN_OUT = "auth/SIGN_OUT";
export const SIGN_UP_FAIL = "auth/SIGN_UP_FAIL";
export const SIGN_UP_SUCCESS = "auth/SIGN_UP_SUCCESS";

export interface ISignInSuccess {
  type: string;
  login: string;
  token: string;
}

export interface ISignInFail {
  type: string;
  errorMsg: string;
}

export interface ISignOut {
  type: string;
}

const signInFail = (errorMsg: string): ISignInFail => {
  return {
    type: SIGN_IN_FAIL,
    errorMsg,
  };
};

const signInSuccess = (login: string, token: string): ISignInSuccess => {
  return {
    type: SIGN_IN_SUCCESS,
    login,
    token,
  };
};

const signOut = (): ISignOut => {
  return {
    type: SIGN_OUT,
  };
};

export interface ISignUpFail {
  type: string;
  errorMsg: string;
}

const signUpFail = (errorMsg: string): ISignUpFail => {
  return {
    type: SIGN_UP_FAIL,
    errorMsg,
  };
};

export interface ISignUpSuccess {
  type: string;
  token: string;
  login: string;
}

const signUpSuccess = (token: string, login: string): ISignUpSuccess => {
  return {
    type: SIGN_UP_SUCCESS,
    token,
    login,
  };
};

type ActionType = ISignInSuccess | ISignOut | ISignUpFail | ISignInFail;

export {
  ActionType,
  signInSuccess,
  signOut,
  signUpFail,
  signUpSuccess,
  signInFail,
};
