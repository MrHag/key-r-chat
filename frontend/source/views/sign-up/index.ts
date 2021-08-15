import { connect, ConnectedProps } from "react-redux";
import { actionSignUp } from "store/auth/actions";
import { SignUp } from "./SignUp";
import { IAppStore } from "store";

const mapState = (state: IAppStore) => ({
  isAuthorized: state.auth.isAuthorized,
  signUp: state.auth.signUp,
});

const mapDispatch = {
  actionSignUp,
};

const connector = connect(mapState, mapDispatch);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(SignUp);

export { connected as SignUp };
