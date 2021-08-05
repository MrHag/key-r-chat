import { connect, ConnectedProps } from "react-redux";
import { actionSignOut } from "store/auth/actions";
import { Header } from "./Header";

const mapDispatch = {
  actionSignOut,
};

const connector = connect(null, mapDispatch);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(Header);

export { connected as Header };
