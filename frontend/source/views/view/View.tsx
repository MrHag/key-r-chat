import { Paper } from "@material-ui/core";
import { withStyles } from "@material-ui/core";

const View = withStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    width: "100%",
    height: "100%",
    padding: "0 8px 0 8px",
    borderRadius: 0,
  },
})(Paper);

export { View };
