import { PrivateRoute } from "./PrivateRoute";
import { connect, ConnectedProps } from "react-redux";
import { IAppStore } from "store";

const mapState = (state: IAppStore) => ({
  isAuthorized: state.auth.isAuthorized,
});

const connector = connect(mapState);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(PrivateRoute);

export { connected as PrivateRoute };
