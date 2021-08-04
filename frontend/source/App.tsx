import React from "react";
import { makeStyles } from "@material-ui/core";
import { Header } from "components/header";
import { BrowserRouter, Switch, Route } from "react-router-dom";

import { Messages } from "views/messages";
import { MyProfile } from "views/my-profile";
import { RootView } from "views/root-view";

import { MESSAGES, MY_PROFILE } from "constants/routes";

const useStyles = makeStyles({
  app: {
    width: "100%",
    height: "100%",
  },
});

const App: React.FC = () => {
  const classes = useStyles();
  return (
    <BrowserRouter>
      <div className={classes.app}>
        <Header />

        <Switch>
          <Route path={MY_PROFILE} component={MyProfile} />
          <Route path={MESSAGES} component={Messages} />
          <Route path="/" component={RootView} />
        </Switch>
      </div>
    </BrowserRouter>
  );
};

export default App;
