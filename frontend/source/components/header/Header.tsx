import React from "react";
import { makeStyles, withStyles } from "@material-ui/core";
import { AppBar, Toolbar, Drawer, ListItem, List } from "@material-ui/core";
import MenuIcon from "@material-ui/icons/Menu";
import { IconButton, MenuItem, Menu } from "@material-ui/core";
import { Select } from "@material-ui/core";
import { useState } from "react";
import { Link as RouterLink, NavLink } from "react-router-dom";
import { themeNames } from "themes";
import { MY_PROFILE, MESSAGES, SETTINGS } from "constants/routes";
import { PropsFromConnector } from ".";
import { Link } from "@material-ui/core";

const useStyles = makeStyles(() => ({
  menu: {
    maxWidth: "356px",
    width: "100%",
  },
  navLink: {
    textDecoration: "none",
    color: "unset",
    padding: "8px 16px",
    "&.active": {
      color: "red",
      pointerEvents: "none",
    },
  },
  logo: {
    color: "white",
    flexGrow: 1,
  },
  themeSelector: {
    marginLeft: "16px",
    color: "white",
  },
  logoutLink: {
    marginTop: "32px",
  },
}));

const StyledListItem = withStyles({
  root: {
    padding: 0,
  },
})(MenuItem);

const StyledDrawer = withStyles({
  paper: {
    maxWidth: "50vw",
    paddingRight: "32px",
  },
})(Drawer);

/* TODO: Maybe you should place profile link on the right side of app bar (on user login) */

const Header: React.FC<PropsFromConnector> = ({
  themeName,
  isAuthorized,
  login,
  actionSignOut,
  actionSetTheme,
}: PropsFromConnector) => {
  const classes = useStyles();
  const [anchorEl, setAnchorEl] = useState(null);

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleOnMenuClick = (e: React.MouseEvent) => {
    setAnchorEl(e.currentTarget);
  };

  const onLogoutClick = () => {
    actionSignOut();
  };

  const onThemeSelectChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    actionSetTheme(e.target.value);
  };

  const listLinks = [
    ["Messages", MESSAGES],
    ["Settings", SETTINGS],
    ["Profile", MY_PROFILE],
  ].map((link, index) => (
    <StyledListItem onClick={handleClose} key={index}>
      <Link component={NavLink} to={link[1]} className={classes.navLink}>
        {link[0]}
      </Link>
    </StyledListItem>
  ));

  return (
    <AppBar position="static">
      <Toolbar>
        {isAuthorized && (
          <IconButton
            edge="start"
            color="inherit"
            aria-label="menu"
            onClick={handleOnMenuClick}
          >
            <MenuIcon />
          </IconButton>
        )}

        <Link
          component={RouterLink}
          to="/"
          className={`${classes.navLink} ${classes.logo}`}
        >
          Chat
        </Link>

        <StyledDrawer
          className={classes.menu}
          id="simple-menu"
          anchor="left"
          open={Boolean(anchorEl)}
          onClose={handleClose}
        >
          <List>
            {listLinks}

            <StyledListItem className={classes.logoutLink}>
              <Link onClick={onLogoutClick} className={classes.navLink}>
                Logout
              </Link>
            </StyledListItem>
          </List>
        </StyledDrawer>

        {isAuthorized && (
          <Link
            component={RouterLink}
            to={MY_PROFILE}
            className={classes.navLink}
          >
            {login}
          </Link>
        )}

        <Select
          className={classes.themeSelector}
          color="primary"
          labelId="theme-select"
          id="theme-select"
          value={themeName}
          label="Theme"
          onChange={onThemeSelectChange}
        >
          {themeNames.map((theme) => (
            <MenuItem value={theme} key={theme}>
              {theme}
            </MenuItem>
          ))}
        </Select>
      </Toolbar>
    </AppBar>
  );
};

export { Header };
