import React from "react";
import { AppBar } from "@material-ui/core";
import { makeStyles, withStyles } from "@material-ui/core";
import { Typography } from "@material-ui/core";
import { Toolbar } from "@material-ui/core";
import MenuIcon from "@material-ui/icons/Menu";
import { IconButton } from "@material-ui/core";
import Menu from "@material-ui/core/Menu";
import MenuItem from "@material-ui/core/MenuItem";
import { useState } from "react";
import { Link, NavLink } from "react-router-dom";

import { MY_PROFILE, MESSAGES } from "constants/routes";

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
    color: "black",
    textDecoration: "none",
    "&.active": {
      color: "red",
    },
  },
  link: {
    color: "white",
    textDecoration: "none",
    flexGrow: 1,
  },
}));

const StyledMenu = withStyles({
  paper: {
    maxWidth: "356px",
    width: "100%",
  },
})(Menu);

/* TODO: Maybe you should place profile link on the right side of app bar (on user login) */

const Header: React.FC = () => {
  const [anchorEl, setAnchorEl] = useState(null);
  const userLogin = "User login(name)";

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleOnMenuClick = (e: React.MouseEvent) => {
    setAnchorEl(e.currentTarget);
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
              Messages
            </NavLink>
          </MenuItem>

          <MenuItem onClick={handleClose}>
            <NavLink className={classes.navLink} to={"/logout"}>
              Logout
            </NavLink>
          </MenuItem>
        </StyledMenu>

        <Typography variant="h6" className={classes.login} title={userLogin}>
          <NavLink className={classes.link} to={MY_PROFILE} exact>
            {userLogin}
          </NavLink>
        </Typography>
      </Toolbar>
    </AppBar>
  );
};

export { Header };
