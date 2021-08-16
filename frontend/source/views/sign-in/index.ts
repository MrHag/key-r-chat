import { connect, ConnectedProps } from "react-redux";
import { actionSignIn } from "store/auth/actions";
import { SignIn } from "./SignIn";
import { IAppStore } from "store";

const mapState = (state: IAppStore) => ({
  isAuthorized: state.auth.isAuthorized,
  signIn: state.auth.signIn,
});

const mapDispatch = {
  actionSignIn,
};

const connector = connect(mapState, mapDispatch);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(SignIn);

export { connected as SignIn };
