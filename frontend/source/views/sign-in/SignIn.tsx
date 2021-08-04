import React from "react";
import { Modal } from "@material-ui/core";
import { makeStyles } from "@material-ui/styles";
import { TextField } from "@material-ui/core";
import { Button } from "@material-ui/core";
import { Typography } from "@material-ui/core";
import { actionSignIn } from "store/user/actions";
import { useDispatch } from "react-redux";
const useStyles = makeStyles(() => ({
  paper: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    position: "absolute",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    width: 400,
    backgroundColor: "white",
    borderRadius: "8px",
    outline: "none",
    padding: "16px",
  },
  title: {
    marginBottom: "32px",
  },
  textField: {
    marginBottom: "32px",
  },
}));

const SignIn: React.FC = () => {
  const dispatch = useDispatch();
  const classes = useStyles();

  const onLoginBtnClick = () => {
    dispatch(actionSignIn());
  };

  return (
    <Modal
      open={true}
      aria-labelledby="simple-modal-title"
      aria-describedby="simple-modal-description"
    >
      <div className={classes.paper}>
        <Typography variant="h6" className={classes.title}>
          Sign in
        </Typography>
        <TextField className={classes.textField} placeholder="Login" />
        <TextField className={classes.textField} placeholder="Password" />
        <Button variant="contained" color="secondary" onClick={onLoginBtnClick}>
          Login
        </Button>
      </div>
    </Modal>
  );
};

export { SignIn };
