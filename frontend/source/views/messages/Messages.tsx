import React from "react";
import { View } from "views/view";
import { Typography, Grid } from "@material-ui/core";
import { makeStyles } from "@material-ui/core/styles";

const useClasses = makeStyles({
  item: {
    flexGrow: 1,
    width: "100%",
  },
});

const Messages: React.FC = () => {
  const classes = useClasses();
  return (
    <View>
      <Grid container>
        <Grid item xs={2}>
          <Typography variant="h6">Messages view</Typography>
        </Grid>
        <Grid item xs={6} className={classes.item}>
          <Typography variant="h6" className={classes.item}>
            Second view
          </Typography>
        </Grid>
      </Grid>
    </View>
  );
};

export { Messages };
