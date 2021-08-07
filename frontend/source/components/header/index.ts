import { connect, ConnectedProps } from "react-redux";
import { actionSignOut } from "store/auth/actions";
import { actionSetTheme } from "store/settings/actions";
import { IAppStore } from "store";
import { Header } from "./Header";

const mapState = (state: IAppStore) => ({
  themeName: state.settings.themeName,
  isAuthorized: state.auth.isAuthorized,
});

const mapDispatch = {
  actionSignOut,
  actionSetTheme,
};

const connector = connect(mapState, mapDispatch);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(Header);

export { connected as Header };
