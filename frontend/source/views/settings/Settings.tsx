import React, { useState } from "react";
import { makeStyles } from "@material-ui/styles";
import { Typography } from "@material-ui/core";
import Select from "@material-ui/core/Select";
import { InputLabel, MenuItem } from "@material-ui/core";
import { PropsFromConnector } from ".";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    width: "100%",
  },
  list: {
    display: "flex",
    flexDirection: "row",
    alignItems: "center",
    width: "100%",
    height: "100%",
  },
  block: {
    display: "flex",
    flexDirection: "column",
    alignItems: "flex-start",
    width: "100%",
    maxWidth: "256px",
  },
  label: {
    marginBottom: "16px",
  },
});

const themeNames = ["light", "dark"];

const Settings: React.FC<PropsFromConnector> = ({
  themeName,
}: PropsFromConnector) => {
  const classes = useStyles();

  const [selectedTheme, setSelectedTheme] = useState(themeName);

  const onThemeSelectChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedTheme(e.target.value);
  };

  return (
    <div className={classes.root}>
      <Typography variant="h6">Settings</Typography>
      <div className={classes.list}>
        <div className={classes.block}>
          <InputLabel className={classes.label} id="demo-simple-select-label">
            Theme
          </InputLabel>
          <Select
            fullWidth
            labelId="demo-simple-select-label"
            id="demo-simple-select"
            value={selectedTheme}
            label="Theme"
            onChange={onThemeSelectChange}
          >
            <MenuItem value={themeNames[0]}>{themeNames[0]}</MenuItem>
            <MenuItem value={themeNames[1]}>{themeNames[1]}</MenuItem>
          </Select>
        </div>
      </div>
    </div>
  );
};

export { Settings };
