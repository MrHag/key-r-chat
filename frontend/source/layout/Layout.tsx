import React from "react";

import { PrivateRoute } from "components/private-route";
import { Header } from "components/header";
import { Route, Switch } from "react-router-dom";

import { Messages } from "views/messages";
import { MyProfile } from "views/my-profile";
import { RootView } from "views/root-view";
import { SignIn } from "views/sign-in";
import { SignUp } from "views/sign-up";
import {
  MESSAGES,
  MY_PROFILE,
  SETTINGS,
  SIGN_IN,
  SIGN_UP,
} from "constants/routes";
import { Settings } from "views/settings";

const Layout: React.FC = () => {
  return (
    <>
      <Header />
      <Switch>
        <Route path={SIGN_IN} component={SignIn} />
        <Route path={SIGN_UP} component={SignUp} />

        <PrivateRoute path={SETTINGS} component={Settings} />
        <PrivateRoute path={MY_PROFILE} component={MyProfile} />
        <PrivateRoute path={MESSAGES} component={Messages} />
        <PrivateRoute path="/" component={RootView} />
      </Switch>
    </>
  );
};

export { Layout };
