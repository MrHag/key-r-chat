import React from "react";
import { makeStyles } from "@material-ui/core";
import { Header } from "components/header";

const useStyles = makeStyles({
  app: {
    width: "100%",
    height: "100%",
  },
});

const App: React.FC = () => {
  const classes = useStyles();
  return (
    <div className={classes.app}>
      <Header />
    </div>
  );
};

export default App;
