import React from "react";
import { Typography } from "@material-ui/core";
import { View } from "views/view";

const RootView: React.FC = () => {
  return (
    <View>
      <Typography variant="h6">Root view</Typography>
    </View>
  );
};

export { RootView };
