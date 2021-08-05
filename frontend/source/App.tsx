import React from "react";
import { ThemeProvider, makeStyles } from "@material-ui/styles";
import { createTheme } from "@material-ui/core/styles";
import { BrowserRouter, Route, Switch } from "react-router-dom";

import { PrivateRoute } from "components/private-route";
import { Header } from "components/header";

import { Messages } from "views/messages";
import { MyProfile } from "views/my-profile";
import { RootView } from "views/root-view";
import { SignIn } from "views/sign-in";
import { MESSAGES, MY_PROFILE, SETTINGS, SIGN_IN } from "constants/routes";
import { Settings } from "views/settings";

const useStyles = makeStyles((theme) => {
  console.log("theme = ", theme);
  return {
    app: {
      width: "100%",
      height: "100%",
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
    },
  };
});

const defaultTheme = createTheme({});

const App: React.FC = () => {
  const classes = useStyles();

  return (
    <ThemeProvider theme={defaultTheme}>
      <BrowserRouter>
        <div className={classes.app}>
          <Header />
          <Switch>
            <Route path={SIGN_IN} component={SignIn} />

            <PrivateRoute path={SETTINGS} component={Settings} />
            <PrivateRoute path={MY_PROFILE} component={MyProfile} />
            <PrivateRoute path={MESSAGES} component={Messages} />
            <PrivateRoute path="/" component={RootView} />
          </Switch>
        </div>
      </BrowserRouter>
    </ThemeProvider>
  );
};

export default App;
