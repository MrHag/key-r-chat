import React from "react";
import { makeStyles, withStyles } from "@material-ui/core";
import { AppBar, Typography, Toolbar } from "@material-ui/core";
import MenuIcon from "@material-ui/icons/Menu";
import { IconButton, Select, MenuItem, Menu } from "@material-ui/core";
import { useState } from "react";
import { Link, NavLink } from "react-router-dom";
import { themeNames } from "themes";
import { MY_PROFILE, MESSAGES, SETTINGS } from "constants/routes";
import { PropsFromConnector } from ".";

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  login: {
    marginRight: theme.spacing(0),
    maxWidth: "64px",
    whiteSpace: "nowrap",
    textOverflow: "ellipsis",
    overflow: "hidden",
  },
  space: {
    flexGrow: 1,
    width: "100%",
  },
  title: {
    flexGrow: 1,
  },
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
  link: {
    color: "white",
    textDecoration: "none",
    flexGrow: 1,
    "&.active": {
      pointerEvents: "none",
    },
  },
  themeSelect: {
    marginLeft: "16px",
    "&:before": {
      borderColor: "white",
    },
  },
  selectUnderline: {
    color: "white",
    borderColor: "white",
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
        <IconButton
          edge="start"
          color="inherit"
          aria-label="menu"
          onClick={handleOnMenuClick}
        >
          <MenuIcon />
        </IconButton>

        <Typography variant="h6" className={classes.space} title={userLogin}>
          <Link className={classes.link} to="/">
            Chat
          </Link>
        </Typography>

        <StyledMenu
          className={classes.menu}
          id="simple-menu"
          anchorEl={anchorEl}
          keepMounted
          open={Boolean(anchorEl)}
          onClose={handleClose}
        >
          <MenuItem onClick={handleClose}>
            <NavLink className={classes.navLink} to={MESSAGES}>
              <Typography variant="h6">Messages</Typography>
            </NavLink>
          </MenuItem>

          <MenuItem onClick={handleClose}>
            <NavLink className={classes.navLink} to={SETTINGS}>
              <Typography variant="h6">Settings</Typography>
            </NavLink>
          </MenuItem>

          <MenuItem onClick={handleClose}>
            <a className={classes.navLink} onClick={onLogoutClick}>
              <Typography variant="h6">Logout</Typography>
            </a>
          </MenuItem>
        </StyledMenu>

        <Typography variant="h6" className={classes.login} title={userLogin}>
          <NavLink className={classes.link} to={MY_PROFILE} exact>
            {userLogin}
          </NavLink>
        </Typography>

        <Select
          className={classes.themeSelect}
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
