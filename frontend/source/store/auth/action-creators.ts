export const SIGN_IN = "auth/SIGN_IN";
export const SIGN_OUT = "auth/SIGN_OUT";

export interface ISignIn {
  type: string;
}

export interface ISignOut {
  type: string;
}

type ActionType = ISignIn | ISignOut;

const signIn = (): ISignIn => {
  return {
    type: SIGN_IN,
  };
};

const signOut = (): ISignOut => {
  return {
    type: SIGN_OUT,
  };
};

export { ActionType, signIn, signOut };
