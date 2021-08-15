import React from "react";
import { makeStyles, withStyles } from "@material-ui/core";
import { AppBar, Toolbar } from "@material-ui/core";
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
}));

const StyledMenu = withStyles({
  paper: {
    maxWidth: "356px",
    width: "100%",
  },
})(Menu);

/* TODO: Maybe you should place profile link on the right side of app bar (on user login) */

const Header: React.FC<PropsFromConnector> = ({
  themeName,
  isAuthorized,
  actionSignOut,
  actionSetTheme,
}: PropsFromConnector) => {
  const [anchorEl, setAnchorEl] = useState(null);
  const userLogin = "User login(name)";

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

  const classes = useStyles();
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

        <StyledMenu
          className={classes.menu}
          id="simple-menu"
          anchorEl={anchorEl}
          keepMounted
          open={Boolean(anchorEl)}
          onClose={handleClose}
        >
          <MenuItem onClick={handleClose}>
            <Link component={NavLink} to={MESSAGES} className={classes.navLink}>
              Messages
            </Link>
          </MenuItem>

          <MenuItem onClick={handleClose}>
            <Link component={NavLink} to={SETTINGS} className={classes.navLink}>
              Settings
            </Link>
          </MenuItem>

          <MenuItem onClick={handleClose}>
            <Link onClick={onLogoutClick} className={classes.navLink}>
              Logout
            </Link>
          </MenuItem>
        </StyledMenu>

        {isAuthorized && (
          <Link
            component={RouterLink}
            to={MY_PROFILE}
            className={classes.navLink}
          >
            {userLogin}
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
