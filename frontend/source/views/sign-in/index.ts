import { connect, ConnectedProps } from "react-redux";
import { actionSignIn } from "store/user/actions";
import { SignIn } from "./SignIn";
import { IAppStore } from "store";

const mapState = (state: IAppStore) => ({
  isAuthorized: state.user.isAuthorized,
});

const mapDispatch = {
  actionSignIn,
};

const connector = connect(mapState, mapDispatch);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(SignIn);

export { connected as SignIn };
