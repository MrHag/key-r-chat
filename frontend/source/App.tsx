import React from "react";
import { makeStyles } from "@material-ui/core";
import { BrowserRouter, Route, Switch } from "react-router-dom";

import { PrivateRoute } from "components/private-route";
import { Header } from "components/header";

import { Messages } from "views/messages";
import { MyProfile } from "views/my-profile";
import { RootView } from "views/root-view";
import { SignIn } from "views/sign-in";

import { MESSAGES, MY_PROFILE, SIGN_IN } from "constants/routes";

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
          <Route path={SIGN_IN} component={SignIn} />

          <PrivateRoute path={MY_PROFILE} component={MyProfile} />
          <PrivateRoute path={MESSAGES} component={Messages} />
          <PrivateRoute path="/" component={RootView} />
        </Switch>
      </div>
    </BrowserRouter>
  );
};

export default App;
