import { Settings } from "./Settings";
import { actionSetTheme } from "store/settings/actions";

import { connect, ConnectedProps } from "react-redux";
import { IAppStore } from "store";

const mapState = (store: IAppStore) => ({
  themeName: store.settings.themeName,
});

const mapDispatch = {
  actionSetTheme,
};

const connector = connect(mapState, mapDispatch);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(Settings);

export { connected as Settings };
