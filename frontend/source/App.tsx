import React from "react";
import { ThemeProvider, makeStyles } from "@material-ui/styles";
import { BrowserRouter } from "react-router-dom";

import { useSelector } from "react-redux";
import { IAppStore } from "store";
import { themes } from "themes";
import { Layout } from "layout";

const useStyles = makeStyles((theme) => {
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

const App: React.FC = () => {
  const classes = useStyles();

  const themeName = useSelector<IAppStore, string>(
    (store) => store.settings.themeName
  );

  return (
    <ThemeProvider theme={themes.get(themeName)}>
      <BrowserRouter>
        <div className={classes.app}>
          <Layout />
        </div>
      </BrowserRouter>
    </ThemeProvider>
  );
};

export default App;
