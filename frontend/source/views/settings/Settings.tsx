import React, { useState } from "react";
import { makeStyles } from "@material-ui/styles";
import { Typography } from "@material-ui/core";
import Select from "@material-ui/core/Select";
import { InputLabel, MenuItem } from "@material-ui/core";
import { PropsFromConnector } from ".";
import { themeNames } from "themes";

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
          <InputLabel className={classes.label} id="theme-select">
            Theme
          </InputLabel>
          <Select
            fullWidth
            labelId="theme-select"
            id="theme-select"
            value={selectedTheme}
            label="Theme"
            onChange={onThemeSelectChange}
          >
            {themeNames.map((theme) => (
              <MenuItem value={theme} key={theme}>
                {theme}
              </MenuItem>
            ))}
          </Select>
        </div>
      </div>
    </div>
  );
};

export { Settings };
