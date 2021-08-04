import { ActionType, SIGN_IN, SIGN_OUT } from "./action-creators";

export interface IUserStore {
  isAuthorized: boolean;
}

const UserDefaultState: IUserStore = {
  isAuthorized: false,
};

function reducer(state = UserDefaultState, action: ActionType): IUserStore {
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
}

export { reducer };
