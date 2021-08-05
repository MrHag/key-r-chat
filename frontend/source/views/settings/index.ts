import { Settings } from "./Settings";

import { connect, ConnectedProps } from "react-redux";
import { IAppStore } from "store";

const mapState = (store: IAppStore) => ({
  themeName: store.settings.themeName,
});

const connector = connect(mapState, null);

export type PropsFromConnector = ConnectedProps<typeof connector>;

const connected = connector(Settings);

export { connected as Settings };
