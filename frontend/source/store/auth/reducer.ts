import { ActionType, SIGN_IN, SIGN_OUT } from "./action-creators";

export interface IAuthStore {
  isAuthorized: boolean;
}

const UserDefaultState: IAuthStore = {
  isAuthorized: false,
};

const reducer = (state = UserDefaultState, action: ActionType): IAuthStore => {
  switch (action.type) {
    case SIGN_IN: {
      return {
        ...state,
        isAuthorized: true,
      };
    }
    case SIGN_OUT: {
      return {
        ...state,
        isAuthorized: false,
      };
    }
    default:
      return state;
  }
};

export { reducer };
