import React from "react";
import { Typography, Paper } from "@material-ui/core";
import { makeStyles } from "@material-ui/core/styles";

const useStyles = makeStyles(() => ({
  root: {
    width: "100%",
    height: "100%",
  },
}));

const RootView: React.FC = () => {
  const classes = useStyles();

  return (
    <Paper className={classes.root}>
      <Typography variant="h6">Root view</Typography>
    </Paper>
  );
};

export { RootView };
